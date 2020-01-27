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
/// <p>Provides options to abort a multipart upload identified by the upload ID.</p> <p>For information about the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a>. For conceptual information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AbortMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToVaultInput {
    /// <p>The tags to add to the vault. Each tag is composed of a key and a value. The value can be an empty string.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p> <p>For information about the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a>. For conceptual information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArchiveCreationOutput {
    /// <p>The ID of the archive. This value is also included as part of the location.</p>
    #[serde(rename = "archiveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// <p>The checksum of the archive computed by Amazon S3 Glacier.</p>
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

/// <p>Provides options to complete a multipart upload operation. This informs Amazon Glacier that all the archive parts have been uploaded and Amazon S3 Glacier (Glacier) can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Glacier returns the URI path of the newly created archive resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The total size, in bytes, of the entire archive. This value should be the sum of all the sizes of the individual parts that you uploaded.</p>
    #[serde(rename = "archiveSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size: Option<String>,
    /// <p>The SHA256 tree hash of the entire archive. It is the tree hash of SHA256 tree hash of the individual parts. If the value you specify in the request does not match the SHA256 tree hash of the final assembled archive as computed by Amazon S3 Glacier (Glacier), Glacier returns an error and the request fails.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides options for deleting an archive from an Amazon S3 Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteArchiveInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for deleting a vault from Amazon S3 Glacier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for deleting a vault notification configuration from an Amazon Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Provides options for retrieving a job description.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVaultOutput {
    /// <p>The Universal Coordinated Time (UTC) date when the vault was created. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The Universal Coordinated Time (UTC) date when Amazon S3 Glacier completed the last vault inventory. This value should be a string in the ISO 8601 date format, for example <code>2012-03-20T17:03:43.221Z</code>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataRetrievalPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

/// <p>Contains the Amazon S3 Glacier response to the <code>GetDataRetrievalPolicy</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataRetrievalPolicyOutput {
    /// <p>Contains the returned data retrieval policy in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<DataRetrievalPolicy>,
}

/// <p>Provides options for downloading output of an Amazon S3 Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobOutputInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The job ID whose data is downloaded.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p><p>The range of bytes to retrieve from the output. For example, if you want to download the first 1,048,576 bytes, specify the range as <code>bytes=0-1048575</code>. By default, this operation downloads the entire output.</p> <p>If the job output is large, then you can use a range to retrieve a portion of the output. This allows you to download the entire output in smaller chunks of bytes. For example, suppose you have 1 GB of job output you want to download and you decide to download 128 MB chunks of data at a time, which is a total of eight Get Job Output requests. You use the following process to download the job output:</p> <ol> <li> <p>Download a 128 MB chunk of output by specifying the appropriate byte range. Verify that all 128 MB of data was received.</p> </li> <li> <p>Along with the data, the response includes a SHA256 tree hash of the payload. You compute the checksum of the payload on the client and compare it with the checksum you received in the response to ensure you received all the expected data.</p> </li> <li> <p>Repeat steps 1 and 2 for all the eight 128 MB chunks of output data, each time specifying the appropriate byte range.</p> </li> <li> <p>After downloading all the parts of the job output, you have a list of eight checksum values. Compute the tree hash of these values to find the checksum of the entire output. Using the <a>DescribeJob</a> API, obtain job information of the job that provided you the output. The response includes the checksum of the entire archive stored in Amazon S3 Glacier. You compare this value with the checksum you computed to ensure you have downloaded the entire archive content with no errors.</p> <p/> </li> </ol></p>
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetJobOutputOutput {
    /// <p>Indicates the range units accepted. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html">RFC2616</a>. </p>
    pub accept_ranges: Option<String>,
    /// <p>The description of an archive.</p>
    pub archive_description: Option<String>,
    /// <p>The job data, either archive data or inventory data.</p>
    pub body: Option<bytes::Bytes>,
    /// <p><p>The checksum of the data in the response. This header is returned only when retrieving the output for an archive retrieval job. Furthermore, this header appears only under the following conditions:</p> <ul> <li> <p>You get the entire range of the archive.</p> </li> <li> <p>You request a range to return of the archive that starts and ends on a multiple of 1 MB. For example, if you have an 3.1 MB archive and you specify a range to return that starts at 1 MB and ends at 2 MB, then the x-amz-sha256-tree-hash is returned as a response header.</p> </li> <li> <p>You request a range of the archive to return that starts on a multiple of 1 MB and goes to the end of the archive. For example, if you have a 3.1 MB archive and you specify a range that starts at 2 MB and ends at 3.1 MB (the end of the archive), then the x-amz-sha256-tree-hash is returned as a response header.</p> </li> </ul></p>
    pub checksum: Option<String>,
    /// <p>The range of bytes returned by Amazon S3 Glacier. If only partial output is downloaded, the response provides the range of bytes Amazon S3 Glacier returned. For example, bytes 0-1048575/8388608 returns the first 1 MB from 8 MB.</p>
    pub content_range: Option<String>,
    /// <p>The Content-Type depends on whether the job output is an archive or a vault inventory. For archive data, the Content-Type is application/octet-stream. For vault inventory, if you requested CSV format when you initiated the job, the Content-Type is text/csv. Otherwise, by default, vault inventory is returned as JSON, and the Content-Type is application/json.</p>
    pub content_type: Option<String>,
    /// <p>The HTTP response code for a job output request. The value depends on whether a range was specified in the request.</p>
    pub status: Option<i64>,
}

/// <p>Input for GetVaultAccessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Output for GetVaultAccessPolicy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVaultAccessPolicyOutput {
    /// <p>Contains the returned vault access policy as a JSON string.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultAccessPolicy>,
}

/// <p>The input values for <code>GetVaultLock</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVaultLockInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVaultNotificationsOutput {
    /// <p>Returns the notification configuration set on the vault.</p>
    #[serde(rename = "vaultNotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_notification_config: Option<VaultNotificationConfig>,
}

/// <p>Contains the description of an Amazon S3 Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>An opaque string that identifies an Amazon S3 Glacier job.</p>
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

/// <p>Provides options for initiating an Amazon S3 Glacier job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitiateJobInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides options for initiating a multipart upload to an Amazon S3 Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitiateMultipartUploadInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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

/// <p>The Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InitiateMultipartUploadOutput {
    /// <p>The relative URI path of the multipart upload ID Amazon S3 Glacier created.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>An opaque string that represents where to continue pagination of the vault inventory retrieval results. You use the marker in a new <b>InitiateJob</b> request to obtain additional inventory items. If there are no more inventory items, this value is <code>null</code>. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html#api-initiate-job-post-vault-inventory-list-filtering"> Range Inventory Retrieval</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The Amazon SNS topic ARN to which Amazon S3 Glacier sends a notification when the job is completed and the output is ready for you to download. The specified topic publishes the notification to its subscribers. The SNS topic must exist.</p>
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

/// <p>Provides options for retrieving a job list for an Amazon S3 Glacier vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMultipartUploadsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPartsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProvisionedCapacityInput {
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProvisionedCapacityOutput {
    /// <p>The response body contains the following JSON fields.</p>
    #[serde(rename = "ProvisionedCapacityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity_list: Option<Vec<ProvisionedCapacityDescription>>,
}

/// <p>The input value for <code>ListTagsForVaultInput</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The name of the vault.</p>
    #[serde(rename = "vaultName")]
    pub vault_name: String,
}

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForVaultOutput {
    /// <p>The tags attached to the vault. Each tag is composed of a key and a value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides options to retrieve the vault list owned by the calling user's account. The list provides metadata information for each vault.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartListElement {
    /// <p>The byte range of a part, inclusive of the upper value of the range.</p>
    #[serde(rename = "RangeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_bytes: Option<String>,
    /// <p>The SHA256 tree hash value that Amazon S3 Glacier calculated for the part. This field is never <code>null</code>.</p>
    #[serde(rename = "SHA256TreeHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_tree_hash: Option<String>,
}

/// <p>The definition for a provisioned capacity unit.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseProvisionedCapacityInput {
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PurchaseProvisionedCapacityOutput {
    /// <p>The ID that identifies the provisioned capacity unit.</p>
    #[serde(rename = "capacityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_id: Option<String>,
}

/// <p>The input value for <code>RemoveTagsFromVaultInput</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromVaultInput {
    /// <p>A list of tag keys. Each corresponding tag is removed from the vault.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetVaultAccessPolicyInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetVaultNotificationsInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UploadArchiveInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
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
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<bytes::Bytes>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UploadMultipartPartInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The data to upload.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<bytes::Bytes>,
    /// <p>The SHA256 tree hash of the data being uploaded.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>Identifies the range of bytes in the assembled archive that will be uploaded in this part. Amazon S3 Glacier uses this information to assemble the archive in the proper sequence. The format of this header follows RFC 2616. An example header is Content-Range:bytes 0-4194303/*.</p>
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

/// <p>Contains the Amazon S3 Glacier response to your request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UploadMultipartPartOutput {
    /// <p>The SHA256 tree hash that Amazon S3 Glacier computed for the uploaded part.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VaultLockPolicy {
    /// <p>The vault lock policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>Represents a vault's notification configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultNotificationConfig {
    /// <p>A list of one or more events for which Amazon S3 Glacier will send a notification to the specified Amazon SNS topic.</p>
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
}

impl AbortMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AbortMultipartUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(AbortMultipartUploadError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(AbortMultipartUploadError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AbortMultipartUploadError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AbortMultipartUploadError::ServiceUnavailable(
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
impl fmt::Display for AbortMultipartUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbortMultipartUploadError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AbortMultipartUploadError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            AbortMultipartUploadError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AbortMultipartUploadError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AbortMultipartUploadError {}
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
}

impl AbortVaultLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AbortVaultLockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(AbortVaultLockError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(AbortVaultLockError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AbortVaultLockError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AbortVaultLockError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AbortVaultLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbortVaultLockError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AbortVaultLockError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            AbortVaultLockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AbortVaultLockError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AbortVaultLockError {}
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
}

impl AddTagsToVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(AddTagsToVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AddTagsToVaultError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(AddTagsToVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddTagsToVaultError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddTagsToVaultError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsToVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AddTagsToVaultError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AddTagsToVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            AddTagsToVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsToVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToVaultError {}
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
}

impl CompleteMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteMultipartUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CompleteMultipartUploadError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        CompleteMultipartUploadError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CompleteMultipartUploadError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CompleteMultipartUploadError::ServiceUnavailable(
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
impl fmt::Display for CompleteMultipartUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteMultipartUploadError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CompleteMultipartUploadError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CompleteMultipartUploadError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CompleteMultipartUploadError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompleteMultipartUploadError {}
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
}

impl CompleteVaultLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteVaultLockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CompleteVaultLockError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(CompleteVaultLockError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CompleteVaultLockError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CompleteVaultLockError::ServiceUnavailable(
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
impl fmt::Display for CompleteVaultLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteVaultLockError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CompleteVaultLockError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            CompleteVaultLockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CompleteVaultLockError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompleteVaultLockError {}
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
}

impl CreateVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateVaultError::InvalidParameterValue(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVaultError::LimitExceeded(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(CreateVaultError::MissingParameterValue(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVaultError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateVaultError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            CreateVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVaultError {}
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
}

impl DeleteArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteArchiveError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteArchiveError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteArchiveError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteArchiveError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteArchiveError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteArchiveError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteArchiveError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteArchiveError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteArchiveError {}
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
}

impl DeleteVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteVaultError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DeleteVaultError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteVaultError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVaultError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVaultError {}
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
}

impl DeleteVaultAccessPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteVaultAccessPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        DeleteVaultAccessPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteVaultAccessPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVaultAccessPolicyError::ServiceUnavailable(
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
impl fmt::Display for DeleteVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVaultAccessPolicyError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVaultAccessPolicyError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVaultAccessPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteVaultAccessPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVaultAccessPolicyError {}
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
}

impl DeleteVaultNotificationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteVaultNotificationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        DeleteVaultNotificationsError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteVaultNotificationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVaultNotificationsError::ServiceUnavailable(
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
impl fmt::Display for DeleteVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVaultNotificationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVaultNotificationsError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVaultNotificationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteVaultNotificationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVaultNotificationsError {}
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
}

impl DescribeJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeJobError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeJobError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobError {}
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
}

impl DescribeVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeVaultError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(DescribeVaultError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeVaultError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeVaultError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVaultError {}
/// Errors returned by GetDataRetrievalPolicy
#[derive(Debug, PartialEq)]
pub enum GetDataRetrievalPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl GetDataRetrievalPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataRetrievalPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetDataRetrievalPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        GetDataRetrievalPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDataRetrievalPolicyError::ServiceUnavailable(
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
impl fmt::Display for GetDataRetrievalPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataRetrievalPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetDataRetrievalPolicyError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetDataRetrievalPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataRetrievalPolicyError {}
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
}

impl GetJobOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobOutputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetJobOutputError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetJobOutputError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetJobOutputError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetJobOutputError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobOutputError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetJobOutputError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetJobOutputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetJobOutputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobOutputError {}
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
}

impl GetVaultAccessPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetVaultAccessPolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetVaultAccessPolicyError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetVaultAccessPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetVaultAccessPolicyError::ServiceUnavailable(
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
impl fmt::Display for GetVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVaultAccessPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultAccessPolicyError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultAccessPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetVaultAccessPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVaultAccessPolicyError {}
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
}

impl GetVaultLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVaultLockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetVaultLockError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetVaultLockError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetVaultLockError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetVaultLockError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVaultLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVaultLockError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultLockError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultLockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetVaultLockError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVaultLockError {}
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
}

impl GetVaultNotificationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetVaultNotificationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(GetVaultNotificationsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetVaultNotificationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetVaultNotificationsError::ServiceUnavailable(
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
impl fmt::Display for GetVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVaultNotificationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultNotificationsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            GetVaultNotificationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetVaultNotificationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVaultNotificationsError {}
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
}

impl InitiateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InsufficientCapacityException" => {
                    return RusotoError::Service(InitiateJobError::InsufficientCapacity(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(InitiateJobError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(InitiateJobError::MissingParameterValue(err.msg))
                }
                "PolicyEnforcedException" => {
                    return RusotoError::Service(InitiateJobError::PolicyEnforced(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateJobError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InitiateJobError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InitiateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateJobError::InsufficientCapacity(ref cause) => write!(f, "{}", cause),
            InitiateJobError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            InitiateJobError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            InitiateJobError::PolicyEnforced(ref cause) => write!(f, "{}", cause),
            InitiateJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InitiateJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitiateJobError {}
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
}

impl InitiateMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateMultipartUploadError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        InitiateMultipartUploadError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        InitiateMultipartUploadError::MissingParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateMultipartUploadError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InitiateMultipartUploadError::ServiceUnavailable(
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
impl fmt::Display for InitiateMultipartUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateMultipartUploadError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateMultipartUploadError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            InitiateMultipartUploadError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InitiateMultipartUploadError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitiateMultipartUploadError {}
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
}

impl InitiateVaultLockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateVaultLockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(InitiateVaultLockError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(InitiateVaultLockError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateVaultLockError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InitiateVaultLockError::ServiceUnavailable(
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
impl fmt::Display for InitiateVaultLockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateVaultLockError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            InitiateVaultLockError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            InitiateVaultLockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InitiateVaultLockError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitiateVaultLockError {}
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
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListJobsError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListJobsError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListJobsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListJobsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListJobsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListJobsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
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
}

impl ListMultipartUploadsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMultipartUploadsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListMultipartUploadsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListMultipartUploadsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListMultipartUploadsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListMultipartUploadsError::ServiceUnavailable(
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
impl fmt::Display for ListMultipartUploadsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMultipartUploadsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListMultipartUploadsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListMultipartUploadsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListMultipartUploadsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMultipartUploadsError {}
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
}

impl ListPartsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPartsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListPartsError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListPartsError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPartsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPartsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPartsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPartsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListPartsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListPartsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPartsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPartsError {}
/// Errors returned by ListProvisionedCapacity
#[derive(Debug, PartialEq)]
pub enum ListProvisionedCapacityError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl ListProvisionedCapacityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProvisionedCapacityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListProvisionedCapacityError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        ListProvisionedCapacityError::MissingParameterValue(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListProvisionedCapacityError::ServiceUnavailable(
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
impl fmt::Display for ListProvisionedCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProvisionedCapacityError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisionedCapacityError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisionedCapacityError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProvisionedCapacityError {}
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
}

impl ListTagsForVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsForVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListTagsForVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForVaultError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsForVaultError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsForVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsForVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForVaultError {}
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
}

impl ListVaultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVaultsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListVaultsError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(ListVaultsError::MissingParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListVaultsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVaultsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVaultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVaultsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListVaultsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            ListVaultsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListVaultsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVaultsError {}
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
}

impl PurchaseProvisionedCapacityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PurchaseProvisionedCapacityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PurchaseProvisionedCapacityError::InvalidParameterValue(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PurchaseProvisionedCapacityError::LimitExceeded(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        PurchaseProvisionedCapacityError::MissingParameterValue(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PurchaseProvisionedCapacityError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PurchaseProvisionedCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseProvisionedCapacityError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseProvisionedCapacityError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PurchaseProvisionedCapacityError::MissingParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseProvisionedCapacityError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PurchaseProvisionedCapacityError {}
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
}

impl RemoveTagsFromVaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromVaultError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RemoveTagsFromVaultError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(RemoveTagsFromVaultError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTagsFromVaultError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveTagsFromVaultError::ServiceUnavailable(
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
impl fmt::Display for RemoveTagsFromVaultError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromVaultError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromVaultError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromVaultError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromVaultError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromVaultError {}
/// Errors returned by SetDataRetrievalPolicy
#[derive(Debug, PartialEq)]
pub enum SetDataRetrievalPolicyError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl SetDataRetrievalPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetDataRetrievalPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        SetDataRetrievalPolicyError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(
                        SetDataRetrievalPolicyError::MissingParameterValue(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(SetDataRetrievalPolicyError::ServiceUnavailable(
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
impl fmt::Display for SetDataRetrievalPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetDataRetrievalPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SetDataRetrievalPolicyError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            SetDataRetrievalPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetDataRetrievalPolicyError {}
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
}

impl SetVaultAccessPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetVaultAccessPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(SetVaultAccessPolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(SetVaultAccessPolicyError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetVaultAccessPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(SetVaultAccessPolicyError::ServiceUnavailable(
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
impl fmt::Display for SetVaultAccessPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetVaultAccessPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SetVaultAccessPolicyError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            SetVaultAccessPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetVaultAccessPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetVaultAccessPolicyError {}
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
}

impl SetVaultNotificationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetVaultNotificationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(SetVaultNotificationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(SetVaultNotificationsError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetVaultNotificationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(SetVaultNotificationsError::ServiceUnavailable(
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
impl fmt::Display for SetVaultNotificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetVaultNotificationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            SetVaultNotificationsError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            SetVaultNotificationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetVaultNotificationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetVaultNotificationsError {}
/// Errors returned by UploadArchive
#[derive(Debug, PartialEq)]
pub enum UploadArchiveError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if, when uploading an archive, Amazon S3 Glacier times out while receiving the upload.</p>
    RequestTimeout(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl UploadArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadArchiveError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UploadArchiveError::InvalidParameterValue(err.msg))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UploadArchiveError::MissingParameterValue(err.msg))
                }
                "RequestTimeoutException" => {
                    return RusotoError::Service(UploadArchiveError::RequestTimeout(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UploadArchiveError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UploadArchiveError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UploadArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadArchiveError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UploadArchiveError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UploadArchiveError::RequestTimeout(ref cause) => write!(f, "{}", cause),
            UploadArchiveError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UploadArchiveError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UploadArchiveError {}
/// Errors returned by UploadMultipartPart
#[derive(Debug, PartialEq)]
pub enum UploadMultipartPartError {
    /// <p>Returned if a parameter of the request is incorrectly specified.</p>
    InvalidParameterValue(String),
    /// <p>Returned if a required header or parameter is missing from the request.</p>
    MissingParameterValue(String),
    /// <p>Returned if, when uploading an archive, Amazon S3 Glacier times out while receiving the upload.</p>
    RequestTimeout(String),
    /// <p>Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Returned if the service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl UploadMultipartPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadMultipartPartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UploadMultipartPartError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingParameterValueException" => {
                    return RusotoError::Service(UploadMultipartPartError::MissingParameterValue(
                        err.msg,
                    ))
                }
                "RequestTimeoutException" => {
                    return RusotoError::Service(UploadMultipartPartError::RequestTimeout(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UploadMultipartPartError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UploadMultipartPartError::ServiceUnavailable(
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
impl fmt::Display for UploadMultipartPartError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadMultipartPartError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UploadMultipartPartError::MissingParameterValue(ref cause) => write!(f, "{}", cause),
            UploadMultipartPartError::RequestTimeout(ref cause) => write!(f, "{}", cause),
            UploadMultipartPartError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UploadMultipartPartError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UploadMultipartPartError {}
/// Trait representing the capabilities of the Amazon Glacier API. Amazon Glacier clients implement this trait.
#[async_trait]
pub trait Glacier {
    /// <p>This operation aborts a multipart upload identified by the upload ID.</p> <p>After the Abort Multipart Upload request succeeds, you cannot upload any more parts to the multipart upload or complete the multipart upload. Aborting a completed upload fails. However, aborting an already-aborted upload will succeed, for a short time. For more information about uploading a part and completing a multipart upload, see <a>UploadMultipartPart</a> and <a>CompleteMultipartUpload</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadInput,
    ) -> Result<(), RusotoError<AbortMultipartUploadError>>;

    /// <p>This operation aborts the vault locking process if the vault lock is not in the <code>Locked</code> state. If the vault lock is in the <code>Locked</code> state when this operation is requested, the operation returns an <code>AccessDeniedException</code> error. Aborting the vault locking process removes the vault lock policy from the specified vault. </p> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can get the state of a vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. For more information about vault lock policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>This operation is idempotent. You can successfully invoke this operation multiple times, if the vault lock is in the <code>InProgress</code> state or if there is no policy associated with the vault.</p>
    async fn abort_vault_lock(
        &self,
        input: AbortVaultLockInput,
    ) -> Result<(), RusotoError<AbortVaultLockError>>;

    /// <p>This operation adds the specified tags to a vault. Each tag is composed of a key and a value. Each vault can have up to 10 tags. If your request would cause the tag limit for the vault to be exceeded, the operation throws the <code>LimitExceededException</code> error. If a tag already exists on the vault under a specified key, the existing key value will be overwritten. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>. </p>
    async fn add_tags_to_vault(
        &self,
        input: AddTagsToVaultInput,
    ) -> Result<(), RusotoError<AddTagsToVaultError>>;

    /// <p>You call this operation to inform Amazon S3 Glacier (Glacier) that all the archive parts have been uploaded and that Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Glacier returns the URI path of the newly created archive resource. Using the URI path, you can then access the archive. After you upload an archive, you should save the archive ID returned to retrieve the archive at a later point. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>.</p> <p>In the request, you must include the computed SHA256 tree hash of the entire archive you have uploaded. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. On the server side, Glacier also constructs the SHA256 tree hash of the assembled archive. If the values match, Glacier saves the archive to the vault; otherwise, it returns an error, and the operation fails. The <a>ListParts</a> operation returns a list of parts uploaded for a specific multipart upload. It includes checksum information for each uploaded part that can be used to debug a bad checksum issue.</p> <p>Additionally, Glacier also checks for any missing content ranges when assembling the archive, if missing content ranges are found, Glacier returns an error and the operation fails.</p> <p>Complete Multipart Upload is an idempotent operation. After your first successful complete multipart upload, if you call the operation again within a short period, the operation will succeed and return the same archive ID. This is useful in the event you experience a network issue that causes an aborted connection or receive a 500 server error, in which case you can repeat your Complete Multipart Upload request and get the same archive ID without creating duplicate archives. Note, however, that after the multipart upload completes, you cannot call the List Parts operation and the multipart upload will not appear in List Multipart Uploads response, even if idempotent complete is possible.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-complete-upload.html">Complete Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadInput,
    ) -> Result<ArchiveCreationOutput, RusotoError<CompleteMultipartUploadError>>;

    /// <p>This operation completes the vault locking process by transitioning the vault lock from the <code>InProgress</code> state to the <code>Locked</code> state, which causes the vault lock policy to become unchangeable. A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. You can obtain the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>This operation is idempotent. This request is always successful if the vault lock is in the <code>Locked</code> state and the provided lock ID matches the lock ID originally used to lock the vault.</p> <p>If an invalid lock ID is passed in the request when the vault lock is in the <code>Locked</code> state, the operation returns an <code>AccessDeniedException</code> error. If an invalid lock ID is passed in the request when the vault lock is in the <code>InProgress</code> state, the operation throws an <code>InvalidParameter</code> error.</p>
    async fn complete_vault_lock(
        &self,
        input: CompleteVaultLockInput,
    ) -> Result<(), RusotoError<CompleteVaultLockError>>;

    /// <p>This operation creates a new vault with the specified name. The name of the vault must be unique within a region for an AWS account. You can create up to 1,000 vaults per account. If you need to create more vaults, contact Amazon S3 Glacier.</p> <p>You must use the following guidelines when naming a vault.</p> <ul> <li> <p>Names can be between 1 and 255 characters long.</p> </li> <li> <p>Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).</p> </li> </ul> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/creating-vaults.html">Creating a Vault in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-put.html">Create Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn create_vault(
        &self,
        input: CreateVaultInput,
    ) -> Result<CreateVaultOutput, RusotoError<CreateVaultError>>;

    /// <p>This operation deletes an archive from a vault. Subsequent requests to initiate a retrieval of this archive will fail. Archive retrievals that are in progress for this archive ID may or may not succeed according to the following scenarios:</p> <ul> <li> <p>If the archive retrieval job is actively preparing the data for download when Amazon S3 Glacier receives the delete archive request, the archival retrieval operation might fail.</p> </li> <li> <p>If the archive retrieval job has successfully prepared the archive for download when Amazon S3 Glacier receives the delete archive request, you will be able to download the output.</p> </li> </ul> <p>This operation is idempotent. Attempting to delete an already-deleted archive does not result in an error.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-an-archive.html">Deleting an Archive in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn delete_archive(
        &self,
        input: DeleteArchiveInput,
    ) -> Result<(), RusotoError<DeleteArchiveError>>;

    /// <p>This operation deletes a vault. Amazon S3 Glacier will delete a vault only if there are no archives in the vault as of the last inventory and there have been no writes to the vault since the last inventory. If either of these conditions is not satisfied, the vault deletion fails (that is, the vault is not removed) and Amazon S3 Glacier returns an error. You can use <a>DescribeVault</a> to return the number of archives in a vault, and you can use <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job (POST jobs)</a> to initiate a new inventory retrieval for a vault. The inventory contains the archive IDs you use to delete archives using <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive (DELETE archive)</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-vaults.html">Deleting a Vault in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-delete.html">Delete Vault </a> in the <i>Amazon S3 Glacier Developer Guide</i>. </p>
    async fn delete_vault(
        &self,
        input: DeleteVaultInput,
    ) -> Result<(), RusotoError<DeleteVaultError>>;

    /// <p>This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon S3 Glacier to completely remove the access policy, and you might still see the effect of the policy for a short time after you send the delete request.</p> <p>This operation is idempotent. You can invoke delete multiple times, even if there is no policy associated with the vault. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    async fn delete_vault_access_policy(
        &self,
        input: DeleteVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<DeleteVaultAccessPolicyError>>;

    /// <p>This operation deletes the notification configuration set for a vault. The operation is eventually consistent; that is, it might take some time for Amazon S3 Glacier to completely disable the notifications and you might still receive some notifications for a short time after you send the delete request.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-delete.html">Delete Vault Notification Configuration </a> in the Amazon S3 Glacier Developer Guide. </p>
    async fn delete_vault_notifications(
        &self,
        input: DeleteVaultNotificationsInput,
    ) -> Result<(), RusotoError<DeleteVaultNotificationsError>>;

    /// <p>This operation returns information about a job you previously initiated, including the job initiation date, the user who initiated the job, the job status code/message and the Amazon SNS topic to notify after Amazon S3 Glacier (Glacier) completes the job. For more information about initiating a job, see <a>InitiateJob</a>. </p> <note> <p>This operation enables you to check the status of your job. However, it is strongly recommended that you set up an Amazon SNS topic and specify it in your initiate job request so that Glacier can notify the topic after it completes the job.</p> </note> <p>A job ID will not expire for at least 24 hours after Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-describe-job-get.html">Describe Job</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn describe_job(
        &self,
        input: DescribeJobInput,
    ) -> Result<GlacierJobDescription, RusotoError<DescribeJobError>>;

    /// <p>This operation returns information about a vault, including the vault's Amazon Resource Name (ARN), the date the vault was created, the number of archives it contains, and the total size of all the archives in the vault. The number of archives and their total size are as of the last inventory generation. This means that if you add or remove an archive from a vault, and then immediately use Describe Vault, the change in contents will not be immediately reflected. If you want to retrieve the latest inventory of the vault, use <a>InitiateJob</a>. Amazon S3 Glacier generates vault inventories approximately daily. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory in Amazon S3 Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-get.html">Describe Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn describe_vault(
        &self,
        input: DescribeVaultInput,
    ) -> Result<DescribeVaultOutput, RusotoError<DescribeVaultError>>;

    /// <p>This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>.</p>
    async fn get_data_retrieval_policy(
        &self,
        input: GetDataRetrievalPolicyInput,
    ) -> Result<GetDataRetrievalPolicyOutput, RusotoError<GetDataRetrievalPolicyError>>;

    /// <p>This operation downloads the output of the job you initiated using <a>InitiateJob</a>. Depending on the job type you specified when you initiated the job, the output will be either the content of an archive or a vault inventory.</p> <p>You can download all the job output or download a portion of the output by specifying a byte range. In the case of an archive retrieval job, depending on the byte range you specify, Amazon S3 Glacier (Glacier) returns the checksum for the portion of the data. You can compute the checksum on the client and verify that the values match to ensure the portion you downloaded is the correct data.</p> <p>A job ID will not expire for at least 24 hours after Glacier completes the job. That a byte range. For both archive and inventory retrieval jobs, you should verify the downloaded size against the size returned in the headers from the <b>Get Job Output</b> response.</p> <p>For archive retrieval jobs, you should also verify that the size is what you expected. If you download a portion of the output, the expected size is based on the range of bytes you specified. For example, if you specify a range of <code>bytes=0-1048575</code>, you should verify your download size is 1,048,576 bytes. If you download an entire archive, the expected size is the size of the archive when you uploaded it to Amazon S3 Glacier The expected size is also returned in the headers from the <b>Get Job Output</b> response.</p> <p>In the case of an archive retrieval job, depending on the byte range you specify, Glacier returns the checksum for the portion of the data. To ensure the portion you downloaded is the correct data, compute the checksum on the client, verify that the values match, and verify that the size is what you expected.</p> <p>A job ID does not expire for at least 24 hours after Glacier completes the job. That is, you can download the job output within the 24 hours period after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory</a>, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/downloading-an-archive.html">Downloading an Archive</a>, and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-job-output-get.html">Get Job Output </a> </p>
    async fn get_job_output(
        &self,
        input: GetJobOutputInput,
    ) -> Result<GetJobOutputOutput, RusotoError<GetJobOutputError>>;

    /// <p>This operation retrieves the <code>access-policy</code> subresource set on the vault; for more information on setting this subresource, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-SetVaultAccessPolicy.html">Set Vault Access Policy (PUT access-policy)</a>. If there is no access policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>.</p>
    async fn get_vault_access_policy(
        &self,
        input: GetVaultAccessPolicyInput,
    ) -> Result<GetVaultAccessPolicyOutput, RusotoError<GetVaultAccessPolicyError>>;

    /// <p>This operation retrieves the following attributes from the <code>lock-policy</code> subresource set on the specified vault: </p> <ul> <li> <p>The vault lock policy set on the vault.</p> </li> <li> <p>The state of the vault lock, which is either <code>InProgess</code> or <code>Locked</code>.</p> </li> <li> <p>When the lock ID expires. The lock ID is used to complete the vault locking process.</p> </li> <li> <p>When the vault lock was initiated and put into the <code>InProgress</code> state.</p> </li> </ul> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can abort the vault locking process by calling <a>AbortVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>If there is no vault lock policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault lock policies, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p>
    async fn get_vault_lock(
        &self,
        input: GetVaultLockInput,
    ) -> Result<GetVaultLockOutput, RusotoError<GetVaultLockError>>;

    /// <p>This operation retrieves the <code>notification-configuration</code> subresource of the specified vault.</p> <p>For information about setting a notification configuration on a vault, see <a>SetVaultNotifications</a>. If a notification configuration for a vault is not set, the operation returns a <code>404 Not Found</code> error. For more information about vault notifications, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-get.html">Get Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn get_vault_notifications(
        &self,
        input: GetVaultNotificationsInput,
    ) -> Result<GetVaultNotificationsOutput, RusotoError<GetVaultNotificationsError>>;

    /// <p>This operation initiates a job of the specified type, which can be a select, an archival retrieval, or a vault retrieval. For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job</a>. </p>
    async fn initiate_job(
        &self,
        input: InitiateJobInput,
    ) -> Result<InitiateJobOutput, RusotoError<InitiateJobError>>;

    /// <p>This operation initiates a multipart upload. Amazon S3 Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive (see <a>UploadMultipartPart</a>).</p> <p>When you initiate a multipart upload, you specify the part size in number of bytes. The part size must be a megabyte (1024 KB) multiplied by a power of 2-for example, 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB.</p> <p>Every part you upload to this resource (see <a>UploadMultipartPart</a>), except the last one, must have the same size. The last one can be the same size or smaller. For example, suppose you want to upload a 16.2 MB file. If you initiate the multipart upload with a part size of 4 MB, you will upload four parts of 4 MB each and one part of 0.2 MB. </p> <note> <p>You don't need to know the size of the archive when you start a multipart upload because Amazon S3 Glacier does not require you to specify the overall archive size.</p> </note> <p>After you complete the multipart upload, Amazon S3 Glacier (Glacier) removes the multipart upload resource referenced by the ID. Glacier also removes the multipart upload resource if you cancel the multipart upload or it may be removed if there is no activity for a period of 24 hours.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-initiate-upload.html">Initiate Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn initiate_multipart_upload(
        &self,
        input: InitiateMultipartUploadInput,
    ) -> Result<InitiateMultipartUploadOutput, RusotoError<InitiateMultipartUploadError>>;

    /// <p>This operation initiates the vault locking process by doing the following:</p> <ul> <li> <p>Installing a vault lock policy on the specified vault.</p> </li> <li> <p>Setting the lock state of vault lock to <code>InProgress</code>.</p> </li> <li> <p>Returning a lock ID, which is used to complete the vault locking process.</p> </li> </ul> <p>You can set one vault lock policy for each vault and this policy can be up to 20 KB in size. For more information about vault lock policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>You must complete the vault locking process within 24 hours after the vault lock enters the <code>InProgress</code> state. After the 24 hour window ends, the lock ID expires, the vault automatically exits the <code>InProgress</code> state, and the vault lock policy is removed from the vault. You call <a>CompleteVaultLock</a> to complete the vault locking process by setting the state of the vault lock to <code>Locked</code>. </p> <p>After a vault lock is in the <code>Locked</code> state, you cannot initiate a new vault lock for the vault.</p> <p>You can abort the vault locking process by calling <a>AbortVaultLock</a>. You can get the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>.</p> <p>If this operation is called when the vault lock is in the <code>InProgress</code> state, the operation returns an <code>AccessDeniedException</code> error. When the vault lock is in the <code>InProgress</code> state you must call <a>AbortVaultLock</a> before you can initiate a new vault lock policy. </p>
    async fn initiate_vault_lock(
        &self,
        input: InitiateVaultLockInput,
    ) -> Result<InitiateVaultLockOutput, RusotoError<InitiateVaultLockError>>;

    /// <p>This operation lists jobs for a vault, including jobs that are in-progress and jobs that have recently finished. The List Job operation returns a list of these jobs sorted by job initiation time.</p> <note> <p>Amazon Glacier retains recently completed jobs for a period before deleting them; however, it eventually removes completed jobs. The output of completed jobs can be retrieved. Retaining completed jobs for a period of time after they have completed enables you to get a job output in the event you miss the job completion notification or your first attempt to download it fails. For example, suppose you start an archive retrieval job to download an archive. After the job completes, you start to download the archive but encounter a network error. In this scenario, you can retry and download the archive while the job exists.</p> </note> <p>The List Jobs operation supports pagination. You should always check the response <code>Marker</code> field. If there are no more jobs to list, the <code>Marker</code> field is set to <code>null</code>. If there are more jobs to list, the <code>Marker</code> field is set to a non-null value, which you can use to continue the pagination of the list. To return a list of jobs that begins at a specific job, set the marker request parameter to the <code>Marker</code> value for that job that you obtained from a previous List Jobs request.</p> <p>You can set a maximum limit for the number of jobs returned in the response by specifying the <code>limit</code> parameter in the request. The default limit is 50. The number of jobs returned might be fewer than the limit, but the number of returned jobs never exceeds the limit.</p> <p>Additionally, you can filter the jobs list returned by specifying the optional <code>statuscode</code> parameter or <code>completed</code> parameter, or both. Using the <code>statuscode</code> parameter, you can specify to return only jobs that match either the <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code> status. Using the <code>completed</code> parameter, you can specify to return only jobs that were completed (<code>true</code>) or jobs that were not completed (<code>false</code>).</p> <p>For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-jobs-get.html">List Jobs</a>. </p>
    async fn list_jobs(
        &self,
        input: ListJobsInput,
    ) -> Result<ListJobsOutput, RusotoError<ListJobsError>>;

    /// <p>This operation lists in-progress multipart uploads for the specified vault. An in-progress multipart upload is a multipart upload that has been initiated by an <a>InitiateMultipartUpload</a> request, but has not yet been completed or aborted. The list returned in the List Multipart Upload response has no guaranteed order. </p> <p>The List Multipart Uploads operation supports pagination. By default, this operation returns up to 50 multipart uploads in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of multipart uploads that begins at a specific upload, set the <code>marker</code> request parameter to the value you obtained from a previous List Multipart Upload request. You can also limit the number of uploads returned in the response by specifying the <code>limit</code> parameter in the request.</p> <p>Note the difference between this operation and listing parts (<a>ListParts</a>). The List Multipart Uploads operation lists all multipart uploads for a vault and does not require a multipart upload ID. The List Parts operation requires a multipart upload ID since parts are associated with a single upload.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-uploads.html">List Multipart Uploads </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsInput,
    ) -> Result<ListMultipartUploadsOutput, RusotoError<ListMultipartUploadsError>>;

    /// <p>This operation lists the parts of an archive that have been uploaded in a specific multipart upload. You can make this request at any time during an in-progress multipart upload before you complete the upload (see <a>CompleteMultipartUpload</a>. List Parts returns an error for completed uploads. The list returned in the List Parts response is sorted by part range. </p> <p>The List Parts operation supports pagination. By default, this operation returns up to 50 uploaded parts in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of parts that begins at a specific part, set the <code>marker</code> request parameter to the value you obtained from a previous List Parts request. You can also limit the number of parts returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-parts.html">List Parts</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn list_parts(
        &self,
        input: ListPartsInput,
    ) -> Result<ListPartsOutput, RusotoError<ListPartsError>>;

    /// <p>This operation lists the provisioned capacity units for the specified AWS account.</p>
    async fn list_provisioned_capacity(
        &self,
        input: ListProvisionedCapacityInput,
    ) -> Result<ListProvisionedCapacityOutput, RusotoError<ListProvisionedCapacityError>>;

    /// <p>This operation lists all the tags attached to a vault. The operation returns an empty map if there are no tags. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>.</p>
    async fn list_tags_for_vault(
        &self,
        input: ListTagsForVaultInput,
    ) -> Result<ListTagsForVaultOutput, RusotoError<ListTagsForVaultError>>;

    /// <p>This operation lists all vaults owned by the calling user's account. The list returned in the response is ASCII-sorted by vault name.</p> <p>By default, this operation returns up to 10 items. If there are more vaults to list, the response <code>marker</code> field contains the vault Amazon Resource Name (ARN) at which to continue the list with a new List Vaults request; otherwise, the <code>marker</code> field is <code>null</code>. To return a list of vaults that begins at a specific vault, set the <code>marker</code> request parameter to the vault ARN you obtained from a previous List Vaults request. You can also limit the number of vaults returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html">List Vaults </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn list_vaults(
        &self,
        input: ListVaultsInput,
    ) -> Result<ListVaultsOutput, RusotoError<ListVaultsError>>;

    /// <p>This operation purchases a provisioned capacity unit for an AWS account. </p>
    async fn purchase_provisioned_capacity(
        &self,
        input: PurchaseProvisionedCapacityInput,
    ) -> Result<PurchaseProvisionedCapacityOutput, RusotoError<PurchaseProvisionedCapacityError>>;

    /// <p>This operation removes one or more tags from the set of tags attached to a vault. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>. This operation is idempotent. The operation will be successful, even if there are no tags attached to the vault. </p>
    async fn remove_tags_from_vault(
        &self,
        input: RemoveTagsFromVaultInput,
    ) -> Result<(), RusotoError<RemoveTagsFromVaultError>>;

    /// <p>This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful PUT operation.</p> <p>The set policy operation does not affect retrieval jobs that were in progress before the policy was enacted. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>. </p>
    async fn set_data_retrieval_policy(
        &self,
        input: SetDataRetrievalPolicyInput,
    ) -> Result<(), RusotoError<SetDataRetrievalPolicyError>>;

    /// <p>This operation configures an access policy for a vault and will overwrite an existing policy. To configure a vault access policy, send a PUT request to the <code>access-policy</code> subresource of the vault. An access policy is specific to a vault and is also called a vault subresource. You can set one access policy per vault and the policy can be up to 20 KB in size. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    async fn set_vault_access_policy(
        &self,
        input: SetVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<SetVaultAccessPolicyError>>;

    /// <p>This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications.</p> <p>To configure vault notifications, send a PUT request to the <code>notification-configuration</code> subresource of the vault. The request should include a JSON document that provides an Amazon SNS topic and specific events for which you want Amazon S3 Glacier to send notifications to the topic.</p> <p>Amazon SNS topics must grant permission to the vault to be allowed to publish notifications to the topic. You can configure a vault to publish a notification for the following vault events:</p> <ul> <li> <p> <b>ArchiveRetrievalCompleted</b> This event occurs when a job that was initiated for an archive retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> <li> <p> <b>InventoryRetrievalCompleted</b> This event occurs when a job that was initiated for an inventory retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> </ul> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-put.html">Set Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn set_vault_notifications(
        &self,
        input: SetVaultNotificationsInput,
    ) -> Result<(), RusotoError<SetVaultNotificationsError>>;

    /// <p>This operation adds an archive to a vault. This is a synchronous operation, and for a successful upload, your data is durably persisted. Amazon S3 Glacier returns the archive ID in the <code>x-amz-archive-id</code> header of the response. </p> <p>You must use the archive ID to access your data in Amazon S3 Glacier. After you upload an archive, you should save the archive ID returned so that you can retrieve or delete the archive later. Besides saving the archive ID, you can also index it and give it a friendly name to allow for better searching. You can also use the optional archive description field to specify how the archive is referred to in an external index of archives, such as you might create in Amazon DynamoDB. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>. </p> <p>You must provide a SHA256 tree hash of the data you are uploading. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. </p> <p>You can optionally specify an archive description of up to 1,024 printable ASCII characters. You can get the archive description when you either retrieve the archive or get the vault inventory. For more information, see <a>InitiateJob</a>. Amazon Glacier does not interpret the description in any way. An archive description does not need to be unique. You cannot use the description to retrieve or sort the archive list. </p> <p>Archives are immutable. After you upload an archive, you cannot edit the archive or its description.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-an-archive.html">Uploading an Archive in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn upload_archive(
        &self,
        input: UploadArchiveInput,
    ) -> Result<ArchiveCreationOutput, RusotoError<UploadArchiveError>>;

    /// <p>This operation uploads a part of an archive. You can upload archive parts in any order. You can also upload them in parallel. You can upload up to 10,000 parts for a multipart upload.</p> <p>Amazon Glacier rejects your upload part request if any of the following conditions is true:</p> <ul> <li> <p> <b>SHA256 tree hash does not match</b>To ensure that part data is not corrupted in transmission, you compute a SHA256 tree hash of the part and include it in your request. Upon receiving the part data, Amazon S3 Glacier also computes a SHA256 tree hash. If these hash values don't match, the operation fails. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>.</p> </li> <li> <p> <b>Part size does not match</b>The size of each part except the last must match the size specified in the corresponding <a>InitiateMultipartUpload</a> request. The size of the last part must be the same size as, or smaller than, the specified size.</p> <note> <p>If you upload a part whose size is smaller than the part size you specified in your initiate multipart upload request and that part is not the last part, then the upload part request will succeed. However, the subsequent Complete Multipart Upload request will fail.</p> </note> </li> <li> <p> <b>Range does not align</b>The byte range value in the request does not align with the part size specified in the corresponding initiate request. For example, if you specify a part size of 4194304 bytes (4 MB), then 0 to 4194303 bytes (4 MB - 1) and 4194304 (4 MB) to 8388607 (8 MB - 1) are valid part ranges. However, if you set a range value of 2 MB to 6 MB, the range does not align with the part size and the upload will fail. </p> </li> </ul> <p>This operation is idempotent. If you upload the same part multiple times, the data included in the most recent request overwrites the previously uploaded data.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-upload-part.html">Upload Part </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn upload_multipart_part(
        &self,
        input: UploadMultipartPartInput,
    ) -> Result<UploadMultipartPartOutput, RusotoError<UploadMultipartPartError>>;
}
/// A client for the Amazon Glacier API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GlacierClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GlacierClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GlacierClient {
        GlacierClient { client, region }
    }
}

#[async_trait]
impl Glacier for GlacierClient {
    /// <p>This operation aborts a multipart upload identified by the upload ID.</p> <p>After the Abort Multipart Upload request succeeds, you cannot upload any more parts to the multipart upload or complete the multipart upload. Aborting a completed upload fails. However, aborting an already-aborted upload will succeed, for a short time. For more information about uploading a part and completing a multipart upload, see <a>UploadMultipartPart</a> and <a>CompleteMultipartUpload</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-abort-upload.html">Abort Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadInput,
    ) -> Result<(), RusotoError<AbortMultipartUploadError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}",
            account_id = input.account_id,
            upload_id = input.upload_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AbortMultipartUploadError::from_response(response))
        }
    }

    /// <p>This operation aborts the vault locking process if the vault lock is not in the <code>Locked</code> state. If the vault lock is in the <code>Locked</code> state when this operation is requested, the operation returns an <code>AccessDeniedException</code> error. Aborting the vault locking process removes the vault lock policy from the specified vault. </p> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can get the state of a vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. For more information about vault lock policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>This operation is idempotent. You can successfully invoke this operation multiple times, if the vault lock is in the <code>InProgress</code> state or if there is no policy associated with the vault.</p>
    async fn abort_vault_lock(
        &self,
        input: AbortVaultLockInput,
    ) -> Result<(), RusotoError<AbortVaultLockError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AbortVaultLockError::from_response(response))
        }
    }

    /// <p>This operation adds the specified tags to a vault. Each tag is composed of a key and a value. Each vault can have up to 10 tags. If your request would cause the tag limit for the vault to be exceeded, the operation throws the <code>LimitExceededException</code> error. If a tag already exists on the vault under a specified key, the existing key value will be overwritten. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>. </p>
    async fn add_tags_to_vault(
        &self,
        input: AddTagsToVaultInput,
    ) -> Result<(), RusotoError<AddTagsToVaultError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsToVaultError::from_response(response))
        }
    }

    /// <p>You call this operation to inform Amazon S3 Glacier (Glacier) that all the archive parts have been uploaded and that Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive to the vault, Glacier returns the URI path of the newly created archive resource. Using the URI path, you can then access the archive. After you upload an archive, you should save the archive ID returned to retrieve the archive at a later point. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>.</p> <p>In the request, you must include the computed SHA256 tree hash of the entire archive you have uploaded. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. On the server side, Glacier also constructs the SHA256 tree hash of the assembled archive. If the values match, Glacier saves the archive to the vault; otherwise, it returns an error, and the operation fails. The <a>ListParts</a> operation returns a list of parts uploaded for a specific multipart upload. It includes checksum information for each uploaded part that can be used to debug a bad checksum issue.</p> <p>Additionally, Glacier also checks for any missing content ranges when assembling the archive, if missing content ranges are found, Glacier returns an error and the operation fails.</p> <p>Complete Multipart Upload is an idempotent operation. After your first successful complete multipart upload, if you call the operation again within a short period, the operation will succeed and return the same archive ID. This is useful in the event you experience a network issue that causes an aborted connection or receive a 500 server error, in which case you can repeat your Complete Multipart Upload request and get the same archive ID without creating duplicate archives. Note, however, that after the multipart upload completes, you cannot call the List Parts operation and the multipart upload will not appear in List Multipart Uploads response, even if idempotent complete is possible.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-complete-upload.html">Complete Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadInput,
    ) -> Result<ArchiveCreationOutput, RusotoError<CompleteMultipartUploadError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ArchiveCreationOutput, _>()?;
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

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CompleteMultipartUploadError::from_response(response))
        }
    }

    /// <p>This operation completes the vault locking process by transitioning the vault lock from the <code>InProgress</code> state to the <code>Locked</code> state, which causes the vault lock policy to become unchangeable. A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. You can obtain the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>This operation is idempotent. This request is always successful if the vault lock is in the <code>Locked</code> state and the provided lock ID matches the lock ID originally used to lock the vault.</p> <p>If an invalid lock ID is passed in the request when the vault lock is in the <code>Locked</code> state, the operation returns an <code>AccessDeniedException</code> error. If an invalid lock ID is passed in the request when the vault lock is in the <code>InProgress</code> state, the operation throws an <code>InvalidParameter</code> error.</p>
    async fn complete_vault_lock(
        &self,
        input: CompleteVaultLockInput,
    ) -> Result<(), RusotoError<CompleteVaultLockError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy/{lock_id}",
            account_id = input.account_id,
            lock_id = input.lock_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CompleteVaultLockError::from_response(response))
        }
    }

    /// <p>This operation creates a new vault with the specified name. The name of the vault must be unique within a region for an AWS account. You can create up to 1,000 vaults per account. If you need to create more vaults, contact Amazon S3 Glacier.</p> <p>You must use the following guidelines when naming a vault.</p> <ul> <li> <p>Names can be between 1 and 255 characters long.</p> </li> <li> <p>Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).</p> </li> </ul> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/creating-vaults.html">Creating a Vault in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-put.html">Create Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn create_vault(
        &self,
        input: CreateVaultInput,
    ) -> Result<CreateVaultOutput, RusotoError<CreateVaultError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVaultOutput, _>()?;
            if let Some(location) = response.headers.get("Location") {
                let value = location.to_owned();
                result.location = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVaultError::from_response(response))
        }
    }

    /// <p>This operation deletes an archive from a vault. Subsequent requests to initiate a retrieval of this archive will fail. Archive retrievals that are in progress for this archive ID may or may not succeed according to the following scenarios:</p> <ul> <li> <p>If the archive retrieval job is actively preparing the data for download when Amazon S3 Glacier receives the delete archive request, the archival retrieval operation might fail.</p> </li> <li> <p>If the archive retrieval job has successfully prepared the archive for download when Amazon S3 Glacier receives the delete archive request, you will be able to download the output.</p> </li> </ul> <p>This operation is idempotent. Attempting to delete an already-deleted archive does not result in an error.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-an-archive.html">Deleting an Archive in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn delete_archive(
        &self,
        input: DeleteArchiveInput,
    ) -> Result<(), RusotoError<DeleteArchiveError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/archives/{archive_id}",
            account_id = input.account_id,
            archive_id = input.archive_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteArchiveError::from_response(response))
        }
    }

    /// <p>This operation deletes a vault. Amazon S3 Glacier will delete a vault only if there are no archives in the vault as of the last inventory and there have been no writes to the vault since the last inventory. If either of these conditions is not satisfied, the vault deletion fails (that is, the vault is not removed) and Amazon S3 Glacier returns an error. You can use <a>DescribeVault</a> to return the number of archives in a vault, and you can use <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job (POST jobs)</a> to initiate a new inventory retrieval for a vault. The inventory contains the archive IDs you use to delete archives using <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-delete.html">Delete Archive (DELETE archive)</a>.</p> <p>This operation is idempotent.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/deleting-vaults.html">Deleting a Vault in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-delete.html">Delete Vault </a> in the <i>Amazon S3 Glacier Developer Guide</i>. </p>
    async fn delete_vault(
        &self,
        input: DeleteVaultInput,
    ) -> Result<(), RusotoError<DeleteVaultError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVaultError::from_response(response))
        }
    }

    /// <p>This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon S3 Glacier to completely remove the access policy, and you might still see the effect of the policy for a short time after you send the delete request.</p> <p>This operation is idempotent. You can invoke delete multiple times, even if there is no policy associated with the vault. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    async fn delete_vault_access_policy(
        &self,
        input: DeleteVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<DeleteVaultAccessPolicyError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/access-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>This operation deletes the notification configuration set for a vault. The operation is eventually consistent; that is, it might take some time for Amazon S3 Glacier to completely disable the notifications and you might still receive some notifications for a short time after you send the delete request.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-delete.html">Delete Vault Notification Configuration </a> in the Amazon S3 Glacier Developer Guide. </p>
    async fn delete_vault_notifications(
        &self,
        input: DeleteVaultNotificationsInput,
    ) -> Result<(), RusotoError<DeleteVaultNotificationsError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/notification-configuration",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("DELETE", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVaultNotificationsError::from_response(response))
        }
    }

    /// <p>This operation returns information about a job you previously initiated, including the job initiation date, the user who initiated the job, the job status code/message and the Amazon SNS topic to notify after Amazon S3 Glacier (Glacier) completes the job. For more information about initiating a job, see <a>InitiateJob</a>. </p> <note> <p>This operation enables you to check the status of your job. However, it is strongly recommended that you set up an Amazon SNS topic and specify it in your initiate job request so that Glacier can notify the topic after it completes the job.</p> </note> <p>A job ID will not expire for at least 24 hours after Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-describe-job-get.html">Describe Job</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn describe_job(
        &self,
        input: DescribeJobInput,
    ) -> Result<GlacierJobDescription, RusotoError<DescribeJobError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/jobs/{job_id}",
            account_id = input.account_id,
            job_id = input.job_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GlacierJobDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobError::from_response(response))
        }
    }

    /// <p>This operation returns information about a vault, including the vault's Amazon Resource Name (ARN), the date the vault was created, the number of archives it contains, and the total size of all the archives in the vault. The number of archives and their total size are as of the last inventory generation. This means that if you add or remove an archive from a vault, and then immediately use Describe Vault, the change in contents will not be immediately reflected. If you want to retrieve the latest inventory of the vault, use <a>InitiateJob</a>. Amazon S3 Glacier generates vault inventories approximately daily. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory in Amazon S3 Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-get.html">Describe Vault </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn describe_vault(
        &self,
        input: DescribeVaultInput,
    ) -> Result<DescribeVaultOutput, RusotoError<DescribeVaultError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVaultOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVaultError::from_response(response))
        }
    }

    /// <p>This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>.</p>
    async fn get_data_retrieval_policy(
        &self,
        input: GetDataRetrievalPolicyInput,
    ) -> Result<GetDataRetrievalPolicyOutput, RusotoError<GetDataRetrievalPolicyError>> {
        let request_uri = format!(
            "/{account_id}/policies/data-retrieval",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDataRetrievalPolicyOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataRetrievalPolicyError::from_response(response))
        }
    }

    /// <p>This operation downloads the output of the job you initiated using <a>InitiateJob</a>. Depending on the job type you specified when you initiated the job, the output will be either the content of an archive or a vault inventory.</p> <p>You can download all the job output or download a portion of the output by specifying a byte range. In the case of an archive retrieval job, depending on the byte range you specify, Amazon S3 Glacier (Glacier) returns the checksum for the portion of the data. You can compute the checksum on the client and verify that the values match to ensure the portion you downloaded is the correct data.</p> <p>A job ID will not expire for at least 24 hours after Glacier completes the job. That a byte range. For both archive and inventory retrieval jobs, you should verify the downloaded size against the size returned in the headers from the <b>Get Job Output</b> response.</p> <p>For archive retrieval jobs, you should also verify that the size is what you expected. If you download a portion of the output, the expected size is based on the range of bytes you specified. For example, if you specify a range of <code>bytes=0-1048575</code>, you should verify your download size is 1,048,576 bytes. If you download an entire archive, the expected size is the size of the archive when you uploaded it to Amazon S3 Glacier The expected size is also returned in the headers from the <b>Get Job Output</b> response.</p> <p>In the case of an archive retrieval job, depending on the byte range you specify, Glacier returns the checksum for the portion of the data. To ensure the portion you downloaded is the correct data, compute the checksum on the client, verify that the values match, and verify that the size is what you expected.</p> <p>A job ID does not expire for at least 24 hours after Glacier completes the job. That is, you can download the job output within the 24 hours period after Amazon Glacier completes the job.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-inventory.html">Downloading a Vault Inventory</a>, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/downloading-an-archive.html">Downloading an Archive</a>, and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-job-output-get.html">Get Job Output </a> </p>
    async fn get_job_output(
        &self,
        input: GetJobOutputInput,
    ) -> Result<GetJobOutputOutput, RusotoError<GetJobOutputError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetJobOutputOutput::default();
            result.body = Some(response.body);

            if let Some(accept_ranges) = response.headers.get("Accept-Ranges") {
                let value = accept_ranges.to_owned();
                result.accept_ranges = Some(value)
            };
            if let Some(archive_description) = response.headers.get("x-amz-archive-description") {
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
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobOutputError::from_response(response))
        }
    }

    /// <p>This operation retrieves the <code>access-policy</code> subresource set on the vault; for more information on setting this subresource, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-SetVaultAccessPolicy.html">Set Vault Access Policy (PUT access-policy)</a>. If there is no access policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>.</p>
    async fn get_vault_access_policy(
        &self,
        input: GetVaultAccessPolicyInput,
    ) -> Result<GetVaultAccessPolicyOutput, RusotoError<GetVaultAccessPolicyError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/access-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVaultAccessPolicyOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>This operation retrieves the following attributes from the <code>lock-policy</code> subresource set on the specified vault: </p> <ul> <li> <p>The vault lock policy set on the vault.</p> </li> <li> <p>The state of the vault lock, which is either <code>InProgess</code> or <code>Locked</code>.</p> </li> <li> <p>When the lock ID expires. The lock ID is used to complete the vault locking process.</p> </li> <li> <p>When the vault lock was initiated and put into the <code>InProgress</code> state.</p> </li> </ul> <p>A vault lock is put into the <code>InProgress</code> state by calling <a>InitiateVaultLock</a>. A vault lock is put into the <code>Locked</code> state by calling <a>CompleteVaultLock</a>. You can abort the vault locking process by calling <a>AbortVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>. </p> <p>If there is no vault lock policy set on the vault, the operation returns a <code>404 Not found</code> error. For more information about vault lock policies, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p>
    async fn get_vault_lock(
        &self,
        input: GetVaultLockInput,
    ) -> Result<GetVaultLockOutput, RusotoError<GetVaultLockError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/lock-policy",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVaultLockOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVaultLockError::from_response(response))
        }
    }

    /// <p>This operation retrieves the <code>notification-configuration</code> subresource of the specified vault.</p> <p>For information about setting a notification configuration on a vault, see <a>SetVaultNotifications</a>. If a notification configuration for a vault is not set, the operation returns a <code>404 Not Found</code> error. For more information about vault notifications, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a>. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-get.html">Get Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn get_vault_notifications(
        &self,
        input: GetVaultNotificationsInput,
    ) -> Result<GetVaultNotificationsOutput, RusotoError<GetVaultNotificationsError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/notification-configuration",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVaultNotificationsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVaultNotificationsError::from_response(response))
        }
    }

    /// <p>This operation initiates a job of the specified type, which can be a select, an archival retrieval, or a vault retrieval. For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-initiate-job-post.html">Initiate a Job</a>. </p>
    async fn initiate_job(
        &self,
        input: InitiateJobInput,
    ) -> Result<InitiateJobOutput, RusotoError<InitiateJobError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateJobOutput, _>()?;
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

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateJobError::from_response(response))
        }
    }

    /// <p>This operation initiates a multipart upload. Amazon S3 Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive (see <a>UploadMultipartPart</a>).</p> <p>When you initiate a multipart upload, you specify the part size in number of bytes. The part size must be a megabyte (1024 KB) multiplied by a power of 2-for example, 1048576 (1 MB), 2097152 (2 MB), 4194304 (4 MB), 8388608 (8 MB), and so on. The minimum allowable part size is 1 MB, and the maximum is 4 GB.</p> <p>Every part you upload to this resource (see <a>UploadMultipartPart</a>), except the last one, must have the same size. The last one can be the same size or smaller. For example, suppose you want to upload a 16.2 MB file. If you initiate the multipart upload with a part size of 4 MB, you will upload four parts of 4 MB each and one part of 0.2 MB. </p> <note> <p>You don't need to know the size of the archive when you start a multipart upload because Amazon S3 Glacier does not require you to specify the overall archive size.</p> </note> <p>After you complete the multipart upload, Amazon S3 Glacier (Glacier) removes the multipart upload resource referenced by the ID. Glacier also removes the multipart upload resource if you cancel the multipart upload or it may be removed if there is no activity for a period of 24 hours.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-initiate-upload.html">Initiate Multipart Upload</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn initiate_multipart_upload(
        &self,
        input: InitiateMultipartUploadInput,
    ) -> Result<InitiateMultipartUploadOutput, RusotoError<InitiateMultipartUploadError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateMultipartUploadOutput, _>()?;
            if let Some(location) = response.headers.get("Location") {
                let value = location.to_owned();
                result.location = Some(value)
            };
            if let Some(upload_id) = response.headers.get("x-amz-multipart-upload-id") {
                let value = upload_id.to_owned();
                result.upload_id = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateMultipartUploadError::from_response(response))
        }
    }

    /// <p>This operation initiates the vault locking process by doing the following:</p> <ul> <li> <p>Installing a vault lock policy on the specified vault.</p> </li> <li> <p>Setting the lock state of vault lock to <code>InProgress</code>.</p> </li> <li> <p>Returning a lock ID, which is used to complete the vault locking process.</p> </li> </ul> <p>You can set one vault lock policy for each vault and this policy can be up to 20 KB in size. For more information about vault lock policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock-policy.html">Amazon Glacier Access Control with Vault Lock Policies</a>. </p> <p>You must complete the vault locking process within 24 hours after the vault lock enters the <code>InProgress</code> state. After the 24 hour window ends, the lock ID expires, the vault automatically exits the <code>InProgress</code> state, and the vault lock policy is removed from the vault. You call <a>CompleteVaultLock</a> to complete the vault locking process by setting the state of the vault lock to <code>Locked</code>. </p> <p>After a vault lock is in the <code>Locked</code> state, you cannot initiate a new vault lock for the vault.</p> <p>You can abort the vault locking process by calling <a>AbortVaultLock</a>. You can get the state of the vault lock by calling <a>GetVaultLock</a>. For more information about the vault locking process, <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-lock.html">Amazon Glacier Vault Lock</a>.</p> <p>If this operation is called when the vault lock is in the <code>InProgress</code> state, the operation returns an <code>AccessDeniedException</code> error. When the vault lock is in the <code>InProgress</code> state you must call <a>AbortVaultLock</a> before you can initiate a new vault lock policy. </p>
    async fn initiate_vault_lock(
        &self,
        input: InitiateVaultLockInput,
    ) -> Result<InitiateVaultLockOutput, RusotoError<InitiateVaultLockError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateVaultLockOutput, _>()?;
            if let Some(lock_id) = response.headers.get("x-amz-lock-id") {
                let value = lock_id.to_owned();
                result.lock_id = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateVaultLockError::from_response(response))
        }
    }

    /// <p>This operation lists jobs for a vault, including jobs that are in-progress and jobs that have recently finished. The List Job operation returns a list of these jobs sorted by job initiation time.</p> <note> <p>Amazon Glacier retains recently completed jobs for a period before deleting them; however, it eventually removes completed jobs. The output of completed jobs can be retrieved. Retaining completed jobs for a period of time after they have completed enables you to get a job output in the event you miss the job completion notification or your first attempt to download it fails. For example, suppose you start an archive retrieval job to download an archive. After the job completes, you start to download the archive but encounter a network error. In this scenario, you can retry and download the archive while the job exists.</p> </note> <p>The List Jobs operation supports pagination. You should always check the response <code>Marker</code> field. If there are no more jobs to list, the <code>Marker</code> field is set to <code>null</code>. If there are more jobs to list, the <code>Marker</code> field is set to a non-null value, which you can use to continue the pagination of the list. To return a list of jobs that begins at a specific job, set the marker request parameter to the <code>Marker</code> value for that job that you obtained from a previous List Jobs request.</p> <p>You can set a maximum limit for the number of jobs returned in the response by specifying the <code>limit</code> parameter in the request. The default limit is 50. The number of jobs returned might be fewer than the limit, but the number of returned jobs never exceeds the limit.</p> <p>Additionally, you can filter the jobs list returned by specifying the optional <code>statuscode</code> parameter or <code>completed</code> parameter, or both. Using the <code>statuscode</code> parameter, you can specify to return only jobs that match either the <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code> status. Using the <code>completed</code> parameter, you can specify to return only jobs that were completed (<code>true</code>) or jobs that were not completed (<code>false</code>).</p> <p>For more information about using this operation, see the documentation for the underlying REST API <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-jobs-get.html">List Jobs</a>. </p>
    async fn list_jobs(
        &self,
        input: ListJobsInput,
    ) -> Result<ListJobsOutput, RusotoError<ListJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListJobsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>This operation lists in-progress multipart uploads for the specified vault. An in-progress multipart upload is a multipart upload that has been initiated by an <a>InitiateMultipartUpload</a> request, but has not yet been completed or aborted. The list returned in the List Multipart Upload response has no guaranteed order. </p> <p>The List Multipart Uploads operation supports pagination. By default, this operation returns up to 50 multipart uploads in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of multipart uploads that begins at a specific upload, set the <code>marker</code> request parameter to the value you obtained from a previous List Multipart Upload request. You can also limit the number of uploads returned in the response by specifying the <code>limit</code> parameter in the request.</p> <p>Note the difference between this operation and listing parts (<a>ListParts</a>). The List Multipart Uploads operation lists all multipart uploads for a vault and does not require a multipart upload ID. The List Parts operation requires a multipart upload ID since parts are associated with a single upload.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-uploads.html">List Multipart Uploads </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsInput,
    ) -> Result<ListMultipartUploadsOutput, RusotoError<ListMultipartUploadsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMultipartUploadsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMultipartUploadsError::from_response(response))
        }
    }

    /// <p>This operation lists the parts of an archive that have been uploaded in a specific multipart upload. You can make this request at any time during an in-progress multipart upload before you complete the upload (see <a>CompleteMultipartUpload</a>. List Parts returns an error for completed uploads. The list returned in the List Parts response is sorted by part range. </p> <p>The List Parts operation supports pagination. By default, this operation returns up to 50 uploaded parts in the response. You should always check the response for a <code>marker</code> at which to continue the list; if there are no more items the <code>marker</code> is <code>null</code>. To return a list of parts that begins at a specific part, set the <code>marker</code> request parameter to the value you obtained from a previous List Parts request. You can also limit the number of parts returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and the underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html">Working with Archives in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-multipart-list-parts.html">List Parts</a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn list_parts(
        &self,
        input: ListPartsInput,
    ) -> Result<ListPartsOutput, RusotoError<ListPartsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListPartsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPartsError::from_response(response))
        }
    }

    /// <p>This operation lists the provisioned capacity units for the specified AWS account.</p>
    async fn list_provisioned_capacity(
        &self,
        input: ListProvisionedCapacityInput,
    ) -> Result<ListProvisionedCapacityOutput, RusotoError<ListProvisionedCapacityError>> {
        let request_uri = format!(
            "/{account_id}/provisioned-capacity",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProvisionedCapacityOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProvisionedCapacityError::from_response(response))
        }
    }

    /// <p>This operation lists all the tags attached to a vault. The operation returns an empty map if there are no tags. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>.</p>
    async fn list_tags_for_vault(
        &self,
        input: ListTagsForVaultInput,
    ) -> Result<ListTagsForVaultOutput, RusotoError<ListTagsForVaultError>> {
        let request_uri = format!(
            "/{account_id}/vaults/{vault_name}/tags",
            account_id = input.account_id,
            vault_name = input.vault_name
        );

        let mut request = SignedRequest::new("GET", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForVaultOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForVaultError::from_response(response))
        }
    }

    /// <p>This operation lists all vaults owned by the calling user's account. The list returned in the response is ASCII-sorted by vault name.</p> <p>By default, this operation returns up to 10 items. If there are more vaults to list, the response <code>marker</code> field contains the vault Amazon Resource Name (ARN) at which to continue the list with a new List Vaults request; otherwise, the <code>marker</code> field is <code>null</code>. To return a list of vaults that begins at a specific vault, set the <code>marker</code> request parameter to the vault ARN you obtained from a previous List Vaults request. You can also limit the number of vaults returned in the response by specifying the <code>limit</code> parameter in the request. </p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/retrieving-vault-info.html">Retrieving Vault Metadata in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html">List Vaults </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn list_vaults(
        &self,
        input: ListVaultsInput,
    ) -> Result<ListVaultsOutput, RusotoError<ListVaultsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVaultsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVaultsError::from_response(response))
        }
    }

    /// <p>This operation purchases a provisioned capacity unit for an AWS account. </p>
    async fn purchase_provisioned_capacity(
        &self,
        input: PurchaseProvisionedCapacityInput,
    ) -> Result<PurchaseProvisionedCapacityOutput, RusotoError<PurchaseProvisionedCapacityError>>
    {
        let request_uri = format!(
            "/{account_id}/provisioned-capacity",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PurchaseProvisionedCapacityOutput, _>()?;
            if let Some(capacity_id) = response.headers.get("x-amz-capacity-id") {
                let value = capacity_id.to_owned();
                result.capacity_id = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PurchaseProvisionedCapacityError::from_response(response))
        }
    }

    /// <p>This operation removes one or more tags from the set of tags attached to a vault. For more information about tags, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/tagging.html">Tagging Amazon S3 Glacier Resources</a>. This operation is idempotent. The operation will be successful, even if there are no tags attached to the vault. </p>
    async fn remove_tags_from_vault(
        &self,
        input: RemoveTagsFromVaultInput,
    ) -> Result<(), RusotoError<RemoveTagsFromVaultError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsFromVaultError::from_response(response))
        }
    }

    /// <p>This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful PUT operation.</p> <p>The set policy operation does not affect retrieval jobs that were in progress before the policy was enacted. For more information about data retrieval policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/data-retrieval-policy.html">Amazon Glacier Data Retrieval Policies</a>. </p>
    async fn set_data_retrieval_policy(
        &self,
        input: SetDataRetrievalPolicyInput,
    ) -> Result<(), RusotoError<SetDataRetrievalPolicyError>> {
        let request_uri = format!(
            "/{account_id}/policies/data-retrieval",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("PUT", "glacier", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-glacier-version", "2012-06-01");

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SetDataRetrievalPolicyError::from_response(response))
        }
    }

    /// <p>This operation configures an access policy for a vault and will overwrite an existing policy. To configure a vault access policy, send a PUT request to the <code>access-policy</code> subresource of the vault. An access policy is specific to a vault and is also called a vault subresource. You can set one access policy per vault and the policy can be up to 20 KB in size. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
    async fn set_vault_access_policy(
        &self,
        input: SetVaultAccessPolicyInput,
    ) -> Result<(), RusotoError<SetVaultAccessPolicyError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SetVaultAccessPolicyError::from_response(response))
        }
    }

    /// <p>This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications.</p> <p>To configure vault notifications, send a PUT request to the <code>notification-configuration</code> subresource of the vault. The request should include a JSON document that provides an Amazon SNS topic and specific events for which you want Amazon S3 Glacier to send notifications to the topic.</p> <p>Amazon SNS topics must grant permission to the vault to be allowed to publish notifications to the topic. You can configure a vault to publish a notification for the following vault events:</p> <ul> <li> <p> <b>ArchiveRetrievalCompleted</b> This event occurs when a job that was initiated for an archive retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> <li> <p> <b>InventoryRetrievalCompleted</b> This event occurs when a job that was initiated for an inventory retrieval is completed (<a>InitiateJob</a>). The status of the completed job can be "Succeeded" or "Failed". The notification sent to the SNS topic is the same output as returned from <a>DescribeJob</a>. </p> </li> </ul> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p>For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/configuring-notifications.html">Configuring Vault Notifications in Amazon S3 Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-vault-notifications-put.html">Set Vault Notification Configuration </a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn set_vault_notifications(
        &self,
        input: SetVaultNotificationsInput,
    ) -> Result<(), RusotoError<SetVaultNotificationsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SetVaultNotificationsError::from_response(response))
        }
    }

    /// <p>This operation adds an archive to a vault. This is a synchronous operation, and for a successful upload, your data is durably persisted. Amazon S3 Glacier returns the archive ID in the <code>x-amz-archive-id</code> header of the response. </p> <p>You must use the archive ID to access your data in Amazon S3 Glacier. After you upload an archive, you should save the archive ID returned so that you can retrieve or delete the archive later. Besides saving the archive ID, you can also index it and give it a friendly name to allow for better searching. You can also use the optional archive description field to specify how the archive is referred to in an external index of archives, such as you might create in Amazon DynamoDB. You can also get the vault inventory to obtain a list of archive IDs in a vault. For more information, see <a>InitiateJob</a>. </p> <p>You must provide a SHA256 tree hash of the data you are uploading. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>. </p> <p>You can optionally specify an archive description of up to 1,024 printable ASCII characters. You can get the archive description when you either retrieve the archive or get the vault inventory. For more information, see <a>InitiateJob</a>. Amazon Glacier does not interpret the description in any way. An archive description does not need to be unique. You cannot use the description to retrieve or sort the archive list. </p> <p>Archives are immutable. After you upload an archive, you cannot edit the archive or its description.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-an-archive.html">Uploading an Archive in Amazon Glacier</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-archive-post.html">Upload Archive</a> in the <i>Amazon Glacier Developer Guide</i>. </p>
    async fn upload_archive(
        &self,
        input: UploadArchiveInput,
    ) -> Result<ArchiveCreationOutput, RusotoError<UploadArchiveError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ArchiveCreationOutput, _>()?;
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

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UploadArchiveError::from_response(response))
        }
    }

    /// <p>This operation uploads a part of an archive. You can upload archive parts in any order. You can also upload them in parallel. You can upload up to 10,000 parts for a multipart upload.</p> <p>Amazon Glacier rejects your upload part request if any of the following conditions is true:</p> <ul> <li> <p> <b>SHA256 tree hash does not match</b>To ensure that part data is not corrupted in transmission, you compute a SHA256 tree hash of the part and include it in your request. Upon receiving the part data, Amazon S3 Glacier also computes a SHA256 tree hash. If these hash values don't match, the operation fails. For information about computing a SHA256 tree hash, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html">Computing Checksums</a>.</p> </li> <li> <p> <b>Part size does not match</b>The size of each part except the last must match the size specified in the corresponding <a>InitiateMultipartUpload</a> request. The size of the last part must be the same size as, or smaller than, the specified size.</p> <note> <p>If you upload a part whose size is smaller than the part size you specified in your initiate multipart upload request and that part is not the last part, then the upload part request will succeed. However, the subsequent Complete Multipart Upload request will fail.</p> </note> </li> <li> <p> <b>Range does not align</b>The byte range value in the request does not align with the part size specified in the corresponding initiate request. For example, if you specify a part size of 4194304 bytes (4 MB), then 0 to 4194303 bytes (4 MB - 1) and 4194304 (4 MB) to 8388607 (8 MB - 1) are valid part ranges. However, if you set a range value of 2 MB to 6 MB, the range does not align with the part size and the upload will fail. </p> </li> </ul> <p>This operation is idempotent. If you upload the same part multiple times, the data included in the most recent request overwrites the previously uploaded data.</p> <p>An AWS account has full permission to perform all operations (actions). However, AWS Identity and Access Management (IAM) users don't have any permissions by default. You must grant them explicit permission to perform specific actions. For more information, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/using-iam-with-amazon-glacier.html">Access Control Using AWS Identity and Access Management (IAM)</a>.</p> <p> For conceptual information and underlying REST API, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/uploading-archive-mpu.html">Uploading Large Archives in Parts (Multipart Upload)</a> and <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/api-upload-part.html">Upload Part </a> in the <i>Amazon Glacier Developer Guide</i>.</p>
    async fn upload_multipart_part(
        &self,
        input: UploadMultipartPartInput,
    ) -> Result<UploadMultipartPartOutput, RusotoError<UploadMultipartPartError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UploadMultipartPartOutput, _>()?;
            if let Some(checksum) = response.headers.get("x-amz-sha256-tree-hash") {
                let value = checksum.to_owned();
                result.checksum = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UploadMultipartPartError::from_response(response))
        }
    }
}
