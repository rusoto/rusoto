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

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl CostAndUsageReportClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "cur", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>The region of the S3 bucket that AWS delivers the report into.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAWSRegion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AWSRegion {
    AfSouth1,
    ApEast1,
    ApNortheast1,
    ApNortheast2,
    ApNortheast3,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    CaCentral1,
    CnNorth1,
    CnNorthwest1,
    EuCentral1,
    EuNorth1,
    EuSouth1,
    EuWest1,
    EuWest2,
    EuWest3,
    MeSouth1,
    SaEast1,
    UsEast1,
    UsEast2,
    UsWest1,
    UsWest2,
    #[doc(hidden)]
    UnknownVariant(UnknownAWSRegion),
}

impl Default for AWSRegion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AWSRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AWSRegion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AWSRegion {
    fn into(self) -> String {
        match self {
            AWSRegion::AfSouth1 => "af-south-1".to_string(),
            AWSRegion::ApEast1 => "ap-east-1".to_string(),
            AWSRegion::ApNortheast1 => "ap-northeast-1".to_string(),
            AWSRegion::ApNortheast2 => "ap-northeast-2".to_string(),
            AWSRegion::ApNortheast3 => "ap-northeast-3".to_string(),
            AWSRegion::ApSouth1 => "ap-south-1".to_string(),
            AWSRegion::ApSoutheast1 => "ap-southeast-1".to_string(),
            AWSRegion::ApSoutheast2 => "ap-southeast-2".to_string(),
            AWSRegion::CaCentral1 => "ca-central-1".to_string(),
            AWSRegion::CnNorth1 => "cn-north-1".to_string(),
            AWSRegion::CnNorthwest1 => "cn-northwest-1".to_string(),
            AWSRegion::EuCentral1 => "eu-central-1".to_string(),
            AWSRegion::EuNorth1 => "eu-north-1".to_string(),
            AWSRegion::EuSouth1 => "eu-south-1".to_string(),
            AWSRegion::EuWest1 => "eu-west-1".to_string(),
            AWSRegion::EuWest2 => "eu-west-2".to_string(),
            AWSRegion::EuWest3 => "eu-west-3".to_string(),
            AWSRegion::MeSouth1 => "me-south-1".to_string(),
            AWSRegion::SaEast1 => "sa-east-1".to_string(),
            AWSRegion::UsEast1 => "us-east-1".to_string(),
            AWSRegion::UsEast2 => "us-east-2".to_string(),
            AWSRegion::UsWest1 => "us-west-1".to_string(),
            AWSRegion::UsWest2 => "us-west-2".to_string(),
            AWSRegion::UnknownVariant(UnknownAWSRegion { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AWSRegion {
    fn into(self) -> &'a str {
        match self {
            AWSRegion::AfSouth1 => &"af-south-1",
            AWSRegion::ApEast1 => &"ap-east-1",
            AWSRegion::ApNortheast1 => &"ap-northeast-1",
            AWSRegion::ApNortheast2 => &"ap-northeast-2",
            AWSRegion::ApNortheast3 => &"ap-northeast-3",
            AWSRegion::ApSouth1 => &"ap-south-1",
            AWSRegion::ApSoutheast1 => &"ap-southeast-1",
            AWSRegion::ApSoutheast2 => &"ap-southeast-2",
            AWSRegion::CaCentral1 => &"ca-central-1",
            AWSRegion::CnNorth1 => &"cn-north-1",
            AWSRegion::CnNorthwest1 => &"cn-northwest-1",
            AWSRegion::EuCentral1 => &"eu-central-1",
            AWSRegion::EuNorth1 => &"eu-north-1",
            AWSRegion::EuSouth1 => &"eu-south-1",
            AWSRegion::EuWest1 => &"eu-west-1",
            AWSRegion::EuWest2 => &"eu-west-2",
            AWSRegion::EuWest3 => &"eu-west-3",
            AWSRegion::MeSouth1 => &"me-south-1",
            AWSRegion::SaEast1 => &"sa-east-1",
            AWSRegion::UsEast1 => &"us-east-1",
            AWSRegion::UsEast2 => &"us-east-2",
            AWSRegion::UsWest1 => &"us-west-1",
            AWSRegion::UsWest2 => &"us-west-2",
            AWSRegion::UnknownVariant(UnknownAWSRegion { name: original }) => original,
        }
    }
}

impl From<&str> for AWSRegion {
    fn from(name: &str) -> Self {
        match name {
            "af-south-1" => AWSRegion::AfSouth1,
            "ap-east-1" => AWSRegion::ApEast1,
            "ap-northeast-1" => AWSRegion::ApNortheast1,
            "ap-northeast-2" => AWSRegion::ApNortheast2,
            "ap-northeast-3" => AWSRegion::ApNortheast3,
            "ap-south-1" => AWSRegion::ApSouth1,
            "ap-southeast-1" => AWSRegion::ApSoutheast1,
            "ap-southeast-2" => AWSRegion::ApSoutheast2,
            "ca-central-1" => AWSRegion::CaCentral1,
            "cn-north-1" => AWSRegion::CnNorth1,
            "cn-northwest-1" => AWSRegion::CnNorthwest1,
            "eu-central-1" => AWSRegion::EuCentral1,
            "eu-north-1" => AWSRegion::EuNorth1,
            "eu-south-1" => AWSRegion::EuSouth1,
            "eu-west-1" => AWSRegion::EuWest1,
            "eu-west-2" => AWSRegion::EuWest2,
            "eu-west-3" => AWSRegion::EuWest3,
            "me-south-1" => AWSRegion::MeSouth1,
            "sa-east-1" => AWSRegion::SaEast1,
            "us-east-1" => AWSRegion::UsEast1,
            "us-east-2" => AWSRegion::UsEast2,
            "us-west-1" => AWSRegion::UsWest1,
            "us-west-2" => AWSRegion::UsWest2,
            _ => AWSRegion::UnknownVariant(UnknownAWSRegion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AWSRegion {
    fn from(name: String) -> Self {
        match &*name {
            "af-south-1" => AWSRegion::AfSouth1,
            "ap-east-1" => AWSRegion::ApEast1,
            "ap-northeast-1" => AWSRegion::ApNortheast1,
            "ap-northeast-2" => AWSRegion::ApNortheast2,
            "ap-northeast-3" => AWSRegion::ApNortheast3,
            "ap-south-1" => AWSRegion::ApSouth1,
            "ap-southeast-1" => AWSRegion::ApSoutheast1,
            "ap-southeast-2" => AWSRegion::ApSoutheast2,
            "ca-central-1" => AWSRegion::CaCentral1,
            "cn-north-1" => AWSRegion::CnNorth1,
            "cn-northwest-1" => AWSRegion::CnNorthwest1,
            "eu-central-1" => AWSRegion::EuCentral1,
            "eu-north-1" => AWSRegion::EuNorth1,
            "eu-south-1" => AWSRegion::EuSouth1,
            "eu-west-1" => AWSRegion::EuWest1,
            "eu-west-2" => AWSRegion::EuWest2,
            "eu-west-3" => AWSRegion::EuWest3,
            "me-south-1" => AWSRegion::MeSouth1,
            "sa-east-1" => AWSRegion::SaEast1,
            "us-east-1" => AWSRegion::UsEast1,
            "us-east-2" => AWSRegion::UsEast2,
            "us-west-1" => AWSRegion::UsWest1,
            "us-west-2" => AWSRegion::UsWest2,
            _ => AWSRegion::UnknownVariant(UnknownAWSRegion { name }),
        }
    }
}

impl ::std::str::FromStr for AWSRegion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AWSRegion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AWSRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The types of manifest that you want AWS to create for this report.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAdditionalArtifact {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AdditionalArtifact {
    Athena,
    Quicksight,
    Redshift,
    #[doc(hidden)]
    UnknownVariant(UnknownAdditionalArtifact),
}

impl Default for AdditionalArtifact {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AdditionalArtifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AdditionalArtifact {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AdditionalArtifact {
    fn into(self) -> String {
        match self {
            AdditionalArtifact::Athena => "ATHENA".to_string(),
            AdditionalArtifact::Quicksight => "QUICKSIGHT".to_string(),
            AdditionalArtifact::Redshift => "REDSHIFT".to_string(),
            AdditionalArtifact::UnknownVariant(UnknownAdditionalArtifact { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a AdditionalArtifact {
    fn into(self) -> &'a str {
        match self {
            AdditionalArtifact::Athena => &"ATHENA",
            AdditionalArtifact::Quicksight => &"QUICKSIGHT",
            AdditionalArtifact::Redshift => &"REDSHIFT",
            AdditionalArtifact::UnknownVariant(UnknownAdditionalArtifact { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for AdditionalArtifact {
    fn from(name: &str) -> Self {
        match name {
            "ATHENA" => AdditionalArtifact::Athena,
            "QUICKSIGHT" => AdditionalArtifact::Quicksight,
            "REDSHIFT" => AdditionalArtifact::Redshift,
            _ => AdditionalArtifact::UnknownVariant(UnknownAdditionalArtifact {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AdditionalArtifact {
    fn from(name: String) -> Self {
        match &*name {
            "ATHENA" => AdditionalArtifact::Athena,
            "QUICKSIGHT" => AdditionalArtifact::Quicksight,
            "REDSHIFT" => AdditionalArtifact::Redshift,
            _ => AdditionalArtifact::UnknownVariant(UnknownAdditionalArtifact { name }),
        }
    }
}

impl ::std::str::FromStr for AdditionalArtifact {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AdditionalArtifact {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AdditionalArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The compression format that AWS uses for the report.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCompressionFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CompressionFormat {
    Gzip,
    Parquet,
    Zip,
    #[doc(hidden)]
    UnknownVariant(UnknownCompressionFormat),
}

impl Default for CompressionFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CompressionFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CompressionFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CompressionFormat {
    fn into(self) -> String {
        match self {
            CompressionFormat::Gzip => "GZIP".to_string(),
            CompressionFormat::Parquet => "Parquet".to_string(),
            CompressionFormat::Zip => "ZIP".to_string(),
            CompressionFormat::UnknownVariant(UnknownCompressionFormat { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a CompressionFormat {
    fn into(self) -> &'a str {
        match self {
            CompressionFormat::Gzip => &"GZIP",
            CompressionFormat::Parquet => &"Parquet",
            CompressionFormat::Zip => &"ZIP",
            CompressionFormat::UnknownVariant(UnknownCompressionFormat { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for CompressionFormat {
    fn from(name: &str) -> Self {
        match name {
            "GZIP" => CompressionFormat::Gzip,
            "Parquet" => CompressionFormat::Parquet,
            "ZIP" => CompressionFormat::Zip,
            _ => CompressionFormat::UnknownVariant(UnknownCompressionFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CompressionFormat {
    fn from(name: String) -> Self {
        match &*name {
            "GZIP" => CompressionFormat::Gzip,
            "Parquet" => CompressionFormat::Parquet,
            "ZIP" => CompressionFormat::Zip,
            _ => CompressionFormat::UnknownVariant(UnknownCompressionFormat { name }),
        }
    }
}

impl ::std::str::FromStr for CompressionFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CompressionFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CompressionFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Deletes the specified report.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReportDefinitionRequest {
    /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
    #[serde(rename = "ReportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename = "ResponseMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message: Option<String>,
}

/// <p>Requests a list of AWS Cost and Usage reports owned by the account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReportDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of AWS Cost and Usage reports owned by the account.</p>
    #[serde(rename = "ReportDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReportDefinitionRequest {
    #[serde(rename = "ReportDefinition")]
    pub report_definition: ReportDefinition,
    #[serde(rename = "ReportName")]
    pub report_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyReportDefinitionResponse {}

/// <p>Creates a Cost and Usage Report.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutReportDefinitionRequest {
    /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed metadata and data file information. </p>
    #[serde(rename = "ReportDefinition")]
    pub report_definition: ReportDefinition,
}

/// <p>If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutReportDefinitionResponse {}

/// <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReportDefinition {
    /// <p>A list of manifests that you want Amazon Web Services to create for this report.</p>
    #[serde(rename = "AdditionalArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_artifacts: Option<Vec<AdditionalArtifact>>,
    /// <p>A list of strings that indicate additional content that Amazon Web Services includes in the report, such as individual resource IDs. </p>
    #[serde(rename = "AdditionalSchemaElements")]
    pub additional_schema_elements: Vec<SchemaElement>,
    #[serde(rename = "Compression")]
    pub compression: CompressionFormat,
    #[serde(rename = "Format")]
    pub format: ReportFormat,
    /// <p>Whether you want Amazon Web Services to update your reports after they have been finalized if Amazon Web Services detects charges related to previous months. These charges can include refunds, credits, or support fees.</p>
    #[serde(rename = "RefreshClosedReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_closed_reports: Option<bool>,
    #[serde(rename = "ReportName")]
    pub report_name: String,
    /// <p>Whether you want Amazon Web Services to overwrite the previous version of each report or to deliver the report in addition to the previous versions.</p>
    #[serde(rename = "ReportVersioning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_versioning: Option<ReportVersioning>,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: String,
    #[serde(rename = "S3Region")]
    pub s3_region: AWSRegion,
    #[serde(rename = "TimeUnit")]
    pub time_unit: TimeUnit,
}

/// <p>The format that AWS saves the report in.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownReportFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ReportFormat {
    Parquet,
    TextORcsv,
    #[doc(hidden)]
    UnknownVariant(UnknownReportFormat),
}

impl Default for ReportFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ReportFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ReportFormat {
    fn into(self) -> String {
        match self {
            ReportFormat::Parquet => "Parquet".to_string(),
            ReportFormat::TextORcsv => "textORcsv".to_string(),
            ReportFormat::UnknownVariant(UnknownReportFormat { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ReportFormat {
    fn into(self) -> &'a str {
        match self {
            ReportFormat::Parquet => &"Parquet",
            ReportFormat::TextORcsv => &"textORcsv",
            ReportFormat::UnknownVariant(UnknownReportFormat { name: original }) => original,
        }
    }
}

impl From<&str> for ReportFormat {
    fn from(name: &str) -> Self {
        match name {
            "Parquet" => ReportFormat::Parquet,
            "textORcsv" => ReportFormat::TextORcsv,
            _ => ReportFormat::UnknownVariant(UnknownReportFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ReportFormat {
    fn from(name: String) -> Self {
        match &*name {
            "Parquet" => ReportFormat::Parquet,
            "textORcsv" => ReportFormat::TextORcsv,
            _ => ReportFormat::UnknownVariant(UnknownReportFormat { name }),
        }
    }
}

impl ::std::str::FromStr for ReportFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ReportFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ReportFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownReportVersioning {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ReportVersioning {
    CreateNewReport,
    OverwriteReport,
    #[doc(hidden)]
    UnknownVariant(UnknownReportVersioning),
}

impl Default for ReportVersioning {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ReportVersioning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ReportVersioning {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ReportVersioning {
    fn into(self) -> String {
        match self {
            ReportVersioning::CreateNewReport => "CREATE_NEW_REPORT".to_string(),
            ReportVersioning::OverwriteReport => "OVERWRITE_REPORT".to_string(),
            ReportVersioning::UnknownVariant(UnknownReportVersioning { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ReportVersioning {
    fn into(self) -> &'a str {
        match self {
            ReportVersioning::CreateNewReport => &"CREATE_NEW_REPORT",
            ReportVersioning::OverwriteReport => &"OVERWRITE_REPORT",
            ReportVersioning::UnknownVariant(UnknownReportVersioning { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ReportVersioning {
    fn from(name: &str) -> Self {
        match name {
            "CREATE_NEW_REPORT" => ReportVersioning::CreateNewReport,
            "OVERWRITE_REPORT" => ReportVersioning::OverwriteReport,
            _ => ReportVersioning::UnknownVariant(UnknownReportVersioning {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ReportVersioning {
    fn from(name: String) -> Self {
        match &*name {
            "CREATE_NEW_REPORT" => ReportVersioning::CreateNewReport,
            "OVERWRITE_REPORT" => ReportVersioning::OverwriteReport,
            _ => ReportVersioning::UnknownVariant(UnknownReportVersioning { name }),
        }
    }
}

impl ::std::str::FromStr for ReportVersioning {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ReportVersioning {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ReportVersioning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Whether or not AWS includes resource IDs in the report. </p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSchemaElement {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SchemaElement {
    Resources,
    #[doc(hidden)]
    UnknownVariant(UnknownSchemaElement),
}

impl Default for SchemaElement {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SchemaElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SchemaElement {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SchemaElement {
    fn into(self) -> String {
        match self {
            SchemaElement::Resources => "RESOURCES".to_string(),
            SchemaElement::UnknownVariant(UnknownSchemaElement { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SchemaElement {
    fn into(self) -> &'a str {
        match self {
            SchemaElement::Resources => &"RESOURCES",
            SchemaElement::UnknownVariant(UnknownSchemaElement { name: original }) => original,
        }
    }
}

impl From<&str> for SchemaElement {
    fn from(name: &str) -> Self {
        match name {
            "RESOURCES" => SchemaElement::Resources,
            _ => SchemaElement::UnknownVariant(UnknownSchemaElement {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SchemaElement {
    fn from(name: String) -> Self {
        match &*name {
            "RESOURCES" => SchemaElement::Resources,
            _ => SchemaElement::UnknownVariant(UnknownSchemaElement { name }),
        }
    }
}

impl ::std::str::FromStr for SchemaElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SchemaElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SchemaElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The length of time covered by the report. </p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTimeUnit {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TimeUnit {
    Daily,
    Hourly,
    Monthly,
    #[doc(hidden)]
    UnknownVariant(UnknownTimeUnit),
}

impl Default for TimeUnit {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TimeUnit {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TimeUnit {
    fn into(self) -> String {
        match self {
            TimeUnit::Daily => "DAILY".to_string(),
            TimeUnit::Hourly => "HOURLY".to_string(),
            TimeUnit::Monthly => "MONTHLY".to_string(),
            TimeUnit::UnknownVariant(UnknownTimeUnit { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TimeUnit {
    fn into(self) -> &'a str {
        match self {
            TimeUnit::Daily => &"DAILY",
            TimeUnit::Hourly => &"HOURLY",
            TimeUnit::Monthly => &"MONTHLY",
            TimeUnit::UnknownVariant(UnknownTimeUnit { name: original }) => original,
        }
    }
}

impl From<&str> for TimeUnit {
    fn from(name: &str) -> Self {
        match name {
            "DAILY" => TimeUnit::Daily,
            "HOURLY" => TimeUnit::Hourly,
            "MONTHLY" => TimeUnit::Monthly,
            _ => TimeUnit::UnknownVariant(UnknownTimeUnit {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TimeUnit {
    fn from(name: String) -> Self {
        match &*name {
            "DAILY" => TimeUnit::Daily,
            "HOURLY" => TimeUnit::Hourly,
            "MONTHLY" => TimeUnit::Monthly,
            _ => TimeUnit::UnknownVariant(UnknownTimeUnit { name }),
        }
    }
}

impl ::std::str::FromStr for TimeUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TimeUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TimeUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// Errors returned by DeleteReportDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteReportDefinitionError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl DeleteReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteReportDefinitionError::InternalError(
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
impl fmt::Display for DeleteReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReportDefinitionError {}
/// Errors returned by DescribeReportDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeReportDefinitionsError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl DescribeReportDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReportDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeReportDefinitionsError::InternalError(
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
impl fmt::Display for DescribeReportDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReportDefinitionsError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReportDefinitionsError {}
/// Errors returned by ModifyReportDefinition
#[derive(Debug, PartialEq)]
pub enum ModifyReportDefinitionError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
}

impl ModifyReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ModifyReportDefinitionError::InternalError(
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
impl fmt::Display for ModifyReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyReportDefinitionError {}
/// Errors returned by PutReportDefinition
#[derive(Debug, PartialEq)]
pub enum PutReportDefinitionError {
    /// <p>A report with the specified name already exists in the account. Specify a different report name.</p>
    DuplicateReportName(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>This account already has five reports defined. To define a new report, you must delete an existing report.</p>
    ReportLimitReached(String),
}

impl PutReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateReportNameException" => {
                    return RusotoError::Service(PutReportDefinitionError::DuplicateReportName(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(PutReportDefinitionError::InternalError(err.msg))
                }
                "ReportLimitReachedException" => {
                    return RusotoError::Service(PutReportDefinitionError::ReportLimitReached(
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
impl fmt::Display for PutReportDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutReportDefinitionError::DuplicateReportName(ref cause) => write!(f, "{}", cause),
            PutReportDefinitionError::InternalError(ref cause) => write!(f, "{}", cause),
            PutReportDefinitionError::ReportLimitReached(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutReportDefinitionError {}
/// Trait representing the capabilities of the AWS Cost and Usage Report Service API. AWS Cost and Usage Report Service clients implement this trait.
#[async_trait]
pub trait CostAndUsageReport {
    /// <p>Deletes the specified report.</p>
    async fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> Result<DeleteReportDefinitionResponse, RusotoError<DeleteReportDefinitionError>>;

    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    async fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> Result<DescribeReportDefinitionsResponse, RusotoError<DescribeReportDefinitionsError>>;

    /// <p>Allows you to programatically update your report preferences.</p>
    async fn modify_report_definition(
        &self,
        input: ModifyReportDefinitionRequest,
    ) -> Result<ModifyReportDefinitionResponse, RusotoError<ModifyReportDefinitionError>>;

    /// <p>Creates a new report using the description that you provide.</p>
    async fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> Result<PutReportDefinitionResponse, RusotoError<PutReportDefinitionError>>;
}
/// A client for the AWS Cost and Usage Report Service API.
#[derive(Clone)]
pub struct CostAndUsageReportClient {
    client: Client,
    region: region::Region,
}

impl CostAndUsageReportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostAndUsageReportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CostAndUsageReportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient { client, region }
    }
}

#[async_trait]
impl CostAndUsageReport for CostAndUsageReportClient {
    /// <p>Deletes the specified report.</p>
    async fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> Result<DeleteReportDefinitionResponse, RusotoError<DeleteReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DeleteReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReportDefinitionResponse, _>()
    }

    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    async fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> Result<DescribeReportDefinitionsResponse, RusotoError<DescribeReportDefinitionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DescribeReportDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReportDefinitionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReportDefinitionsResponse, _>()
    }

    /// <p>Allows you to programatically update your report preferences.</p>
    async fn modify_report_definition(
        &self,
        input: ModifyReportDefinitionRequest,
    ) -> Result<ModifyReportDefinitionResponse, RusotoError<ModifyReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.ModifyReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyReportDefinitionResponse, _>()
    }

    /// <p>Creates a new report using the description that you provide.</p>
    async fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> Result<PutReportDefinitionResponse, RusotoError<PutReportDefinitionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.PutReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutReportDefinitionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutReportDefinitionResponse, _>()
    }
}
