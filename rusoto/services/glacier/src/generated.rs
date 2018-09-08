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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Provides options to abort a multipart upload identified by the upload ID.</p> <p>For information about the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a>. For conceptual information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AbortMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The upload ID of the multipart upload to delete.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>The input values for <code>AbortVaultLock</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AbortVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>The input values for <code>AddTagsToVault</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToVaultInput {
    /// <p>The tags to add to the vault. Each tag is composed of a key and a value. The value can be an empty string.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p> <p>For information about the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a>. For conceptual information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ArchiveCreationOutput {
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    #[serde(rename = "archiveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// <p>The checksum of the archive computed by Amazon Glacier.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The relative URI path of the newly added archive resource.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// <p>Contains information about the comma-separated value (CSV) file to select from.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSVInput {
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row.</p>
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// <p>A value used to separate individual fields from each other within a record.</p>
    #[serde(rename = "FieldDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    /// <p>Describes the first line of input. Valid values are <code>None</code>, <code>Ignore</code>, and <code>Use</code>.</p>
    #[serde(rename = "FileHeaderInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_header_info: Option<String>,
    /// <p>A value used as an escape character where the field delimiter is part of the value.</p>
    #[serde(rename = "QuoteCharacter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    /// <p>A single character used for escaping the quotation-mark character inside an already escaped value.</p>
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    /// <p>A value used to separate individual records from each other.</p>
    #[serde(rename = "RecordDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

/// <p>Contains information about the comma-separated value (CSV) file that the job results are stored in.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSVOutput {
    /// <p>A value used to separate individual fields from each other within a record.</p>
    #[serde(rename = "FieldDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    /// <p>A value used as an escape character where the field delimiter is part of the value.</p>
    #[serde(rename = "QuoteCharacter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    /// <p>A single character used for escaping the quotation-mark character inside an already escaped value.</p>
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    /// <p>A value that indicates whether all output fields should be contained within quotation marks.</p>
    #[serde(rename = "QuoteFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_fields: Option<String>,
    /// <p>A value used to separate individual records from each other.</p>
    #[serde(rename = "RecordDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

/// <p>Provides options to complete a multipart upload operation. This informs Amazon Glacier that all the archive parts have been uploaded and Amazon Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Amazon Glacier returns the URI path of the newly created archive resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CompleteMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The total size, in bytes, of the entire archive. This value should be the sum of all the sizes of the individual parts that you uploaded.</p>
    #[serde(rename = "archiveSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size: Option<String>,
    /// <p>The SHA256 tree hash of the entire archive. It is the tree hash of SHA256 tree hash of the individual parts. If the value you specify in the request does not match the SHA256 tree hash of the final assembled archive as computed by Amazon Glacier, Amazon Glacier returns an error and the request fails.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The upload ID of the multipart upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>The input values for <code>CompleteVaultLock</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CompleteVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The <code>lockId</code> value is the lock ID obtained from a <a>InitiateVaultLock</a> request.</p>
    #[serde(rename = "lockId")]
    pub lock_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options to create a vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateVaultOutput {
    /// <p>The URI of the vault that was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// <p>Data retrieval policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataRetrievalPolicy {
    /// <p>The policy rule. Although this is a list type, currently there must be only one rule, which contains a Strategy field and optionally a BytesPerHour field.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DataRetrievalRule>>,
}

/// <p>Data retrieval policy rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataRetrievalRule {
    /// <p>The maximum number of bytes that can be retrieved in an hour.</p> <p>This field is required only if the value of the Strategy field is <code>BytesPerHour</code>. Your PUT operation will be rejected if the Strategy field is not set to <code>BytesPerHour</code> and you set this field.</p>
    #[serde(rename = "BytesPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_per_hour: Option<i64>,
    /// <p>The type of data retrieval policy to set.</p> <p>Valid values: BytesPerHour|FreeTier|None</p>
    #[serde(rename = "Strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

/// <p>Provides options for deleting an archive from an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteArchiveInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The ID of the archive to delete.</p>
    #[serde(rename = "archiveId")]
    pub archive_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>DeleteVaultAccessPolicy input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for deleting a vault from Amazon Glacier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for deleting a vault notification configuration from an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for retrieving a job description.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The ID of the job to describe.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for retrieving metadata for a specific vault in Amazon Glacier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeVaultOutput {
    /// <p>The Universal Coordinated Time (UTC) date when the vault was created. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The Universal Coordinated Time (UTC) date when Amazon Glacier completed the last vault inventory. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
    #[serde(rename = "LastInventoryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_inventory_date: Option<String>,
    /// <p>The number of archives in the vault as of the last inventory date. This field will return <code>null</code> if an inventory has not yet run on the vault, for example if you just created the vault.</p>
    #[serde(rename = "NumberOfArchives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_archives: Option<i64>,
    /// <p>Total size, in bytes, of the archives in the vault as of the last inventory date. This field will return null if an inventory has not yet run on the vault, for example if you just created the vault.</p>
    #[serde(rename = "SizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the vault.</p>
    #[serde(rename = "VaultARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_arn: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "VaultName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
}

/// <p>Contains information about the encryption used to store the job results in Amazon S3. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3, for example <code>AES256</code> or <code>aws:kms</code>.</p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Optional. If the encryption type is <code>aws:kms</code>, you can use this value to specify the encryption context for the job results.</p>
    #[serde(rename = "KMSContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_context: Option<String>,
    /// <p>The AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS fail if not made by using Secure Sockets Layer (SSL) or Signature Version 4. </p>
    #[serde(rename = "KMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p>Input for GetDataRetrievalPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataRetrievalPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

/// <p>Contains the Amazon Glacier response to the <code>GetDataRetrievalPolicy</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDataRetrievalPolicyOutput {
    /// <p>Contains the returned data retrieval policy in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<DataRetrievalPolicy>,
}

/// <p>Provides options for downloading output of an Amazon Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobOutputInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The job ID whose data is downloaded.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p><p>The range of bytes to retrieve from the output. For example, if you want to download the first 1,048,576 bytes, specify the range as <code>bytes=0-1048575</code>. By default, this operation downloads the entire output.</p> <p>If the job output is large, then you can use a range to retrieve a portion of the output. This allows you to download the entire output in smaller chunks of bytes. For example, suppose you have 1 GB of job output you want to download and you decide to download 128 MB chunks of data at a time, which is a total of eight Get Job Output requests. You use the following process to download the job output:</p> <ol> <li> <p>Download a 128 MB chunk of output by specifying the appropriate byte range. Verify that all 128 MB of data was received.</p> </li> <li> <p>Along with the data, the response includes a SHA256 tree hash of the payload. You compute the checksum of the payload on the client and compare it with the checksum you received in the response to ensure you received all the expected data.</p> </li> <li> <p>Repeat steps 1 and 2 for all the eight 128 MB chunks of output data, each time specifying the appropriate byte range.</p> </li> <li> <p>After downloading all the parts of the job output, you have a list of eight checksum values. Compute the tree hash of these values to find the checksum of the entire output. Using the <a>DescribeJob</a> API, obtain job information of the job that provided you the output. The response includes the checksum of the entire archive stored in Amazon Glacier. You compare this value with the checksum you computed to ensure you have downloaded the entire archive content with no errors.</p> <p/> </li> </ol></p>
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetJobOutputOutput {
    /// <p>Indicates the range units accepted. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html">RFC2616</a>. </p>
    pub accept_ranges: Option<String>,
    /// <p>The description of an archive.</p>
    pub archive_description: Option<String>,
    /// <p>The job data, either archive data or inventory data.</p>
    pub body: Option<Vec<u8>>,
    /// <p><p>The checksum of the data in the response. This header is returned only when retrieving the output for an archive retrieval job. Furthermore, this header appears only under the following conditions:</p> <ul> <li> <p>You get the entire range of the archive.</p> </li> <li> <p>You request a range to return of the archive that starts and ends on a multiple of 1 MB. For example, if you have an 3.1 MB archive and you specify a range to return that starts at 1 MB and ends at 2 MB, then the x-amz-sha256-tree-hash is returned as a response header.</p> </li> <li> <p>You request a range of the archive to return that starts on a multiple of 1 MB and goes to the end of the archive. For example, if you have a 3.1 MB archive and you specify a range that starts at 2 MB and ends at 3.1 MB (the end of the archive), then the x-amz-sha256-tree-hash is returned as a response header.</p> </li> </ul></p>
    pub checksum: Option<String>,
    /// <p>The range of bytes returned by Amazon Glacier. If only partial output is downloaded, the response provides the range of bytes Amazon Glacier returned. For example, bytes 0-1048575/8388608 returns the first 1 MB from 8 MB.</p>
    pub content_range: Option<String>,
    /// <p>The Content-Type depends on whether the job output is an archive or a vault inventory. For archive data, the Content-Type is application/octet-stream. For vault inventory, if you requested CSV format when you initiated the job, the Content-Type is text/csv. Otherwise, by default, vault inventory is returned as JSON, and the Content-Type is application/json.</p>
    pub content_type: Option<String>,
    /// <p>The HTTP response code for a job output request. The value depends on whether a range was specified in the request.</p>
    pub status: Option<i64>,
}

/// <p>Input for GetVaultAccessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Output for GetVaultAccessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetVaultAccessPolicyOutput {
    /// <p>Contains the returned vault access policy as a JSON string.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultAccessPolicy>,
}

/// <p>The input values for <code>GetVaultLock</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetVaultLockOutput {
    /// <p>The UTC date and time at which the vault lock was put into the <code>InProgress</code> state.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The UTC date and time at which the lock ID expires. This value can be <code>null</code> if the vault lock is in a <code>Locked</code> state.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The state of the vault lock. <code>InProgress</code> or <code>Locked</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Provides options for retrieving the notification configuration set on an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetVaultNotificationsOutput {
    /// <p>Returns the notification configuration set on the vault.</p>
    #[serde(rename = "vaultNotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_notification_config: Option<VaultNotificationConfig>,
}

/// <p>Contains the description of an Amazon Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GlacierJobDescription {
    /// <p>The job type. This value is either <code>ArchiveRetrieval</code>, <code>InventoryRetrieval</code>, or <code>Select</code>. </p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The archive ID requested for a select job or archive retrieval. Otherwise, this field is null.</p>
    #[serde(rename = "ArchiveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// <p>The SHA256 tree hash of the entire archive for an archive retrieval. For inventory retrieval or select jobs, this field is null.</p>
    #[serde(rename = "ArchiveSHA256TreeHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_sha256_tree_hash: Option<String>,
    /// <p>For an archive retrieval job, this value is the size in bytes of the archive being requested for download. For an inventory retrieval or select job, this value is null.</p>
    #[serde(rename = "ArchiveSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size_in_bytes: Option<i64>,
    /// <p>The job status. When a job is completed, you get the job's output using Get Job Output (GET output).</p>
    #[serde(rename = "Completed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// <p>The UTC time that the job request completed. While the job is in progress, the value is null.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The UTC date when the job was created. This value is a string representation of ISO 8601 date format, for example <code>"2012-03-20T17:03:43.221Z"</code>.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Parameters used for range inventory retrieval.</p>
    #[serde(rename = "InventoryRetrievalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_retrieval_parameters: Option<InventoryRetrievalJobDescription>,
    /// <p>For an inventory retrieval job, this value is the size in bytes of the inventory requested for download. For an archive retrieval or select job, this value is null.</p>
    #[serde(rename = "InventorySizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size_in_bytes: Option<i64>,
    /// <p>The job description provided when initiating the job.</p>
    #[serde(rename = "JobDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    /// <p>An opaque string that identifies an Amazon Glacier job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Contains the job output location.</p>
    #[serde(rename = "JobOutputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output_path: Option<String>,
    /// <p>Contains the location where the data from the select job is stored.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>The retrieved byte range for archive retrieval jobs in the form <i>StartByteValue</i>-<i>EndByteValue</i>. If no range was specified in the archive retrieval, then the whole archive is retrieved. In this case, <i>StartByteValue</i> equals 0 and <i>EndByteValue</i> equals the size of the archive minus 1. For inventory retrieval or select jobs, this field is null. </p>
    #[serde(rename = "RetrievalByteRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_byte_range: Option<String>,
    /// <p><p>For an archive retrieval job, this value is the checksum of the archive. Otherwise, this value is null.</p> <p>The SHA256 tree hash value for the requested range of an archive. If the <b>InitiateJob</b> request for an archive specified a tree-hash aligned range, then this field returns a value.</p> <p>If the whole archive is retrieved, this value is the same as the ArchiveSHA256TreeHash value.</p> <p>This field is null for the following:</p> <ul> <li> <p>Archive retrieval jobs that specify a range that is not tree-hash aligned</p> </li> </ul> <ul> <li> <p>Archival jobs that specify a range that is equal to the whole archive, when the job status is <code>InProgress</code> </p> </li> </ul> <ul> <li> <p>Inventory jobs</p> </li> <li> <p>Select jobs</p> </li> </ul></p>
    #[serde(rename = "SHA256TreeHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_tree_hash: Option<String>,
    /// <p>An Amazon SNS topic that receives notification.</p>
    #[serde(rename = "SNSTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
    /// <p>Contains the parameters used for a select.</p>
    #[serde(rename = "SelectParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_parameters: Option<SelectParameters>,
    /// <p>The status code can be <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code>, and indicates the status of the job.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// <p>A friendly message that describes the job status.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The tier to use for a select or an archive retrieval. Valid values are <code>Expedited</code>, <code>Standard</code>, or <code>Bulk</code>. <code>Standard</code> is the default.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the vault from which an archive retrieval was requested.</p>
    #[serde(rename = "VaultARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_arn: Option<String>,
}

/// <p>Contains information about a grant.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grant {
    /// <p>The grantee.</p>
    #[serde(rename = "Grantee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    /// <p>Specifies the permission given to the grantee. </p>
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

/// <p>Contains information about the grantee.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grantee {
    /// <p>Screen name of the grantee.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>Email address of the grantee.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The canonical user ID of the grantee.</p>
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Type of grantee</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>URI of the grantee group.</p>
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Provides options for initiating an Amazon Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitiateJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>Provides options for specifying job information.</p>
    #[serde(rename = "jobParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<JobParameters>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InitiateJobOutput {
    /// <p>The ID of the job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The path to the location of where the select results are stored.</p>
    #[serde(rename = "jobOutputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output_path: Option<String>,
    /// <p>The relative URI path of the job.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// <p>Provides options for initiating a multipart upload to an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitiateMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The archive description that you are uploading in parts.</p> <p>The part size must be a megabyte (1024 KB) multiplied by a power of 2, for example 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB (4096 MB).</p>
    #[serde(rename = "archiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    /// <p>The size of each part except the last, in bytes. The last part can be smaller than this part size.</p>
    #[serde(rename = "partSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>The Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InitiateMultipartUploadOutput {
    /// <p>The relative URI path of the multipart upload ID Amazon Glacier created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The ID of the multipart upload. This value is also included as part of the location.</p>
    #[serde(rename = "uploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

/// <p>The input values for <code>InitiateVaultLock</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitiateVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultLockPolicy>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InitiateVaultLockOutput {
    /// <p>The lock ID, which is used to complete the vault locking process.</p>
    #[serde(rename = "lockId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_id: Option<String>,
}

/// <p>Describes how the archive is serialized.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputSerialization {
    /// <p>Describes the serialization of a CSV-encoded object.</p>
    #[serde(rename = "csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CSVInput>,
}

/// <p>Describes the options for a range inventory retrieval job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InventoryRetrievalJobDescription {
    /// <p>The end of the date range in UTC for vault inventory retrieval that includes archives created before this date. This value should be a string in the ISO 8601 date format, for example <code>2013-03-20T17:03:43Z</code>.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The output format for the vault inventory list, which is set by the <b>InitiateJob</b> request when initiating a job to retrieve a vault inventory. Valid values are <code>CSV</code> and <code>JSON</code>.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The maximum number of inventory items returned per vault inventory retrieval request. This limit is set when initiating the job with the a <b>InitiateJob</b> request. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>An opaque string that represents where to continue pagination of the vault inventory retrieval results. You use the marker in a new <b>InitiateJob</b> request to obtain additional inventory items. If there are no more inventory items, this value is <code>null</code>. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html#api-initiate-job-post-vault-inventory-list-filtering"> Range Inventory Retrieval</a>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The start of the date range in Universal Coordinated Time (UTC) for vault inventory retrieval that includes archives created on or after this date. This value should be a string in the ISO 8601 date format, for example <code>2013-03-20T17:03:43Z</code>.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

/// <p>Provides options for specifying a range inventory retrieval job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryRetrievalJobInput {
    /// <p>The end of the date range in UTC for vault inventory retrieval that includes archives created before this date. This value should be a string in the ISO 8601 date format, for example <code>2013-03-20T17:03:43Z</code>.</p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>Specifies the maximum number of inventory items returned per vault inventory retrieval request. Valid values are greater than or equal to 1.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>An opaque string that represents where to continue pagination of the vault inventory retrieval results. You use the marker in a new <b>InitiateJob</b> request to obtain additional inventory items. If there are no more inventory items, this value is <code>null</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The start of the date range in UTC for vault inventory retrieval that includes archives created on or after this date. This value should be a string in the ISO 8601 date format, for example <code>2013-03-20T17:03:43Z</code>.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

/// <p>Provides options for defining a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct JobParameters {
    /// <p>The ID of the archive that you want to retrieve. This field is required only if <code>Type</code> is set to <code>select</code> or <code>archive-retrieval</code>code&gt;. An error occurs if you specify this request parameter for an inventory retrieval job request. </p>
    #[serde(rename = "ArchiveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// <p>The optional description for the job. The description must be less than or equal to 1,024 bytes. The allowable characters are 7-bit ASCII without control codes-specifically, ASCII values 32-126 decimal or 0x20-0x7E hexadecimal.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When initiating a job to retrieve a vault inventory, you can optionally add this parameter to your request to specify the output format. If you are initiating an inventory job and do not specify a Format field, JSON is the default format. Valid values are "CSV" and "JSON".</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Input parameters used for range inventory retrieval.</p>
    #[serde(rename = "InventoryRetrievalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_retrieval_parameters: Option<InventoryRetrievalJobInput>,
    /// <p>Contains information about the location where the select job results are stored.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    /// <p>The byte range to retrieve for an archive retrieval. in the form "<i>StartByteValue</i>-<i>EndByteValue</i>" If not specified, the whole archive is retrieved. If specified, the byte range must be megabyte (1024*1024) aligned which means that <i>StartByteValue</i> must be divisible by 1 MB and <i>EndByteValue</i> plus 1 must be divisible by 1 MB or be the end of the archive specified as the archive byte size value minus 1. If RetrievalByteRange is not megabyte aligned, this operation returns a 400 response. </p> <p>An error occurs if you specify this field for an inventory retrieval job request.</p>
    #[serde(rename = "RetrievalByteRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_byte_range: Option<String>,
    /// <p>The Amazon SNS topic ARN to which Amazon Glacier sends a notification when the job is completed and the output is ready for you to download. The specified topic publishes the notification to its subscribers. The SNS topic must exist.</p>
    #[serde(rename = "SNSTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
    /// <p>Contains the parameters that define a job.</p>
    #[serde(rename = "SelectParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_parameters: Option<SelectParameters>,
    /// <p>The tier to use for a select or an archive retrieval job. Valid values are <code>Expedited</code>, <code>Standard</code>, or <code>Bulk</code>. <code>Standard</code> is the default.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p>The job type. You can initiate a job to perform a select query on an archive, retrieve an archive, or get an inventory of a vault. Valid values are "select", "archive-retrieval" and "inventory-retrieval".</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides options for retrieving a job list for an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The state of the jobs to return. You can specify <code>true</code> or <code>false</code>.</p>
    #[serde(rename = "completed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// <p>The maximum number of jobs to be returned. The default limit is 50. The number of jobs returned might be fewer than the specified limit, but the number of returned jobs never exceeds the limit.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>An opaque string used for pagination. This value specifies the job at which the listing of jobs should begin. Get the marker value from a previous List Jobs response. You only need to include the marker if you are continuing the pagination of results started in a previous List Jobs request.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The type of job status to return. You can specify the following values: <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code>.</p>
    #[serde(rename = "statuscode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuscode: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListJobsOutput {
    /// <p>A list of job objects. Each job object contains metadata describing the job.</p>
    #[serde(rename = "JobList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list: Option<Vec<GlacierJobDescription>>,
    /// <p> An opaque string used for pagination that specifies the job at which the listing of jobs should begin. You get the <code>marker</code> value from a previous List Jobs response. You only need to include the marker if you are continuing the pagination of the results started in a previous List Jobs request. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Provides options for retrieving list of in-progress multipart uploads for an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMultipartUploadsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>Specifies the maximum number of uploads returned in the response body. If this value is not specified, the List Uploads operation returns up to 50 uploads.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>An opaque string used for pagination. This value specifies the upload at which the listing of uploads should begin. Get the marker value from a previous List Uploads response. You need only include the marker if you are continuing the pagination of results started in a previous List Uploads request.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListMultipartUploadsOutput {
    /// <p>An opaque string that represents where to continue pagination of the results. You use the marker in a new List Multipart Uploads request to obtain more uploads in the list. If there are no more uploads, this value is <code>null</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>A list of in-progress multipart uploads.</p>
    #[serde(rename = "UploadsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads_list: Option<Vec<UploadListElement>>,
}

/// <p>Provides options for retrieving a list of parts of an archive that have been uploaded in a specific multipart upload.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPartsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The maximum number of parts to be returned. The default limit is 50. The number of parts returned might be fewer than the specified limit, but the number of returned parts never exceeds the limit.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>An opaque string used for pagination. This value specifies the part at which the listing of parts should begin. Get the marker value from the response of a previous List Parts response. You need only include the marker if you are continuing the pagination of results started in a previous List Parts request.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The upload ID of the multipart upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPartsOutput {
    /// <p>The description of the archive that was specified in the Initiate Multipart Upload request.</p>
    #[serde(rename = "ArchiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    /// <p>The UTC time at which the multipart upload was initiated.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>An opaque string that represents where to continue pagination of the results. You use the marker in a new List Parts request to obtain more jobs in the list. If there are no more parts, this value is <code>null</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the upload to which the parts are associated.</p>
    #[serde(rename = "MultipartUploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    /// <p>The part size in bytes. This is the same value that you specified in the Initiate Multipart Upload request.</p>
    #[serde(rename = "PartSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size_in_bytes: Option<i64>,
    /// <p>A list of the part sizes of the multipart upload. Each object in the array contains a <code>RangeBytes</code> and <code>sha256-tree-hash</code> name/value pair.</p>
    #[serde(rename = "Parts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<PartListElement>>,
    /// <p>The Amazon Resource Name (ARN) of the vault to which the multipart upload was initiated.</p>
    #[serde(rename = "VaultARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProvisionedCapacityInput {
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListProvisionedCapacityOutput {
    /// <p>The response body contains the following JSON fields.</p>
    #[serde(rename = "ProvisionedCapacityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity_list: Option<Vec<ProvisionedCapacityDescription>>,
}

/// <p>The input value for <code>ListTagsForVaultInput</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForVaultOutput {
    /// <p>The tags attached to the vault. Each tag is composed of a key and a value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides options to retrieve the vault list owned by the calling user's account. The list provides metadata information for each vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVaultsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The maximum number of vaults to be returned. The default limit is 10. The number of vaults returned might be fewer than the specified limit, but the number of returned vaults never exceeds the limit.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// <p>A string used for pagination. The marker specifies the vault ARN after which the listing of vaults should begin.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVaultsOutput {
    /// <p>The vault ARN at which to continue pagination of the results. You use the marker in another List Vaults request to obtain more vaults in the list.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>List of vaults.</p>
    #[serde(rename = "VaultList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_list: Option<Vec<DescribeVaultOutput>>,
}

/// <p>Contains information about the location where the select job results are stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the job request.</p>
    #[serde(rename = "S3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Location>,
}

/// <p>Describes how the select output is serialized.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded query results.</p>
    #[serde(rename = "csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CSVOutput>,
}

/// <p>A list of the part sizes of the multipart upload.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PartListElement {
    /// <p>The byte range of a part, inclusive of the upper value of the range.</p>
    #[serde(rename = "RangeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_bytes: Option<String>,
    /// <p>The SHA256 tree hash value that Amazon Glacier calculated for the part. This field is never <code>null</code>.</p>
    #[serde(rename = "SHA256TreeHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_tree_hash: Option<String>,
}

/// <p>The definition for a provisioned capacity unit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProvisionedCapacityDescription {
    /// <p>The ID that identifies the provisioned capacity unit.</p>
    #[serde(rename = "CapacityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_id: Option<String>,
    /// <p>The date that the provisioned capacity unit expires, in Universal Coordinated Time (UTC).</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// <p>The date that the provisioned capacity unit was purchased, in Universal Coordinated Time (UTC).</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PurchaseProvisionedCapacityInput {
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PurchaseProvisionedCapacityOutput {
    /// <p>The ID that identifies the provisioned capacity unit.</p>
    #[serde(rename = "capacityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_id: Option<String>,
}

/// <p>The input value for <code>RemoveTagsFromVaultInput</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromVaultInput {
    /// <p>A list of tag keys. Each corresponding tag is removed from the vault.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains information about the location in Amazon S3 where the select job results are stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Location {
    /// <p>A list of grants that control access to the staged results.</p>
    #[serde(rename = "AccessControlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<Grant>>,
    /// <p>The name of the Amazon S3 bucket where the job results are stored.</p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>The canned access control list (ACL) to apply to the job results.</p>
    #[serde(rename = "CannedACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
    /// <p>Contains information about the encryption used to store the job results in Amazon S3.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The prefix that is prepended to the results for this request.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The storage class used to store the job results.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    /// <p>The tag-set that is applied to the job results.</p>
    #[serde(rename = "Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map of metadata to store with the job results in Amazon S3.</p>
    #[serde(rename = "UserMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about the parameters used for a select.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectParameters {
    /// <p>The expression that is used to select the object.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of the provided expression, for example <code>SQL</code>.</p>
    #[serde(rename = "ExpressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_type: Option<String>,
    /// <p>Describes the serialization format of the object.</p>
    #[serde(rename = "InputSerialization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_serialization: Option<InputSerialization>,
    /// <p>Describes how the results of the select job are serialized.</p>
    #[serde(rename = "OutputSerialization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_serialization: Option<OutputSerialization>,
}

/// <p>SetDataRetrievalPolicy input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetDataRetrievalPolicyInput {
    /// <p>The data retrieval policy in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<DataRetrievalPolicy>,
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

/// <p>SetVaultAccessPolicy input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The vault access policy as a JSON string.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultAccessPolicy>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options to configure notifications that will be sent when specific events happen to a vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    /// <p>Provides options for specifying notification configuration.</p>
    #[serde(rename = "vaultNotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_notification_config: Option<VaultNotificationConfig>,
}

/// <p>Provides options to add an archive to a vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UploadArchiveInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The optional description of the archive you are uploading.</p>
    #[serde(rename = "archiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    /// <p>The data to upload.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Option<Vec<u8>>,
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>A list of in-progress multipart uploads for a vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UploadListElement {
    /// <p>The description of the archive that was specified in the Initiate Multipart Upload request.</p>
    #[serde(rename = "ArchiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    /// <p>The UTC time at which the multipart upload was initiated.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The ID of a multipart upload.</p>
    #[serde(rename = "MultipartUploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    /// <p>The part size, in bytes, specified in the Initiate Multipart Upload request. This is the size of all the parts in the upload except the last part, which may be smaller than this size.</p>
    #[serde(rename = "PartSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size_in_bytes: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the vault that contains the archive.</p>
    #[serde(rename = "VaultARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_arn: Option<String>,
}

/// <p>Provides options to upload a part of an archive in a multipart upload operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UploadMultipartPartInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The data to upload.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub body: Option<Vec<u8>>,
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// <p>The upload ID of the multipart upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UploadMultipartPartOutput {
    /// <p>The SHA256 tree hash that Amazon Glacier computed for the uploaded part.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

/// <p>Contains the vault access policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicy {
    /// <p>The vault access policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>Contains the vault lock policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VaultLockPolicy {
    /// <p>The vault lock policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>Represents a vault's notification configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultNotificationConfig {
    /// <p>A list of one or more events for which Amazon Glacier will send a notification to the specified Amazon SNS topic.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) topic Amazon Resource Name (ARN).</p>
    #[serde(rename = "SNSTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
}

/// Errors returned by AbortMultipartUpload
#[derive(Debug, PartialEq)]
pub enum AbortMultipartUploadError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AbortMultipartUploadError {
    pub fn from_body(body: &str) -> AbortMultipartUploadError {
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
                    "InvalidParameterValueException" => {
                        AbortMultipartUploadError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        AbortMultipartUploadError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        AbortMultipartUploadError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AbortMultipartUploadError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        AbortMultipartUploadError::Validation(error_message.to_string())
                    }
                    _ => AbortMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => AbortMultipartUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AbortMultipartUploadError {
    fn from(err: serde_json::error::Error) -> AbortMultipartUploadError {
        AbortMultipartUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AbortMultipartUploadError {
    fn from(err: CredentialsError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortMultipartUploadError {
    fn from(err: HttpDispatchError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortMultipartUploadError {
    fn from(err: io::Error) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            AbortMultipartUploadError::InvalidParameterValue(ref cause) => cause,
            AbortMultipartUploadError::MissingParameterValue(ref cause) => cause,
            AbortMultipartUploadError::ResourceNotFound(ref cause) => cause,
            AbortMultipartUploadError::ServiceUnavailable(ref cause) => cause,
            AbortMultipartUploadError::Validation(ref cause) => cause,
            AbortMultipartUploadError::Credentials(ref err) => err.description(),
            AbortMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AbortMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AbortVaultLock
#[derive(Debug, PartialEq)]
pub enum AbortVaultLockError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AbortVaultLockError {
    pub fn from_body(body: &str) -> AbortVaultLockError {
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
                    "InvalidParameterValueException" => {
                        AbortVaultLockError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        AbortVaultLockError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AbortVaultLockError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AbortVaultLockError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        AbortVaultLockError::Validation(error_message.to_string())
                    }
                    _ => AbortVaultLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => AbortVaultLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AbortVaultLockError {
    fn from(err: serde_json::error::Error) -> AbortVaultLockError {
        AbortVaultLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AbortVaultLockError {
    fn from(err: CredentialsError) -> AbortVaultLockError {
        AbortVaultLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortVaultLockError {
    fn from(err: HttpDispatchError) -> AbortVaultLockError {
        AbortVaultLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortVaultLockError {
    fn from(err: io::Error) -> AbortVaultLockError {
        AbortVaultLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortVaultLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortVaultLockError {
    fn description(&self) -> &str {
        match *self {
            AbortVaultLockError::InvalidParameterValue(ref cause) => cause,
            AbortVaultLockError::MissingParameterValue(ref cause) => cause,
            AbortVaultLockError::ResourceNotFound(ref cause) => cause,
            AbortVaultLockError::ServiceUnavailable(ref cause) => cause,
            AbortVaultLockError::Validation(ref cause) => cause,
            AbortVaultLockError::Credentials(ref err) => err.description(),
            AbortVaultLockError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AbortVaultLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTagsToVault
#[derive(Debug, PartialEq)]
pub enum AddTagsToVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if the request results in a vault or account limit being exceeded.</p>
    LimitExceeded(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsToVaultError {
    pub fn from_body(body: &str) -> AddTagsToVaultError {
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
                    "InvalidParameterValueException" => {
                        AddTagsToVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AddTagsToVaultError::LimitExceeded(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        AddTagsToVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddTagsToVaultError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AddTagsToVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToVaultError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToVaultError {
    fn from(err: serde_json::error::Error) -> AddTagsToVaultError {
        AddTagsToVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToVaultError {
    fn from(err: CredentialsError) -> AddTagsToVaultError {
        AddTagsToVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToVaultError {
    fn from(err: HttpDispatchError) -> AddTagsToVaultError {
        AddTagsToVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToVaultError {
    fn from(err: io::Error) -> AddTagsToVaultError {
        AddTagsToVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToVaultError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToVaultError::InvalidParameterValue(ref cause) => cause,
            AddTagsToVaultError::LimitExceeded(ref cause) => cause,
            AddTagsToVaultError::MissingParameterValue(ref cause) => cause,
            AddTagsToVaultError::ResourceNotFound(ref cause) => cause,
            AddTagsToVaultError::ServiceUnavailable(ref cause) => cause,
            AddTagsToVaultError::Validation(ref cause) => cause,
            AddTagsToVaultError::Credentials(ref err) => err.description(),
            AddTagsToVaultError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsToVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CompleteMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CompleteMultipartUploadError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CompleteMultipartUploadError {
    pub fn from_body(body: &str) -> CompleteMultipartUploadError {
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
                    "InvalidParameterValueException" => {
                        CompleteMultipartUploadError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        CompleteMultipartUploadError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        CompleteMultipartUploadError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CompleteMultipartUploadError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CompleteMultipartUploadError::Validation(error_message.to_string())
                    }
                    _ => CompleteMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => CompleteMultipartUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CompleteMultipartUploadError {
    fn from(err: serde_json::error::Error) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CompleteMultipartUploadError {
    fn from(err: CredentialsError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteMultipartUploadError {
    fn from(err: HttpDispatchError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteMultipartUploadError {
    fn from(err: io::Error) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            CompleteMultipartUploadError::InvalidParameterValue(ref cause) => cause,
            CompleteMultipartUploadError::MissingParameterValue(ref cause) => cause,
            CompleteMultipartUploadError::ResourceNotFound(ref cause) => cause,
            CompleteMultipartUploadError::ServiceUnavailable(ref cause) => cause,
            CompleteMultipartUploadError::Validation(ref cause) => cause,
            CompleteMultipartUploadError::Credentials(ref err) => err.description(),
            CompleteMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CompleteVaultLock
#[derive(Debug, PartialEq)]
pub enum CompleteVaultLockError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CompleteVaultLockError {
    pub fn from_body(body: &str) -> CompleteVaultLockError {
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
                    "InvalidParameterValueException" => {
                        CompleteVaultLockError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        CompleteVaultLockError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CompleteVaultLockError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CompleteVaultLockError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CompleteVaultLockError::Validation(error_message.to_string())
                    }
                    _ => CompleteVaultLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => CompleteVaultLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CompleteVaultLockError {
    fn from(err: serde_json::error::Error) -> CompleteVaultLockError {
        CompleteVaultLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CompleteVaultLockError {
    fn from(err: CredentialsError) -> CompleteVaultLockError {
        CompleteVaultLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteVaultLockError {
    fn from(err: HttpDispatchError) -> CompleteVaultLockError {
        CompleteVaultLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteVaultLockError {
    fn from(err: io::Error) -> CompleteVaultLockError {
        CompleteVaultLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteVaultLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteVaultLockError {
    fn description(&self) -> &str {
        match *self {
            CompleteVaultLockError::InvalidParameterValue(ref cause) => cause,
            CompleteVaultLockError::MissingParameterValue(ref cause) => cause,
            CompleteVaultLockError::ResourceNotFound(ref cause) => cause,
            CompleteVaultLockError::ServiceUnavailable(ref cause) => cause,
            CompleteVaultLockError::Validation(ref cause) => cause,
            CompleteVaultLockError::Credentials(ref err) => err.description(),
            CompleteVaultLockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteVaultLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVault
#[derive(Debug, PartialEq)]
pub enum CreateVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if the request results in a vault or account limit being exceeded.</p>
    LimitExceeded(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateVaultError {
    pub fn from_body(body: &str) -> CreateVaultError {
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
                    "InvalidParameterValueException" => {
                        CreateVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateVaultError::LimitExceeded(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        CreateVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateVaultError::Validation(error_message.to_string())
                    }
                    _ => CreateVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateVaultError {
    fn from(err: serde_json::error::Error) -> CreateVaultError {
        CreateVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateVaultError {
    fn from(err: CredentialsError) -> CreateVaultError {
        CreateVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateVaultError {
    fn from(err: HttpDispatchError) -> CreateVaultError {
        CreateVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateVaultError {
    fn from(err: io::Error) -> CreateVaultError {
        CreateVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVaultError {
    fn description(&self) -> &str {
        match *self {
            CreateVaultError::InvalidParameterValue(ref cause) => cause,
            CreateVaultError::LimitExceeded(ref cause) => cause,
            CreateVaultError::MissingParameterValue(ref cause) => cause,
            CreateVaultError::ServiceUnavailable(ref cause) => cause,
            CreateVaultError::Validation(ref cause) => cause,
            CreateVaultError::Credentials(ref err) => err.description(),
            CreateVaultError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteArchive
#[derive(Debug, PartialEq)]
pub enum DeleteArchiveError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteArchiveError {
    pub fn from_body(body: &str) -> DeleteArchiveError {
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
                    "InvalidParameterValueException" => {
                        DeleteArchiveError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        DeleteArchiveError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteArchiveError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteArchiveError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteArchiveError::Validation(error_message.to_string())
                    }
                    _ => DeleteArchiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteArchiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteArchiveError {
    fn from(err: serde_json::error::Error) -> DeleteArchiveError {
        DeleteArchiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteArchiveError {
    fn from(err: CredentialsError) -> DeleteArchiveError {
        DeleteArchiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteArchiveError {
    fn from(err: HttpDispatchError) -> DeleteArchiveError {
        DeleteArchiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteArchiveError {
    fn from(err: io::Error) -> DeleteArchiveError {
        DeleteArchiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteArchiveError {
    fn description(&self) -> &str {
        match *self {
            DeleteArchiveError::InvalidParameterValue(ref cause) => cause,
            DeleteArchiveError::MissingParameterValue(ref cause) => cause,
            DeleteArchiveError::ResourceNotFound(ref cause) => cause,
            DeleteArchiveError::ServiceUnavailable(ref cause) => cause,
            DeleteArchiveError::Validation(ref cause) => cause,
            DeleteArchiveError::Credentials(ref err) => err.description(),
            DeleteArchiveError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteArchiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVault
#[derive(Debug, PartialEq)]
pub enum DeleteVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVaultError {
    pub fn from_body(body: &str) -> DeleteVaultError {
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
                    "InvalidParameterValueException" => {
                        DeleteVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        DeleteVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteVaultError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteVaultError::Validation(error_message.to_string())
                    }
                    _ => DeleteVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVaultError {
    fn from(err: serde_json::error::Error) -> DeleteVaultError {
        DeleteVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVaultError {
    fn from(err: CredentialsError) -> DeleteVaultError {
        DeleteVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVaultError {
    fn from(err: HttpDispatchError) -> DeleteVaultError {
        DeleteVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVaultError {
    fn from(err: io::Error) -> DeleteVaultError {
        DeleteVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVaultError {
    fn description(&self) -> &str {
        match *self {
            DeleteVaultError::InvalidParameterValue(ref cause) => cause,
            DeleteVaultError::MissingParameterValue(ref cause) => cause,
            DeleteVaultError::ResourceNotFound(ref cause) => cause,
            DeleteVaultError::ServiceUnavailable(ref cause) => cause,
            DeleteVaultError::Validation(ref cause) => cause,
            DeleteVaultError::Credentials(ref err) => err.description(),
            DeleteVaultError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteVaultAccessPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVaultAccessPolicyError {
    pub fn from_body(body: &str) -> DeleteVaultAccessPolicyError {
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
                    "InvalidParameterValueException" => {
                        DeleteVaultAccessPolicyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        DeleteVaultAccessPolicyError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteVaultAccessPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteVaultAccessPolicyError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteVaultAccessPolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteVaultAccessPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVaultAccessPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVaultAccessPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteVaultAccessPolicyError {
        DeleteVaultAccessPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVaultAccessPolicyError {
    fn from(err: CredentialsError) -> DeleteVaultAccessPolicyError {
        DeleteVaultAccessPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVaultAccessPolicyError {
    fn from(err: HttpDispatchError) -> DeleteVaultAccessPolicyError {
        DeleteVaultAccessPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVaultAccessPolicyError {
    fn from(err: io::Error) -> DeleteVaultAccessPolicyError {
        DeleteVaultAccessPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVaultAccessPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVaultAccessPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteVaultAccessPolicyError::InvalidParameterValue(ref cause) => cause,
            DeleteVaultAccessPolicyError::MissingParameterValue(ref cause) => cause,
            DeleteVaultAccessPolicyError::ResourceNotFound(ref cause) => cause,
            DeleteVaultAccessPolicyError::ServiceUnavailable(ref cause) => cause,
            DeleteVaultAccessPolicyError::Validation(ref cause) => cause,
            DeleteVaultAccessPolicyError::Credentials(ref err) => err.description(),
            DeleteVaultAccessPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVaultAccessPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVaultNotifications
#[derive(Debug, PartialEq)]
pub enum DeleteVaultNotificationsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVaultNotificationsError {
    pub fn from_body(body: &str) -> DeleteVaultNotificationsError {
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
                    "InvalidParameterValueException" => {
                        DeleteVaultNotificationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        DeleteVaultNotificationsError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteVaultNotificationsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteVaultNotificationsError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteVaultNotificationsError::Validation(error_message.to_string())
                    }
                    _ => DeleteVaultNotificationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVaultNotificationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVaultNotificationsError {
    fn from(err: serde_json::error::Error) -> DeleteVaultNotificationsError {
        DeleteVaultNotificationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVaultNotificationsError {
    fn from(err: CredentialsError) -> DeleteVaultNotificationsError {
        DeleteVaultNotificationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVaultNotificationsError {
    fn from(err: HttpDispatchError) -> DeleteVaultNotificationsError {
        DeleteVaultNotificationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVaultNotificationsError {
    fn from(err: io::Error) -> DeleteVaultNotificationsError {
        DeleteVaultNotificationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVaultNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVaultNotificationsError {
    fn description(&self) -> &str {
        match *self {
            DeleteVaultNotificationsError::InvalidParameterValue(ref cause) => cause,
            DeleteVaultNotificationsError::MissingParameterValue(ref cause) => cause,
            DeleteVaultNotificationsError::ResourceNotFound(ref cause) => cause,
            DeleteVaultNotificationsError::ServiceUnavailable(ref cause) => cause,
            DeleteVaultNotificationsError::Validation(ref cause) => cause,
            DeleteVaultNotificationsError::Credentials(ref err) => err.description(),
            DeleteVaultNotificationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVaultNotificationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJob
#[derive(Debug, PartialEq)]
pub enum DescribeJobError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeJobError {
    pub fn from_body(body: &str) -> DescribeJobError {
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
                    "InvalidParameterValueException" => {
                        DescribeJobError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        DescribeJobError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeJobError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobError {
    fn from(err: serde_json::error::Error) -> DescribeJobError {
        DescribeJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobError {
    fn from(err: CredentialsError) -> DescribeJobError {
        DescribeJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobError {
    fn from(err: HttpDispatchError) -> DescribeJobError {
        DescribeJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobError {
    fn from(err: io::Error) -> DescribeJobError {
        DescribeJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobError::InvalidParameterValue(ref cause) => cause,
            DescribeJobError::MissingParameterValue(ref cause) => cause,
            DescribeJobError::ResourceNotFound(ref cause) => cause,
            DescribeJobError::ServiceUnavailable(ref cause) => cause,
            DescribeJobError::Validation(ref cause) => cause,
            DescribeJobError::Credentials(ref err) => err.description(),
            DescribeJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVault
#[derive(Debug, PartialEq)]
pub enum DescribeVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeVaultError {
    pub fn from_body(body: &str) -> DescribeVaultError {
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
                    "InvalidParameterValueException" => {
                        DescribeVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        DescribeVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeVaultError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeVaultError::Validation(error_message.to_string())
                    }
                    _ => DescribeVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeVaultError {
    fn from(err: serde_json::error::Error) -> DescribeVaultError {
        DescribeVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVaultError {
    fn from(err: CredentialsError) -> DescribeVaultError {
        DescribeVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVaultError {
    fn from(err: HttpDispatchError) -> DescribeVaultError {
        DescribeVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVaultError {
    fn from(err: io::Error) -> DescribeVaultError {
        DescribeVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVaultError {
    fn description(&self) -> &str {
        match *self {
            DescribeVaultError::InvalidParameterValue(ref cause) => cause,
            DescribeVaultError::MissingParameterValue(ref cause) => cause,
            DescribeVaultError::ResourceNotFound(ref cause) => cause,
            DescribeVaultError::ServiceUnavailable(ref cause) => cause,
            DescribeVaultError::Validation(ref cause) => cause,
            DescribeVaultError::Credentials(ref err) => err.description(),
            DescribeVaultError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataRetrievalPolicy
#[derive(Debug, PartialEq)]
pub enum GetDataRetrievalPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDataRetrievalPolicyError {
    pub fn from_body(body: &str) -> GetDataRetrievalPolicyError {
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
                    "InvalidParameterValueException" => {
                        GetDataRetrievalPolicyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        GetDataRetrievalPolicyError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        GetDataRetrievalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDataRetrievalPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetDataRetrievalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDataRetrievalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDataRetrievalPolicyError {
    fn from(err: serde_json::error::Error) -> GetDataRetrievalPolicyError {
        GetDataRetrievalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDataRetrievalPolicyError {
    fn from(err: CredentialsError) -> GetDataRetrievalPolicyError {
        GetDataRetrievalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDataRetrievalPolicyError {
    fn from(err: HttpDispatchError) -> GetDataRetrievalPolicyError {
        GetDataRetrievalPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDataRetrievalPolicyError {
    fn from(err: io::Error) -> GetDataRetrievalPolicyError {
        GetDataRetrievalPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDataRetrievalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataRetrievalPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetDataRetrievalPolicyError::InvalidParameterValue(ref cause) => cause,
            GetDataRetrievalPolicyError::MissingParameterValue(ref cause) => cause,
            GetDataRetrievalPolicyError::ServiceUnavailable(ref cause) => cause,
            GetDataRetrievalPolicyError::Validation(ref cause) => cause,
            GetDataRetrievalPolicyError::Credentials(ref err) => err.description(),
            GetDataRetrievalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDataRetrievalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobOutput
#[derive(Debug, PartialEq)]
pub enum GetJobOutputError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetJobOutputError {
    pub fn from_body(body: &str) -> GetJobOutputError {
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
                    "InvalidParameterValueException" => {
                        GetJobOutputError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        GetJobOutputError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetJobOutputError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetJobOutputError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetJobOutputError::Validation(error_message.to_string())
                    }
                    _ => GetJobOutputError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobOutputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobOutputError {
    fn from(err: serde_json::error::Error) -> GetJobOutputError {
        GetJobOutputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobOutputError {
    fn from(err: CredentialsError) -> GetJobOutputError {
        GetJobOutputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobOutputError {
    fn from(err: HttpDispatchError) -> GetJobOutputError {
        GetJobOutputError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobOutputError {
    fn from(err: io::Error) -> GetJobOutputError {
        GetJobOutputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobOutputError {
    fn description(&self) -> &str {
        match *self {
            GetJobOutputError::InvalidParameterValue(ref cause) => cause,
            GetJobOutputError::MissingParameterValue(ref cause) => cause,
            GetJobOutputError::ResourceNotFound(ref cause) => cause,
            GetJobOutputError::ServiceUnavailable(ref cause) => cause,
            GetJobOutputError::Validation(ref cause) => cause,
            GetJobOutputError::Credentials(ref err) => err.description(),
            GetJobOutputError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobOutputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum GetVaultAccessPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetVaultAccessPolicyError {
    pub fn from_body(body: &str) -> GetVaultAccessPolicyError {
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
                    "InvalidParameterValueException" => {
                        GetVaultAccessPolicyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        GetVaultAccessPolicyError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        GetVaultAccessPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetVaultAccessPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetVaultAccessPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetVaultAccessPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVaultAccessPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVaultAccessPolicyError {
    fn from(err: serde_json::error::Error) -> GetVaultAccessPolicyError {
        GetVaultAccessPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVaultAccessPolicyError {
    fn from(err: CredentialsError) -> GetVaultAccessPolicyError {
        GetVaultAccessPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVaultAccessPolicyError {
    fn from(err: HttpDispatchError) -> GetVaultAccessPolicyError {
        GetVaultAccessPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVaultAccessPolicyError {
    fn from(err: io::Error) -> GetVaultAccessPolicyError {
        GetVaultAccessPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVaultAccessPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVaultAccessPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetVaultAccessPolicyError::InvalidParameterValue(ref cause) => cause,
            GetVaultAccessPolicyError::MissingParameterValue(ref cause) => cause,
            GetVaultAccessPolicyError::ResourceNotFound(ref cause) => cause,
            GetVaultAccessPolicyError::ServiceUnavailable(ref cause) => cause,
            GetVaultAccessPolicyError::Validation(ref cause) => cause,
            GetVaultAccessPolicyError::Credentials(ref err) => err.description(),
            GetVaultAccessPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetVaultAccessPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVaultLock
#[derive(Debug, PartialEq)]
pub enum GetVaultLockError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetVaultLockError {
    pub fn from_body(body: &str) -> GetVaultLockError {
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
                    "InvalidParameterValueException" => {
                        GetVaultLockError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        GetVaultLockError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetVaultLockError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetVaultLockError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetVaultLockError::Validation(error_message.to_string())
                    }
                    _ => GetVaultLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVaultLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVaultLockError {
    fn from(err: serde_json::error::Error) -> GetVaultLockError {
        GetVaultLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVaultLockError {
    fn from(err: CredentialsError) -> GetVaultLockError {
        GetVaultLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVaultLockError {
    fn from(err: HttpDispatchError) -> GetVaultLockError {
        GetVaultLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVaultLockError {
    fn from(err: io::Error) -> GetVaultLockError {
        GetVaultLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVaultLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVaultLockError {
    fn description(&self) -> &str {
        match *self {
            GetVaultLockError::InvalidParameterValue(ref cause) => cause,
            GetVaultLockError::MissingParameterValue(ref cause) => cause,
            GetVaultLockError::ResourceNotFound(ref cause) => cause,
            GetVaultLockError::ServiceUnavailable(ref cause) => cause,
            GetVaultLockError::Validation(ref cause) => cause,
            GetVaultLockError::Credentials(ref err) => err.description(),
            GetVaultLockError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetVaultLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVaultNotifications
#[derive(Debug, PartialEq)]
pub enum GetVaultNotificationsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetVaultNotificationsError {
    pub fn from_body(body: &str) -> GetVaultNotificationsError {
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
                    "InvalidParameterValueException" => {
                        GetVaultNotificationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        GetVaultNotificationsError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        GetVaultNotificationsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetVaultNotificationsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetVaultNotificationsError::Validation(error_message.to_string())
                    }
                    _ => GetVaultNotificationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVaultNotificationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVaultNotificationsError {
    fn from(err: serde_json::error::Error) -> GetVaultNotificationsError {
        GetVaultNotificationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVaultNotificationsError {
    fn from(err: CredentialsError) -> GetVaultNotificationsError {
        GetVaultNotificationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVaultNotificationsError {
    fn from(err: HttpDispatchError) -> GetVaultNotificationsError {
        GetVaultNotificationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVaultNotificationsError {
    fn from(err: io::Error) -> GetVaultNotificationsError {
        GetVaultNotificationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVaultNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVaultNotificationsError {
    fn description(&self) -> &str {
        match *self {
            GetVaultNotificationsError::InvalidParameterValue(ref cause) => cause,
            GetVaultNotificationsError::MissingParameterValue(ref cause) => cause,
            GetVaultNotificationsError::ResourceNotFound(ref cause) => cause,
            GetVaultNotificationsError::ServiceUnavailable(ref cause) => cause,
            GetVaultNotificationsError::Validation(ref cause) => cause,
            GetVaultNotificationsError::Credentials(ref err) => err.description(),
            GetVaultNotificationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetVaultNotificationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateJob
#[derive(Debug, PartialEq)]
pub enum InitiateJobError {
    /// <p>Returned if there is insufficient capacity to process this expedited request. This error only applies to expedited retrievals and not to standard or bulk retrievals.</p>
    InsufficientCapacity(String),
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if a retrieval job would exceed the current data policy's retrieval rate limit. For more information about data retrieval policies,</p>
    PolicyEnforced(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitiateJobError {
    pub fn from_body(body: &str) -> InitiateJobError {
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
                    "InsufficientCapacityException" => {
                        InitiateJobError::InsufficientCapacity(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        InitiateJobError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        InitiateJobError::MissingParameterValue(String::from(error_message))
                    }
                    "PolicyEnforcedException" => {
                        InitiateJobError::PolicyEnforced(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InitiateJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        InitiateJobError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        InitiateJobError::Validation(error_message.to_string())
                    }
                    _ => InitiateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateJobError {
    fn from(err: serde_json::error::Error) -> InitiateJobError {
        InitiateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateJobError {
    fn from(err: CredentialsError) -> InitiateJobError {
        InitiateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateJobError {
    fn from(err: HttpDispatchError) -> InitiateJobError {
        InitiateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateJobError {
    fn from(err: io::Error) -> InitiateJobError {
        InitiateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateJobError {
    fn description(&self) -> &str {
        match *self {
            InitiateJobError::InsufficientCapacity(ref cause) => cause,
            InitiateJobError::InvalidParameterValue(ref cause) => cause,
            InitiateJobError::MissingParameterValue(ref cause) => cause,
            InitiateJobError::PolicyEnforced(ref cause) => cause,
            InitiateJobError::ResourceNotFound(ref cause) => cause,
            InitiateJobError::ServiceUnavailable(ref cause) => cause,
            InitiateJobError::Validation(ref cause) => cause,
            InitiateJobError::Credentials(ref err) => err.description(),
            InitiateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InitiateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateMultipartUpload
#[derive(Debug, PartialEq)]
pub enum InitiateMultipartUploadError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitiateMultipartUploadError {
    pub fn from_body(body: &str) -> InitiateMultipartUploadError {
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
                    "InvalidParameterValueException" => {
                        InitiateMultipartUploadError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        InitiateMultipartUploadError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        InitiateMultipartUploadError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        InitiateMultipartUploadError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        InitiateMultipartUploadError::Validation(error_message.to_string())
                    }
                    _ => InitiateMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateMultipartUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateMultipartUploadError {
    fn from(err: serde_json::error::Error) -> InitiateMultipartUploadError {
        InitiateMultipartUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateMultipartUploadError {
    fn from(err: CredentialsError) -> InitiateMultipartUploadError {
        InitiateMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateMultipartUploadError {
    fn from(err: HttpDispatchError) -> InitiateMultipartUploadError {
        InitiateMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateMultipartUploadError {
    fn from(err: io::Error) -> InitiateMultipartUploadError {
        InitiateMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            InitiateMultipartUploadError::InvalidParameterValue(ref cause) => cause,
            InitiateMultipartUploadError::MissingParameterValue(ref cause) => cause,
            InitiateMultipartUploadError::ResourceNotFound(ref cause) => cause,
            InitiateMultipartUploadError::ServiceUnavailable(ref cause) => cause,
            InitiateMultipartUploadError::Validation(ref cause) => cause,
            InitiateMultipartUploadError::Credentials(ref err) => err.description(),
            InitiateMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitiateMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateVaultLock
#[derive(Debug, PartialEq)]
pub enum InitiateVaultLockError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitiateVaultLockError {
    pub fn from_body(body: &str) -> InitiateVaultLockError {
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
                    "InvalidParameterValueException" => {
                        InitiateVaultLockError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        InitiateVaultLockError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InitiateVaultLockError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        InitiateVaultLockError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        InitiateVaultLockError::Validation(error_message.to_string())
                    }
                    _ => InitiateVaultLockError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateVaultLockError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateVaultLockError {
    fn from(err: serde_json::error::Error) -> InitiateVaultLockError {
        InitiateVaultLockError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateVaultLockError {
    fn from(err: CredentialsError) -> InitiateVaultLockError {
        InitiateVaultLockError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateVaultLockError {
    fn from(err: HttpDispatchError) -> InitiateVaultLockError {
        InitiateVaultLockError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateVaultLockError {
    fn from(err: io::Error) -> InitiateVaultLockError {
        InitiateVaultLockError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateVaultLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateVaultLockError {
    fn description(&self) -> &str {
        match *self {
            InitiateVaultLockError::InvalidParameterValue(ref cause) => cause,
            InitiateVaultLockError::MissingParameterValue(ref cause) => cause,
            InitiateVaultLockError::ResourceNotFound(ref cause) => cause,
            InitiateVaultLockError::ServiceUnavailable(ref cause) => cause,
            InitiateVaultLockError::Validation(ref cause) => cause,
            InitiateVaultLockError::Credentials(ref err) => err.description(),
            InitiateVaultLockError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitiateVaultLockError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
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
                    "InvalidParameterValueException" => {
                        ListJobsError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        ListJobsError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListJobsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListJobsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => ListJobsError::Validation(error_message.to_string()),
                    _ => ListJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::InvalidParameterValue(ref cause) => cause,
            ListJobsError::MissingParameterValue(ref cause) => cause,
            ListJobsError::ResourceNotFound(ref cause) => cause,
            ListJobsError::ServiceUnavailable(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMultipartUploads
#[derive(Debug, PartialEq)]
pub enum ListMultipartUploadsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListMultipartUploadsError {
    pub fn from_body(body: &str) -> ListMultipartUploadsError {
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
                    "InvalidParameterValueException" => {
                        ListMultipartUploadsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        ListMultipartUploadsError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListMultipartUploadsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListMultipartUploadsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListMultipartUploadsError::Validation(error_message.to_string())
                    }
                    _ => ListMultipartUploadsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListMultipartUploadsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListMultipartUploadsError {
    fn from(err: serde_json::error::Error) -> ListMultipartUploadsError {
        ListMultipartUploadsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListMultipartUploadsError {
    fn from(err: CredentialsError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMultipartUploadsError {
    fn from(err: HttpDispatchError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMultipartUploadsError {
    fn from(err: io::Error) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMultipartUploadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMultipartUploadsError {
    fn description(&self) -> &str {
        match *self {
            ListMultipartUploadsError::InvalidParameterValue(ref cause) => cause,
            ListMultipartUploadsError::MissingParameterValue(ref cause) => cause,
            ListMultipartUploadsError::ResourceNotFound(ref cause) => cause,
            ListMultipartUploadsError::ServiceUnavailable(ref cause) => cause,
            ListMultipartUploadsError::Validation(ref cause) => cause,
            ListMultipartUploadsError::Credentials(ref err) => err.description(),
            ListMultipartUploadsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListMultipartUploadsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListParts
#[derive(Debug, PartialEq)]
pub enum ListPartsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPartsError {
    pub fn from_body(body: &str) -> ListPartsError {
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
                    "InvalidParameterValueException" => {
                        ListPartsError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        ListPartsError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPartsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPartsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => ListPartsError::Validation(error_message.to_string()),
                    _ => ListPartsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPartsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPartsError {
    fn from(err: serde_json::error::Error) -> ListPartsError {
        ListPartsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPartsError {
    fn from(err: CredentialsError) -> ListPartsError {
        ListPartsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPartsError {
    fn from(err: HttpDispatchError) -> ListPartsError {
        ListPartsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPartsError {
    fn from(err: io::Error) -> ListPartsError {
        ListPartsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPartsError {
    fn description(&self) -> &str {
        match *self {
            ListPartsError::InvalidParameterValue(ref cause) => cause,
            ListPartsError::MissingParameterValue(ref cause) => cause,
            ListPartsError::ResourceNotFound(ref cause) => cause,
            ListPartsError::ServiceUnavailable(ref cause) => cause,
            ListPartsError::Validation(ref cause) => cause,
            ListPartsError::Credentials(ref err) => err.description(),
            ListPartsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPartsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProvisionedCapacity
#[derive(Debug, PartialEq)]
pub enum ListProvisionedCapacityError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListProvisionedCapacityError {
    pub fn from_body(body: &str) -> ListProvisionedCapacityError {
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
                    "InvalidParameterValueException" => {
                        ListProvisionedCapacityError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        ListProvisionedCapacityError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        ListProvisionedCapacityError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListProvisionedCapacityError::Validation(error_message.to_string())
                    }
                    _ => ListProvisionedCapacityError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProvisionedCapacityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProvisionedCapacityError {
    fn from(err: serde_json::error::Error) -> ListProvisionedCapacityError {
        ListProvisionedCapacityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProvisionedCapacityError {
    fn from(err: CredentialsError) -> ListProvisionedCapacityError {
        ListProvisionedCapacityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProvisionedCapacityError {
    fn from(err: HttpDispatchError) -> ListProvisionedCapacityError {
        ListProvisionedCapacityError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProvisionedCapacityError {
    fn from(err: io::Error) -> ListProvisionedCapacityError {
        ListProvisionedCapacityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListProvisionedCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProvisionedCapacityError {
    fn description(&self) -> &str {
        match *self {
            ListProvisionedCapacityError::InvalidParameterValue(ref cause) => cause,
            ListProvisionedCapacityError::MissingParameterValue(ref cause) => cause,
            ListProvisionedCapacityError::ServiceUnavailable(ref cause) => cause,
            ListProvisionedCapacityError::Validation(ref cause) => cause,
            ListProvisionedCapacityError::Credentials(ref err) => err.description(),
            ListProvisionedCapacityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListProvisionedCapacityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForVault
#[derive(Debug, PartialEq)]
pub enum ListTagsForVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForVaultError {
    pub fn from_body(body: &str) -> ListTagsForVaultError {
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
                    "InvalidParameterValueException" => {
                        ListTagsForVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        ListTagsForVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsForVaultError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListTagsForVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForVaultError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForVaultError {
    fn from(err: serde_json::error::Error) -> ListTagsForVaultError {
        ListTagsForVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForVaultError {
    fn from(err: CredentialsError) -> ListTagsForVaultError {
        ListTagsForVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForVaultError {
    fn from(err: HttpDispatchError) -> ListTagsForVaultError {
        ListTagsForVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForVaultError {
    fn from(err: io::Error) -> ListTagsForVaultError {
        ListTagsForVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForVaultError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForVaultError::InvalidParameterValue(ref cause) => cause,
            ListTagsForVaultError::MissingParameterValue(ref cause) => cause,
            ListTagsForVaultError::ResourceNotFound(ref cause) => cause,
            ListTagsForVaultError::ServiceUnavailable(ref cause) => cause,
            ListTagsForVaultError::Validation(ref cause) => cause,
            ListTagsForVaultError::Credentials(ref err) => err.description(),
            ListTagsForVaultError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsForVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVaults
#[derive(Debug, PartialEq)]
pub enum ListVaultsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVaultsError {
    pub fn from_body(body: &str) -> ListVaultsError {
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
                    "InvalidParameterValueException" => {
                        ListVaultsError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        ListVaultsError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListVaultsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListVaultsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => ListVaultsError::Validation(error_message.to_string()),
                    _ => ListVaultsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVaultsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVaultsError {
    fn from(err: serde_json::error::Error) -> ListVaultsError {
        ListVaultsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVaultsError {
    fn from(err: CredentialsError) -> ListVaultsError {
        ListVaultsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVaultsError {
    fn from(err: HttpDispatchError) -> ListVaultsError {
        ListVaultsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVaultsError {
    fn from(err: io::Error) -> ListVaultsError {
        ListVaultsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVaultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVaultsError {
    fn description(&self) -> &str {
        match *self {
            ListVaultsError::InvalidParameterValue(ref cause) => cause,
            ListVaultsError::MissingParameterValue(ref cause) => cause,
            ListVaultsError::ResourceNotFound(ref cause) => cause,
            ListVaultsError::ServiceUnavailable(ref cause) => cause,
            ListVaultsError::Validation(ref cause) => cause,
            ListVaultsError::Credentials(ref err) => err.description(),
            ListVaultsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListVaultsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PurchaseProvisionedCapacity
#[derive(Debug, PartialEq)]
pub enum PurchaseProvisionedCapacityError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if the request results in a vault or account limit being exceeded.</p>
    LimitExceeded(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PurchaseProvisionedCapacityError {
    pub fn from_body(body: &str) -> PurchaseProvisionedCapacityError {
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
                    "InvalidParameterValueException" => {
                        PurchaseProvisionedCapacityError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        PurchaseProvisionedCapacityError::LimitExceeded(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        PurchaseProvisionedCapacityError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        PurchaseProvisionedCapacityError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        PurchaseProvisionedCapacityError::Validation(error_message.to_string())
                    }
                    _ => PurchaseProvisionedCapacityError::Unknown(String::from(body)),
                }
            }
            Err(_) => PurchaseProvisionedCapacityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PurchaseProvisionedCapacityError {
    fn from(err: serde_json::error::Error) -> PurchaseProvisionedCapacityError {
        PurchaseProvisionedCapacityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PurchaseProvisionedCapacityError {
    fn from(err: CredentialsError) -> PurchaseProvisionedCapacityError {
        PurchaseProvisionedCapacityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PurchaseProvisionedCapacityError {
    fn from(err: HttpDispatchError) -> PurchaseProvisionedCapacityError {
        PurchaseProvisionedCapacityError::HttpDispatch(err)
    }
}
impl From<io::Error> for PurchaseProvisionedCapacityError {
    fn from(err: io::Error) -> PurchaseProvisionedCapacityError {
        PurchaseProvisionedCapacityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PurchaseProvisionedCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PurchaseProvisionedCapacityError {
    fn description(&self) -> &str {
        match *self {
            PurchaseProvisionedCapacityError::InvalidParameterValue(ref cause) => cause,
            PurchaseProvisionedCapacityError::LimitExceeded(ref cause) => cause,
            PurchaseProvisionedCapacityError::MissingParameterValue(ref cause) => cause,
            PurchaseProvisionedCapacityError::ServiceUnavailable(ref cause) => cause,
            PurchaseProvisionedCapacityError::Validation(ref cause) => cause,
            PurchaseProvisionedCapacityError::Credentials(ref err) => err.description(),
            PurchaseProvisionedCapacityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PurchaseProvisionedCapacityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromVault
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromVaultError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsFromVaultError {
    pub fn from_body(body: &str) -> RemoveTagsFromVaultError {
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
                    "InvalidParameterValueException" => {
                        RemoveTagsFromVaultError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        RemoveTagsFromVaultError::MissingParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveTagsFromVaultError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        RemoveTagsFromVaultError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveTagsFromVaultError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromVaultError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromVaultError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromVaultError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromVaultError {
        RemoveTagsFromVaultError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromVaultError {
    fn from(err: CredentialsError) -> RemoveTagsFromVaultError {
        RemoveTagsFromVaultError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromVaultError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromVaultError {
        RemoveTagsFromVaultError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromVaultError {
    fn from(err: io::Error) -> RemoveTagsFromVaultError {
        RemoveTagsFromVaultError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromVaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromVaultError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromVaultError::InvalidParameterValue(ref cause) => cause,
            RemoveTagsFromVaultError::MissingParameterValue(ref cause) => cause,
            RemoveTagsFromVaultError::ResourceNotFound(ref cause) => cause,
            RemoveTagsFromVaultError::ServiceUnavailable(ref cause) => cause,
            RemoveTagsFromVaultError::Validation(ref cause) => cause,
            RemoveTagsFromVaultError::Credentials(ref err) => err.description(),
            RemoveTagsFromVaultError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromVaultError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetDataRetrievalPolicy
#[derive(Debug, PartialEq)]
pub enum SetDataRetrievalPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetDataRetrievalPolicyError {
    pub fn from_body(body: &str) -> SetDataRetrievalPolicyError {
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
                    "InvalidParameterValueException" => {
                        SetDataRetrievalPolicyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        SetDataRetrievalPolicyError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        SetDataRetrievalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetDataRetrievalPolicyError::Validation(error_message.to_string())
                    }
                    _ => SetDataRetrievalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetDataRetrievalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetDataRetrievalPolicyError {
    fn from(err: serde_json::error::Error) -> SetDataRetrievalPolicyError {
        SetDataRetrievalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetDataRetrievalPolicyError {
    fn from(err: CredentialsError) -> SetDataRetrievalPolicyError {
        SetDataRetrievalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetDataRetrievalPolicyError {
    fn from(err: HttpDispatchError) -> SetDataRetrievalPolicyError {
        SetDataRetrievalPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetDataRetrievalPolicyError {
    fn from(err: io::Error) -> SetDataRetrievalPolicyError {
        SetDataRetrievalPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetDataRetrievalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetDataRetrievalPolicyError {
    fn description(&self) -> &str {
        match *self {
            SetDataRetrievalPolicyError::InvalidParameterValue(ref cause) => cause,
            SetDataRetrievalPolicyError::MissingParameterValue(ref cause) => cause,
            SetDataRetrievalPolicyError::ServiceUnavailable(ref cause) => cause,
            SetDataRetrievalPolicyError::Validation(ref cause) => cause,
            SetDataRetrievalPolicyError::Credentials(ref err) => err.description(),
            SetDataRetrievalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetDataRetrievalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetVaultAccessPolicy
#[derive(Debug, PartialEq)]
pub enum SetVaultAccessPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetVaultAccessPolicyError {
    pub fn from_body(body: &str) -> SetVaultAccessPolicyError {
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
                    "InvalidParameterValueException" => {
                        SetVaultAccessPolicyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        SetVaultAccessPolicyError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        SetVaultAccessPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetVaultAccessPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetVaultAccessPolicyError::Validation(error_message.to_string())
                    }
                    _ => SetVaultAccessPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetVaultAccessPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetVaultAccessPolicyError {
    fn from(err: serde_json::error::Error) -> SetVaultAccessPolicyError {
        SetVaultAccessPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetVaultAccessPolicyError {
    fn from(err: CredentialsError) -> SetVaultAccessPolicyError {
        SetVaultAccessPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetVaultAccessPolicyError {
    fn from(err: HttpDispatchError) -> SetVaultAccessPolicyError {
        SetVaultAccessPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetVaultAccessPolicyError {
    fn from(err: io::Error) -> SetVaultAccessPolicyError {
        SetVaultAccessPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetVaultAccessPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetVaultAccessPolicyError {
    fn description(&self) -> &str {
        match *self {
            SetVaultAccessPolicyError::InvalidParameterValue(ref cause) => cause,
            SetVaultAccessPolicyError::MissingParameterValue(ref cause) => cause,
            SetVaultAccessPolicyError::ResourceNotFound(ref cause) => cause,
            SetVaultAccessPolicyError::ServiceUnavailable(ref cause) => cause,
            SetVaultAccessPolicyError::Validation(ref cause) => cause,
            SetVaultAccessPolicyError::Credentials(ref err) => err.description(),
            SetVaultAccessPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetVaultAccessPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetVaultNotifications
#[derive(Debug, PartialEq)]
pub enum SetVaultNotificationsError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetVaultNotificationsError {
    pub fn from_body(body: &str) -> SetVaultNotificationsError {
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
                    "InvalidParameterValueException" => {
                        SetVaultNotificationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "MissingParameterValueException" => {
                        SetVaultNotificationsError::MissingParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        SetVaultNotificationsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetVaultNotificationsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetVaultNotificationsError::Validation(error_message.to_string())
                    }
                    _ => SetVaultNotificationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetVaultNotificationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetVaultNotificationsError {
    fn from(err: serde_json::error::Error) -> SetVaultNotificationsError {
        SetVaultNotificationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetVaultNotificationsError {
    fn from(err: CredentialsError) -> SetVaultNotificationsError {
        SetVaultNotificationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetVaultNotificationsError {
    fn from(err: HttpDispatchError) -> SetVaultNotificationsError {
        SetVaultNotificationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetVaultNotificationsError {
    fn from(err: io::Error) -> SetVaultNotificationsError {
        SetVaultNotificationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetVaultNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetVaultNotificationsError {
    fn description(&self) -> &str {
        match *self {
            SetVaultNotificationsError::InvalidParameterValue(ref cause) => cause,
            SetVaultNotificationsError::MissingParameterValue(ref cause) => cause,
            SetVaultNotificationsError::ResourceNotFound(ref cause) => cause,
            SetVaultNotificationsError::ServiceUnavailable(ref cause) => cause,
            SetVaultNotificationsError::Validation(ref cause) => cause,
            SetVaultNotificationsError::Credentials(ref err) => err.description(),
            SetVaultNotificationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetVaultNotificationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadArchive
#[derive(Debug, PartialEq)]
pub enum UploadArchiveError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if, when uploading an archive, Amazon Glacier times out while receiving the upload.</p>
    RequestTimeout(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UploadArchiveError {
    pub fn from_body(body: &str) -> UploadArchiveError {
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
                    "InvalidParameterValueException" => {
                        UploadArchiveError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        UploadArchiveError::MissingParameterValue(String::from(error_message))
                    }
                    "RequestTimeoutException" => {
                        UploadArchiveError::RequestTimeout(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UploadArchiveError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UploadArchiveError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        UploadArchiveError::Validation(error_message.to_string())
                    }
                    _ => UploadArchiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => UploadArchiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UploadArchiveError {
    fn from(err: serde_json::error::Error) -> UploadArchiveError {
        UploadArchiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UploadArchiveError {
    fn from(err: CredentialsError) -> UploadArchiveError {
        UploadArchiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadArchiveError {
    fn from(err: HttpDispatchError) -> UploadArchiveError {
        UploadArchiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadArchiveError {
    fn from(err: io::Error) -> UploadArchiveError {
        UploadArchiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadArchiveError {
    fn description(&self) -> &str {
        match *self {
            UploadArchiveError::InvalidParameterValue(ref cause) => cause,
            UploadArchiveError::MissingParameterValue(ref cause) => cause,
            UploadArchiveError::RequestTimeout(ref cause) => cause,
            UploadArchiveError::ResourceNotFound(ref cause) => cause,
            UploadArchiveError::ServiceUnavailable(ref cause) => cause,
            UploadArchiveError::Validation(ref cause) => cause,
            UploadArchiveError::Credentials(ref err) => err.description(),
            UploadArchiveError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadArchiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadMultipartPart
#[derive(Debug, PartialEq)]
pub enum UploadMultipartPartError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if, when uploading an archive, Amazon Glacier times out while receiving the upload.</p>
    RequestTimeout(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UploadMultipartPartError {
    pub fn from_body(body: &str) -> UploadMultipartPartError {
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
                    "InvalidParameterValueException" => {
                        UploadMultipartPartError::InvalidParameterValue(String::from(error_message))
                    }
                    "MissingParameterValueException" => {
                        UploadMultipartPartError::MissingParameterValue(String::from(error_message))
                    }
                    "RequestTimeoutException" => {
                        UploadMultipartPartError::RequestTimeout(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UploadMultipartPartError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UploadMultipartPartError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        UploadMultipartPartError::Validation(error_message.to_string())
                    }
                    _ => UploadMultipartPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => UploadMultipartPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UploadMultipartPartError {
    fn from(err: serde_json::error::Error) -> UploadMultipartPartError {
        UploadMultipartPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UploadMultipartPartError {
    fn from(err: CredentialsError) -> UploadMultipartPartError {
        UploadMultipartPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadMultipartPartError {
    fn from(err: HttpDispatchError) -> UploadMultipartPartError {
        UploadMultipartPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadMultipartPartError {
    fn from(err: io::Error) -> UploadMultipartPartError {
        UploadMultipartPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadMultipartPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadMultipartPartError {
    fn description(&self) -> &str {
        match *self {
            UploadMultipartPartError::InvalidParameterValue(ref cause) => cause,
            UploadMultipartPartError::MissingParameterValue(ref cause) => cause,
            UploadMultipartPartError::RequestTimeout(ref cause) => cause,
            UploadMultipartPartError::ResourceNotFound(ref cause) => cause,
            UploadMultipartPartError::ServiceUnavailable(ref cause) => cause,
            UploadMultipartPartError::Validation(ref cause) => cause,
            UploadMultipartPartError::Credentials(ref err) => err.description(),
            UploadMultipartPartError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UploadMultipartPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Glacier API. Amazon Glacier clients implement this trait.
pub trait Glacier {
    /// <p>This operation aborts a multipart upload identified by the upload ID.</p> <p>After the Abort Multipart Upload request succeeds, you cannot upload any more parts to the multipart upload or complete the multipart upload. Aborting a completed upload fails. However, aborting an already-aborted upload will succeed, for a short time. For more information about uploading a part and completing a multipart upload, see <a>UploadMultipartPart</a> and <a>CompleteMultipartUpload</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadInput,
    ) -> RusotoFuture<(), AbortMultipartUploadError>;

    /// <p>This operation aborts the vault locking process if the vault lock is not in the <code>Locked</code> state. If the vault lock is in the <code>Locked</code> state when this operation is requested, the operation returns an <code>AccessDeniedException</code> error. Aborting the vault locking process removes the vault lock policy from the specified vault. </p> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can get the state of a vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. For more information about vault lock policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>This operation is idempotent. You can successfully invoke this operation multiple times, if the vault lock is in the <code>InProgress</code> state or if there is no policy associated with the vault.</p>
    fn abort_vault_lock(&self, input: AbortVaultLockInput)
        -> RusotoFuture<(), AbortVaultLockError>;

    /// <p>This operation adds the specified tags to a vault. Each tag is composed of a key and a value. Each vault can have up to 10 tags. If your request would cause the tag limit for the vault to be exceeded, the operation throws the <code>LimitExceededException</code> error. If a tag already exists on the vault under a specified key, the existing key value will be overwritten. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>. </p>
    fn add_tags_to_vault(
        &self,
        input: AddTagsToVaultInput,
    ) -> RusotoFuture<(), AddTagsToVaultError>;

    /// <p>You call this operation to inform Amazon Glacier that all the archive parts have been uploaded and that Amazon Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Amazon Glacier returns the URI path of the newly created archive resource. Using the URI path, you can then access the archive. After you upload an archive, you should save the archive ID returned to retrieve the archive at a later point. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>.</p> <p>In the request, you must include the computed SHA256 tree hash of the entire archive you have uploaded. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. On the server side, Amazon Glacier also constructs the SHA256 tree hash of the assembled archive. If the values match, Amazon Glacier saves the archive to the vault; otherwise, it returns an error, and the operation fails. The <a>ListParts</a> operation returns a list of parts uploaded for a specific multipart upload. It includes checksum information for each uploaded part that can be used to debug a bad checksum issue.</p> <p>Additionally, Amazon Glacier also checks for any missing content ranges when assembling the archive, if missing content ranges are found, Amazon Glacier returns an error and the operation fails.</p> <p>Complete Multipart Upload is an idempotent operation. After your first successful complete multipart upload, if you call the operation again within a short period, the operation will succeed and return the same archive ID. This is useful in the event you experience a network issue that causes an aborted connection or receive a 500 server error, in which case you can repeat your Complete Multipart Upload request and get the same archive ID without creating duplicate archives. Note, however, that after the multipart upload completes, you cannot call the List Parts operation and the multipart upload will not appear in List Multipart Uploads response, even if idempotent complete is possible.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-complete-upload.html">Complete Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadInput,
    ) -> RusotoFuture<ArchiveCreationOutput, CompleteMultipartUploadError>;

    /// <p>This operation completes the vault locking process by transitioning the vault lock from the <code>InProgress</code> state to the <code>Locked</code> state, which causes the vault lock policy to become unchangeable. A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. You can obtain the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>This operation is idempotent. This request is always successful if the vault lock is in the <code>Locked</code> state and the provided lock ID matches the lock ID originally used to lock the vault.</p> <p>If an invalid lock ID is passed in the request when the vault lock is in the <code>Locked</code> state, the operation returns an <code>AccessDeniedException</code> error. If an invalid lock ID is passed in the request when the vault lock is in the <code>InProgress</code> state, the operation throws an <code>InvalidParameter</code> error.</p>
    fn complete_vault_lock(
        &self,
        input: CompleteVaultLockInput,
    ) -> RusotoFuture<(), CompleteVaultLockError>;

    /// <p>This operation creates a new vault with the specified name. The name of the vault must be unique within a region for an AWS account. You can create up to 1,000 vaults per account. If you need to create more vaults, contact Amazon Glacier.</p> <p>You must use the following guidelines when naming a vault.</p> <ul> <li> <p>Names can be between 1 and 255 characters long.</p> </li> <li> <p>Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).</p> </li> </ul> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/creating-vaults.html">Creating a Vault in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-put.html">Create Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn create_vault(
        &self,
        input: CreateVaultInput,
    ) -> RusotoFuture<CreateVaultOutput, CreateVaultError>;

    /// <p>This operation deletes an archive from a vault. Subsequent requests to initiate a retrieval of this archive will fail. Archive retrievals that are in progress for this archive ID may or may not succeed according to the following scenarios:</p> <ul> <li> <p>If the archive retrieval job is actively preparing the data for download when Amazon Glacier receives the delete archive request, the archival retrieval operation might fail.</p> </li> <li> <p>If the archive retrieval job has successfully prepared the archive for download when Amazon Glacier receives the delete archive request, you will be able to download the output.</p> </li> </ul> <p>This operation is idempotent. Attempting to delete an already-deleted archive does not result in an error.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-an-archive.html">Deleting an Archive in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn delete_archive(&self, input: DeleteArchiveInput) -> RusotoFuture<(), DeleteArchiveError>;

    /// <p>This operation deletes a vault. Amazon Glacier will delete a vault only if there are no archives in the vault as of the last inventory and there have been no writes to the vault since the last inventory. If either of these conditions is not satisfied, the vault deletion fails (that is, the vault is not removed) and Amazon Glacier returns an error. You can use <a>DescribeVault</a> to return the number of archives in a vault, and you can use <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job (POST jobs)</a> to initiate a new inventory retrieval for a vault. The inventory contains the archive IDs you use to delete archives using <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive (DELETE archive)</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-vaults.html">Deleting a Vault in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-delete.html">Delete Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn delete_vault(&self, input: DeleteVaultInput) -> RusotoFuture<(), DeleteVaultError>;

    /// <p>This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely remove the access policy, and you might still see the effect of the policy for a short time after you send the delete request.</p> <p>This operation is idempotent. You can invoke delete multiple times, even if there is no policy associated with the vault. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    fn delete_vault_access_policy(
        &self,
        input: DeleteVaultAccessPolicyInput,
    ) -> RusotoFuture<(), DeleteVaultAccessPolicyError>;

    /// <p>This operation deletes the notification configuration set for a vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely disable the notifications and you might still receive some notifications for a short time after you send the delete request.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-delete.html">Delete Vault Notification Configuration </a> in the Amazon Glacier Developer Guide. </p>
    fn delete_vault_notifications(
        &self,
        input: DeleteVaultNotificationsInput,
    ) -> RusotoFuture<(), DeleteVaultNotificationsError>;

    /// <p>This operation returns information about a job you previously initiated, including the job initiation date, the user who initiated the job, the job status code/message and the Amazon SNS topic to notify after Amazon Glacier completes the job. For more information about initiating a job, see <a>InitiateJob</a>. </p> <note> <p>This operation enables you to check the status of your job. However, it is strongly recommended that you set up an Amazon SNS topic and specify it in your initiate job request so that Amazon Glacier can notify the topic after it completes the job.</p> </note> <p>A job ID will not expire for at least 24 hours after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-describe-job-get.html">Describe Job</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn describe_job(
        &self,
        input: DescribeJobInput,
    ) -> RusotoFuture<GlacierJobDescription, DescribeJobError>;

    /// <p>This operation returns information about a vault, including the vault's Amazon Resource Name (ARN), the date the vault was created, the number of archives it contains, and the total size of all the archives in the vault. The number of archives and their total size are as of the last inventory generation. This means that if you add or remove an archive from a vault, and then immediately use Describe Vault, the change in contents will not be immediately reflected. If you want to retrieve the latest inventory of the vault, use <a>InitiateJob</a>. Amazon Glacier generates vault inventories approximately daily. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory in Amazon Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-get.html">Describe Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn describe_vault(
        &self,
        input: DescribeVaultInput,
    ) -> RusotoFuture<DescribeVaultOutput, DescribeVaultError>;

    /// <p>This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>.</p>
    fn get_data_retrieval_policy(
        &self,
        input: GetDataRetrievalPolicyInput,
    ) -> RusotoFuture<GetDataRetrievalPolicyOutput, GetDataRetrievalPolicyError>;

    /// <p>This operation downloads the output of the job you initiated using <a>InitiateJob</a>. Depending on the job type you specified when you initiated the job, the output will be either the content of an archive or a vault inventory.</p> <p>You can download all the job output or download a portion of the output by specifying a byte range. In the case of an archive retrieval job, depending on the byte range you specify, Amazon Glacier returns the checksum for the portion of the data. You can compute the checksum on the client and verify that the values match to ensure the portion you downloaded is the correct data.</p> <p>A job ID will not expire for at least 24 hours after Amazon Glacier completes the job. That a byte range. For both archive and inventory retrieval jobs, you should verify the downloaded size against the size returned in the headers from the <b>Get Job Output</b> response.</p> <p>For archive retrieval jobs, you should also verify that the size is what you expected. If you download a portion of the output, the expected size is based on the range of bytes you specified. For example, if you specify a range of <code>bytes=0-1048575</code>, you should verify your download size is 1,048,576 bytes. If you download an entire archive, the expected size is the size of the archive when you uploaded it to Amazon Glacier The expected size is also returned in the headers from the <b>Get Job Output</b> response.</p> <p>In the case of an archive retrieval job, depending on the byte range you specify, Amazon Glacier returns the checksum for the portion of the data. To ensure the portion you downloaded is the correct data, compute the checksum on the client, verify that the values match, and verify that the size is what you expected.</p> <p>A job ID does not expire for at least 24 hours after Amazon Glacier completes the job. That is, you can download the job output within the 24 hours period after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory</a>, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/downloading-an-archive.html">Downloading an Archive</a>, and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-job-output-get.html">Get Job Output </a> </p>
    fn get_job_output(
        &self,
        input: GetJobOutputInput,
    ) -> RusotoFuture<GetJobOutputOutput, GetJobOutputError>;

    /// <p>This operation retrieves the <code>access-policy</code> subresource set on the vault; for more information on setting this subresource, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-SetVaultAccessPolicy.html">Set Vault Access Policy (PUT access-policy)</a>. If there is no access policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>.</p>
    fn get_vault_access_policy(
        &self,
        input: GetVaultAccessPolicyInput,
    ) -> RusotoFuture<GetVaultAccessPolicyOutput, GetVaultAccessPolicyError>;

    /// <p>This operation retrieves the following attributes from the <code>lock-policy</code> subresource set on the specified vault: </p> <ul> <li> <p>The vault lock policy set on the vault.</p> </li> <li> <p>The state of the vault lock, which is either <code>InProgess</code> or <code>Locked</code>.</p> </li> <li> <p>When the lock ID expires. The lock ID is used to complete the vault locking process.</p> </li> <li> <p>When the vault lock was initiated and put into the <code>InProgress</code> state.</p> </li> </ul> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can abort the vault locking process by calling <a>AbortVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>If there is no vault lock policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault lock policies, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p>
    fn get_vault_lock(
        &self,
        input: GetVaultLockInput,
    ) -> RusotoFuture<GetVaultLockOutput, GetVaultLockError>;

    /// <p>This operation retrieves the <code>notification-configuration</code> subresource of the specified vault.</p> <p>For information about setting a notification configuration on a vault, see <a>SetVaultNotifications</a>. If a notification configuration for a vault is not set, the operation returns a <code>404 Not Found</code> error. For more information about vault notifications, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-get.html">Get Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn get_vault_notifications(
        &self,
        input: GetVaultNotificationsInput,
    ) -> RusotoFuture<GetVaultNotificationsOutput, GetVaultNotificationsError>;

    /// <p>This operation initiates a job of the specified type, which can be a select, an archival retrieval, or a vault retrieval. For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job</a>. </p>
    fn initiate_job(
        &self,
        input: InitiateJobInput,
    ) -> RusotoFuture<InitiateJobOutput, InitiateJobError>;

    /// <p>This operation initiates a multipart upload. Amazon Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive (see <a>UploadMultipartPart</a>).</p> <p>When you initiate a multipart upload, you specify the part size in number of bytes. The part size must be a megabyte (1024 KB) multiplied by a power of 2-for example, 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB.</p> <p>Every part you upload to this resource (see <a>UploadMultipartPart</a>), except the last one, must have the same size. The last one can be the same size or smaller. For example, suppose you want to upload a 16.2 MB file. If you initiate the multipart upload with a part size of 4 MB, you will upload four parts of 4 MB each and one part of 0.2 MB. </p> <note> <p>You don't need to know the size of the archive when you start a multipart upload because Amazon Glacier does not require you to specify the overall archive size.</p> </note> <p>After you complete the multipart upload, Amazon Glacier removes the multipart upload resource referenced by the ID. Amazon Glacier also removes the multipart upload resource if you cancel the multipart upload or it may be removed if there is no activity for a period of 24 hours.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-initiate-upload.html">Initiate Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn initiate_multipart_upload(
        &self,
        input: InitiateMultipartUploadInput,
    ) -> RusotoFuture<InitiateMultipartUploadOutput, InitiateMultipartUploadError>;

    /// <p>This operation initiates the vault locking process by doing the following:</p> <ul> <li> <p>Installing a vault lock policy on the specified vault.</p> </li> <li> <p>Setting the lock state of vault lock to <code>InProgress</code>.</p> </li> <li> <p>Returning a lock ID, which is used to complete the vault locking process.</p> </li> </ul> <p>You can set one vault lock policy for each vault and this policy can be up to 20 KB in size. For more information about vault lock policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>You must complete the vault locking process within 24 hours after the vault lock enters the <code>InProgress</code> state. After the 24 hour window ends, the lock ID expires, the vault automatically exits the <code>InProgress</code> state, and the vault lock policy is removed from the vault. You call <a>CompleteVaultLock</a> to complete the vault locking process by setting the state of the vault lock to <code>Locked</code>. </p> <p>After a vault lock is in the <code>Locked</code> state, you cannot initiate a new vault lock for the vault.</p> <p>You can abort the vault locking process by calling <a>AbortVaultLock</a>. You can get the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>.</p> <p>If this operation is called when the vault lock is in the <code>InProgress</code> state, the operation returns an <code>AccessDeniedException</code> error. When the vault lock is in the <code>InProgress</code> state you must call <a>AbortVaultLock</a> before you can initiate a new vault lock policy. </p>
    fn initiate_vault_lock(
        &self,
        input: InitiateVaultLockInput,
    ) -> RusotoFuture<InitiateVaultLockOutput, InitiateVaultLockError>;

    /// <p>This operation lists jobs for a vault, including jobs that are in-progress and jobs that have recently finished. The List Job operation returns a list of these jobs sorted by job initiation time.</p> <note> <p>Amazon Glacier retains recently completed jobs for a period before deleting them; however, it eventually removes completed jobs. The output of completed jobs can be retrieved. Retaining completed jobs for a period of time after they have completed enables you to get a job output in the event you miss the job completion notification or your first attempt to download it fails. For example, suppose you start an archive retrieval job to download an archive. After the job completes, you start to download the archive but encounter a network error. In this scenario, you can retry and download the archive while the job exists.</p> </note> <p>The List Jobs operation supports pagination. You should always check the response <code>Marker</code> field. If there are no more jobs to list, the <code>Marker</code> field is set to <code>null</code>. If there are more jobs to list, the <code>Marker</code> field is set to a non-null value, which you can use to continue the pagination of the list. To return a list of jobs that begins at a specific job, set the marker request parameter to the <code>Marker</code> value for that job that you obtained from a previous List Jobs request.</p> <p>You can set a maximum limit for the number of jobs returned in the response by specifying the <code>limit</code> parameter in the request. The default limit is 50. The number of jobs returned might be fewer than the limit, but the number of returned jobs never exceeds the limit.</p> <p>Additionally, you can filter the jobs list returned by specifying the optional <code>statuscode</code> parameter or <code>completed</code> parameter, or both. Using the <code>statuscode</code> parameter, you can specify to return only jobs that match either the <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code> status. Using the <code>completed</code> parameter, you can specify to return only jobs that were completed (<code>true</code>) or jobs that were not completed (<code>false</code>).</p> <p>For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-jobs-get.html">List Jobs</a>. </p>
    fn list_jobs(&self, input: ListJobsInput) -> RusotoFuture<ListJobsOutput, ListJobsError>;

    /// <p>This operation lists in-progress multipart uploads for the specified vault. An in-progress multipart upload is a multipart upload that has been initiated by an <a>InitiateMultipartUpload</a> request, but has not yet been completed or aborted. The list returned in the List Multipart Upload response has no guaranteed order. </p> <p>The List Multipart Uploads operation supports pagination. By default, this operation returns up to 50 multipart uploads in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of multipart uploads that begins at a specific upload, set the <code>marker</code> request parameter to the value you obtained from a previous List Multipart Upload request. You can also limit the number of uploads returned in the response by specifying the <code>limit</code> parameter in the request.</p> <p>Note the difference between this operation and listing parts (<a>ListParts</a>). The List Multipart Uploads operation lists all multipart uploads for a vault and does not require a multipart upload ID. The List Parts operation requires a multipart upload ID since parts are associated with a single upload.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-uploads.html">List Multipart Uploads </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsInput,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError>;

    /// <p>This operation lists the parts of an archive that have been uploaded in a specific multipart upload. You can make this request at any time during an in-progress multipart upload before you complete the upload (see <a>CompleteMultipartUpload</a>. List Parts returns an error for completed uploads. The list returned in the List Parts response is sorted by part range. </p> <p>The List Parts operation supports pagination. By default, this operation returns up to 50 uploaded parts in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of parts that begins at a specific part, set the <code>marker</code> request parameter to the value you obtained from a previous List Parts request. You can also limit the number of parts returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-parts.html">List Parts</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn list_parts(&self, input: ListPartsInput) -> RusotoFuture<ListPartsOutput, ListPartsError>;

    /// <p>This operation lists the provisioned capacity units for the specified AWS account.</p>
    fn list_provisioned_capacity(
        &self,
        input: ListProvisionedCapacityInput,
    ) -> RusotoFuture<ListProvisionedCapacityOutput, ListProvisionedCapacityError>;

    /// <p>This operation lists all the tags attached to a vault. The operation returns an empty map if there are no tags. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>.</p>
    fn list_tags_for_vault(
        &self,
        input: ListTagsForVaultInput,
    ) -> RusotoFuture<ListTagsForVaultOutput, ListTagsForVaultError>;

    /// <p>This operation lists all vaults owned by the calling user's account. The list returned in the response is ASCII-sorted by vault name.</p> <p>By default, this operation returns up to 10 items. If there are more vaults to list, the response <code>marker</code> field contains the vault Amazon Resource Name (ARN) at which to continue the list with a new List Vaults request; otherwise, the <code>marker</code> field is <code>null</code>. To return a list of vaults that begins at a specific vault, set the <code>marker</code> request parameter to the vault ARN you obtained from a previous List Vaults request. You can also limit the number of vaults returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html">List Vaults </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn list_vaults(
        &self,
        input: ListVaultsInput,
    ) -> RusotoFuture<ListVaultsOutput, ListVaultsError>;

    /// <p>This operation purchases a provisioned capacity unit for an AWS account. </p>
    fn purchase_provisioned_capacity(
        &self,
        input: PurchaseProvisionedCapacityInput,
    ) -> RusotoFuture<PurchaseProvisionedCapacityOutput, PurchaseProvisionedCapacityError>;

    /// <p>This operation removes one or more tags from the set of tags attached to a vault. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>. This operation is idempotent. The operation will be successful, even if there are no tags attached to the vault. </p>
    fn remove_tags_from_vault(
        &self,
        input: RemoveTagsFromVaultInput,
    ) -> RusotoFuture<(), RemoveTagsFromVaultError>;

    /// <p>This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful PUT operation.</p> <p>The set policy operation does not affect retrieval jobs that were in progress before the policy was enacted. For more information about data retrieval policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>. </p>
    fn set_data_retrieval_policy(
        &self,
        input: SetDataRetrievalPolicyInput,
    ) -> RusotoFuture<(), SetDataRetrievalPolicyError>;

    /// <p>This operation configures an access policy for a vault and will overwrite an existing policy. To configure a vault access policy, send a PUT request to the <code>access-policy</code> subresource of the vault. An access policy is specific to a vault and is also called a vault subresource. You can set one access policy per vault and the policy can be up to 20 KB in size. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    fn set_vault_access_policy(
        &self,
        input: SetVaultAccessPolicyInput,
    ) -> RusotoFuture<(), SetVaultAccessPolicyError>;

    /// <p>This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications.</p> <p>To configure vault notifications, send a PUT request to the <code>notification-configuration</code> subresource of the vault. The request should include a JSON document that provides an Amazon SNS topic and specific events for which you want Amazon Glacier to send notifications to the topic.</p> <p>Amazon SNS topics must grant permission to the vault to be allowed to publish notifications to the topic. You can configure a vault to publish a notification for the following vault events:</p> <ul> <li> <p> <b>ArchiveRetrievalCompleted</b> This event occurs when a job that was initiated for an archive retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> <li> <p> <b>InventoryRetrievalCompleted</b> This event occurs when a job that was initiated for an inventory retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> </ul> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-put.html">Set Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn set_vault_notifications(
        &self,
        input: SetVaultNotificationsInput,
    ) -> RusotoFuture<(), SetVaultNotificationsError>;

    /// <p>This operation adds an archive to a vault. This is a synchronous operation, and for a successful upload, your data is durably persisted. Amazon Glacier returns the archive ID in the <code>x-amz-archive-id</code> header of the response. </p> <p>You must use the archive ID to access your data in Amazon Glacier. After you upload an archive, you should save the archive ID returned so that you can retrieve or delete the archive later. Besides saving the archive ID, you can also index it and give it a friendly name to allow for better searching. You can also use the optional archive description field to specify how the archive is referred to in an external index of archives, such as you might create in Amazon DynamoDB. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>. </p> <p>You must provide a SHA256 tree hash of the data you are uploading. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. </p> <p>You can optionally specify an archive description of up to 1,024 printable ASCII characters. You can get the archive description when you either retrieve the archive or get the vault inventory. For more information, see <a>InitiateJob</a>. Amazon Glacier does not interpret the description in any way. An archive description does not need to be unique. You cannot use the description to retrieve or sort the archive list. </p> <p>Archives are immutable. After you upload an archive, you cannot edit the archive or its description.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-an-archive.html">Uploading an Archive in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn upload_archive(
        &self,
        input: UploadArchiveInput,
    ) -> RusotoFuture<ArchiveCreationOutput, UploadArchiveError>;

    /// <p>This operation uploads a part of an archive. You can upload archive parts in any order. You can also upload them in parallel. You can upload up to 10,000 parts for a multipart upload.</p> <p>Amazon Glacier rejects your upload part request if any of the following conditions is true:</p> <ul> <li> <p> <b>SHA256 tree hash does not match</b>To ensure that part data is not corrupted in transmission, you compute a SHA256 tree hash of the part and include it in your request. Upon receiving the part data, Amazon Glacier also computes a SHA256 tree hash. If these hash values don't match, the operation fails. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>.</p> </li> <li> <p> <b>Part size does not match</b>The size of each part except the last must match the size specified in the corresponding <a>InitiateMultipartUpload</a> request. The size of the last part must be the same size as, or smaller than, the specified size.</p> <note> <p>If you upload a part whose size is smaller than the part size you specified in your initiate multipart upload request and that part is not the last part, then the upload part request will succeed. However, the subsequent Complete Multipart Upload request will fail.</p> </note> </li> <li> <p> <b>Range does not align</b>The byte range value in the request does not align with the part size specified in the corresponding initiate request. For example, if you specify a part size of 4194304 bytes (4 MB), then 0 to 4194303 bytes (4 MB - 1) and 4194304 (4 MB) to 8388607 (8 MB - 1) are valid part ranges. However, if you set a range value of 2 MB to 6 MB, the range does not align with the part size and the upload will fail. </p> </li> </ul> <p>This operation is idempotent. If you upload the same part multiple times, the data included in the most recent request overwrites the previously uploaded data.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-upload-part.html">Upload Part </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn upload_multipart_part(
        &self,
        input: UploadMultipartPartInput,
    ) -> RusotoFuture<UploadMultipartPartOutput, UploadMultipartPartError>;
}
/// A client for the Amazon Glacier API.
pub struct GlacierClient {
    client: Client,
    region: region::Region,
}

impl GlacierClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GlacierClient {
        GlacierClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GlacierClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        GlacierClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Glacier for GlacierClient {
    /// <p>This operation aborts a multipart upload identified by the upload ID.</p> <p>After the Abort Multipart Upload request succeeds, you cannot upload any more parts to the multipart upload or complete the multipart upload. Aborting a completed upload fails. However, aborting an already-aborted upload will succeed, for a short time. For more information about uploading a part and completing a multipart upload, see <a>UploadMultipartPart</a> and <a>CompleteMultipartUpload</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadInput,
    ) -> RusotoFuture<(), AbortMultipartUploadError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}",
            account_id = input.account_id,
            upload_id = input.upload_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AbortMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation aborts the vault locking process if the vault lock is not in the <code>Locked</code> state. If the vault lock is in the <code>Locked</code> state when this operation is requested, the operation returns an <code>AccessDeniedException</code> error. Aborting the vault locking process removes the vault lock policy from the specified vault. </p> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can get the state of a vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. For more information about vault lock policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>This operation is idempotent. You can successfully invoke this operation multiple times, if the vault lock is in the <code>InProgress</code> state or if there is no policy associated with the vault.</p>
    fn abort_vault_lock(
        &self,
        input: AbortVaultLockInput,
    ) -> RusotoFuture<(), AbortVaultLockError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AbortVaultLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation adds the specified tags to a vault. Each tag is composed of a key and a value. Each vault can have up to 10 tags. If your request would cause the tag limit for the vault to be exceeded, the operation throws the <code>LimitExceededException</code> error. If a tag already exists on the vault under a specified key, the existing key value will be overwritten. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>. </p>
    fn add_tags_to_vault(
        &self,
        input: AddTagsToVaultInput,
    ) -> RusotoFuture<(), AddTagsToVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/tags",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "add");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>You call this operation to inform Amazon Glacier that all the archive parts have been uploaded and that Amazon Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Amazon Glacier returns the URI path of the newly created archive resource. Using the URI path, you can then access the archive. After you upload an archive, you should save the archive ID returned to retrieve the archive at a later point. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>.</p> <p>In the request, you must include the computed SHA256 tree hash of the entire archive you have uploaded. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. On the server side, Amazon Glacier also constructs the SHA256 tree hash of the assembled archive. If the values match, Amazon Glacier saves the archive to the vault; otherwise, it returns an error, and the operation fails. The <a>ListParts</a> operation returns a list of parts uploaded for a specific multipart upload. It includes checksum information for each uploaded part that can be used to debug a bad checksum issue.</p> <p>Additionally, Amazon Glacier also checks for any missing content ranges when assembling the archive, if missing content ranges are found, Amazon Glacier returns an error and the operation fails.</p> <p>Complete Multipart Upload is an idempotent operation. After your first successful complete multipart upload, if you call the operation again within a short period, the operation will succeed and return the same archive ID. This is useful in the event you experience a network issue that causes an aborted connection or receive a 500 server error, in which case you can repeat your Complete Multipart Upload request and get the same archive ID without creating duplicate archives. Note, however, that after the multipart upload completes, you cannot call the List Parts operation and the multipart upload will not appear in List Multipart Uploads response, even if idempotent complete is possible.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-complete-upload.html">Complete Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadInput,
    ) -> RusotoFuture<ArchiveCreationOutput, CompleteMultipartUploadError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}",
            account_id = input.account_id,
            upload_id = input.upload_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        if let Some(ref archive_size) = input.archive_size {
            request.add_header("x-amz-archive-size", &archive_size.to_string());
        }

        if let Some(ref checksum) = input.checksum {
            request.add_header("x-amz-sha256-tree-hash", &checksum.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<ArchiveCreationOutput>(&body).unwrap();

                    if let Some(archive_id) = response.headers.get("x-amz-archive-id") {
                        let value = archive_id.to_owned();
                        result.archive_id = Some(value)
                    };
                    if let Some(checksum) = response.headers.get("x-amz-sha256-tree-hash") {
                        let value = checksum.to_owned();
                        result.checksum = Some(value)
                    };
                    if let Some(location) = response.headers.get("Location") {
                        let value = location.to_owned();
                        result.location = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CompleteMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation completes the vault locking process by transitioning the vault lock from the <code>InProgress</code> state to the <code>Locked</code> state, which causes the vault lock policy to become unchangeable. A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. You can obtain the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>This operation is idempotent. This request is always successful if the vault lock is in the <code>Locked</code> state and the provided lock ID matches the lock ID originally used to lock the vault.</p> <p>If an invalid lock ID is passed in the request when the vault lock is in the <code>Locked</code> state, the operation returns an <code>AccessDeniedException</code> error. If an invalid lock ID is passed in the request when the vault lock is in the <code>InProgress</code> state, the operation throws an <code>InvalidParameter</code> error.</p>
    fn complete_vault_lock(
        &self,
        input: CompleteVaultLockInput,
    ) -> RusotoFuture<(), CompleteVaultLockError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy/{lock_id}",
            account_id = input.account_id,
            lock_id = input.lock_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CompleteVaultLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation creates a new vault with the specified name. The name of the vault must be unique within a region for an AWS account. You can create up to 1,000 vaults per account. If you need to create more vaults, contact Amazon Glacier.</p> <p>You must use the following guidelines when naming a vault.</p> <ul> <li> <p>Names can be between 1 and 255 characters long.</p> </li> <li> <p>Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).</p> </li> </ul> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/creating-vaults.html">Creating a Vault in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-put.html">Create Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn create_vault(
        &self,
        input: CreateVaultInput,
    ) -> RusotoFuture<CreateVaultOutput, CreateVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<CreateVaultOutput>(&body).unwrap();

                    if let Some(location) = response.headers.get("Location") {
                        let value = location.to_owned();
                        result.location = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation deletes an archive from a vault. Subsequent requests to initiate a retrieval of this archive will fail. Archive retrievals that are in progress for this archive ID may or may not succeed according to the following scenarios:</p> <ul> <li> <p>If the archive retrieval job is actively preparing the data for download when Amazon Glacier receives the delete archive request, the archival retrieval operation might fail.</p> </li> <li> <p>If the archive retrieval job has successfully prepared the archive for download when Amazon Glacier receives the delete archive request, you will be able to download the output.</p> </li> </ul> <p>This operation is idempotent. Attempting to delete an already-deleted archive does not result in an error.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-an-archive.html">Deleting an Archive in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn delete_archive(&self, input: DeleteArchiveInput) -> RusotoFuture<(), DeleteArchiveError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/archives/{archive_id}",
            account_id = input.account_id,
            archive_id = input.archive_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteArchiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation deletes a vault. Amazon Glacier will delete a vault only if there are no archives in the vault as of the last inventory and there have been no writes to the vault since the last inventory. If either of these conditions is not satisfied, the vault deletion fails (that is, the vault is not removed) and Amazon Glacier returns an error. You can use <a>DescribeVault</a> to return the number of archives in a vault, and you can use <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job (POST jobs)</a> to initiate a new inventory retrieval for a vault. The inventory contains the archive IDs you use to delete archives using <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive (DELETE archive)</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-vaults.html">Deleting a Vault in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-delete.html">Delete Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn delete_vault(&self, input: DeleteVaultInput) -> RusotoFuture<(), DeleteVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely remove the access policy, and you might still see the effect of the policy for a short time after you send the delete request.</p> <p>This operation is idempotent. You can invoke delete multiple times, even if there is no policy associated with the vault. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    fn delete_vault_access_policy(
        &self,
        input: DeleteVaultAccessPolicyInput,
    ) -> RusotoFuture<(), DeleteVaultAccessPolicyError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/access-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVaultAccessPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation deletes the notification configuration set for a vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely disable the notifications and you might still receive some notifications for a short time after you send the delete request.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-delete.html">Delete Vault Notification Configuration </a> in the Amazon Glacier Developer Guide. </p>
    fn delete_vault_notifications(
        &self,
        input: DeleteVaultNotificationsInput,
    ) -> RusotoFuture<(), DeleteVaultNotificationsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/notification-configuration",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVaultNotificationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation returns information about a job you previously initiated, including the job initiation date, the user who initiated the job, the job status code/message and the Amazon SNS topic to notify after Amazon Glacier completes the job. For more information about initiating a job, see <a>InitiateJob</a>. </p> <note> <p>This operation enables you to check the status of your job. However, it is strongly recommended that you set up an Amazon SNS topic and specify it in your initiate job request so that Amazon Glacier can notify the topic after it completes the job.</p> </note> <p>A job ID will not expire for at least 24 hours after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-describe-job-get.html">Describe Job</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn describe_job(
        &self,
        input: DescribeJobInput,
    ) -> RusotoFuture<GlacierJobDescription, DescribeJobError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/jobs/{job_id}",
            account_id = input.account_id,
            job_id = input.job_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GlacierJobDescription>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation returns information about a vault, including the vault's Amazon Resource Name (ARN), the date the vault was created, the number of archives it contains, and the total size of all the archives in the vault. The number of archives and their total size are as of the last inventory generation. This means that if you add or remove an archive from a vault, and then immediately use Describe Vault, the change in contents will not be immediately reflected. If you want to retrieve the latest inventory of the vault, use <a>InitiateJob</a>. Amazon Glacier generates vault inventories approximately daily. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory in Amazon Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-get.html">Describe Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn describe_vault(
        &self,
        input: DescribeVaultInput,
    ) -> RusotoFuture<DescribeVaultOutput, DescribeVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeVaultOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>.</p>
    fn get_data_retrieval_policy(
        &self,
        input: GetDataRetrievalPolicyInput,
    ) -> RusotoFuture<GetDataRetrievalPolicyOutput, GetDataRetrievalPolicyError> {
        let request_uri = format!(
            "/{account_id}/policies/data-retrieval",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDataRetrievalPolicyOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDataRetrievalPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation downloads the output of the job you initiated using <a>InitiateJob</a>. Depending on the job type you specified when you initiated the job, the output will be either the content of an archive or a vault inventory.</p> <p>You can download all the job output or download a portion of the output by specifying a byte range. In the case of an archive retrieval job, depending on the byte range you specify, Amazon Glacier returns the checksum for the portion of the data. You can compute the checksum on the client and verify that the values match to ensure the portion you downloaded is the correct data.</p> <p>A job ID will not expire for at least 24 hours after Amazon Glacier completes the job. That a byte range. For both archive and inventory retrieval jobs, you should verify the downloaded size against the size returned in the headers from the <b>Get Job Output</b> response.</p> <p>For archive retrieval jobs, you should also verify that the size is what you expected. If you download a portion of the output, the expected size is based on the range of bytes you specified. For example, if you specify a range of <code>bytes=0-1048575</code>, you should verify your download size is 1,048,576 bytes. If you download an entire archive, the expected size is the size of the archive when you uploaded it to Amazon Glacier The expected size is also returned in the headers from the <b>Get Job Output</b> response.</p> <p>In the case of an archive retrieval job, depending on the byte range you specify, Amazon Glacier returns the checksum for the portion of the data. To ensure the portion you downloaded is the correct data, compute the checksum on the client, verify that the values match, and verify that the size is what you expected.</p> <p>A job ID does not expire for at least 24 hours after Amazon Glacier completes the job. That is, you can download the job output within the 24 hours period after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory</a>, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/downloading-an-archive.html">Downloading an Archive</a>, and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-job-output-get.html">Get Job Output </a> </p>
    fn get_job_output(
        &self,
        input: GetJobOutputInput,
    ) -> RusotoFuture<GetJobOutputOutput, GetJobOutputError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/jobs/{job_id}/output",
            account_id = input.account_id,
            job_id = input.job_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetJobOutputOutput::default();
                    result.body = Some(response.body);

                    if let Some(accept_ranges) = response.headers.get("Accept-Ranges") {
                        let value = accept_ranges.to_owned();
                        result.accept_ranges = Some(value)
                    };
                    if let Some(archive_description) =
                        response.headers.get("x-amz-archive-description")
                    {
                        let value = archive_description.to_owned();
                        result.archive_description = Some(value)
                    };
                    if let Some(checksum) = response.headers.get("x-amz-sha256-tree-hash") {
                        let value = checksum.to_owned();
                        result.checksum = Some(value)
                    };
                    if let Some(content_range) = response.headers.get("Content-Range") {
                        let value = content_range.to_owned();
                        result.content_range = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };
                    result.status = Some(response.status.as_u16() as i64);
                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetJobOutputError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation retrieves the <code>access-policy</code> subresource set on the vault; for more information on setting this subresource, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-SetVaultAccessPolicy.html">Set Vault Access Policy (PUT access-policy)</a>. If there is no access policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>.</p>
    fn get_vault_access_policy(
        &self,
        input: GetVaultAccessPolicyInput,
    ) -> RusotoFuture<GetVaultAccessPolicyOutput, GetVaultAccessPolicyError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/access-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetVaultAccessPolicyOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVaultAccessPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation retrieves the following attributes from the <code>lock-policy</code> subresource set on the specified vault: </p> <ul> <li> <p>The vault lock policy set on the vault.</p> </li> <li> <p>The state of the vault lock, which is either <code>InProgess</code> or <code>Locked</code>.</p> </li> <li> <p>When the lock ID expires. The lock ID is used to complete the vault locking process.</p> </li> <li> <p>When the vault lock was initiated and put into the <code>InProgress</code> state.</p> </li> </ul> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can abort the vault locking process by calling <a>AbortVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>If there is no vault lock policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault lock policies, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p>
    fn get_vault_lock(
        &self,
        input: GetVaultLockInput,
    ) -> RusotoFuture<GetVaultLockOutput, GetVaultLockError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetVaultLockOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVaultLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation retrieves the <code>notification-configuration</code> subresource of the specified vault.</p> <p>For information about setting a notification configuration on a vault, see <a>SetVaultNotifications</a>. If a notification configuration for a vault is not set, the operation returns a <code>404 Not Found</code> error. For more information about vault notifications, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-get.html">Get Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn get_vault_notifications(
        &self,
        input: GetVaultNotificationsInput,
    ) -> RusotoFuture<GetVaultNotificationsOutput, GetVaultNotificationsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/notification-configuration",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetVaultNotificationsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVaultNotificationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation initiates a job of the specified type, which can be a select, an archival retrieval, or a vault retrieval. For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job</a>. </p>
    fn initiate_job(
        &self,
        input: InitiateJobInput,
    ) -> RusotoFuture<InitiateJobOutput, InitiateJobError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/jobs",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input.job_parameters).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<InitiateJobOutput>(&body).unwrap();

                    if let Some(job_id) = response.headers.get("x-amz-job-id") {
                        let value = job_id.to_owned();
                        result.job_id = Some(value)
                    };
                    if let Some(job_output_path) = response.headers.get("x-amz-job-output-path") {
                        let value = job_output_path.to_owned();
                        result.job_output_path = Some(value)
                    };
                    if let Some(location) = response.headers.get("Location") {
                        let value = location.to_owned();
                        result.location = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InitiateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation initiates a multipart upload. Amazon Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive (see <a>UploadMultipartPart</a>).</p> <p>When you initiate a multipart upload, you specify the part size in number of bytes. The part size must be a megabyte (1024 KB) multiplied by a power of 2-for example, 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB.</p> <p>Every part you upload to this resource (see <a>UploadMultipartPart</a>), except the last one, must have the same size. The last one can be the same size or smaller. For example, suppose you want to upload a 16.2 MB file. If you initiate the multipart upload with a part size of 4 MB, you will upload four parts of 4 MB each and one part of 0.2 MB. </p> <note> <p>You don't need to know the size of the archive when you start a multipart upload because Amazon Glacier does not require you to specify the overall archive size.</p> </note> <p>After you complete the multipart upload, Amazon Glacier removes the multipart upload resource referenced by the ID. Amazon Glacier also removes the multipart upload resource if you cancel the multipart upload or it may be removed if there is no activity for a period of 24 hours.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-initiate-upload.html">Initiate Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn initiate_multipart_upload(
        &self,
        input: InitiateMultipartUploadInput,
    ) -> RusotoFuture<InitiateMultipartUploadOutput, InitiateMultipartUploadError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        if let Some(ref archive_description) = input.archive_description {
            request.add_header(
                "x-amz-archive-description",
                &archive_description.to_string(),
            );
        }

        if let Some(ref part_size) = input.part_size {
            request.add_header("x-amz-part-size", &part_size.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<InitiateMultipartUploadOutput>(&body).unwrap();

                    if let Some(location) = response.headers.get("Location") {
                        let value = location.to_owned();
                        result.location = Some(value)
                    };
                    if let Some(upload_id) = response.headers.get("x-amz-multipart-upload-id") {
                        let value = upload_id.to_owned();
                        result.upload_id = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InitiateMultipartUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation initiates the vault locking process by doing the following:</p> <ul> <li> <p>Installing a vault lock policy on the specified vault.</p> </li> <li> <p>Setting the lock state of vault lock to <code>InProgress</code>.</p> </li> <li> <p>Returning a lock ID, which is used to complete the vault locking process.</p> </li> </ul> <p>You can set one vault lock policy for each vault and this policy can be up to 20 KB in size. For more information about vault lock policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>You must complete the vault locking process within 24 hours after the vault lock enters the <code>InProgress</code> state. After the 24 hour window ends, the lock ID expires, the vault automatically exits the <code>InProgress</code> state, and the vault lock policy is removed from the vault. You call <a>CompleteVaultLock</a> to complete the vault locking process by setting the state of the vault lock to <code>Locked</code>. </p> <p>After a vault lock is in the <code>Locked</code> state, you cannot initiate a new vault lock for the vault.</p> <p>You can abort the vault locking process by calling <a>AbortVaultLock</a>. You can get the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>.</p> <p>If this operation is called when the vault lock is in the <code>InProgress</code> state, the operation returns an <code>AccessDeniedException</code> error. When the vault lock is in the <code>InProgress</code> state you must call <a>AbortVaultLock</a> before you can initiate a new vault lock policy. </p>
    fn initiate_vault_lock(
        &self,
        input: InitiateVaultLockInput,
    ) -> RusotoFuture<InitiateVaultLockOutput, InitiateVaultLockError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input.policy).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<InitiateVaultLockOutput>(&body).unwrap();

                    if let Some(lock_id) = response.headers.get("x-amz-lock-id") {
                        let value = lock_id.to_owned();
                        result.lock_id = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InitiateVaultLockError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists jobs for a vault, including jobs that are in-progress and jobs that have recently finished. The List Job operation returns a list of these jobs sorted by job initiation time.</p> <note> <p>Amazon Glacier retains recently completed jobs for a period before deleting them; however, it eventually removes completed jobs. The output of completed jobs can be retrieved. Retaining completed jobs for a period of time after they have completed enables you to get a job output in the event you miss the job completion notification or your first attempt to download it fails. For example, suppose you start an archive retrieval job to download an archive. After the job completes, you start to download the archive but encounter a network error. In this scenario, you can retry and download the archive while the job exists.</p> </note> <p>The List Jobs operation supports pagination. You should always check the response <code>Marker</code> field. If there are no more jobs to list, the <code>Marker</code> field is set to <code>null</code>. If there are more jobs to list, the <code>Marker</code> field is set to a non-null value, which you can use to continue the pagination of the list. To return a list of jobs that begins at a specific job, set the marker request parameter to the <code>Marker</code> value for that job that you obtained from a previous List Jobs request.</p> <p>You can set a maximum limit for the number of jobs returned in the response by specifying the <code>limit</code> parameter in the request. The default limit is 50. The number of jobs returned might be fewer than the limit, but the number of returned jobs never exceeds the limit.</p> <p>Additionally, you can filter the jobs list returned by specifying the optional <code>statuscode</code> parameter or <code>completed</code> parameter, or both. Using the <code>statuscode</code> parameter, you can specify to return only jobs that match either the <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code> status. Using the <code>completed</code> parameter, you can specify to return only jobs that were completed (<code>true</code>) or jobs that were not completed (<code>false</code>).</p> <p>For more information about using this operation, see the documentation for the underlying REST API <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-jobs-get.html">List Jobs</a>. </p>
    fn list_jobs(&self, input: ListJobsInput) -> RusotoFuture<ListJobsOutput, ListJobsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/jobs",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut params = Params::new();
        if let Some(ref x) = input.completed {
            params.put("completed", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.statuscode {
            params.put("statuscode", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListJobsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListJobsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists in-progress multipart uploads for the specified vault. An in-progress multipart upload is a multipart upload that has been initiated by an <a>InitiateMultipartUpload</a> request, but has not yet been completed or aborted. The list returned in the List Multipart Upload response has no guaranteed order. </p> <p>The List Multipart Uploads operation supports pagination. By default, this operation returns up to 50 multipart uploads in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of multipart uploads that begins at a specific upload, set the <code>marker</code> request parameter to the value you obtained from a previous List Multipart Upload request. You can also limit the number of uploads returned in the response by specifying the <code>limit</code> parameter in the request.</p> <p>Note the difference between this operation and listing parts (<a>ListParts</a>). The List Multipart Uploads operation lists all multipart uploads for a vault and does not require a multipart upload ID. The List Parts operation requires a multipart upload ID since parts are associated with a single upload.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-uploads.html">List Multipart Uploads </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsInput,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListMultipartUploadsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListMultipartUploadsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists the parts of an archive that have been uploaded in a specific multipart upload. You can make this request at any time during an in-progress multipart upload before you complete the upload (see <a>CompleteMultipartUpload</a>. List Parts returns an error for completed uploads. The list returned in the List Parts response is sorted by part range. </p> <p>The List Parts operation supports pagination. By default, this operation returns up to 50 uploaded parts in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of parts that begins at a specific part, set the <code>marker</code> request parameter to the value you obtained from a previous List Parts request. You can also limit the number of parts returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-parts.html">List Parts</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn list_parts(&self, input: ListPartsInput) -> RusotoFuture<ListPartsOutput, ListPartsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}",
            account_id = input.account_id,
            upload_id = input.upload_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListPartsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPartsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists the provisioned capacity units for the specified AWS account.</p>
    fn list_provisioned_capacity(
        &self,
        input: ListProvisionedCapacityInput,
    ) -> RusotoFuture<ListProvisionedCapacityOutput, ListProvisionedCapacityError> {
        let request_uri = format!(
            "/{account_id}/provisioned-capacity",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListProvisionedCapacityOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProvisionedCapacityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists all the tags attached to a vault. The operation returns an empty map if there are no tags. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>.</p>
    fn list_tags_for_vault(
        &self,
        input: ListTagsForVaultInput,
    ) -> RusotoFuture<ListTagsForVaultOutput, ListTagsForVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/tags",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListTagsForVaultOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation lists all vaults owned by the calling user's account. The list returned in the response is ASCII-sorted by vault name.</p> <p>By default, this operation returns up to 10 items. If there are more vaults to list, the response <code>marker</code> field contains the vault Amazon Resource Name (ARN) at which to continue the list with a new List Vaults request; otherwise, the <code>marker</code> field is <code>null</code>. To return a list of vaults that begins at a specific vault, set the <code>marker</code> request parameter to the vault ARN you obtained from a previous List Vaults request. You can also limit the number of vaults returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html">List Vaults </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn list_vaults(
        &self,
        input: ListVaultsInput,
    ) -> RusotoFuture<ListVaultsOutput, ListVaultsError> {
        let request_uri = format!("/{account_id}/vaults", account_id = input.account_id);

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListVaultsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListVaultsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation purchases a provisioned capacity unit for an AWS account. </p>
    fn purchase_provisioned_capacity(
        &self,
        input: PurchaseProvisionedCapacityInput,
    ) -> RusotoFuture<PurchaseProvisionedCapacityOutput, PurchaseProvisionedCapacityError> {
        let request_uri = format!(
            "/{account_id}/provisioned-capacity",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<PurchaseProvisionedCapacityOutput>(&body).unwrap();

                    if let Some(capacity_id) = response.headers.get("x-amz-capacity-id") {
                        let value = capacity_id.to_owned();
                        result.capacity_id = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PurchaseProvisionedCapacityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation removes one or more tags from the set of tags attached to a vault. For more information about tags, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon Glacier Resources</a>. This operation is idempotent. The operation will be successful, even if there are no tags attached to the vault. </p>
    fn remove_tags_from_vault(
        &self,
        input: RemoveTagsFromVaultInput,
    ) -> RusotoFuture<(), RemoveTagsFromVaultError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/tags",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "remove");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromVaultError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful PUT operation.</p> <p>The set policy operation does not affect retrieval jobs that were in progress before the policy was enacted. For more information about data retrieval policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>. </p>
    fn set_data_retrieval_policy(
        &self,
        input: SetDataRetrievalPolicyInput,
    ) -> RusotoFuture<(), SetDataRetrievalPolicyError> {
        let request_uri = format!(
            "/{account_id}/policies/data-retrieval",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SetDataRetrievalPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation configures an access policy for a vault and will overwrite an existing policy. To configure a vault access policy, send a PUT request to the <code>access-policy</code> subresource of the vault. An access policy is specific to a vault and is also called a vault subresource. You can set one access policy per vault and the policy can be up to 20 KB in size. For more information about vault access policies, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    fn set_vault_access_policy(
        &self,
        input: SetVaultAccessPolicyInput,
    ) -> RusotoFuture<(), SetVaultAccessPolicyError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/access-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input.policy).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SetVaultAccessPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications.</p> <p>To configure vault notifications, send a PUT request to the <code>notification-configuration</code> subresource of the vault. The request should include a JSON document that provides an Amazon SNS topic and specific events for which you want Amazon Glacier to send notifications to the topic.</p> <p>Amazon SNS topics must grant permission to the vault to be allowed to publish notifications to the topic. You can configure a vault to publish a notification for the following vault events:</p> <ul> <li> <p> <b>ArchiveRetrievalCompleted</b> This event occurs when a job that was initiated for an archive retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> <li> <p> <b>InventoryRetrievalCompleted</b> This event occurs when a job that was initiated for an inventory retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> </ul> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-put.html">Set Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn set_vault_notifications(
        &self,
        input: SetVaultNotificationsInput,
    ) -> RusotoFuture<(), SetVaultNotificationsError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/notification-configuration",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input.vault_notification_config).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SetVaultNotificationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation adds an archive to a vault. This is a synchronous operation, and for a successful upload, your data is durably persisted. Amazon Glacier returns the archive ID in the <code>x-amz-archive-id</code> header of the response. </p> <p>You must use the archive ID to access your data in Amazon Glacier. After you upload an archive, you should save the archive ID returned so that you can retrieve or delete the archive later. Besides saving the archive ID, you can also index it and give it a friendly name to allow for better searching. You can also use the optional archive description field to specify how the archive is referred to in an external index of archives, such as you might create in Amazon DynamoDB. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>. </p> <p>You must provide a SHA256 tree hash of the data you are uploading. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. </p> <p>You can optionally specify an archive description of up to 1,024 printable ASCII characters. You can get the archive description when you either retrieve the archive or get the vault inventory. For more information, see <a>InitiateJob</a>. Amazon Glacier does not interpret the description in any way. An archive description does not need to be unique. You cannot use the description to retrieve or sort the archive list. </p> <p>Archives are immutable. After you upload an archive, you cannot edit the archive or its description.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-an-archive.html">Uploading an Archive in Amazon Glacier</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    fn upload_archive(
        &self,
        input: UploadArchiveInput,
    ) -> RusotoFuture<ArchiveCreationOutput, UploadArchiveError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/archives",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = if let Some(ref payload) = input.body {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        if let Some(ref archive_description) = input.archive_description {
            request.add_header(
                "x-amz-archive-description",
                &archive_description.to_string(),
            );
        }

        if let Some(ref checksum) = input.checksum {
            request.add_header("x-amz-sha256-tree-hash", &checksum.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<ArchiveCreationOutput>(&body).unwrap();

                    if let Some(archive_id) = response.headers.get("x-amz-archive-id") {
                        let value = archive_id.to_owned();
                        result.archive_id = Some(value)
                    };
                    if let Some(checksum) = response.headers.get("x-amz-sha256-tree-hash") {
                        let value = checksum.to_owned();
                        result.checksum = Some(value)
                    };
                    if let Some(location) = response.headers.get("Location") {
                        let value = location.to_owned();
                        result.location = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UploadArchiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>This operation uploads a part of an archive. You can upload archive parts in any order. You can also upload them in parallel. You can upload up to 10,000 parts for a multipart upload.</p> <p>Amazon Glacier rejects your upload part request if any of the following conditions is true:</p> <ul> <li> <p> <b>SHA256 tree hash does not match</b>To ensure that part data is not corrupted in transmission, you compute a SHA256 tree hash of the part and include it in your request. Upon receiving the part data, Amazon Glacier also computes a SHA256 tree hash. If these hash values don't match, the operation fails. For information about computing a SHA256 tree hash, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>.</p> </li> <li> <p> <b>Part size does not match</b>The size of each part except the last must match the size specified in the corresponding <a>InitiateMultipartUpload</a> request. The size of the last part must be the same size as, or smaller than, the specified size.</p> <note> <p>If you upload a part whose size is smaller than the part size you specified in your initiate multipart upload request and that part is not the last part, then the upload part request will succeed. However, the subsequent Complete Multipart Upload request will fail.</p> </note> </li> <li> <p> <b>Range does not align</b>The byte range value in the request does not align with the part size specified in the corresponding initiate request. For example, if you specify a part size of 4194304 bytes (4 MB), then 0 to 4194303 bytes (4 MB - 1) and 4194304 (4 MB) to 8388607 (8 MB - 1) are valid part ranges. However, if you set a range value of 2 MB to 6 MB, the range does not align with the part size and the upload will fail. </p> </li> </ul> <p>This operation is idempotent. If you upload the same part multiple times, the data included in the most recent request overwrites the previously uploaded data.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="http://docs.aws.amazon.com/amazonglacier/latest/dev/api-upload-part.html">Upload Part </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    fn upload_multipart_part(
        &self,
        input: UploadMultipartPartInput,
    ) -> RusotoFuture<UploadMultipartPartOutput, UploadMultipartPartError> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}",
            account_id = input.account_id,
            upload_id = input.upload_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = if let Some(ref payload) = input.body {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        if let Some(ref checksum) = input.checksum {
            request.add_header("x-amz-sha256-tree-hash", &checksum.to_string());
        }

        if let Some(ref range) = input.range {
            request.add_header("Content-Range", &range.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UploadMultipartPartOutput>(&body).unwrap();

                    if let Some(checksum) = response.headers.get("x-amz-sha256-tree-hash") {
                        let value = checksum.to_owned();
                        result.checksum = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UploadMultipartPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
