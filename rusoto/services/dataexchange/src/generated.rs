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
/// <p>The destination for the asset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssetDestinationEntry {
    /// <p>The unique identifier for the asset.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The S3 bucket that is the destination for the asset.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>The name of the object in Amazon S3 for the asset.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssetDetails {
    #[serde(rename = "S3SnapshotAsset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_snapshot_asset: Option<S3SnapshotAsset>,
}

/// <p>An asset in AWS Data Exchange is a piece of data that can be stored as an S3 object. The asset can be a structured data file, an image file, or some other data file. When you create an import job for your files, you create an asset in AWS Data Exchange for each of those files.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssetEntry {
    /// <p>The ARN for the asset.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>Information about the asset, including its size.</p>
    #[serde(rename = "AssetDetails")]
    pub asset_details: AssetDetails,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    pub asset_type: AssetType,
    /// <p>The date and time that the asset was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>The unique identifier for the data set associated with this asset.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for the asset.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the asset. When importing from Amazon S3, the S3 object key is used as the asset name. When exporting to Amazon S3, the asset name is used as default target S3 object key.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The unique identifier for the revision associated with this asset.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
    /// <p>The asset ID of the owned asset corresponding to the entitled asset being viewed. This parameter is returned when an asset owner is viewing the entitled copy of its owned asset.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the asset was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

/// <p>The source of the assets.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssetSourceEntry {
    /// <p>The S3 bucket that's part of the source of the asset.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>The name of the object in Amazon S3 for the asset.</p>
    #[serde(rename = "Key")]
    pub key: String,
}

/// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAssetType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AssetType {
    S3Snapshot,
    #[doc(hidden)]
    UnknownVariant(UnknownAssetType),
}

impl Default for AssetType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AssetType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AssetType {
    fn into(self) -> String {
        match self {
            AssetType::S3Snapshot => "S3_SNAPSHOT".to_string(),
            AssetType::UnknownVariant(UnknownAssetType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AssetType {
    fn into(self) -> &'a str {
        match self {
            AssetType::S3Snapshot => &"S3_SNAPSHOT",
            AssetType::UnknownVariant(UnknownAssetType { name: original }) => original,
        }
    }
}

impl From<&str> for AssetType {
    fn from(name: &str) -> Self {
        match name {
            "S3_SNAPSHOT" => AssetType::S3Snapshot,
            _ => AssetType::UnknownVariant(UnknownAssetType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AssetType {
    fn from(name: String) -> Self {
        match &*name {
            "S3_SNAPSHOT" => AssetType::S3Snapshot,
            _ => AssetType::UnknownVariant(UnknownAssetType { name }),
        }
    }
}

impl ::std::str::FromStr for AssetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AssetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AssetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRequest {
    /// <p>The unique identifier for a job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Code {
    AccessDeniedException,
    InternalServerException,
    MalwareDetected,
    MalwareScanEncryptedFile,
    ResourceNotFoundException,
    ServiceQuotaExceededException,
    ValidationException,
    #[doc(hidden)]
    UnknownVariant(UnknownCode),
}

impl Default for Code {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Code {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Code {
    fn into(self) -> String {
        match self {
            Code::AccessDeniedException => "ACCESS_DENIED_EXCEPTION".to_string(),
            Code::InternalServerException => "INTERNAL_SERVER_EXCEPTION".to_string(),
            Code::MalwareDetected => "MALWARE_DETECTED".to_string(),
            Code::MalwareScanEncryptedFile => "MALWARE_SCAN_ENCRYPTED_FILE".to_string(),
            Code::ResourceNotFoundException => "RESOURCE_NOT_FOUND_EXCEPTION".to_string(),
            Code::ServiceQuotaExceededException => "SERVICE_QUOTA_EXCEEDED_EXCEPTION".to_string(),
            Code::ValidationException => "VALIDATION_EXCEPTION".to_string(),
            Code::UnknownVariant(UnknownCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Code {
    fn into(self) -> &'a str {
        match self {
            Code::AccessDeniedException => &"ACCESS_DENIED_EXCEPTION",
            Code::InternalServerException => &"INTERNAL_SERVER_EXCEPTION",
            Code::MalwareDetected => &"MALWARE_DETECTED",
            Code::MalwareScanEncryptedFile => &"MALWARE_SCAN_ENCRYPTED_FILE",
            Code::ResourceNotFoundException => &"RESOURCE_NOT_FOUND_EXCEPTION",
            Code::ServiceQuotaExceededException => &"SERVICE_QUOTA_EXCEEDED_EXCEPTION",
            Code::ValidationException => &"VALIDATION_EXCEPTION",
            Code::UnknownVariant(UnknownCode { name: original }) => original,
        }
    }
}

impl From<&str> for Code {
    fn from(name: &str) -> Self {
        match name {
            "ACCESS_DENIED_EXCEPTION" => Code::AccessDeniedException,
            "INTERNAL_SERVER_EXCEPTION" => Code::InternalServerException,
            "MALWARE_DETECTED" => Code::MalwareDetected,
            "MALWARE_SCAN_ENCRYPTED_FILE" => Code::MalwareScanEncryptedFile,
            "RESOURCE_NOT_FOUND_EXCEPTION" => Code::ResourceNotFoundException,
            "SERVICE_QUOTA_EXCEEDED_EXCEPTION" => Code::ServiceQuotaExceededException,
            "VALIDATION_EXCEPTION" => Code::ValidationException,
            _ => Code::UnknownVariant(UnknownCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Code {
    fn from(name: String) -> Self {
        match &*name {
            "ACCESS_DENIED_EXCEPTION" => Code::AccessDeniedException,
            "INTERNAL_SERVER_EXCEPTION" => Code::InternalServerException,
            "MALWARE_DETECTED" => Code::MalwareDetected,
            "MALWARE_SCAN_ENCRYPTED_FILE" => Code::MalwareScanEncryptedFile,
            "RESOURCE_NOT_FOUND_EXCEPTION" => Code::ResourceNotFoundException,
            "SERVICE_QUOTA_EXCEEDED_EXCEPTION" => Code::ServiceQuotaExceededException,
            "VALIDATION_EXCEPTION" => Code::ValidationException,
            _ => Code::UnknownVariant(UnknownCode { name }),
        }
    }
}

impl ::std::str::FromStr for Code {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The request body for CreateDataSet.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSetRequest {
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    pub asset_type: AssetType,
    /// <p>A description for the data set. This value can be up to 16,348 characters long.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A data set tag is an optional label that you can assign to a data set when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to these data sets and revisions.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSetResponse {
    /// <p>The ARN for the data set.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    /// <p>The date and time that the data set was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description for the data set.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier for the data set.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS Marketplace.</p>
    #[serde(rename = "OriginDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_details: Option<OriginDetails>,
    /// <p>The data set ID of the owned data set corresponding to the entitled data set being viewed. This parameter is returned when a data set owner is viewing the entitled copy of its owned data set.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The tags for the data set.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time that the data set was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The request body for CreateJob.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobRequest {
    /// <p>The details for the CreateJob request.</p>
    #[serde(rename = "Details")]
    pub details: RequestDetails,
    /// <p>The type of job to be created.</p>
    #[serde(rename = "Type")]
    pub type_: Type,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobResponse {
    /// <p>The ARN for the job.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the job was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Details about the job.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResponseDetails>,
    /// <p>The errors associated with jobs.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<JobError>>,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The state of the job.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// <p>The job type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// <p>The date and time that the job was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The request body for CreateRevision.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRevisionRequest {
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>A revision tag is an optional label that you can assign to a revision when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to these data sets and revisions.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRevisionResponse {
    /// <p>The ARN for the revision</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The date and time that the revision was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data set associated with this revision.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>To publish a revision to a data set in a product, the revision must first be finalized. Finalizing a revision tells AWS Data Exchange that your changes to the assets in the revision are complete. After it's in this read-only state, you can publish the revision to your products.</p> <p>Finalized revisions can be published through the AWS Data Exchange console or the AWS Marketplace Catalog API, using the StartChangeSet AWS Marketplace Catalog API action. When using the API, revisions are uniquely identified by their ARN.</p>
    #[serde(rename = "Finalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    /// <p>The unique identifier for the revision.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision ID of the owned revision corresponding to the entitled revision being viewed. This parameter is returned when a revision owner is viewing the entitled copy of its owned revision.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The tags for the revision.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time that the revision was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>A data set is an AWS resource with one or more revisions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSetEntry {
    /// <p>The ARN for the data set.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    pub asset_type: AssetType,
    /// <p>The date and time that the data set was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>The description for the data set.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The unique identifier for the data set.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    #[serde(rename = "Origin")]
    pub origin: Origin,
    /// <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS Marketplace.</p>
    #[serde(rename = "OriginDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_details: Option<OriginDetails>,
    /// <p>The data set ID of the owned data set corresponding to the entitled data set being viewed. This parameter is returned when a data set owner is viewing the entitled copy of its owned data set.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the data set was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAssetRequest {
    /// <p>The unique identifier for an asset.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataSetRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRevisionRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Details {
    #[serde(rename = "ImportAssetFromSignedUrlJobErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_asset_from_signed_url_job_error_details:
        Option<ImportAssetFromSignedUrlJobErrorDetails>,
    #[serde(rename = "ImportAssetsFromS3JobErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_assets_from_s3_job_error_details: Option<Vec<AssetSourceEntry>>,
}

/// <p>Details of the operation to be performed by the job.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportAssetToSignedUrlRequestDetails {
    /// <p>The unique identifier for the asset that is exported to a signed URL.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The unique identifier for the data set associated with this export job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for the revision associated with this export request.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>The details of the export to signed URL response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportAssetToSignedUrlResponseDetails {
    /// <p>The unique identifier for the asset associated with this export job.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The unique identifier for the data set associated with this export job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for the revision associated with this export response.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
    /// <p>The signed URL for the export request.</p>
    #[serde(rename = "SignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url: Option<String>,
    /// <p>The date and time that the signed URL expires, in ISO 8601 format.</p>
    #[serde(rename = "SignedUrlExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url_expires_at: Option<f64>,
}

/// <p>Details of the operation to be performed by the job.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportAssetsToS3RequestDetails {
    /// <p>The destination for the asset.</p>
    #[serde(rename = "AssetDestinations")]
    pub asset_destinations: Vec<AssetDestinationEntry>,
    /// <p>The unique identifier for the data set associated with this export job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>Encryption configuration for the export job.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<ExportServerSideEncryption>,
    /// <p>The unique identifier for the revision associated with this export request.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>Details about the export to Amazon S3 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportAssetsToS3ResponseDetails {
    /// <p>The destination in Amazon S3 where the asset is exported.</p>
    #[serde(rename = "AssetDestinations")]
    pub asset_destinations: Vec<AssetDestinationEntry>,
    /// <p>The unique identifier for the data set associated with this export job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>Encryption configuration of the export job.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<ExportServerSideEncryption>,
    /// <p>The unique identifier for the revision associated with this export response.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>Encryption configuration of the export job. Includes the encryption type as well as the AWS KMS key. The KMS key is only necessary if you chose the KMS encryption type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExportServerSideEncryption {
    /// <p>The Amazon Resource Name (ARN) of the the AWS KMS key you want to use to encrypt the Amazon S3 objects. This parameter is required if you choose aws:kms as an encryption type.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The type of server side encryption used for encrypting the objects in Amazon S3.</p>
    #[serde(rename = "Type")]
    pub type_: ServerSideEncryptionTypes,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAssetRequest {
    /// <p>The unique identifier for an asset.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAssetResponse {
    /// <p>The ARN for the asset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the asset, including its size.</p>
    #[serde(rename = "AssetDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_details: Option<AssetDetails>,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    /// <p>The date and time that the asset was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data set associated with this asset.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The unique identifier for the asset.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the asset When importing from Amazon S3, the S3 object key is used as the asset name. When exporting to Amazon S3, the asset name is used as default target S3 object key.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier for the revision associated with this asset.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The asset ID of the owned asset corresponding to the entitled asset being viewed. This parameter is returned when an asset owner is viewing the entitled copy of its owned asset.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the asset was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataSetRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataSetResponse {
    /// <p>The ARN for the data set.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    /// <p>The date and time that the data set was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description for the data set.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier for the data set.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS Marketplace.</p>
    #[serde(rename = "OriginDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_details: Option<OriginDetails>,
    /// <p>The data set ID of the owned data set corresponding to the entitled data set being viewed. This parameter is returned when a data set owner is viewing the entitled copy of its owned data set.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The tags for the data set.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time that the data set was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobRequest {
    /// <p>The unique identifier for a job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResponse {
    /// <p>The ARN for the job.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the job was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Details about the job.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResponseDetails>,
    /// <p>The errors associated with jobs.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<JobError>>,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The state of the job.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// <p>The job type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// <p>The date and time that the job was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRevisionRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRevisionResponse {
    /// <p>The ARN for the revision</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The date and time that the revision was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data set associated with this revision.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>To publish a revision to a data set in a product, the revision must first be finalized. Finalizing a revision tells AWS Data Exchange that your changes to the assets in the revision are complete. After it's in this read-only state, you can publish the revision to your products.</p> <p>Finalized revisions can be published through the AWS Data Exchange console or the AWS Marketplace Catalog API, using the StartChangeSet AWS Marketplace Catalog API action. When using the API, revisions are uniquely identified by their ARN.</p>
    #[serde(rename = "Finalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    /// <p>The unique identifier for the revision.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision ID of the owned revision corresponding to the entitled revision being viewed. This parameter is returned when a revision owner is viewing the entitled copy of its owned revision.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The tags for the revision.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time that the revision was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportAssetFromSignedUrlJobErrorDetails {
    #[serde(rename = "AssetName")]
    pub asset_name: String,
}

/// <p>Details of the operation to be performed by the job.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportAssetFromSignedUrlRequestDetails {
    /// <p>The name of the asset. When importing from Amazon S3, the S3 object key is used as the asset name.</p>
    #[serde(rename = "AssetName")]
    pub asset_name: String,
    /// <p>The unique identifier for the data set associated with this import job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    #[serde(rename = "Md5Hash")]
    pub md_5_hash: String,
    /// <p>The unique identifier for the revision associated with this import request.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>The details in the response for an import request, including the signed URL and other information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportAssetFromSignedUrlResponseDetails {
    /// <p>The name for the asset associated with this import response.</p>
    #[serde(rename = "AssetName")]
    pub asset_name: String,
    /// <p>The unique identifier for the data set associated with this import job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The Base64-encoded Md5 hash for the asset, used to ensure the integrity of the file at that location.</p>
    #[serde(rename = "Md5Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_5_hash: Option<String>,
    /// <p>The unique identifier for the revision associated with this import response.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
    /// <p>The signed URL.</p>
    #[serde(rename = "SignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url: Option<String>,
    /// <p>The time and date at which the signed URL expires, in ISO 8601 format.</p>
    #[serde(rename = "SignedUrlExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url_expires_at: Option<f64>,
}

/// <p>Details of the operation to be performed by the job.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportAssetsFromS3RequestDetails {
    /// <p>Is a list of S3 bucket and object key pairs.</p>
    #[serde(rename = "AssetSources")]
    pub asset_sources: Vec<AssetSourceEntry>,
    /// <p>The unique identifier for the data set associated with this import job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for the revision associated with this import request.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>Details from an import from Amazon S3 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportAssetsFromS3ResponseDetails {
    /// <p>Is a list of Amazon S3 bucket and object key pairs.</p>
    #[serde(rename = "AssetSources")]
    pub asset_sources: Vec<AssetSourceEntry>,
    /// <p>The unique identifier for the data set associated with this import job.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The unique identifier for the revision associated with this import response.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

/// <p>AWS Data Exchange Jobs are asynchronous import or export operations used to create or copy assets. A data set owner can both import and export as they see fit. Someone with an entitlement to a data set can only export. Jobs are deleted 90 days after they are created.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobEntry {
    /// <p>The ARN for the job.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The date and time that the job was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>Details of the operation to be performed by the job, such as export destination details or import source details.</p>
    #[serde(rename = "Details")]
    pub details: ResponseDetails,
    /// <p>Errors for jobs.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<JobError>>,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The state of the job.</p>
    #[serde(rename = "State")]
    pub state: State,
    /// <p>The job type.</p>
    #[serde(rename = "Type")]
    pub type_: Type,
    /// <p>The date and time that the job was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

/// <p>An error that occurred with the job request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobError {
    /// <p>The code for the job error.</p>
    #[serde(rename = "Code")]
    pub code: Code,
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
    /// <p>The name of the limit that was reached.</p>
    #[serde(rename = "LimitName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<JobErrorLimitName>,
    /// <p>The value of the exceeded limit.</p>
    #[serde(rename = "LimitValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_value: Option<f64>,
    /// <p>The message related to the job error.</p>
    #[serde(rename = "Message")]
    pub message: String,
    /// <p>The unique identifier for the resource related to the error.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of resource related to the error.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<JobErrorResourceTypes>,
}

/// <p>The name of the limit that was reached.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobErrorLimitName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobErrorLimitName {
    AssetSizeInGB,
    AssetsPerRevision,
    #[doc(hidden)]
    UnknownVariant(UnknownJobErrorLimitName),
}

impl Default for JobErrorLimitName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobErrorLimitName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobErrorLimitName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobErrorLimitName {
    fn into(self) -> String {
        match self {
            JobErrorLimitName::AssetSizeInGB => "Asset size in GB".to_string(),
            JobErrorLimitName::AssetsPerRevision => "Assets per revision".to_string(),
            JobErrorLimitName::UnknownVariant(UnknownJobErrorLimitName { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a JobErrorLimitName {
    fn into(self) -> &'a str {
        match self {
            JobErrorLimitName::AssetSizeInGB => &"Asset size in GB",
            JobErrorLimitName::AssetsPerRevision => &"Assets per revision",
            JobErrorLimitName::UnknownVariant(UnknownJobErrorLimitName { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for JobErrorLimitName {
    fn from(name: &str) -> Self {
        match name {
            "Asset size in GB" => JobErrorLimitName::AssetSizeInGB,
            "Assets per revision" => JobErrorLimitName::AssetsPerRevision,
            _ => JobErrorLimitName::UnknownVariant(UnknownJobErrorLimitName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobErrorLimitName {
    fn from(name: String) -> Self {
        match &*name {
            "Asset size in GB" => JobErrorLimitName::AssetSizeInGB,
            "Assets per revision" => JobErrorLimitName::AssetsPerRevision,
            _ => JobErrorLimitName::UnknownVariant(UnknownJobErrorLimitName { name }),
        }
    }
}

impl ::std::str::FromStr for JobErrorLimitName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for JobErrorLimitName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JobErrorLimitName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The types of resource which the job error can apply to.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobErrorResourceTypes {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobErrorResourceTypes {
    Asset,
    Revision,
    #[doc(hidden)]
    UnknownVariant(UnknownJobErrorResourceTypes),
}

impl Default for JobErrorResourceTypes {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobErrorResourceTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobErrorResourceTypes {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobErrorResourceTypes {
    fn into(self) -> String {
        match self {
            JobErrorResourceTypes::Asset => "ASSET".to_string(),
            JobErrorResourceTypes::Revision => "REVISION".to_string(),
            JobErrorResourceTypes::UnknownVariant(UnknownJobErrorResourceTypes {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JobErrorResourceTypes {
    fn into(self) -> &'a str {
        match self {
            JobErrorResourceTypes::Asset => &"ASSET",
            JobErrorResourceTypes::Revision => &"REVISION",
            JobErrorResourceTypes::UnknownVariant(UnknownJobErrorResourceTypes {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for JobErrorResourceTypes {
    fn from(name: &str) -> Self {
        match name {
            "ASSET" => JobErrorResourceTypes::Asset,
            "REVISION" => JobErrorResourceTypes::Revision,
            _ => JobErrorResourceTypes::UnknownVariant(UnknownJobErrorResourceTypes {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobErrorResourceTypes {
    fn from(name: String) -> Self {
        match &*name {
            "ASSET" => JobErrorResourceTypes::Asset,
            "REVISION" => JobErrorResourceTypes::Revision,
            _ => JobErrorResourceTypes::UnknownVariant(UnknownJobErrorResourceTypes { name }),
        }
    }
}

impl ::std::str::FromStr for JobErrorResourceTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for JobErrorResourceTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JobErrorResourceTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLimitName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LimitName {
    AssetPerExportJobFromAmazonS3,
    AssetSizeInGB,
    AssetsPerImportJobFromAmazonS3,
    AssetsPerRevision,
    ConcurrentInProgressJobsToExportAssetsToAmazonS3,
    ConcurrentInProgressJobsToExportAssetsToASignedURL,
    ConcurrentInProgressJobsToImportAssetsFromAmazonS3,
    ConcurrentInProgressJobsToImportAssetsFromASignedURL,
    DataSetsPerAccount,
    DataSetsPerProduct,
    ProductsPerAccount,
    RevisionsPerDataSet,
    #[doc(hidden)]
    UnknownVariant(UnknownLimitName),
}

impl Default for LimitName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LimitName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LimitName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LimitName {
    fn into(self) -> String {
        match self {
            LimitName::AssetPerExportJobFromAmazonS3 => {
                "Asset per export job from Amazon S3".to_string()
            }
            LimitName::AssetSizeInGB => "Asset size in GB".to_string(),
            LimitName::AssetsPerImportJobFromAmazonS3 => {
                "Assets per import job from Amazon S3".to_string()
            }
            LimitName::AssetsPerRevision => "Assets per revision".to_string(),
            LimitName::ConcurrentInProgressJobsToExportAssetsToAmazonS3 => {
                "Concurrent in progress jobs to export assets to Amazon S3".to_string()
            }
            LimitName::ConcurrentInProgressJobsToExportAssetsToASignedURL => {
                "Concurrent in progress jobs to export assets to a signed URL".to_string()
            }
            LimitName::ConcurrentInProgressJobsToImportAssetsFromAmazonS3 => {
                "Concurrent in progress jobs to import assets from Amazon S3".to_string()
            }
            LimitName::ConcurrentInProgressJobsToImportAssetsFromASignedURL => {
                "Concurrent in progress jobs to import assets from a signed URL".to_string()
            }
            LimitName::DataSetsPerAccount => "Data sets per account".to_string(),
            LimitName::DataSetsPerProduct => "Data sets per product".to_string(),
            LimitName::ProductsPerAccount => "Products per account".to_string(),
            LimitName::RevisionsPerDataSet => "Revisions per data set".to_string(),
            LimitName::UnknownVariant(UnknownLimitName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LimitName {
    fn into(self) -> &'a str {
        match self {
            LimitName::AssetPerExportJobFromAmazonS3 => &"Asset per export job from Amazon S3",
            LimitName::AssetSizeInGB => &"Asset size in GB",
            LimitName::AssetsPerImportJobFromAmazonS3 => &"Assets per import job from Amazon S3",
            LimitName::AssetsPerRevision => &"Assets per revision",
            LimitName::ConcurrentInProgressJobsToExportAssetsToAmazonS3 => {
                &"Concurrent in progress jobs to export assets to Amazon S3"
            }
            LimitName::ConcurrentInProgressJobsToExportAssetsToASignedURL => {
                &"Concurrent in progress jobs to export assets to a signed URL"
            }
            LimitName::ConcurrentInProgressJobsToImportAssetsFromAmazonS3 => {
                &"Concurrent in progress jobs to import assets from Amazon S3"
            }
            LimitName::ConcurrentInProgressJobsToImportAssetsFromASignedURL => {
                &"Concurrent in progress jobs to import assets from a signed URL"
            }
            LimitName::DataSetsPerAccount => &"Data sets per account",
            LimitName::DataSetsPerProduct => &"Data sets per product",
            LimitName::ProductsPerAccount => &"Products per account",
            LimitName::RevisionsPerDataSet => &"Revisions per data set",
            LimitName::UnknownVariant(UnknownLimitName { name: original }) => original,
        }
    }
}

impl From<&str> for LimitName {
    fn from(name: &str) -> Self {
        match name {
            "Asset per export job from Amazon S3" => LimitName::AssetPerExportJobFromAmazonS3,
            "Asset size in GB" => LimitName::AssetSizeInGB,
            "Assets per import job from Amazon S3" => LimitName::AssetsPerImportJobFromAmazonS3,
            "Assets per revision" => LimitName::AssetsPerRevision,
            "Concurrent in progress jobs to export assets to Amazon S3" => {
                LimitName::ConcurrentInProgressJobsToExportAssetsToAmazonS3
            }
            "Concurrent in progress jobs to export assets to a signed URL" => {
                LimitName::ConcurrentInProgressJobsToExportAssetsToASignedURL
            }
            "Concurrent in progress jobs to import assets from Amazon S3" => {
                LimitName::ConcurrentInProgressJobsToImportAssetsFromAmazonS3
            }
            "Concurrent in progress jobs to import assets from a signed URL" => {
                LimitName::ConcurrentInProgressJobsToImportAssetsFromASignedURL
            }
            "Data sets per account" => LimitName::DataSetsPerAccount,
            "Data sets per product" => LimitName::DataSetsPerProduct,
            "Products per account" => LimitName::ProductsPerAccount,
            "Revisions per data set" => LimitName::RevisionsPerDataSet,
            _ => LimitName::UnknownVariant(UnknownLimitName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LimitName {
    fn from(name: String) -> Self {
        match &*name {
            "Asset per export job from Amazon S3" => LimitName::AssetPerExportJobFromAmazonS3,
            "Asset size in GB" => LimitName::AssetSizeInGB,
            "Assets per import job from Amazon S3" => LimitName::AssetsPerImportJobFromAmazonS3,
            "Assets per revision" => LimitName::AssetsPerRevision,
            "Concurrent in progress jobs to export assets to Amazon S3" => {
                LimitName::ConcurrentInProgressJobsToExportAssetsToAmazonS3
            }
            "Concurrent in progress jobs to export assets to a signed URL" => {
                LimitName::ConcurrentInProgressJobsToExportAssetsToASignedURL
            }
            "Concurrent in progress jobs to import assets from Amazon S3" => {
                LimitName::ConcurrentInProgressJobsToImportAssetsFromAmazonS3
            }
            "Concurrent in progress jobs to import assets from a signed URL" => {
                LimitName::ConcurrentInProgressJobsToImportAssetsFromASignedURL
            }
            "Data sets per account" => LimitName::DataSetsPerAccount,
            "Data sets per product" => LimitName::DataSetsPerProduct,
            "Products per account" => LimitName::ProductsPerAccount,
            "Revisions per data set" => LimitName::RevisionsPerDataSet,
            _ => LimitName::UnknownVariant(UnknownLimitName { name }),
        }
    }
}

impl ::std::str::FromStr for LimitName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(feature = "serialize_structs")]
impl Serialize for LimitName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for LimitName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSetRevisionsRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The maximum number of results returned by a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSetRevisionsResponse {
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The asset objects listed by the request.</p>
    #[serde(rename = "Revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionEntry>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSetsRequest {
    /// <p>The maximum number of results returned by a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSetsResponse {
    /// <p>The data set objects listed by the request.</p>
    #[serde(rename = "DataSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<DataSetEntry>>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The maximum number of results returned by a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>The jobs listed by the request.</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobEntry>>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRevisionAssetsRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The maximum number of results returned by a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRevisionAssetsResponse {
    /// <p>The asset objects listed by the request.</p>
    #[serde(rename = "Assets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetEntry>>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies an AWS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A label that consists of a customer-defined key and an optional value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers). When an owned data set is published in a product, AWS Data Exchange creates a copy of the data set. Subscribers can access that copy of the data set as an entitled data set.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrigin {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Origin {
    Entitled,
    Owned,
    #[doc(hidden)]
    UnknownVariant(UnknownOrigin),
}

impl Default for Origin {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Origin {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Origin {
    fn into(self) -> String {
        match self {
            Origin::Entitled => "ENTITLED".to_string(),
            Origin::Owned => "OWNED".to_string(),
            Origin::UnknownVariant(UnknownOrigin { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Origin {
    fn into(self) -> &'a str {
        match self {
            Origin::Entitled => &"ENTITLED",
            Origin::Owned => &"OWNED",
            Origin::UnknownVariant(UnknownOrigin { name: original }) => original,
        }
    }
}

impl From<&str> for Origin {
    fn from(name: &str) -> Self {
        match name {
            "ENTITLED" => Origin::Entitled,
            "OWNED" => Origin::Owned,
            _ => Origin::UnknownVariant(UnknownOrigin {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Origin {
    fn from(name: String) -> Self {
        match &*name {
            "ENTITLED" => Origin::Entitled,
            "OWNED" => Origin::Owned,
            _ => Origin::UnknownVariant(UnknownOrigin { name }),
        }
    }
}

impl ::std::str::FromStr for Origin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OriginDetails {
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

/// <p>The details for the request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestDetails {
    /// <p>Details about the export to signed URL request.</p>
    #[serde(rename = "ExportAssetToSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_asset_to_signed_url: Option<ExportAssetToSignedUrlRequestDetails>,
    /// <p>Details about the export to Amazon S3 request.</p>
    #[serde(rename = "ExportAssetsToS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_assets_to_s3: Option<ExportAssetsToS3RequestDetails>,
    /// <p>Details about the import from signed URL request.</p>
    #[serde(rename = "ImportAssetFromSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_asset_from_signed_url: Option<ImportAssetFromSignedUrlRequestDetails>,
    /// <p>Details about the import from Amazon S3 request.</p>
    #[serde(rename = "ImportAssetsFromS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_assets_from_s3: Option<ImportAssetsFromS3RequestDetails>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceType {
    Asset,
    DataSet,
    Job,
    Revision,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceType),
}

impl Default for ResourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceType {
    fn into(self) -> String {
        match self {
            ResourceType::Asset => "ASSET".to_string(),
            ResourceType::DataSet => "DATA_SET".to_string(),
            ResourceType::Job => "JOB".to_string(),
            ResourceType::Revision => "REVISION".to_string(),
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceType {
    fn into(self) -> &'a str {
        match self {
            ResourceType::Asset => &"ASSET",
            ResourceType::DataSet => &"DATA_SET",
            ResourceType::Job => &"JOB",
            ResourceType::Revision => &"REVISION",
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl From<&str> for ResourceType {
    fn from(name: &str) -> Self {
        match name {
            "ASSET" => ResourceType::Asset,
            "DATA_SET" => ResourceType::DataSet,
            "JOB" => ResourceType::Job,
            "REVISION" => ResourceType::Revision,
            _ => ResourceType::UnknownVariant(UnknownResourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceType {
    fn from(name: String) -> Self {
        match &*name {
            "ASSET" => ResourceType::Asset,
            "DATA_SET" => ResourceType::DataSet,
            "JOB" => ResourceType::Job,
            "REVISION" => ResourceType::Revision,
            _ => ResourceType::UnknownVariant(UnknownResourceType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(feature = "serialize_structs")]
impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Details for the response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseDetails {
    /// <p>Details for the export to signed URL response.</p>
    #[serde(rename = "ExportAssetToSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_asset_to_signed_url: Option<ExportAssetToSignedUrlResponseDetails>,
    /// <p>Details for the export to Amazon S3 response.</p>
    #[serde(rename = "ExportAssetsToS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_assets_to_s3: Option<ExportAssetsToS3ResponseDetails>,
    /// <p>Details for the import from signed URL response.</p>
    #[serde(rename = "ImportAssetFromSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_asset_from_signed_url: Option<ImportAssetFromSignedUrlResponseDetails>,
    /// <p>Details for the import from Amazon S3 response.</p>
    #[serde(rename = "ImportAssetsFromS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_assets_from_s3: Option<ImportAssetsFromS3ResponseDetails>,
}

/// <p>A revision is a container for one or more assets.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevisionEntry {
    /// <p>The ARN for the revision.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The date and time that the revision was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>The unique identifier for the data set associated with this revision.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>To publish a revision to a data set in a product, the revision must first be finalized. Finalizing a revision tells AWS Data Exchange that your changes to the assets in the revision are complete. After it's in this read-only state, you can publish the revision to your products.</p> <p>Finalized revisions can be published through the AWS Data Exchange console or the AWS Marketplace Catalog API, using the StartChangeSet AWS Marketplace Catalog API action. When using the API, revisions are uniquely identified by their ARN.</p>
    #[serde(rename = "Finalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    /// <p>The unique identifier for the revision.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The revision ID of the owned revision corresponding to the entitled revision being viewed. This parameter is returned when a revision owner is viewing the entitled copy of its owned revision.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the revision was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

/// <p>The S3 object that is the asset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3SnapshotAsset {
    /// <p>The size of the S3 object that is the object.</p>
    #[serde(rename = "Size")]
    pub size: f64,
}

/// <p>The types of encryption supported in export jobs to Amazon S3.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownServerSideEncryptionTypes {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ServerSideEncryptionTypes {
    Aes256,
    AwsKms,
    #[doc(hidden)]
    UnknownVariant(UnknownServerSideEncryptionTypes),
}

impl Default for ServerSideEncryptionTypes {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ServerSideEncryptionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ServerSideEncryptionTypes {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ServerSideEncryptionTypes {
    fn into(self) -> String {
        match self {
            ServerSideEncryptionTypes::Aes256 => "AES256".to_string(),
            ServerSideEncryptionTypes::AwsKms => "aws:kms".to_string(),
            ServerSideEncryptionTypes::UnknownVariant(UnknownServerSideEncryptionTypes {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ServerSideEncryptionTypes {
    fn into(self) -> &'a str {
        match self {
            ServerSideEncryptionTypes::Aes256 => &"AES256",
            ServerSideEncryptionTypes::AwsKms => &"aws:kms",
            ServerSideEncryptionTypes::UnknownVariant(UnknownServerSideEncryptionTypes {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ServerSideEncryptionTypes {
    fn from(name: &str) -> Self {
        match name {
            "AES256" => ServerSideEncryptionTypes::Aes256,
            "aws:kms" => ServerSideEncryptionTypes::AwsKms,
            _ => ServerSideEncryptionTypes::UnknownVariant(UnknownServerSideEncryptionTypes {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ServerSideEncryptionTypes {
    fn from(name: String) -> Self {
        match &*name {
            "AES256" => ServerSideEncryptionTypes::Aes256,
            "aws:kms" => ServerSideEncryptionTypes::AwsKms,
            _ => {
                ServerSideEncryptionTypes::UnknownVariant(UnknownServerSideEncryptionTypes { name })
            }
        }
    }
}

impl ::std::str::FromStr for ServerSideEncryptionTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ServerSideEncryptionTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ServerSideEncryptionTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartJobRequest {
    /// <p>The unique identifier for a job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartJobResponse {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum State {
    Cancelled,
    Completed,
    Error,
    InProgress,
    TimedOut,
    Waiting,
    #[doc(hidden)]
    UnknownVariant(UnknownState),
}

impl Default for State {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for State {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for State {
    fn into(self) -> String {
        match self {
            State::Cancelled => "CANCELLED".to_string(),
            State::Completed => "COMPLETED".to_string(),
            State::Error => "ERROR".to_string(),
            State::InProgress => "IN_PROGRESS".to_string(),
            State::TimedOut => "TIMED_OUT".to_string(),
            State::Waiting => "WAITING".to_string(),
            State::UnknownVariant(UnknownState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a State {
    fn into(self) -> &'a str {
        match self {
            State::Cancelled => &"CANCELLED",
            State::Completed => &"COMPLETED",
            State::Error => &"ERROR",
            State::InProgress => &"IN_PROGRESS",
            State::TimedOut => &"TIMED_OUT",
            State::Waiting => &"WAITING",
            State::UnknownVariant(UnknownState { name: original }) => original,
        }
    }
}

impl From<&str> for State {
    fn from(name: &str) -> Self {
        match name {
            "CANCELLED" => State::Cancelled,
            "COMPLETED" => State::Completed,
            "ERROR" => State::Error,
            "IN_PROGRESS" => State::InProgress,
            "TIMED_OUT" => State::TimedOut,
            "WAITING" => State::Waiting,
            _ => State::UnknownVariant(UnknownState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for State {
    fn from(name: String) -> Self {
        match &*name {
            "CANCELLED" => State::Cancelled,
            "COMPLETED" => State::Completed,
            "ERROR" => State::Error,
            "IN_PROGRESS" => State::InProgress,
            "TIMED_OUT" => State::TimedOut,
            "WAITING" => State::Waiting,
            _ => State::UnknownVariant(UnknownState { name }),
        }
    }
}

impl ::std::str::FromStr for State {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The request body for TagResource.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies an AWS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A label that consists of a customer-defined key and an optional value.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Type {
    ExportAssetsToS3,
    ExportAssetToSignedUrl,
    ImportAssetsFromS3,
    ImportAssetFromSignedUrl,
    #[doc(hidden)]
    UnknownVariant(UnknownType),
}

impl Default for Type {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Type {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Type {
    fn into(self) -> String {
        match self {
            Type::ExportAssetsToS3 => "EXPORT_ASSETS_TO_S3".to_string(),
            Type::ExportAssetToSignedUrl => "EXPORT_ASSET_TO_SIGNED_URL".to_string(),
            Type::ImportAssetsFromS3 => "IMPORT_ASSETS_FROM_S3".to_string(),
            Type::ImportAssetFromSignedUrl => "IMPORT_ASSET_FROM_SIGNED_URL".to_string(),
            Type::UnknownVariant(UnknownType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Type {
    fn into(self) -> &'a str {
        match self {
            Type::ExportAssetsToS3 => &"EXPORT_ASSETS_TO_S3",
            Type::ExportAssetToSignedUrl => &"EXPORT_ASSET_TO_SIGNED_URL",
            Type::ImportAssetsFromS3 => &"IMPORT_ASSETS_FROM_S3",
            Type::ImportAssetFromSignedUrl => &"IMPORT_ASSET_FROM_SIGNED_URL",
            Type::UnknownVariant(UnknownType { name: original }) => original,
        }
    }
}

impl From<&str> for Type {
    fn from(name: &str) -> Self {
        match name {
            "EXPORT_ASSETS_TO_S3" => Type::ExportAssetsToS3,
            "EXPORT_ASSET_TO_SIGNED_URL" => Type::ExportAssetToSignedUrl,
            "IMPORT_ASSETS_FROM_S3" => Type::ImportAssetsFromS3,
            "IMPORT_ASSET_FROM_SIGNED_URL" => Type::ImportAssetFromSignedUrl,
            _ => Type::UnknownVariant(UnknownType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Type {
    fn from(name: String) -> Self {
        match &*name {
            "EXPORT_ASSETS_TO_S3" => Type::ExportAssetsToS3,
            "EXPORT_ASSET_TO_SIGNED_URL" => Type::ExportAssetToSignedUrl,
            "IMPORT_ASSETS_FROM_S3" => Type::ImportAssetsFromS3,
            "IMPORT_ASSET_FROM_SIGNED_URL" => Type::ImportAssetFromSignedUrl,
            _ => Type::UnknownVariant(UnknownType { name }),
        }
    }
}

impl ::std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies an AWS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key tags.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The request body for UpdateAsset.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAssetRequest {
    /// <p>The unique identifier for an asset.</p>
    #[serde(rename = "AssetId")]
    pub asset_id: String,
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The name of the asset. When importing from Amazon S3, the S3 object key is used as the asset name. When exporting to Amazon S3, the asset name is used as default target S3 object key.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAssetResponse {
    /// <p>The ARN for the asset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the asset, including its size.</p>
    #[serde(rename = "AssetDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_details: Option<AssetDetails>,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    /// <p>The date and time that the asset was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data set associated with this asset.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>The unique identifier for the asset.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the asset When importing from Amazon S3, the S3 object key is used as the asset name. When exporting to Amazon S3, the asset name is used as default target S3 object key.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier for the revision associated with this asset.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The asset ID of the owned asset corresponding to the entitled asset being viewed. This parameter is returned when an asset owner is viewing the entitled copy of its owned asset.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the asset was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The request body for UpdateDataSet.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSetRequest {
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>The description for the data set.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSetResponse {
    /// <p>The ARN for the data set.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type of file your data is stored in. Currently, the supported asset type is S3_SNAPSHOT.</p>
    #[serde(rename = "AssetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    /// <p>The date and time that the data set was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description for the data set.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier for the data set.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    /// <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS Marketplace.</p>
    #[serde(rename = "OriginDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_details: Option<OriginDetails>,
    /// <p>The data set ID of the owned data set corresponding to the entitled data set being viewed. This parameter is returned when a data set owner is viewing the entitled copy of its owned data set.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the data set was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The request body for UpdateRevision.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRevisionRequest {
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The unique identifier for a data set.</p>
    #[serde(rename = "DataSetId")]
    pub data_set_id: String,
    /// <p>Finalizing a revision tells AWS Data Exchange that your changes to the assets in the revision are complete. After it's in this read-only state, you can publish the revision to your products.</p>
    #[serde(rename = "Finalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    /// <p>The unique identifier for a revision.</p>
    #[serde(rename = "RevisionId")]
    pub revision_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRevisionResponse {
    /// <p>The ARN for the revision.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional comment about the revision.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The date and time that the revision was created, in ISO 8601 format.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data set associated with this revision.</p>
    #[serde(rename = "DataSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    /// <p>To publish a revision to a data set in a product, the revision must first be finalized. Finalizing a revision tells AWS Data Exchange that changes to the assets in the revision are complete. After it's in this read-only state, you can publish the revision to your products.</p> <p>Finalized revisions can be published through the AWS Data Exchange console or the AWS Marketplace Catalog API, using the StartChangeSet AWS Marketplace Catalog API action. When using the API, revisions are uniquely identified by their ARN.</p>
    #[serde(rename = "Finalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    /// <p>The unique identifier for the revision.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision ID of the owned revision corresponding to the entitled revision being viewed. This parameter is returned when a revision owner is viewing the entitled copy of its owned revision.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The date and time that the revision was last updated, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CancelJobError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CancelJobError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobError::Conflict(ref cause) => write!(f, "{}", cause),
            CancelJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateDataSet
#[derive(Debug, PartialEq)]
pub enum CreateDataSetError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The request has exceeded the quotas imposed by the service.</p>
    ServiceLimitExceeded(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl CreateDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDataSetError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataSetError::InternalServer(err.msg))
                }
                "ServiceLimitExceededException" => {
                    return RusotoError::Service(CreateDataSetError::ServiceLimitExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::ServiceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSetError {}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateJobError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateJobError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobError {}
/// Errors returned by CreateRevision
#[derive(Debug, PartialEq)]
pub enum CreateRevisionError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl CreateRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateRevisionError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateRevisionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateRevisionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRevisionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRevisionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateRevisionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRevisionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateRevisionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRevisionError {}
/// Errors returned by DeleteAsset
#[derive(Debug, PartialEq)]
pub enum DeleteAssetError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl DeleteAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAssetError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteAssetError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteAssetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAssetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteAssetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAssetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAssetError {}
/// Errors returned by DeleteDataSet
#[derive(Debug, PartialEq)]
pub enum DeleteDataSetError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl DeleteDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDataSetError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDataSetError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDataSetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataSetError {}
/// Errors returned by DeleteRevision
#[derive(Debug, PartialEq)]
pub enum DeleteRevisionError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl DeleteRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteRevisionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteRevisionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteRevisionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRevisionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteRevisionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRevisionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteRevisionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteRevisionError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteRevisionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRevisionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRevisionError {}
/// Errors returned by GetAsset
#[derive(Debug, PartialEq)]
pub enum GetAssetError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetAssetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAssetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetAssetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAssetError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetAssetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetAssetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAssetError {}
/// Errors returned by GetDataSet
#[derive(Debug, PartialEq)]
pub enum GetDataSetError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDataSetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataSetError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataSetError {}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetJobError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobError {}
/// Errors returned by GetRevision
#[derive(Debug, PartialEq)]
pub enum GetRevisionError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetRevisionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRevisionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetRevisionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRevisionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetRevisionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRevisionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRevisionError {}
/// Errors returned by ListDataSetRevisions
#[derive(Debug, PartialEq)]
pub enum ListDataSetRevisionsError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListDataSetRevisionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSetRevisionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDataSetRevisionsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDataSetRevisionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDataSetRevisionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataSetRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataSetRevisionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDataSetRevisionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDataSetRevisionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSetRevisionsError {}
/// Errors returned by ListDataSets
#[derive(Debug, PartialEq)]
pub enum ListDataSetsError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListDataSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDataSetsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDataSetsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDataSetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataSetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDataSetsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSetsError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListJobsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListJobsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListJobsError::Throttling(err.msg))
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
            ListJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListRevisionAssets
#[derive(Debug, PartialEq)]
pub enum ListRevisionAssetsError {
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl ListRevisionAssetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRevisionAssetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRevisionAssetsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRevisionAssetsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRevisionAssetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRevisionAssetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRevisionAssetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRevisionAssetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRevisionAssetsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRevisionAssetsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by StartJob
#[derive(Debug, PartialEq)]
pub enum StartJobError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl StartJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartJobError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartJobError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartJobError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartJobError::Conflict(ref cause) => write!(f, "{}", cause),
            StartJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAsset
#[derive(Debug, PartialEq)]
pub enum UpdateAssetError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl UpdateAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateAssetError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateAssetError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateAssetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAssetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAssetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAssetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateAssetError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateAssetError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateAssetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAssetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAssetError {}
/// Errors returned by UpdateDataSet
#[derive(Debug, PartialEq)]
pub enum UpdateDataSetError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl UpdateDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSetError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDataSetError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDataSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSetError {}
/// Errors returned by UpdateRevision
#[derive(Debug, PartialEq)]
pub enum UpdateRevisionError {
    /// <p>Access to the resource is denied.</p>
    AccessDenied(String),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    Conflict(String),
    /// <p>An exception occurred with the service.</p>
    InternalServer(String),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFound(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl UpdateRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRevisionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateRevisionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateRevisionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRevisionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRevisionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRevisionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateRevisionError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateRevisionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateRevisionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRevisionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRevisionError {}
/// Trait representing the capabilities of the AWS Data Exchange API. AWS Data Exchange clients implement this trait.
#[async_trait]
pub trait DataExchange {
    /// <p>This operation cancels a job. Jobs can be cancelled only when they are in the WAITING state.</p>
    async fn cancel_job(&self, input: CancelJobRequest) -> Result<(), RusotoError<CancelJobError>>;

    /// <p>This operation creates a data set.</p>
    async fn create_data_set(
        &self,
        input: CreateDataSetRequest,
    ) -> Result<CreateDataSetResponse, RusotoError<CreateDataSetError>>;

    /// <p>This operation creates a job.</p>
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>>;

    /// <p>This operation creates a revision for a data set.</p>
    async fn create_revision(
        &self,
        input: CreateRevisionRequest,
    ) -> Result<CreateRevisionResponse, RusotoError<CreateRevisionError>>;

    /// <p>This operation deletes an asset.</p>
    async fn delete_asset(
        &self,
        input: DeleteAssetRequest,
    ) -> Result<(), RusotoError<DeleteAssetError>>;

    /// <p>This operation deletes a data set.</p>
    async fn delete_data_set(
        &self,
        input: DeleteDataSetRequest,
    ) -> Result<(), RusotoError<DeleteDataSetError>>;

    /// <p>This operation deletes a revision.</p>
    async fn delete_revision(
        &self,
        input: DeleteRevisionRequest,
    ) -> Result<(), RusotoError<DeleteRevisionError>>;

    /// <p>This operation returns information about an asset.</p>
    async fn get_asset(
        &self,
        input: GetAssetRequest,
    ) -> Result<GetAssetResponse, RusotoError<GetAssetError>>;

    /// <p>This operation returns information about a data set.</p>
    async fn get_data_set(
        &self,
        input: GetDataSetRequest,
    ) -> Result<GetDataSetResponse, RusotoError<GetDataSetError>>;

    /// <p>This operation returns information about a job.</p>
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>>;

    /// <p>This operation returns information about a revision.</p>
    async fn get_revision(
        &self,
        input: GetRevisionRequest,
    ) -> Result<GetRevisionResponse, RusotoError<GetRevisionError>>;

    /// <p>This operation lists a data set's revisions sorted by CreatedAt in descending order.</p>
    async fn list_data_set_revisions(
        &self,
        input: ListDataSetRevisionsRequest,
    ) -> Result<ListDataSetRevisionsResponse, RusotoError<ListDataSetRevisionsError>>;

    /// <p>This operation lists your data sets. When listing by origin OWNED, results are sorted by CreatedAt in descending order. When listing by origin ENTITLED, there is no order and the maxResults parameter is ignored.</p>
    async fn list_data_sets(
        &self,
        input: ListDataSetsRequest,
    ) -> Result<ListDataSetsResponse, RusotoError<ListDataSetsError>>;

    /// <p>This operation lists your jobs sorted by CreatedAt in descending order.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>>;

    /// <p>This operation lists a revision's assets sorted alphabetically in descending order.</p>
    async fn list_revision_assets(
        &self,
        input: ListRevisionAssetsRequest,
    ) -> Result<ListRevisionAssetsResponse, RusotoError<ListRevisionAssetsError>>;

    /// <p>This operation lists the tags on the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>This operation starts a job.</p>
    async fn start_job(
        &self,
        input: StartJobRequest,
    ) -> Result<StartJobResponse, RusotoError<StartJobError>>;

    /// <p>This operation tags a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>This operation removes one or more tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>This operation updates an asset.</p>
    async fn update_asset(
        &self,
        input: UpdateAssetRequest,
    ) -> Result<UpdateAssetResponse, RusotoError<UpdateAssetError>>;

    /// <p>This operation updates a data set.</p>
    async fn update_data_set(
        &self,
        input: UpdateDataSetRequest,
    ) -> Result<UpdateDataSetResponse, RusotoError<UpdateDataSetError>>;

    /// <p>This operation updates a revision.</p>
    async fn update_revision(
        &self,
        input: UpdateRevisionRequest,
    ) -> Result<UpdateRevisionResponse, RusotoError<UpdateRevisionError>>;
}
/// A client for the AWS Data Exchange API.
#[derive(Clone)]
pub struct DataExchangeClient {
    client: Client,
    region: region::Region,
}

impl DataExchangeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DataExchangeClient {
        DataExchangeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DataExchangeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DataExchangeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DataExchangeClient {
        DataExchangeClient { client, region }
    }
}

#[async_trait]
impl DataExchange for DataExchangeClient {
    /// <p>This operation cancels a job. Jobs can be cancelled only when they are in the WAITING state.</p>
    #[allow(unused_mut)]
    async fn cancel_job(&self, input: CancelJobRequest) -> Result<(), RusotoError<CancelJobError>> {
        let request_uri = format!("/v1/jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("DELETE", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobError::from_response(response))
        }
    }

    /// <p>This operation creates a data set.</p>
    #[allow(unused_mut)]
    async fn create_data_set(
        &self,
        input: CreateDataSetRequest,
    ) -> Result<CreateDataSetResponse, RusotoError<CreateDataSetError>> {
        let request_uri = "/v1/data-sets";

        let mut request = SignedRequest::new("POST", "dataexchange", &self.region, &request_uri);
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
                .deserialize::<CreateDataSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSetError::from_response(response))
        }
    }

    /// <p>This operation creates a job.</p>
    #[allow(unused_mut)]
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>> {
        let request_uri = "/v1/jobs";

        let mut request = SignedRequest::new("POST", "dataexchange", &self.region, &request_uri);
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
                .deserialize::<CreateJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobError::from_response(response))
        }
    }

    /// <p>This operation creates a revision for a data set.</p>
    #[allow(unused_mut)]
    async fn create_revision(
        &self,
        input: CreateRevisionRequest,
    ) -> Result<CreateRevisionResponse, RusotoError<CreateRevisionError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions",
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("POST", "dataexchange", &self.region, &request_uri);
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
                .deserialize::<CreateRevisionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRevisionError::from_response(response))
        }
    }

    /// <p>This operation deletes an asset.</p>
    #[allow(unused_mut)]
    async fn delete_asset(
        &self,
        input: DeleteAssetRequest,
    ) -> Result<(), RusotoError<DeleteAssetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}/assets/{asset_id}",
            asset_id = input.asset_id,
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("DELETE", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAssetError::from_response(response))
        }
    }

    /// <p>This operation deletes a data set.</p>
    #[allow(unused_mut)]
    async fn delete_data_set(
        &self,
        input: DeleteDataSetRequest,
    ) -> Result<(), RusotoError<DeleteDataSetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}",
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("DELETE", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataSetError::from_response(response))
        }
    }

    /// <p>This operation deletes a revision.</p>
    #[allow(unused_mut)]
    async fn delete_revision(
        &self,
        input: DeleteRevisionRequest,
    ) -> Result<(), RusotoError<DeleteRevisionError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}",
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("DELETE", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRevisionError::from_response(response))
        }
    }

    /// <p>This operation returns information about an asset.</p>
    #[allow(unused_mut)]
    async fn get_asset(
        &self,
        input: GetAssetRequest,
    ) -> Result<GetAssetResponse, RusotoError<GetAssetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}/assets/{asset_id}",
            asset_id = input.asset_id,
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAssetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAssetError::from_response(response))
        }
    }

    /// <p>This operation returns information about a data set.</p>
    #[allow(unused_mut)]
    async fn get_data_set(
        &self,
        input: GetDataSetRequest,
    ) -> Result<GetDataSetResponse, RusotoError<GetDataSetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}",
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDataSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataSetError::from_response(response))
        }
    }

    /// <p>This operation returns information about a job.</p>
    #[allow(unused_mut)]
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>> {
        let request_uri = format!("/v1/jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobError::from_response(response))
        }
    }

    /// <p>This operation returns information about a revision.</p>
    #[allow(unused_mut)]
    async fn get_revision(
        &self,
        input: GetRevisionRequest,
    ) -> Result<GetRevisionResponse, RusotoError<GetRevisionError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}",
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRevisionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRevisionError::from_response(response))
        }
    }

    /// <p>This operation lists a data set's revisions sorted by CreatedAt in descending order.</p>
    #[allow(unused_mut)]
    async fn list_data_set_revisions(
        &self,
        input: ListDataSetRevisionsRequest,
    ) -> Result<ListDataSetRevisionsResponse, RusotoError<ListDataSetRevisionsError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions",
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDataSetRevisionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataSetRevisionsError::from_response(response))
        }
    }

    /// <p>This operation lists your data sets. When listing by origin OWNED, results are sorted by CreatedAt in descending order. When listing by origin ENTITLED, there is no order and the maxResults parameter is ignored.</p>
    #[allow(unused_mut)]
    async fn list_data_sets(
        &self,
        input: ListDataSetsRequest,
    ) -> Result<ListDataSetsResponse, RusotoError<ListDataSetsError>> {
        let request_uri = "/v1/data-sets";

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.origin {
            params.put("origin", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDataSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataSetsError::from_response(response))
        }
    }

    /// <p>This operation lists your jobs sorted by CreatedAt in descending order.</p>
    #[allow(unused_mut)]
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>> {
        let request_uri = "/v1/jobs";

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.data_set_id {
            params.put("dataSetId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.revision_id {
            params.put("revisionId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>This operation lists a revision's assets sorted alphabetically in descending order.</p>
    #[allow(unused_mut)]
    async fn list_revision_assets(
        &self,
        input: ListRevisionAssetsRequest,
    ) -> Result<ListRevisionAssetsResponse, RusotoError<ListRevisionAssetsError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}/assets",
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRevisionAssetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRevisionAssetsError::from_response(response))
        }
    }

    /// <p>This operation lists the tags on the resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>This operation starts a job.</p>
    #[allow(unused_mut)]
    async fn start_job(
        &self,
        input: StartJobRequest,
    ) -> Result<StartJobResponse, RusotoError<StartJobError>> {
        let request_uri = format!("/v1/jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("PATCH", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartJobError::from_response(response))
        }
    }

    /// <p>This operation tags a resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>This operation removes one or more tags from a resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "dataexchange", &self.region, &request_uri);
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
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>This operation updates an asset.</p>
    #[allow(unused_mut)]
    async fn update_asset(
        &self,
        input: UpdateAssetRequest,
    ) -> Result<UpdateAssetResponse, RusotoError<UpdateAssetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}/assets/{asset_id}",
            asset_id = input.asset_id,
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("PATCH", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAssetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAssetError::from_response(response))
        }
    }

    /// <p>This operation updates a data set.</p>
    #[allow(unused_mut)]
    async fn update_data_set(
        &self,
        input: UpdateDataSetRequest,
    ) -> Result<UpdateDataSetResponse, RusotoError<UpdateDataSetError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}",
            data_set_id = input.data_set_id
        );

        let mut request = SignedRequest::new("PATCH", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSetError::from_response(response))
        }
    }

    /// <p>This operation updates a revision.</p>
    #[allow(unused_mut)]
    async fn update_revision(
        &self,
        input: UpdateRevisionRequest,
    ) -> Result<UpdateRevisionResponse, RusotoError<UpdateRevisionError>> {
        let request_uri = format!(
            "/v1/data-sets/{data_set_id}/revisions/{revision_id}",
            data_set_id = input.data_set_id,
            revision_id = input.revision_id
        );

        let mut request = SignedRequest::new("PATCH", "dataexchange", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRevisionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRevisionError::from_response(response))
        }
    }
}
