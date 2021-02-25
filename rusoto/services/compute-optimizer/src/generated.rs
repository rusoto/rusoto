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

impl ComputeOptimizerClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "compute-optimizer", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.0".to_owned());

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
/// <p>Describes the configuration of an Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupConfiguration {
    /// <p>The desired capacity, or number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "desiredCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i64>,
    /// <p>The instance type for the Auto Scaling group.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The maximum size, or maximum number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    /// <p>The minimum size, or minimum number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
}

/// <p>Describes an Auto Scaling group recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupRecommendation {
    /// <p>The AWS account ID of the Auto Scaling group.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling group.</p>
    #[serde(rename = "autoScalingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arn: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    #[serde(rename = "autoScalingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    /// <p>An array of objects that describe the current configuration of the Auto Scaling group.</p>
    #[serde(rename = "currentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_configuration: Option<AutoScalingGroupConfiguration>,
    /// <p><p>The finding classification for the Auto Scaling group.</p> <p>Findings for Auto Scaling groups include:</p> <ul> <li> <p> <b> <code>NotOptimized</code> </b>—An Auto Scaling group is considered not optimized when AWS Compute Optimizer identifies a recommendation that can provide better performance for your workload.</p> </li> <li> <p> <b> <code>Optimized</code> </b>—An Auto Scaling group is considered optimized when Compute Optimizer determines that the group is correctly provisioned to run your workload based on the chosen instance type. For optimized resources, Compute Optimizer might recommend a new generation instance type.</p> </li> </ul></p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Finding>,
    /// <p>The time stamp of when the Auto Scaling group recommendation was last refreshed.</p>
    #[serde(rename = "lastRefreshTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_timestamp: Option<f64>,
    /// <p>The number of days for which utilization metrics were analyzed for the Auto Scaling group.</p>
    #[serde(rename = "lookBackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_period_in_days: Option<f64>,
    /// <p>An array of objects that describe the recommendation options for the Auto Scaling group.</p>
    #[serde(rename = "recommendationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_options: Option<Vec<AutoScalingGroupRecommendationOption>>,
    /// <p>An array of objects that describe the utilization metrics of the Auto Scaling group.</p>
    #[serde(rename = "utilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_metrics: Option<Vec<UtilizationMetric>>,
}

/// <p>Describes a recommendation option for an Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupRecommendationOption {
    /// <p>An array of objects that describe an Auto Scaling group configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AutoScalingGroupConfiguration>,
    /// <p>The performance risk of the Auto Scaling group configuration recommendation.</p> <p>Performance risk is the likelihood of the recommended instance type not meeting the performance requirement of your workload.</p> <p>The lowest performance risk is categorized as <code>0</code>, and the highest as <code>5</code>.</p>
    #[serde(rename = "performanceRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_risk: Option<f64>,
    /// <p><p>An array of objects that describe the projected utilization metrics of the Auto Scaling group recommendation option.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    #[serde(rename = "projectedUtilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_utilization_metrics: Option<Vec<UtilizationMetric>>,
    /// <p>The rank of the Auto Scaling group recommendation option.</p> <p>The top recommendation option is ranked as <code>1</code>.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRecommendationExportJobsRequest {
    /// <p>An array of objects that describe a filter to return a more specific list of export jobs.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<JobFilter>>,
    /// <p>The identification numbers of the export jobs to return.</p> <p>An export job ID is returned when you create an export using the <code>ExportAutoScalingGroupRecommendations</code> or <code>ExportEC2InstanceRecommendations</code> actions.</p> <p>All export jobs created in the last seven days are returned if this parameter is omitted.</p>
    #[serde(rename = "jobIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_ids: Option<Vec<String>>,
    /// <p>The maximum number of export jobs to return with a single request.</p> <p>To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of export jobs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRecommendationExportJobsResponse {
    /// <p>The token to use to advance to the next page of export jobs.</p> <p>This value is null when there are no more pages of export jobs to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that describe recommendation export jobs.</p>
    #[serde(rename = "recommendationExportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_export_jobs: Option<Vec<RecommendationExportJob>>,
}

/// <p>Describes a filter that returns a more specific list of Amazon Elastic Block Store (Amazon EBS) volume recommendations.</p> <p>This filter is used with the <code>GetEBSVolumeRecommendations</code> action.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EBSFilter {
    /// <p>The name of the filter.</p> <p>Specify <code>Finding</code> to return recommendations with a specific finding classification (e.g., <code>Optimized</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<EBSFilterName>,
    /// <p>The value of the filter.</p> <p>The valid values are <code>Optimized</code>, or <code>NotOptimized</code>.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEBSFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EBSFilterName {
    Finding,
    #[doc(hidden)]
    UnknownVariant(UnknownEBSFilterName),
}

impl Default for EBSFilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EBSFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EBSFilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EBSFilterName {
    fn into(self) -> String {
        match self {
            EBSFilterName::Finding => "Finding".to_string(),
            EBSFilterName::UnknownVariant(UnknownEBSFilterName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EBSFilterName {
    fn into(self) -> &'a str {
        match self {
            EBSFilterName::Finding => &"Finding",
            EBSFilterName::UnknownVariant(UnknownEBSFilterName { name: original }) => original,
        }
    }
}

impl From<&str> for EBSFilterName {
    fn from(name: &str) -> Self {
        match name {
            "Finding" => EBSFilterName::Finding,
            _ => EBSFilterName::UnknownVariant(UnknownEBSFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EBSFilterName {
    fn from(name: String) -> Self {
        match &*name {
            "Finding" => EBSFilterName::Finding,
            _ => EBSFilterName::UnknownVariant(UnknownEBSFilterName { name }),
        }
    }
}

impl ::std::str::FromStr for EBSFilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for EBSFilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for EBSFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEBSFinding {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EBSFinding {
    NotOptimized,
    Optimized,
    #[doc(hidden)]
    UnknownVariant(UnknownEBSFinding),
}

impl Default for EBSFinding {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EBSFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EBSFinding {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EBSFinding {
    fn into(self) -> String {
        match self {
            EBSFinding::NotOptimized => "NotOptimized".to_string(),
            EBSFinding::Optimized => "Optimized".to_string(),
            EBSFinding::UnknownVariant(UnknownEBSFinding { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EBSFinding {
    fn into(self) -> &'a str {
        match self {
            EBSFinding::NotOptimized => &"NotOptimized",
            EBSFinding::Optimized => &"Optimized",
            EBSFinding::UnknownVariant(UnknownEBSFinding { name: original }) => original,
        }
    }
}

impl From<&str> for EBSFinding {
    fn from(name: &str) -> Self {
        match name {
            "NotOptimized" => EBSFinding::NotOptimized,
            "Optimized" => EBSFinding::Optimized,
            _ => EBSFinding::UnknownVariant(UnknownEBSFinding {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EBSFinding {
    fn from(name: String) -> Self {
        match &*name {
            "NotOptimized" => EBSFinding::NotOptimized,
            "Optimized" => EBSFinding::Optimized,
            _ => EBSFinding::UnknownVariant(UnknownEBSFinding { name }),
        }
    }
}

impl ::std::str::FromStr for EBSFinding {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EBSFinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EBSFinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEBSMetricName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EBSMetricName {
    VolumeReadBytesPerSecond,
    VolumeReadOpsPerSecond,
    VolumeWriteBytesPerSecond,
    VolumeWriteOpsPerSecond,
    #[doc(hidden)]
    UnknownVariant(UnknownEBSMetricName),
}

impl Default for EBSMetricName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EBSMetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EBSMetricName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EBSMetricName {
    fn into(self) -> String {
        match self {
            EBSMetricName::VolumeReadBytesPerSecond => "VolumeReadBytesPerSecond".to_string(),
            EBSMetricName::VolumeReadOpsPerSecond => "VolumeReadOpsPerSecond".to_string(),
            EBSMetricName::VolumeWriteBytesPerSecond => "VolumeWriteBytesPerSecond".to_string(),
            EBSMetricName::VolumeWriteOpsPerSecond => "VolumeWriteOpsPerSecond".to_string(),
            EBSMetricName::UnknownVariant(UnknownEBSMetricName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EBSMetricName {
    fn into(self) -> &'a str {
        match self {
            EBSMetricName::VolumeReadBytesPerSecond => &"VolumeReadBytesPerSecond",
            EBSMetricName::VolumeReadOpsPerSecond => &"VolumeReadOpsPerSecond",
            EBSMetricName::VolumeWriteBytesPerSecond => &"VolumeWriteBytesPerSecond",
            EBSMetricName::VolumeWriteOpsPerSecond => &"VolumeWriteOpsPerSecond",
            EBSMetricName::UnknownVariant(UnknownEBSMetricName { name: original }) => original,
        }
    }
}

impl From<&str> for EBSMetricName {
    fn from(name: &str) -> Self {
        match name {
            "VolumeReadBytesPerSecond" => EBSMetricName::VolumeReadBytesPerSecond,
            "VolumeReadOpsPerSecond" => EBSMetricName::VolumeReadOpsPerSecond,
            "VolumeWriteBytesPerSecond" => EBSMetricName::VolumeWriteBytesPerSecond,
            "VolumeWriteOpsPerSecond" => EBSMetricName::VolumeWriteOpsPerSecond,
            _ => EBSMetricName::UnknownVariant(UnknownEBSMetricName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EBSMetricName {
    fn from(name: String) -> Self {
        match &*name {
            "VolumeReadBytesPerSecond" => EBSMetricName::VolumeReadBytesPerSecond,
            "VolumeReadOpsPerSecond" => EBSMetricName::VolumeReadOpsPerSecond,
            "VolumeWriteBytesPerSecond" => EBSMetricName::VolumeWriteBytesPerSecond,
            "VolumeWriteOpsPerSecond" => EBSMetricName::VolumeWriteOpsPerSecond,
            _ => EBSMetricName::UnknownVariant(UnknownEBSMetricName { name }),
        }
    }
}

impl ::std::str::FromStr for EBSMetricName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EBSMetricName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EBSMetricName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a utilization metric of an Amazon Elastic Block Store (Amazon EBS) volume.</p> <p>Compare the utilization metric data of your resource against its projected utilization metric data to determine the performance difference between your current resource and the recommended option.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EBSUtilizationMetric {
    /// <p><p>The name of the utilization metric.</p> <p>The following utilization metrics are available:</p> <ul> <li> <p> <code>VolumeReadOpsPerSecond</code> - The completed read operations per second from the volume in a specified period of time.</p> <p>Unit: Count</p> </li> <li> <p> <code>VolumeWriteOpsPerSecond</code> - The completed write operations per second to the volume in a specified period of time.</p> <p>Unit: Count</p> </li> <li> <p> <code>VolumeReadBytesPerSecond</code> - The bytes read per second from the volume in a specified period of time.</p> <p>Unit: Bytes</p> </li> <li> <p> <code>VolumeWriteBytesPerSecond</code> - The bytes written to the volume in a specified period of time.</p> <p>Unit: Bytes</p> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<EBSMetricName>,
    /// <p><p>The statistic of the utilization metric.</p> <p>The following statistics are available:</p> <ul> <li> <p> <code>Average</code> - This is the value of Sum / SampleCount during the specified period, or the average value observed during the specified period.</p> </li> <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li> </ul></p>
    #[serde(rename = "statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<MetricStatistic>,
    /// <p>The value of the utilization metric.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportAutoScalingGroupRecommendationsRequest {
    /// <p>The IDs of the AWS accounts for which to export Auto Scaling group recommendations.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to export recommendations.</p> <p>This parameter cannot be specified together with the include member accounts parameter. The parameters are mutually exclusive.</p> <p>Recommendations for member accounts are not included in the export if this parameter, or the include member accounts parameter, is omitted.</p> <p>You can specify multiple account IDs per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The recommendations data to include in the export file. For more information about the fields that can be exported, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html#exported-files">Exported files</a> in the <i>Compute Optimizer User Guide</i>.</p>
    #[serde(rename = "fieldsToExport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_export: Option<Vec<ExportableAutoScalingGroupField>>,
    /// <p>The format of the export file.</p> <p>The only export file format currently supported is <code>Csv</code>.</p>
    #[serde(rename = "fileFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FileFormat>,
    /// <p>An array of objects that describe a filter to export a more specific set of Auto Scaling group recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Indicates whether to include recommendations for resources in all member accounts of the organization if your account is the management account of an organization.</p> <p>The member accounts must also be opted in to Compute Optimizer.</p> <p>Recommendations for member accounts of the organization are not included in the export file if this parameter is omitted.</p> <p>This parameter cannot be specified together with the account IDs parameter. The parameters are mutually exclusive.</p> <p>Recommendations for member accounts are not included in the export if this parameter, or the account IDs parameter, is omitted.</p>
    #[serde(rename = "includeMemberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_member_accounts: Option<bool>,
    /// <p>An object to specify the destination Amazon Simple Storage Service (Amazon S3) bucket name and key prefix for the export job.</p> <p>You must create the destination Amazon S3 bucket for your recommendations export before you create the export job. Compute Optimizer does not create the S3 bucket for you. After you create the S3 bucket, ensure that it has the required permission policy to allow Compute Optimizer to write the export file to it. If you plan to specify an object prefix when you create the export job, you must include the object prefix in the policy that you add to the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/create-s3-bucket-policy-for-compute-optimizer.html">Amazon S3 Bucket Policy for Compute Optimizer</a> in the <i>Compute Optimizer user guide</i>.</p>
    #[serde(rename = "s3DestinationConfig")]
    pub s_3_destination_config: S3DestinationConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportAutoScalingGroupRecommendationsResponse {
    /// <p>The identification number of the export job.</p> <p>Use the <code>DescribeRecommendationExportJobs</code> action, and specify the job ID to view the status of an export job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>An object that describes the destination Amazon S3 bucket of a recommendations export file.</p>
    #[serde(rename = "s3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_destination: Option<S3Destination>,
}

/// <p>Describes the destination of the recommendations export and metadata files.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportDestination {
    /// <p>An object that describes the destination Amazon Simple Storage Service (Amazon S3) bucket name and object keys of a recommendations export file, and its associated metadata file.</p>
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3: Option<S3Destination>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportEC2InstanceRecommendationsRequest {
    /// <p>The IDs of the AWS accounts for which to export instance recommendations.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to export recommendations.</p> <p>This parameter cannot be specified together with the include member accounts parameter. The parameters are mutually exclusive.</p> <p>Recommendations for member accounts are not included in the export if this parameter, or the include member accounts parameter, is omitted.</p> <p>You can specify multiple account IDs per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The recommendations data to include in the export file. For more information about the fields that can be exported, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html#exported-files">Exported files</a> in the <i>Compute Optimizer User Guide</i>.</p>
    #[serde(rename = "fieldsToExport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_export: Option<Vec<ExportableInstanceField>>,
    /// <p>The format of the export file.</p> <p>The only export file format currently supported is <code>Csv</code>.</p>
    #[serde(rename = "fileFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<FileFormat>,
    /// <p>An array of objects that describe a filter to export a more specific set of instance recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Indicates whether to include recommendations for resources in all member accounts of the organization if your account is the management account of an organization.</p> <p>The member accounts must also be opted in to Compute Optimizer.</p> <p>Recommendations for member accounts of the organization are not included in the export file if this parameter is omitted.</p> <p>Recommendations for member accounts are not included in the export if this parameter, or the account IDs parameter, is omitted.</p>
    #[serde(rename = "includeMemberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_member_accounts: Option<bool>,
    /// <p>An object to specify the destination Amazon Simple Storage Service (Amazon S3) bucket name and key prefix for the export job.</p> <p>You must create the destination Amazon S3 bucket for your recommendations export before you create the export job. Compute Optimizer does not create the S3 bucket for you. After you create the S3 bucket, ensure that it has the required permission policy to allow Compute Optimizer to write the export file to it. If you plan to specify an object prefix when you create the export job, you must include the object prefix in the policy that you add to the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/create-s3-bucket-policy-for-compute-optimizer.html">Amazon S3 Bucket Policy for Compute Optimizer</a> in the <i>Compute Optimizer user guide</i>.</p>
    #[serde(rename = "s3DestinationConfig")]
    pub s_3_destination_config: S3DestinationConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportEC2InstanceRecommendationsResponse {
    /// <p>The identification number of the export job.</p> <p>Use the <code>DescribeRecommendationExportJobs</code> action, and specify the job ID to view the status of an export job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>An object that describes the destination Amazon S3 bucket of a recommendations export file.</p>
    #[serde(rename = "s3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_destination: Option<S3Destination>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownExportableAutoScalingGroupField {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ExportableAutoScalingGroupField {
    AccountId,
    AutoScalingGroupArn,
    AutoScalingGroupName,
    CurrentConfigurationDesiredCapacity,
    CurrentConfigurationInstanceType,
    CurrentConfigurationMaxSize,
    CurrentConfigurationMinSize,
    CurrentMemory,
    CurrentNetwork,
    CurrentOnDemandPrice,
    CurrentStandardOneYearNoUpfrontReservedPrice,
    CurrentStandardThreeYearNoUpfrontReservedPrice,
    CurrentStorage,
    CurrentVCpus,
    Finding,
    LastRefreshTimestamp,
    LookbackPeriodInDays,
    RecommendationOptionsConfigurationDesiredCapacity,
    RecommendationOptionsConfigurationInstanceType,
    RecommendationOptionsConfigurationMaxSize,
    RecommendationOptionsConfigurationMinSize,
    RecommendationOptionsMemory,
    RecommendationOptionsNetwork,
    RecommendationOptionsOnDemandPrice,
    RecommendationOptionsPerformanceRisk,
    RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
    RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
    RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
    RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
    RecommendationOptionsStorage,
    RecommendationOptionsVcpus,
    UtilizationMetricsCpuMaximum,
    UtilizationMetricsEbsReadBytesPerSecondMaximum,
    UtilizationMetricsEbsReadOpsPerSecondMaximum,
    UtilizationMetricsEbsWriteBytesPerSecondMaximum,
    UtilizationMetricsEbsWriteOpsPerSecondMaximum,
    UtilizationMetricsMemoryMaximum,
    #[doc(hidden)]
    UnknownVariant(UnknownExportableAutoScalingGroupField),
}

impl Default for ExportableAutoScalingGroupField {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ExportableAutoScalingGroupField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ExportableAutoScalingGroupField {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ExportableAutoScalingGroupField {
    fn into(self) -> String {
        match self {
                    ExportableAutoScalingGroupField::AccountId => "AccountId".to_string(),
ExportableAutoScalingGroupField::AutoScalingGroupArn => "AutoScalingGroupArn".to_string(),
ExportableAutoScalingGroupField::AutoScalingGroupName => "AutoScalingGroupName".to_string(),
ExportableAutoScalingGroupField::CurrentConfigurationDesiredCapacity => "CurrentConfigurationDesiredCapacity".to_string(),
ExportableAutoScalingGroupField::CurrentConfigurationInstanceType => "CurrentConfigurationInstanceType".to_string(),
ExportableAutoScalingGroupField::CurrentConfigurationMaxSize => "CurrentConfigurationMaxSize".to_string(),
ExportableAutoScalingGroupField::CurrentConfigurationMinSize => "CurrentConfigurationMinSize".to_string(),
ExportableAutoScalingGroupField::CurrentMemory => "CurrentMemory".to_string(),
ExportableAutoScalingGroupField::CurrentNetwork => "CurrentNetwork".to_string(),
ExportableAutoScalingGroupField::CurrentOnDemandPrice => "CurrentOnDemandPrice".to_string(),
ExportableAutoScalingGroupField::CurrentStandardOneYearNoUpfrontReservedPrice => "CurrentStandardOneYearNoUpfrontReservedPrice".to_string(),
ExportableAutoScalingGroupField::CurrentStandardThreeYearNoUpfrontReservedPrice => "CurrentStandardThreeYearNoUpfrontReservedPrice".to_string(),
ExportableAutoScalingGroupField::CurrentStorage => "CurrentStorage".to_string(),
ExportableAutoScalingGroupField::CurrentVCpus => "CurrentVCpus".to_string(),
ExportableAutoScalingGroupField::Finding => "Finding".to_string(),
ExportableAutoScalingGroupField::LastRefreshTimestamp => "LastRefreshTimestamp".to_string(),
ExportableAutoScalingGroupField::LookbackPeriodInDays => "LookbackPeriodInDays".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationDesiredCapacity => "RecommendationOptionsConfigurationDesiredCapacity".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationInstanceType => "RecommendationOptionsConfigurationInstanceType".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMaxSize => "RecommendationOptionsConfigurationMaxSize".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMinSize => "RecommendationOptionsConfigurationMinSize".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsMemory => "RecommendationOptionsMemory".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsNetwork => "RecommendationOptionsNetwork".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsOnDemandPrice => "RecommendationOptionsOnDemandPrice".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsPerformanceRisk => "RecommendationOptionsPerformanceRisk".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => "RecommendationOptionsProjectedUtilizationMetricsCpuMaximum".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => "RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => "RecommendationOptionsStandardOneYearNoUpfrontReservedPrice".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => "RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsStorage => "RecommendationOptionsStorage".to_string(),
ExportableAutoScalingGroupField::RecommendationOptionsVcpus => "RecommendationOptionsVcpus".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsCpuMaximum => "UtilizationMetricsCpuMaximum".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsEbsReadBytesPerSecondMaximum => "UtilizationMetricsEbsReadBytesPerSecondMaximum".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsEbsReadOpsPerSecondMaximum => "UtilizationMetricsEbsReadOpsPerSecondMaximum".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => "UtilizationMetricsEbsWriteBytesPerSecondMaximum".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => "UtilizationMetricsEbsWriteOpsPerSecondMaximum".to_string(),
ExportableAutoScalingGroupField::UtilizationMetricsMemoryMaximum => "UtilizationMetricsMemoryMaximum".to_string(),
                    ExportableAutoScalingGroupField::UnknownVariant(UnknownExportableAutoScalingGroupField{name: original}) => original
                }
    }
}

impl<'a> Into<&'a str> for &'a ExportableAutoScalingGroupField {
    fn into(self) -> &'a str {
        match self {
                    ExportableAutoScalingGroupField::AccountId => &"AccountId",
ExportableAutoScalingGroupField::AutoScalingGroupArn => &"AutoScalingGroupArn",
ExportableAutoScalingGroupField::AutoScalingGroupName => &"AutoScalingGroupName",
ExportableAutoScalingGroupField::CurrentConfigurationDesiredCapacity => &"CurrentConfigurationDesiredCapacity",
ExportableAutoScalingGroupField::CurrentConfigurationInstanceType => &"CurrentConfigurationInstanceType",
ExportableAutoScalingGroupField::CurrentConfigurationMaxSize => &"CurrentConfigurationMaxSize",
ExportableAutoScalingGroupField::CurrentConfigurationMinSize => &"CurrentConfigurationMinSize",
ExportableAutoScalingGroupField::CurrentMemory => &"CurrentMemory",
ExportableAutoScalingGroupField::CurrentNetwork => &"CurrentNetwork",
ExportableAutoScalingGroupField::CurrentOnDemandPrice => &"CurrentOnDemandPrice",
ExportableAutoScalingGroupField::CurrentStandardOneYearNoUpfrontReservedPrice => &"CurrentStandardOneYearNoUpfrontReservedPrice",
ExportableAutoScalingGroupField::CurrentStandardThreeYearNoUpfrontReservedPrice => &"CurrentStandardThreeYearNoUpfrontReservedPrice",
ExportableAutoScalingGroupField::CurrentStorage => &"CurrentStorage",
ExportableAutoScalingGroupField::CurrentVCpus => &"CurrentVCpus",
ExportableAutoScalingGroupField::Finding => &"Finding",
ExportableAutoScalingGroupField::LastRefreshTimestamp => &"LastRefreshTimestamp",
ExportableAutoScalingGroupField::LookbackPeriodInDays => &"LookbackPeriodInDays",
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationDesiredCapacity => &"RecommendationOptionsConfigurationDesiredCapacity",
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationInstanceType => &"RecommendationOptionsConfigurationInstanceType",
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMaxSize => &"RecommendationOptionsConfigurationMaxSize",
ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMinSize => &"RecommendationOptionsConfigurationMinSize",
ExportableAutoScalingGroupField::RecommendationOptionsMemory => &"RecommendationOptionsMemory",
ExportableAutoScalingGroupField::RecommendationOptionsNetwork => &"RecommendationOptionsNetwork",
ExportableAutoScalingGroupField::RecommendationOptionsOnDemandPrice => &"RecommendationOptionsOnDemandPrice",
ExportableAutoScalingGroupField::RecommendationOptionsPerformanceRisk => &"RecommendationOptionsPerformanceRisk",
ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => &"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum",
ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => &"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum",
ExportableAutoScalingGroupField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => &"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice",
ExportableAutoScalingGroupField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => &"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice",
ExportableAutoScalingGroupField::RecommendationOptionsStorage => &"RecommendationOptionsStorage",
ExportableAutoScalingGroupField::RecommendationOptionsVcpus => &"RecommendationOptionsVcpus",
ExportableAutoScalingGroupField::UtilizationMetricsCpuMaximum => &"UtilizationMetricsCpuMaximum",
ExportableAutoScalingGroupField::UtilizationMetricsEbsReadBytesPerSecondMaximum => &"UtilizationMetricsEbsReadBytesPerSecondMaximum",
ExportableAutoScalingGroupField::UtilizationMetricsEbsReadOpsPerSecondMaximum => &"UtilizationMetricsEbsReadOpsPerSecondMaximum",
ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => &"UtilizationMetricsEbsWriteBytesPerSecondMaximum",
ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => &"UtilizationMetricsEbsWriteOpsPerSecondMaximum",
ExportableAutoScalingGroupField::UtilizationMetricsMemoryMaximum => &"UtilizationMetricsMemoryMaximum",
                    ExportableAutoScalingGroupField::UnknownVariant(UnknownExportableAutoScalingGroupField{name: original}) => original
                }
    }
}

impl From<&str> for ExportableAutoScalingGroupField {
    fn from(name: &str) -> Self {
        match name {
                    "AccountId" => ExportableAutoScalingGroupField::AccountId,
"AutoScalingGroupArn" => ExportableAutoScalingGroupField::AutoScalingGroupArn,
"AutoScalingGroupName" => ExportableAutoScalingGroupField::AutoScalingGroupName,
"CurrentConfigurationDesiredCapacity" => ExportableAutoScalingGroupField::CurrentConfigurationDesiredCapacity,
"CurrentConfigurationInstanceType" => ExportableAutoScalingGroupField::CurrentConfigurationInstanceType,
"CurrentConfigurationMaxSize" => ExportableAutoScalingGroupField::CurrentConfigurationMaxSize,
"CurrentConfigurationMinSize" => ExportableAutoScalingGroupField::CurrentConfigurationMinSize,
"CurrentMemory" => ExportableAutoScalingGroupField::CurrentMemory,
"CurrentNetwork" => ExportableAutoScalingGroupField::CurrentNetwork,
"CurrentOnDemandPrice" => ExportableAutoScalingGroupField::CurrentOnDemandPrice,
"CurrentStandardOneYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::CurrentStandardOneYearNoUpfrontReservedPrice,
"CurrentStandardThreeYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::CurrentStandardThreeYearNoUpfrontReservedPrice,
"CurrentStorage" => ExportableAutoScalingGroupField::CurrentStorage,
"CurrentVCpus" => ExportableAutoScalingGroupField::CurrentVCpus,
"Finding" => ExportableAutoScalingGroupField::Finding,
"LastRefreshTimestamp" => ExportableAutoScalingGroupField::LastRefreshTimestamp,
"LookbackPeriodInDays" => ExportableAutoScalingGroupField::LookbackPeriodInDays,
"RecommendationOptionsConfigurationDesiredCapacity" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationDesiredCapacity,
"RecommendationOptionsConfigurationInstanceType" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationInstanceType,
"RecommendationOptionsConfigurationMaxSize" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMaxSize,
"RecommendationOptionsConfigurationMinSize" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMinSize,
"RecommendationOptionsMemory" => ExportableAutoScalingGroupField::RecommendationOptionsMemory,
"RecommendationOptionsNetwork" => ExportableAutoScalingGroupField::RecommendationOptionsNetwork,
"RecommendationOptionsOnDemandPrice" => ExportableAutoScalingGroupField::RecommendationOptionsOnDemandPrice,
"RecommendationOptionsPerformanceRisk" => ExportableAutoScalingGroupField::RecommendationOptionsPerformanceRisk,
"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum" => ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum" => ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
"RecommendationOptionsStorage" => ExportableAutoScalingGroupField::RecommendationOptionsStorage,
"RecommendationOptionsVcpus" => ExportableAutoScalingGroupField::RecommendationOptionsVcpus,
"UtilizationMetricsCpuMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsCpuMaximum,
"UtilizationMetricsEbsReadBytesPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsReadBytesPerSecondMaximum,
"UtilizationMetricsEbsReadOpsPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsReadOpsPerSecondMaximum,
"UtilizationMetricsEbsWriteBytesPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteBytesPerSecondMaximum,
"UtilizationMetricsEbsWriteOpsPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteOpsPerSecondMaximum,
"UtilizationMetricsMemoryMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsMemoryMaximum,
                    _ => ExportableAutoScalingGroupField::UnknownVariant(UnknownExportableAutoScalingGroupField{name: name.to_owned()})
                }
    }
}

impl From<String> for ExportableAutoScalingGroupField {
    fn from(name: String) -> Self {
        match &*name {
                    "AccountId" => ExportableAutoScalingGroupField::AccountId,
"AutoScalingGroupArn" => ExportableAutoScalingGroupField::AutoScalingGroupArn,
"AutoScalingGroupName" => ExportableAutoScalingGroupField::AutoScalingGroupName,
"CurrentConfigurationDesiredCapacity" => ExportableAutoScalingGroupField::CurrentConfigurationDesiredCapacity,
"CurrentConfigurationInstanceType" => ExportableAutoScalingGroupField::CurrentConfigurationInstanceType,
"CurrentConfigurationMaxSize" => ExportableAutoScalingGroupField::CurrentConfigurationMaxSize,
"CurrentConfigurationMinSize" => ExportableAutoScalingGroupField::CurrentConfigurationMinSize,
"CurrentMemory" => ExportableAutoScalingGroupField::CurrentMemory,
"CurrentNetwork" => ExportableAutoScalingGroupField::CurrentNetwork,
"CurrentOnDemandPrice" => ExportableAutoScalingGroupField::CurrentOnDemandPrice,
"CurrentStandardOneYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::CurrentStandardOneYearNoUpfrontReservedPrice,
"CurrentStandardThreeYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::CurrentStandardThreeYearNoUpfrontReservedPrice,
"CurrentStorage" => ExportableAutoScalingGroupField::CurrentStorage,
"CurrentVCpus" => ExportableAutoScalingGroupField::CurrentVCpus,
"Finding" => ExportableAutoScalingGroupField::Finding,
"LastRefreshTimestamp" => ExportableAutoScalingGroupField::LastRefreshTimestamp,
"LookbackPeriodInDays" => ExportableAutoScalingGroupField::LookbackPeriodInDays,
"RecommendationOptionsConfigurationDesiredCapacity" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationDesiredCapacity,
"RecommendationOptionsConfigurationInstanceType" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationInstanceType,
"RecommendationOptionsConfigurationMaxSize" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMaxSize,
"RecommendationOptionsConfigurationMinSize" => ExportableAutoScalingGroupField::RecommendationOptionsConfigurationMinSize,
"RecommendationOptionsMemory" => ExportableAutoScalingGroupField::RecommendationOptionsMemory,
"RecommendationOptionsNetwork" => ExportableAutoScalingGroupField::RecommendationOptionsNetwork,
"RecommendationOptionsOnDemandPrice" => ExportableAutoScalingGroupField::RecommendationOptionsOnDemandPrice,
"RecommendationOptionsPerformanceRisk" => ExportableAutoScalingGroupField::RecommendationOptionsPerformanceRisk,
"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum" => ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum" => ExportableAutoScalingGroupField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice" => ExportableAutoScalingGroupField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
"RecommendationOptionsStorage" => ExportableAutoScalingGroupField::RecommendationOptionsStorage,
"RecommendationOptionsVcpus" => ExportableAutoScalingGroupField::RecommendationOptionsVcpus,
"UtilizationMetricsCpuMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsCpuMaximum,
"UtilizationMetricsEbsReadBytesPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsReadBytesPerSecondMaximum,
"UtilizationMetricsEbsReadOpsPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsReadOpsPerSecondMaximum,
"UtilizationMetricsEbsWriteBytesPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteBytesPerSecondMaximum,
"UtilizationMetricsEbsWriteOpsPerSecondMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsEbsWriteOpsPerSecondMaximum,
"UtilizationMetricsMemoryMaximum" => ExportableAutoScalingGroupField::UtilizationMetricsMemoryMaximum,
                    _ => ExportableAutoScalingGroupField::UnknownVariant(UnknownExportableAutoScalingGroupField{name})
                }
    }
}

impl ::std::str::FromStr for ExportableAutoScalingGroupField {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ExportableAutoScalingGroupField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ExportableAutoScalingGroupField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownExportableInstanceField {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ExportableInstanceField {
    AccountId,
    CurrentInstanceType,
    CurrentMemory,
    CurrentNetwork,
    CurrentOnDemandPrice,
    CurrentStandardOneYearNoUpfrontReservedPrice,
    CurrentStandardThreeYearNoUpfrontReservedPrice,
    CurrentStorage,
    CurrentVCpus,
    Finding,
    InstanceArn,
    InstanceName,
    LastRefreshTimestamp,
    LookbackPeriodInDays,
    RecommendationOptionsInstanceType,
    RecommendationOptionsMemory,
    RecommendationOptionsNetwork,
    RecommendationOptionsOnDemandPrice,
    RecommendationOptionsPerformanceRisk,
    RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
    RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
    RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
    RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
    RecommendationOptionsStorage,
    RecommendationOptionsVcpus,
    RecommendationsSourcesRecommendationSourceArn,
    RecommendationsSourcesRecommendationSourceType,
    UtilizationMetricsCpuMaximum,
    UtilizationMetricsEbsReadBytesPerSecondMaximum,
    UtilizationMetricsEbsReadOpsPerSecondMaximum,
    UtilizationMetricsEbsWriteBytesPerSecondMaximum,
    UtilizationMetricsEbsWriteOpsPerSecondMaximum,
    UtilizationMetricsMemoryMaximum,
    #[doc(hidden)]
    UnknownVariant(UnknownExportableInstanceField),
}

impl Default for ExportableInstanceField {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ExportableInstanceField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ExportableInstanceField {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ExportableInstanceField {
    fn into(self) -> String {
        match self {
                    ExportableInstanceField::AccountId => "AccountId".to_string(),
ExportableInstanceField::CurrentInstanceType => "CurrentInstanceType".to_string(),
ExportableInstanceField::CurrentMemory => "CurrentMemory".to_string(),
ExportableInstanceField::CurrentNetwork => "CurrentNetwork".to_string(),
ExportableInstanceField::CurrentOnDemandPrice => "CurrentOnDemandPrice".to_string(),
ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice => "CurrentStandardOneYearNoUpfrontReservedPrice".to_string(),
ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice => "CurrentStandardThreeYearNoUpfrontReservedPrice".to_string(),
ExportableInstanceField::CurrentStorage => "CurrentStorage".to_string(),
ExportableInstanceField::CurrentVCpus => "CurrentVCpus".to_string(),
ExportableInstanceField::Finding => "Finding".to_string(),
ExportableInstanceField::InstanceArn => "InstanceArn".to_string(),
ExportableInstanceField::InstanceName => "InstanceName".to_string(),
ExportableInstanceField::LastRefreshTimestamp => "LastRefreshTimestamp".to_string(),
ExportableInstanceField::LookbackPeriodInDays => "LookbackPeriodInDays".to_string(),
ExportableInstanceField::RecommendationOptionsInstanceType => "RecommendationOptionsInstanceType".to_string(),
ExportableInstanceField::RecommendationOptionsMemory => "RecommendationOptionsMemory".to_string(),
ExportableInstanceField::RecommendationOptionsNetwork => "RecommendationOptionsNetwork".to_string(),
ExportableInstanceField::RecommendationOptionsOnDemandPrice => "RecommendationOptionsOnDemandPrice".to_string(),
ExportableInstanceField::RecommendationOptionsPerformanceRisk => "RecommendationOptionsPerformanceRisk".to_string(),
ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => "RecommendationOptionsProjectedUtilizationMetricsCpuMaximum".to_string(),
ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => "RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum".to_string(),
ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => "RecommendationOptionsStandardOneYearNoUpfrontReservedPrice".to_string(),
ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => "RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice".to_string(),
ExportableInstanceField::RecommendationOptionsStorage => "RecommendationOptionsStorage".to_string(),
ExportableInstanceField::RecommendationOptionsVcpus => "RecommendationOptionsVcpus".to_string(),
ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn => "RecommendationsSourcesRecommendationSourceArn".to_string(),
ExportableInstanceField::RecommendationsSourcesRecommendationSourceType => "RecommendationsSourcesRecommendationSourceType".to_string(),
ExportableInstanceField::UtilizationMetricsCpuMaximum => "UtilizationMetricsCpuMaximum".to_string(),
ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum => "UtilizationMetricsEbsReadBytesPerSecondMaximum".to_string(),
ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum => "UtilizationMetricsEbsReadOpsPerSecondMaximum".to_string(),
ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => "UtilizationMetricsEbsWriteBytesPerSecondMaximum".to_string(),
ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => "UtilizationMetricsEbsWriteOpsPerSecondMaximum".to_string(),
ExportableInstanceField::UtilizationMetricsMemoryMaximum => "UtilizationMetricsMemoryMaximum".to_string(),
                    ExportableInstanceField::UnknownVariant(UnknownExportableInstanceField{name: original}) => original
                }
    }
}

impl<'a> Into<&'a str> for &'a ExportableInstanceField {
    fn into(self) -> &'a str {
        match self {
                    ExportableInstanceField::AccountId => &"AccountId",
ExportableInstanceField::CurrentInstanceType => &"CurrentInstanceType",
ExportableInstanceField::CurrentMemory => &"CurrentMemory",
ExportableInstanceField::CurrentNetwork => &"CurrentNetwork",
ExportableInstanceField::CurrentOnDemandPrice => &"CurrentOnDemandPrice",
ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice => &"CurrentStandardOneYearNoUpfrontReservedPrice",
ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice => &"CurrentStandardThreeYearNoUpfrontReservedPrice",
ExportableInstanceField::CurrentStorage => &"CurrentStorage",
ExportableInstanceField::CurrentVCpus => &"CurrentVCpus",
ExportableInstanceField::Finding => &"Finding",
ExportableInstanceField::InstanceArn => &"InstanceArn",
ExportableInstanceField::InstanceName => &"InstanceName",
ExportableInstanceField::LastRefreshTimestamp => &"LastRefreshTimestamp",
ExportableInstanceField::LookbackPeriodInDays => &"LookbackPeriodInDays",
ExportableInstanceField::RecommendationOptionsInstanceType => &"RecommendationOptionsInstanceType",
ExportableInstanceField::RecommendationOptionsMemory => &"RecommendationOptionsMemory",
ExportableInstanceField::RecommendationOptionsNetwork => &"RecommendationOptionsNetwork",
ExportableInstanceField::RecommendationOptionsOnDemandPrice => &"RecommendationOptionsOnDemandPrice",
ExportableInstanceField::RecommendationOptionsPerformanceRisk => &"RecommendationOptionsPerformanceRisk",
ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum => &"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum",
ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum => &"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum",
ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice => &"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice",
ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice => &"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice",
ExportableInstanceField::RecommendationOptionsStorage => &"RecommendationOptionsStorage",
ExportableInstanceField::RecommendationOptionsVcpus => &"RecommendationOptionsVcpus",
ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn => &"RecommendationsSourcesRecommendationSourceArn",
ExportableInstanceField::RecommendationsSourcesRecommendationSourceType => &"RecommendationsSourcesRecommendationSourceType",
ExportableInstanceField::UtilizationMetricsCpuMaximum => &"UtilizationMetricsCpuMaximum",
ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum => &"UtilizationMetricsEbsReadBytesPerSecondMaximum",
ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum => &"UtilizationMetricsEbsReadOpsPerSecondMaximum",
ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum => &"UtilizationMetricsEbsWriteBytesPerSecondMaximum",
ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum => &"UtilizationMetricsEbsWriteOpsPerSecondMaximum",
ExportableInstanceField::UtilizationMetricsMemoryMaximum => &"UtilizationMetricsMemoryMaximum",
                    ExportableInstanceField::UnknownVariant(UnknownExportableInstanceField{name: original}) => original
                }
    }
}

impl From<&str> for ExportableInstanceField {
    fn from(name: &str) -> Self {
        match name {
                    "AccountId" => ExportableInstanceField::AccountId,
"CurrentInstanceType" => ExportableInstanceField::CurrentInstanceType,
"CurrentMemory" => ExportableInstanceField::CurrentMemory,
"CurrentNetwork" => ExportableInstanceField::CurrentNetwork,
"CurrentOnDemandPrice" => ExportableInstanceField::CurrentOnDemandPrice,
"CurrentStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice,
"CurrentStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice,
"CurrentStorage" => ExportableInstanceField::CurrentStorage,
"CurrentVCpus" => ExportableInstanceField::CurrentVCpus,
"Finding" => ExportableInstanceField::Finding,
"InstanceArn" => ExportableInstanceField::InstanceArn,
"InstanceName" => ExportableInstanceField::InstanceName,
"LastRefreshTimestamp" => ExportableInstanceField::LastRefreshTimestamp,
"LookbackPeriodInDays" => ExportableInstanceField::LookbackPeriodInDays,
"RecommendationOptionsInstanceType" => ExportableInstanceField::RecommendationOptionsInstanceType,
"RecommendationOptionsMemory" => ExportableInstanceField::RecommendationOptionsMemory,
"RecommendationOptionsNetwork" => ExportableInstanceField::RecommendationOptionsNetwork,
"RecommendationOptionsOnDemandPrice" => ExportableInstanceField::RecommendationOptionsOnDemandPrice,
"RecommendationOptionsPerformanceRisk" => ExportableInstanceField::RecommendationOptionsPerformanceRisk,
"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
"RecommendationOptionsStorage" => ExportableInstanceField::RecommendationOptionsStorage,
"RecommendationOptionsVcpus" => ExportableInstanceField::RecommendationOptionsVcpus,
"RecommendationsSourcesRecommendationSourceArn" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn,
"RecommendationsSourcesRecommendationSourceType" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceType,
"UtilizationMetricsCpuMaximum" => ExportableInstanceField::UtilizationMetricsCpuMaximum,
"UtilizationMetricsEbsReadBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum,
"UtilizationMetricsEbsReadOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum,
"UtilizationMetricsEbsWriteBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum,
"UtilizationMetricsEbsWriteOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum,
"UtilizationMetricsMemoryMaximum" => ExportableInstanceField::UtilizationMetricsMemoryMaximum,
                    _ => ExportableInstanceField::UnknownVariant(UnknownExportableInstanceField{name: name.to_owned()})
                }
    }
}

impl From<String> for ExportableInstanceField {
    fn from(name: String) -> Self {
        match &*name {
                    "AccountId" => ExportableInstanceField::AccountId,
"CurrentInstanceType" => ExportableInstanceField::CurrentInstanceType,
"CurrentMemory" => ExportableInstanceField::CurrentMemory,
"CurrentNetwork" => ExportableInstanceField::CurrentNetwork,
"CurrentOnDemandPrice" => ExportableInstanceField::CurrentOnDemandPrice,
"CurrentStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardOneYearNoUpfrontReservedPrice,
"CurrentStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::CurrentStandardThreeYearNoUpfrontReservedPrice,
"CurrentStorage" => ExportableInstanceField::CurrentStorage,
"CurrentVCpus" => ExportableInstanceField::CurrentVCpus,
"Finding" => ExportableInstanceField::Finding,
"InstanceArn" => ExportableInstanceField::InstanceArn,
"InstanceName" => ExportableInstanceField::InstanceName,
"LastRefreshTimestamp" => ExportableInstanceField::LastRefreshTimestamp,
"LookbackPeriodInDays" => ExportableInstanceField::LookbackPeriodInDays,
"RecommendationOptionsInstanceType" => ExportableInstanceField::RecommendationOptionsInstanceType,
"RecommendationOptionsMemory" => ExportableInstanceField::RecommendationOptionsMemory,
"RecommendationOptionsNetwork" => ExportableInstanceField::RecommendationOptionsNetwork,
"RecommendationOptionsOnDemandPrice" => ExportableInstanceField::RecommendationOptionsOnDemandPrice,
"RecommendationOptionsPerformanceRisk" => ExportableInstanceField::RecommendationOptionsPerformanceRisk,
"RecommendationOptionsProjectedUtilizationMetricsCpuMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsCpuMaximum,
"RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum" => ExportableInstanceField::RecommendationOptionsProjectedUtilizationMetricsMemoryMaximum,
"RecommendationOptionsStandardOneYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardOneYearNoUpfrontReservedPrice,
"RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice" => ExportableInstanceField::RecommendationOptionsStandardThreeYearNoUpfrontReservedPrice,
"RecommendationOptionsStorage" => ExportableInstanceField::RecommendationOptionsStorage,
"RecommendationOptionsVcpus" => ExportableInstanceField::RecommendationOptionsVcpus,
"RecommendationsSourcesRecommendationSourceArn" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceArn,
"RecommendationsSourcesRecommendationSourceType" => ExportableInstanceField::RecommendationsSourcesRecommendationSourceType,
"UtilizationMetricsCpuMaximum" => ExportableInstanceField::UtilizationMetricsCpuMaximum,
"UtilizationMetricsEbsReadBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadBytesPerSecondMaximum,
"UtilizationMetricsEbsReadOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsReadOpsPerSecondMaximum,
"UtilizationMetricsEbsWriteBytesPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteBytesPerSecondMaximum,
"UtilizationMetricsEbsWriteOpsPerSecondMaximum" => ExportableInstanceField::UtilizationMetricsEbsWriteOpsPerSecondMaximum,
"UtilizationMetricsMemoryMaximum" => ExportableInstanceField::UtilizationMetricsMemoryMaximum,
                    _ => ExportableInstanceField::UnknownVariant(UnknownExportableInstanceField{name})
                }
    }
}

impl ::std::str::FromStr for ExportableInstanceField {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ExportableInstanceField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ExportableInstanceField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFileFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FileFormat {
    Csv,
    #[doc(hidden)]
    UnknownVariant(UnknownFileFormat),
}

impl Default for FileFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FileFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FileFormat {
    fn into(self) -> String {
        match self {
            FileFormat::Csv => "Csv".to_string(),
            FileFormat::UnknownVariant(UnknownFileFormat { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FileFormat {
    fn into(self) -> &'a str {
        match self {
            FileFormat::Csv => &"Csv",
            FileFormat::UnknownVariant(UnknownFileFormat { name: original }) => original,
        }
    }
}

impl From<&str> for FileFormat {
    fn from(name: &str) -> Self {
        match name {
            "Csv" => FileFormat::Csv,
            _ => FileFormat::UnknownVariant(UnknownFileFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FileFormat {
    fn from(name: String) -> Self {
        match &*name {
            "Csv" => FileFormat::Csv,
            _ => FileFormat::UnknownVariant(UnknownFileFormat { name }),
        }
    }
}

impl ::std::str::FromStr for FileFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FileFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for FileFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a filter that returns a more specific list of recommendations.</p> <p>This filter is used with the <code>GetAutoScalingGroupRecommendations</code> and <code>GetEC2InstanceRecommendations</code> actions.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter.</p> <p>Specify <code>Finding</code> to return recommendations with a specific finding classification (e.g., <code>Overprovisioned</code>).</p> <p>Specify <code>RecommendationSourceType</code> to return recommendations of a specific resource type (e.g., <code>AutoScalingGroup</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<FilterName>,
    /// <p><p>The value of the filter.</p> <p>The valid values for this parameter are as follows, depending on what you specify for the <code>name</code> parameter and the resource type that you wish to filter results for:</p> <ul> <li> <p>Specify <code>Optimized</code> or <code>NotOptimized</code> if you specified the <code>name</code> parameter as <code>Finding</code> and you want to filter results for Auto Scaling groups.</p> </li> <li> <p>Specify <code>Underprovisioned</code>, <code>Overprovisioned</code>, or <code>Optimized</code> if you specified the <code>name</code> parameter as <code>Finding</code> and you want to filter results for EC2 instances.</p> </li> <li> <p>Specify <code>Ec2Instance</code> or <code>AutoScalingGroup</code> if you specified the <code>name</code> parameter as <code>RecommendationSourceType</code>.</p> </li> </ul></p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FilterName {
    Finding,
    RecommendationSourceType,
    #[doc(hidden)]
    UnknownVariant(UnknownFilterName),
}

impl Default for FilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FilterName {
    fn into(self) -> String {
        match self {
            FilterName::Finding => "Finding".to_string(),
            FilterName::RecommendationSourceType => "RecommendationSourceType".to_string(),
            FilterName::UnknownVariant(UnknownFilterName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FilterName {
    fn into(self) -> &'a str {
        match self {
            FilterName::Finding => &"Finding",
            FilterName::RecommendationSourceType => &"RecommendationSourceType",
            FilterName::UnknownVariant(UnknownFilterName { name: original }) => original,
        }
    }
}

impl From<&str> for FilterName {
    fn from(name: &str) -> Self {
        match name {
            "Finding" => FilterName::Finding,
            "RecommendationSourceType" => FilterName::RecommendationSourceType,
            _ => FilterName::UnknownVariant(UnknownFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FilterName {
    fn from(name: String) -> Self {
        match &*name {
            "Finding" => FilterName::Finding,
            "RecommendationSourceType" => FilterName::RecommendationSourceType,
            _ => FilterName::UnknownVariant(UnknownFilterName { name }),
        }
    }
}

impl ::std::str::FromStr for FilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for FilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFinding {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Finding {
    NotOptimized,
    Optimized,
    Overprovisioned,
    Underprovisioned,
    #[doc(hidden)]
    UnknownVariant(UnknownFinding),
}

impl Default for Finding {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Finding {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Finding {
    fn into(self) -> String {
        match self {
            Finding::NotOptimized => "NotOptimized".to_string(),
            Finding::Optimized => "Optimized".to_string(),
            Finding::Overprovisioned => "Overprovisioned".to_string(),
            Finding::Underprovisioned => "Underprovisioned".to_string(),
            Finding::UnknownVariant(UnknownFinding { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Finding {
    fn into(self) -> &'a str {
        match self {
            Finding::NotOptimized => &"NotOptimized",
            Finding::Optimized => &"Optimized",
            Finding::Overprovisioned => &"Overprovisioned",
            Finding::Underprovisioned => &"Underprovisioned",
            Finding::UnknownVariant(UnknownFinding { name: original }) => original,
        }
    }
}

impl From<&str> for Finding {
    fn from(name: &str) -> Self {
        match name {
            "NotOptimized" => Finding::NotOptimized,
            "Optimized" => Finding::Optimized,
            "Overprovisioned" => Finding::Overprovisioned,
            "Underprovisioned" => Finding::Underprovisioned,
            _ => Finding::UnknownVariant(UnknownFinding {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Finding {
    fn from(name: String) -> Self {
        match &*name {
            "NotOptimized" => Finding::NotOptimized,
            "Optimized" => Finding::Optimized,
            "Overprovisioned" => Finding::Overprovisioned,
            "Underprovisioned" => Finding::Underprovisioned,
            _ => Finding::UnknownVariant(UnknownFinding { name }),
        }
    }
}

impl ::std::str::FromStr for Finding {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for Finding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Finding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAutoScalingGroupRecommendationsRequest {
    /// <p>The IDs of the AWS accounts for which to return Auto Scaling group recommendations.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to return Auto Scaling group recommendations.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling groups for which to return recommendations.</p>
    #[serde(rename = "autoScalingGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arns: Option<Vec<String>>,
    /// <p>An array of objects that describe a filter that returns a more specific list of Auto Scaling group recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of Auto Scaling group recommendations to return with a single request.</p> <p>To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of Auto Scaling group recommendations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAutoScalingGroupRecommendationsResponse {
    /// <p>An array of objects that describe Auto Scaling group recommendations.</p>
    #[serde(rename = "autoScalingGroupRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_recommendations: Option<Vec<AutoScalingGroupRecommendation>>,
    /// <p>An array of objects that describe errors of the request.</p> <p>For example, an error is returned if you request recommendations for an unsupported Auto Scaling group.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetRecommendationError>>,
    /// <p>The token to use to advance to the next page of Auto Scaling group recommendations.</p> <p>This value is null when there are no more pages of Auto Scaling group recommendations to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEBSVolumeRecommendationsRequest {
    /// <p>The IDs of the AWS accounts for which to return volume recommendations.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to return volume recommendations.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>An array of objects that describe a filter that returns a more specific list of volume recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<EBSFilter>>,
    /// <p>The maximum number of volume recommendations to return with a single request.</p> <p>To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of volume recommendations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volumes for which to return recommendations.</p>
    #[serde(rename = "volumeArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEBSVolumeRecommendationsResponse {
    /// <p>An array of objects that describe errors of the request.</p> <p>For example, an error is returned if you request recommendations for an unsupported volume.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetRecommendationError>>,
    /// <p>The token to use to advance to the next page of volume recommendations.</p> <p>This value is null when there are no more pages of volume recommendations to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that describe volume recommendations.</p>
    #[serde(rename = "volumeRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recommendations: Option<Vec<VolumeRecommendation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEC2InstanceRecommendationsRequest {
    /// <p>The IDs of the AWS accounts for which to return instance recommendations.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to return instance recommendations.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>An array of objects that describe a filter that returns a more specific list of instance recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Resource Name (ARN) of the instances for which to return recommendations.</p>
    #[serde(rename = "instanceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arns: Option<Vec<String>>,
    /// <p>The maximum number of instance recommendations to return with a single request.</p> <p>To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of instance recommendations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEC2InstanceRecommendationsResponse {
    /// <p>An array of objects that describe errors of the request.</p> <p>For example, an error is returned if you request recommendations for an instance of an unsupported instance family.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetRecommendationError>>,
    /// <p>An array of objects that describe instance recommendations.</p>
    #[serde(rename = "instanceRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_recommendations: Option<Vec<InstanceRecommendation>>,
    /// <p>The token to use to advance to the next page of instance recommendations.</p> <p>This value is null when there are no more pages of instance recommendations to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEC2RecommendationProjectedMetricsRequest {
    /// <p>The time stamp of the last projected metrics data point to return.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the instances for which to return recommendation projected metrics.</p>
    #[serde(rename = "instanceArn")]
    pub instance_arn: String,
    /// <p>The granularity, in seconds, of the projected metrics data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The time stamp of the first projected metrics data point to return.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The statistic of the projected metrics.</p>
    #[serde(rename = "stat")]
    pub stat: MetricStatistic,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEC2RecommendationProjectedMetricsResponse {
    /// <p>An array of objects that describe a projected metrics.</p>
    #[serde(rename = "recommendedOptionProjectedMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_option_projected_metrics: Option<Vec<RecommendedOptionProjectedMetric>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEnrollmentStatusRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEnrollmentStatusResponse {
    /// <p>Confirms the enrollment status of member accounts within the organization, if the account is a management account of an organization.</p>
    #[serde(rename = "memberAccountsEnrolled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_accounts_enrolled: Option<bool>,
    /// <p>The enrollment status of the account.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// <p>The reason for the enrollment status of the account.</p> <p>For example, an account might show a status of <code>Pending</code> because member accounts of an organization require more time to be enrolled in the service.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Describes an error experienced when getting recommendations.</p> <p>For example, an error is returned if you request recommendations for an unsupported Auto Scaling group, or if you request recommendations for an instance of an unsupported instance family.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationError {
    /// <p>The error code.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The ID of the error.</p>
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>The message, or reason, for the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecommendationSummariesRequest {
    /// <p>The IDs of the AWS accounts for which to return recommendation summaries.</p> <p>If your account is the management account of an organization, use this parameter to specify the member accounts for which you want to return recommendation summaries.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The maximum number of recommendation summaries to return with a single request.</p> <p>To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of recommendation summaries.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationSummariesResponse {
    /// <p>The token to use to advance to the next page of recommendation summaries.</p> <p>This value is null when there are no more pages of recommendation summaries to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that summarize a recommendation.</p>
    #[serde(rename = "recommendationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summaries: Option<Vec<RecommendationSummary>>,
}

/// <p>Describes an Amazon EC2 instance recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceRecommendation {
    /// <p>The AWS account ID of the instance.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The instance type of the current instance.</p>
    #[serde(rename = "currentInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instance_type: Option<String>,
    /// <p><p>The finding classification for the instance.</p> <p>Findings for instances include:</p> <ul> <li> <p> <b> <code>Underprovisioned</code> </b>—An instance is considered under-provisioned when at least one specification of your instance, such as CPU, memory, or network, does not meet the performance requirements of your workload. Under-provisioned instances may lead to poor application performance.</p> </li> <li> <p> <b> <code>Overprovisioned</code> </b>—An instance is considered over-provisioned when at least one specification of your instance, such as CPU, memory, or network, can be sized down while still meeting the performance requirements of your workload, and no specification is under-provisioned. Over-provisioned instances may lead to unnecessary infrastructure cost.</p> </li> <li> <p> <b> <code>Optimized</code> </b>—An instance is considered optimized when all specifications of your instance, such as CPU, memory, and network, meet the performance requirements of your workload and is not over provisioned. An optimized instance runs your workloads with optimal performance and infrastructure cost. For optimized resources, AWS Compute Optimizer might recommend a new generation instance type.</p> </li> </ul></p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Finding>,
    /// <p>The Amazon Resource Name (ARN) of the current instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p>The name of the current instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The time stamp of when the instance recommendation was last refreshed.</p>
    #[serde(rename = "lastRefreshTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_timestamp: Option<f64>,
    /// <p>The number of days for which utilization metrics were analyzed for the instance.</p>
    #[serde(rename = "lookBackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_period_in_days: Option<f64>,
    /// <p>An array of objects that describe the recommendation options for the instance.</p>
    #[serde(rename = "recommendationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_options: Option<Vec<InstanceRecommendationOption>>,
    /// <p>An array of objects that describe the source resource of the recommendation.</p>
    #[serde(rename = "recommendationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_sources: Option<Vec<RecommendationSource>>,
    /// <p>An array of objects that describe the utilization metrics of the instance.</p>
    #[serde(rename = "utilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_metrics: Option<Vec<UtilizationMetric>>,
}

/// <p>Describes a recommendation option for an Amazon EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceRecommendationOption {
    /// <p>The instance type of the instance recommendation.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The performance risk of the instance recommendation option.</p> <p>Performance risk is the likelihood of the recommended instance type not meeting the performance requirement of your workload.</p> <p>The lowest performance risk is categorized as <code>0</code>, and the highest as <code>5</code>.</p>
    #[serde(rename = "performanceRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_risk: Option<f64>,
    /// <p><p>An array of objects that describe the projected utilization metrics of the instance recommendation option.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    #[serde(rename = "projectedUtilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_utilization_metrics: Option<Vec<UtilizationMetric>>,
    /// <p>The rank of the instance recommendation option.</p> <p>The top recommendation option is ranked as <code>1</code>.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>Describes a filter that returns a more specific list of recommendation export jobs.</p> <p>This filter is used with the <code>DescribeRecommendationExportJobs</code> action.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct JobFilter {
    /// <p>The name of the filter.</p> <p>Specify <code>ResourceType</code> to return export jobs of a specific resource type (e.g., <code>Ec2Instance</code>).</p> <p>Specify <code>JobStatus</code> to return export jobs with a specific status (e.g, <code>Complete</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<JobFilterName>,
    /// <p><p>The value of the filter.</p> <p>The valid values for this parameter are as follows, depending on what you specify for the <code>name</code> parameter:</p> <ul> <li> <p>Specify <code>Ec2Instance</code> or <code>AutoScalingGroup</code> if you specified the <code>name</code> parameter as <code>ResourceType</code>. There is no filter for EBS volumes because volume recommendations cannot be exported at this time.</p> </li> <li> <p>Specify <code>Queued</code>, <code>InProgress</code>, <code>Complete</code>, or <code>Failed</code> if you specified the <code>name</code> parameter as <code>JobStatus</code>.</p> </li> </ul></p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobFilterName {
    JobStatus,
    ResourceType,
    #[doc(hidden)]
    UnknownVariant(UnknownJobFilterName),
}

impl Default for JobFilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobFilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobFilterName {
    fn into(self) -> String {
        match self {
            JobFilterName::JobStatus => "JobStatus".to_string(),
            JobFilterName::ResourceType => "ResourceType".to_string(),
            JobFilterName::UnknownVariant(UnknownJobFilterName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JobFilterName {
    fn into(self) -> &'a str {
        match self {
            JobFilterName::JobStatus => &"JobStatus",
            JobFilterName::ResourceType => &"ResourceType",
            JobFilterName::UnknownVariant(UnknownJobFilterName { name: original }) => original,
        }
    }
}

impl From<&str> for JobFilterName {
    fn from(name: &str) -> Self {
        match name {
            "JobStatus" => JobFilterName::JobStatus,
            "ResourceType" => JobFilterName::ResourceType,
            _ => JobFilterName::UnknownVariant(UnknownJobFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobFilterName {
    fn from(name: String) -> Self {
        match &*name {
            "JobStatus" => JobFilterName::JobStatus,
            "ResourceType" => JobFilterName::ResourceType,
            _ => JobFilterName::UnknownVariant(UnknownJobFilterName { name }),
        }
    }
}

impl ::std::str::FromStr for JobFilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for JobFilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for JobFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobStatus {
    Complete,
    Failed,
    InProgress,
    Queued,
    #[doc(hidden)]
    UnknownVariant(UnknownJobStatus),
}

impl Default for JobStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobStatus {
    fn into(self) -> String {
        match self {
            JobStatus::Complete => "Complete".to_string(),
            JobStatus::Failed => "Failed".to_string(),
            JobStatus::InProgress => "InProgress".to_string(),
            JobStatus::Queued => "Queued".to_string(),
            JobStatus::UnknownVariant(UnknownJobStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JobStatus {
    fn into(self) -> &'a str {
        match self {
            JobStatus::Complete => &"Complete",
            JobStatus::Failed => &"Failed",
            JobStatus::InProgress => &"InProgress",
            JobStatus::Queued => &"Queued",
            JobStatus::UnknownVariant(UnknownJobStatus { name: original }) => original,
        }
    }
}

impl From<&str> for JobStatus {
    fn from(name: &str) -> Self {
        match name {
            "Complete" => JobStatus::Complete,
            "Failed" => JobStatus::Failed,
            "InProgress" => JobStatus::InProgress,
            "Queued" => JobStatus::Queued,
            _ => JobStatus::UnknownVariant(UnknownJobStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobStatus {
    fn from(name: String) -> Self {
        match &*name {
            "Complete" => JobStatus::Complete,
            "Failed" => JobStatus::Failed,
            "InProgress" => JobStatus::InProgress,
            "Queued" => JobStatus::Queued,
            _ => JobStatus::UnknownVariant(UnknownJobStatus { name }),
        }
    }
}

impl ::std::str::FromStr for JobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMetricName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MetricName {
    Cpu,
    EbsReadBytesPerSecond,
    EbsReadOpsPerSecond,
    EbsWriteBytesPerSecond,
    EbsWriteOpsPerSecond,
    Memory,
    #[doc(hidden)]
    UnknownVariant(UnknownMetricName),
}

impl Default for MetricName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for MetricName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for MetricName {
    fn into(self) -> String {
        match self {
            MetricName::Cpu => "Cpu".to_string(),
            MetricName::EbsReadBytesPerSecond => "EBS_READ_BYTES_PER_SECOND".to_string(),
            MetricName::EbsReadOpsPerSecond => "EBS_READ_OPS_PER_SECOND".to_string(),
            MetricName::EbsWriteBytesPerSecond => "EBS_WRITE_BYTES_PER_SECOND".to_string(),
            MetricName::EbsWriteOpsPerSecond => "EBS_WRITE_OPS_PER_SECOND".to_string(),
            MetricName::Memory => "Memory".to_string(),
            MetricName::UnknownVariant(UnknownMetricName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a MetricName {
    fn into(self) -> &'a str {
        match self {
            MetricName::Cpu => &"Cpu",
            MetricName::EbsReadBytesPerSecond => &"EBS_READ_BYTES_PER_SECOND",
            MetricName::EbsReadOpsPerSecond => &"EBS_READ_OPS_PER_SECOND",
            MetricName::EbsWriteBytesPerSecond => &"EBS_WRITE_BYTES_PER_SECOND",
            MetricName::EbsWriteOpsPerSecond => &"EBS_WRITE_OPS_PER_SECOND",
            MetricName::Memory => &"Memory",
            MetricName::UnknownVariant(UnknownMetricName { name: original }) => original,
        }
    }
}

impl From<&str> for MetricName {
    fn from(name: &str) -> Self {
        match name {
            "Cpu" => MetricName::Cpu,
            "EBS_READ_BYTES_PER_SECOND" => MetricName::EbsReadBytesPerSecond,
            "EBS_READ_OPS_PER_SECOND" => MetricName::EbsReadOpsPerSecond,
            "EBS_WRITE_BYTES_PER_SECOND" => MetricName::EbsWriteBytesPerSecond,
            "EBS_WRITE_OPS_PER_SECOND" => MetricName::EbsWriteOpsPerSecond,
            "Memory" => MetricName::Memory,
            _ => MetricName::UnknownVariant(UnknownMetricName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for MetricName {
    fn from(name: String) -> Self {
        match &*name {
            "Cpu" => MetricName::Cpu,
            "EBS_READ_BYTES_PER_SECOND" => MetricName::EbsReadBytesPerSecond,
            "EBS_READ_OPS_PER_SECOND" => MetricName::EbsReadOpsPerSecond,
            "EBS_WRITE_BYTES_PER_SECOND" => MetricName::EbsWriteBytesPerSecond,
            "EBS_WRITE_OPS_PER_SECOND" => MetricName::EbsWriteOpsPerSecond,
            "Memory" => MetricName::Memory,
            _ => MetricName::UnknownVariant(UnknownMetricName { name }),
        }
    }
}

impl ::std::str::FromStr for MetricName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for MetricName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MetricName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMetricStatistic {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MetricStatistic {
    Average,
    Maximum,
    #[doc(hidden)]
    UnknownVariant(UnknownMetricStatistic),
}

impl Default for MetricStatistic {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for MetricStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for MetricStatistic {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for MetricStatistic {
    fn into(self) -> String {
        match self {
            MetricStatistic::Average => "Average".to_string(),
            MetricStatistic::Maximum => "Maximum".to_string(),
            MetricStatistic::UnknownVariant(UnknownMetricStatistic { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a MetricStatistic {
    fn into(self) -> &'a str {
        match self {
            MetricStatistic::Average => &"Average",
            MetricStatistic::Maximum => &"Maximum",
            MetricStatistic::UnknownVariant(UnknownMetricStatistic { name: original }) => original,
        }
    }
}

impl From<&str> for MetricStatistic {
    fn from(name: &str) -> Self {
        match name {
            "Average" => MetricStatistic::Average,
            "Maximum" => MetricStatistic::Maximum,
            _ => MetricStatistic::UnknownVariant(UnknownMetricStatistic {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for MetricStatistic {
    fn from(name: String) -> Self {
        match &*name {
            "Average" => MetricStatistic::Average,
            "Maximum" => MetricStatistic::Maximum,
            _ => MetricStatistic::UnknownVariant(UnknownMetricStatistic { name }),
        }
    }
}

impl ::std::str::FromStr for MetricStatistic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for MetricStatistic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MetricStatistic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p><p>Describes a projected utilization metric of a recommendation option, such as an Amazon EC2 instance. This represents the projected utilization of a recommendation option had you used that resource during the analyzed period.</p> <p>Compare the utilization metric data of your resource against its projected utilization metric data to determine the performance difference between your current resource and the recommended option.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned when you run the <code>GetEC2RecommendationProjectedMetrics</code> action. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectedMetric {
    /// <p><p>The name of the projected utilization metric.</p> <p>The following projected utilization metrics are returned:</p> <ul> <li> <p> <code>Cpu</code> - The projected percentage of allocated EC2 compute units that would be in use on the recommendation option had you used that resource during the analyzed period. This metric identifies the processing power required to run an application on the recommendation option.</p> <p>Depending on the instance type, tools in your operating system can show a lower percentage than CloudWatch when the instance is not allocated a full processor core.</p> <p>Units: Percent</p> </li> <li> <p> <code>Memory</code> - The percentage of memory that would be in use on the recommendation option had you used that resource during the analyzed period. This metric identifies the amount of memory required to run an application on the recommendation option.</p> <p>Units: Percent</p> <note> <p>The <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    /// <p>The time stamps of the projected utilization metric.</p>
    #[serde(rename = "timestamps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    /// <p>The values of the projected utilization metrics.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

/// <p>Describes a recommendation export job.</p> <p>Use the <code>DescribeRecommendationExportJobs</code> action to view your recommendation export jobs.</p> <p>Use the <code>ExportAutoScalingGroupRecommendations</code> or <code>ExportEC2InstanceRecommendations</code> actions to request an export of your recommendations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationExportJob {
    /// <p>The timestamp of when the export job was created.</p>
    #[serde(rename = "creationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>An object that describes the destination of the export file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ExportDestination>,
    /// <p>The reason for an export job failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The identification number of the export job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The timestamp of when the export job was last updated.</p>
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    /// <p>The resource type of the exported recommendations.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceType>,
    /// <p>The status of the export job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
}

/// <p>Describes the source of a recommendation, such as an Amazon EC2 instance or Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationSource {
    /// <p>The Amazon Resource Name (ARN) of the recommendation source.</p>
    #[serde(rename = "recommendationSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_source_arn: Option<String>,
    /// <p>The resource type of the recommendation source.</p>
    #[serde(rename = "recommendationSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_source_type: Option<RecommendationSourceType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRecommendationSourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RecommendationSourceType {
    AutoScalingGroup,
    EbsVolume,
    Ec2Instance,
    #[doc(hidden)]
    UnknownVariant(UnknownRecommendationSourceType),
}

impl Default for RecommendationSourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RecommendationSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RecommendationSourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RecommendationSourceType {
    fn into(self) -> String {
        match self {
            RecommendationSourceType::AutoScalingGroup => "AutoScalingGroup".to_string(),
            RecommendationSourceType::EbsVolume => "EbsVolume".to_string(),
            RecommendationSourceType::Ec2Instance => "Ec2Instance".to_string(),
            RecommendationSourceType::UnknownVariant(UnknownRecommendationSourceType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RecommendationSourceType {
    fn into(self) -> &'a str {
        match self {
            RecommendationSourceType::AutoScalingGroup => &"AutoScalingGroup",
            RecommendationSourceType::EbsVolume => &"EbsVolume",
            RecommendationSourceType::Ec2Instance => &"Ec2Instance",
            RecommendationSourceType::UnknownVariant(UnknownRecommendationSourceType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for RecommendationSourceType {
    fn from(name: &str) -> Self {
        match name {
            "AutoScalingGroup" => RecommendationSourceType::AutoScalingGroup,
            "EbsVolume" => RecommendationSourceType::EbsVolume,
            "Ec2Instance" => RecommendationSourceType::Ec2Instance,
            _ => RecommendationSourceType::UnknownVariant(UnknownRecommendationSourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RecommendationSourceType {
    fn from(name: String) -> Self {
        match &*name {
            "AutoScalingGroup" => RecommendationSourceType::AutoScalingGroup,
            "EbsVolume" => RecommendationSourceType::EbsVolume,
            "Ec2Instance" => RecommendationSourceType::Ec2Instance,
            _ => RecommendationSourceType::UnknownVariant(UnknownRecommendationSourceType { name }),
        }
    }
}

impl ::std::str::FromStr for RecommendationSourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for RecommendationSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RecommendationSourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A summary of a recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationSummary {
    /// <p>The AWS account ID of the recommendation summary.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The resource type of the recommendation.</p>
    #[serde(rename = "recommendationResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_resource_type: Option<RecommendationSourceType>,
    /// <p>An array of objects that describe a recommendation summary.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<Summary>>,
}

/// <p><p>Describes a projected utilization metric of a recommendation option.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned when you run the <code>GetEC2RecommendationProjectedMetrics</code> action. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendedOptionProjectedMetric {
    /// <p>An array of objects that describe a projected utilization metric.</p>
    #[serde(rename = "projectedMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_metrics: Option<Vec<ProjectedMetric>>,
    /// <p>The rank of the recommendation option projected metric.</p> <p>The top recommendation option is ranked as <code>1</code>.</p> <p>The projected metric rank correlates to the recommendation option rank. For example, the projected metric ranked as <code>1</code> is related to the recommendation option that is also ranked as <code>1</code> in the same response.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The recommended instance type.</p>
    #[serde(rename = "recommendedInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_instance_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceType {
    AutoScalingGroup,
    Ec2Instance,
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
            ResourceType::AutoScalingGroup => "AutoScalingGroup".to_string(),
            ResourceType::Ec2Instance => "Ec2Instance".to_string(),
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceType {
    fn into(self) -> &'a str {
        match self {
            ResourceType::AutoScalingGroup => &"AutoScalingGroup",
            ResourceType::Ec2Instance => &"Ec2Instance",
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl From<&str> for ResourceType {
    fn from(name: &str) -> Self {
        match name {
            "AutoScalingGroup" => ResourceType::AutoScalingGroup,
            "Ec2Instance" => ResourceType::Ec2Instance,
            _ => ResourceType::UnknownVariant(UnknownResourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceType {
    fn from(name: String) -> Self {
        match &*name {
            "AutoScalingGroup" => ResourceType::AutoScalingGroup,
            "Ec2Instance" => ResourceType::Ec2Instance,
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

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the destination Amazon Simple Storage Service (Amazon S3) bucket name and object keys of a recommendations export file, and its associated metadata file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3Destination {
    /// <p>The name of the Amazon S3 bucket used as the destination of an export file.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The Amazon S3 bucket key of an export file.</p> <p>The key uniquely identifies the object, or export file, in the S3 bucket.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The Amazon S3 bucket key of a metadata file.</p> <p>The key uniquely identifies the object, or metadata file, in the S3 bucket.</p>
    #[serde(rename = "metadataKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,
}

/// <p>Describes the destination Amazon Simple Storage Service (Amazon S3) bucket name and key prefix for a recommendations export job.</p> <p>You must create the destination Amazon S3 bucket for your recommendations export before you create the export job. Compute Optimizer does not create the S3 bucket for you. After you create the S3 bucket, ensure that it has the required permission policy to allow Compute Optimizer to write the export file to it. If you plan to specify an object prefix when you create the export job, you must include the object prefix in the policy that you add to the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/create-s3-bucket-policy-for-compute-optimizer.html">Amazon S3 Bucket Policy for Compute Optimizer</a> in the <i>Compute Optimizer user guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3DestinationConfig {
    /// <p>The name of the Amazon S3 bucket to use as the destination for an export job.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The Amazon S3 bucket prefix for an export job.</p>
    #[serde(rename = "keyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Status {
    Active,
    Failed,
    Inactive,
    Pending,
    #[doc(hidden)]
    UnknownVariant(UnknownStatus),
}

impl Default for Status {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Status {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Status::Active => "Active".to_string(),
            Status::Failed => "Failed".to_string(),
            Status::Inactive => "Inactive".to_string(),
            Status::Pending => "Pending".to_string(),
            Status::UnknownVariant(UnknownStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Status {
    fn into(self) -> &'a str {
        match self {
            Status::Active => &"Active",
            Status::Failed => &"Failed",
            Status::Inactive => &"Inactive",
            Status::Pending => &"Pending",
            Status::UnknownVariant(UnknownStatus { name: original }) => original,
        }
    }
}

impl From<&str> for Status {
    fn from(name: &str) -> Self {
        match name {
            "Active" => Status::Active,
            "Failed" => Status::Failed,
            "Inactive" => Status::Inactive,
            "Pending" => Status::Pending,
            _ => Status::UnknownVariant(UnknownStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Status {
    fn from(name: String) -> Self {
        match &*name {
            "Active" => Status::Active,
            "Failed" => Status::Failed,
            "Inactive" => Status::Inactive,
            "Pending" => Status::Pending,
            _ => Status::UnknownVariant(UnknownStatus { name }),
        }
    }
}

impl ::std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The summary of a recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Summary {
    /// <p>The finding classification of the recommendation.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Finding>,
    /// <p>The value of the recommendation summary.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEnrollmentStatusRequest {
    /// <p>Indicates whether to enroll member accounts of the organization if the your account is the management account of an organization.</p>
    #[serde(rename = "includeMemberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_member_accounts: Option<bool>,
    /// <p>The new enrollment status of the account.</p> <p>Accepted options are <code>Active</code> or <code>Inactive</code>. You will get an error if <code>Pending</code> or <code>Failed</code> are specified.</p>
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEnrollmentStatusResponse {
    /// <p>The enrollment status of the account.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// <p>The reason for the enrollment status of the account. For example, an account might show a status of <code>Pending</code> because member accounts of an organization require more time to be enrolled in the service.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Describes a utilization metric of a resource, such as an Amazon EC2 instance.</p> <p>Compare the utilization metric data of your resource against its projected utilization metric data to determine the performance difference between your current resource and the recommended option.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UtilizationMetric {
    /// <p><p>The name of the utilization metric.</p> <p>The following utilization metrics are available:</p> <ul> <li> <p> <code>Cpu</code> - The percentage of allocated EC2 compute units that are currently in use on the instance. This metric identifies the processing power required to run an application on the instance.</p> <p>Depending on the instance type, tools in your operating system can show a lower percentage than CloudWatch when the instance is not allocated a full processor core.</p> <p>Units: Percent</p> </li> <li> <p> <code>Memory</code> - The percentage of memory that is currently in use on the instance. This metric identifies the amount of memory required to run an application on the instance.</p> <p>Units: Percent</p> <note> <p>The <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note> </li> <li> <p> <code>EBS<em>READ</em>OPS<em>PER</em>SECOND</code> - The completed read operations from all EBS volumes attached to the instance in a specified period of time.</p> <p>Unit: Count</p> </li> <li> <p> <code>EBS<em>WRITE</em>OPS<em>PER</em>SECOND</code> - The completed write operations to all EBS volumes attached to the instance in a specified period of time.</p> <p>Unit: Count</p> </li> <li> <p> <code>EBS<em>READ</em>BYTES<em>PER</em>SECOND</code> - The bytes read from all EBS volumes attached to the instance in a specified period of time.</p> <p>Unit: Bytes</p> </li> <li> <p> <code>EBS<em>WRITE</em>BYTES<em>PER</em>SECOND</code> - The bytes written to all EBS volumes attached to the instance in a specified period of time.</p> <p>Unit: Bytes</p> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    /// <p><p>The statistic of the utilization metric.</p> <p>The following statistics are available:</p> <ul> <li> <p> <code>Average</code> - This is the value of Sum / SampleCount during the specified period, or the average value observed during the specified period.</p> </li> <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li> </ul></p>
    #[serde(rename = "statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<MetricStatistic>,
    /// <p>The value of the utilization metric.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Describes the configuration of an Amazon Elastic Block Store (Amazon EBS) volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeConfiguration {
    /// <p>The baseline IOPS of the volume.</p>
    #[serde(rename = "volumeBaselineIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_baseline_iops: Option<i64>,
    /// <p>The baseline throughput of the volume.</p>
    #[serde(rename = "volumeBaselineThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_baseline_throughput: Option<i64>,
    /// <p>The burst IOPS of the volume.</p>
    #[serde(rename = "volumeBurstIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_burst_iops: Option<i64>,
    /// <p>The burst throughput of the volume.</p>
    #[serde(rename = "volumeBurstThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_burst_throughput: Option<i64>,
    /// <p>The size of the volume, in GiB.</p>
    #[serde(rename = "volumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
    /// <p>The volume type.</p> <p>This can be <code>gp2</code> for General Purpose SSD, <code>io1</code> or <code>io2</code> for Provisioned IOPS SSD, <code>st1</code> for Throughput Optimized HDD, <code>sc1</code> for Cold HDD, or <code>standard</code> for Magnetic volumes.</p>
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes an Amazon Elastic Block Store (Amazon EBS) volume recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeRecommendation {
    /// <p>The AWS account ID of the volume.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>An array of objects that describe the current configuration of the volume.</p>
    #[serde(rename = "currentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_configuration: Option<VolumeConfiguration>,
    /// <p><p>The finding classification for the volume.</p> <p>Findings for volumes include:</p> <ul> <li> <p> <b> <code>NotOptimized</code> </b>—A volume is considered not optimized when AWS Compute Optimizer identifies a recommendation that can provide better performance for your workload.</p> </li> <li> <p> <b> <code>Optimized</code> </b>—An volume is considered optimized when Compute Optimizer determines that the volume is correctly provisioned to run your workload based on the chosen volume type. For optimized resources, Compute Optimizer might recommend a new generation volume type.</p> </li> </ul></p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<EBSFinding>,
    /// <p>The time stamp of when the volume recommendation was last refreshed.</p>
    #[serde(rename = "lastRefreshTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_timestamp: Option<f64>,
    /// <p>The number of days for which utilization metrics were analyzed for the volume.</p>
    #[serde(rename = "lookBackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_period_in_days: Option<f64>,
    /// <p>An array of objects that describe the utilization metrics of the volume.</p>
    #[serde(rename = "utilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_metrics: Option<Vec<EBSUtilizationMetric>>,
    /// <p>The Amazon Resource Name (ARN) of the current volume.</p>
    #[serde(rename = "volumeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>An array of objects that describe the recommendation options for the volume.</p>
    #[serde(rename = "volumeRecommendationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recommendation_options: Option<Vec<VolumeRecommendationOption>>,
}

/// <p>Describes a recommendation option for an Amazon Elastic Block Store (Amazon EBS) instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeRecommendationOption {
    /// <p>An array of objects that describe a volume configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<VolumeConfiguration>,
    /// <p>The performance risk of the volume recommendation option.</p> <p>Performance risk is the likelihood of the recommended volume type not meeting the performance requirement of your workload.</p> <p>The lowest performance risk is categorized as <code>0</code>, and the highest as <code>5</code>.</p>
    #[serde(rename = "performanceRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_risk: Option<f64>,
    /// <p>The rank of the volume recommendation option.</p> <p>The top recommendation option is ranked as <code>1</code>.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// Errors returned by DescribeRecommendationExportJobs
#[derive(Debug, PartialEq)]
pub enum DescribeRecommendationExportJobsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeRecommendationExportJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRecommendationExportJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::OptInRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeRecommendationExportJobsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRecommendationExportJobsError::Throttling(
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
impl fmt::Display for DescribeRecommendationExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRecommendationExportJobsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRecommendationExportJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRecommendationExportJobsError {}
/// Errors returned by ExportAutoScalingGroupRecommendations
#[derive(Debug, PartialEq)]
pub enum ExportAutoScalingGroupRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request exceeds a limit of the service.</p>
    LimitExceeded(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ExportAutoScalingGroupRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExportAutoScalingGroupRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::LimitExceeded(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::MissingAuthenticationToken(
                            err.msg,
                        ),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::OptInRequired(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        ExportAutoScalingGroupRecommendationsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExportAutoScalingGroupRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportAutoScalingGroupRecommendationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportAutoScalingGroupRecommendationsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ExportAutoScalingGroupRecommendationsError {}
/// Errors returned by ExportEC2InstanceRecommendations
#[derive(Debug, PartialEq)]
pub enum ExportEC2InstanceRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request exceeds a limit of the service.</p>
    LimitExceeded(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ExportEC2InstanceRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExportEC2InstanceRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::LimitExceeded(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::OptInRequired(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ExportEC2InstanceRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ExportEC2InstanceRecommendationsError::Throttling(
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
impl fmt::Display for ExportEC2InstanceRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportEC2InstanceRecommendationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ExportEC2InstanceRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportEC2InstanceRecommendationsError {}
/// Errors returned by GetAutoScalingGroupRecommendations
#[derive(Debug, PartialEq)]
pub enum GetAutoScalingGroupRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetAutoScalingGroupRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAutoScalingGroupRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::MissingAuthenticationToken(
                            err.msg,
                        ),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::OptInRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAutoScalingGroupRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAutoScalingGroupRecommendationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAutoScalingGroupRecommendationsError {}
/// Errors returned by GetEBSVolumeRecommendations
#[derive(Debug, PartialEq)]
pub enum GetEBSVolumeRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetEBSVolumeRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEBSVolumeRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEBSVolumeRecommendationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEBSVolumeRecommendationsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetEBSVolumeRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEBSVolumeRecommendationsError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(GetEBSVolumeRecommendationsError::OptInRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetEBSVolumeRecommendationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetEBSVolumeRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEBSVolumeRecommendationsError::Throttling(
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
impl fmt::Display for GetEBSVolumeRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEBSVolumeRecommendationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEBSVolumeRecommendationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEBSVolumeRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEBSVolumeRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEBSVolumeRecommendationsError::OptInRequired(ref cause) => write!(f, "{}", cause),
            GetEBSVolumeRecommendationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetEBSVolumeRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEBSVolumeRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEBSVolumeRecommendationsError {}
/// Errors returned by GetEC2InstanceRecommendations
#[derive(Debug, PartialEq)]
pub enum GetEC2InstanceRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetEC2InstanceRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEC2InstanceRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::OptInRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::Throttling(
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
impl fmt::Display for GetEC2InstanceRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEC2InstanceRecommendationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::OptInRequired(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEC2InstanceRecommendationsError {}
/// Errors returned by GetEC2RecommendationProjectedMetrics
#[derive(Debug, PartialEq)]
pub enum GetEC2RecommendationProjectedMetricsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>A resource that is required for the action doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetEC2RecommendationProjectedMetricsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEC2RecommendationProjectedMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::MissingAuthenticationToken(
                            err.msg,
                        ),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::OptInRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEC2RecommendationProjectedMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEC2RecommendationProjectedMetricsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetEC2RecommendationProjectedMetricsError {}
/// Errors returned by GetEnrollmentStatus
#[derive(Debug, PartialEq)]
pub enum GetEnrollmentStatusError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetEnrollmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEnrollmentStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEnrollmentStatusError::MissingAuthenticationToken(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEnrollmentStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEnrollmentStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEnrollmentStatusError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEnrollmentStatusError {}
/// Errors returned by GetRecommendationSummaries
#[derive(Debug, PartialEq)]
pub enum GetRecommendationSummariesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The account is not opted in to AWS Compute Optimizer.</p>
    OptInRequired(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetRecommendationSummariesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRecommendationSummariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::OptInRequired(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::Throttling(
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
impl fmt::Display for GetRecommendationSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecommendationSummariesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::OptInRequired(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecommendationSummariesError {}
/// Errors returned by UpdateEnrollmentStatus
#[derive(Debug, PartialEq)]
pub enum UpdateEnrollmentStatusError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred. Try your call again.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateEnrollmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEnrollmentStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateEnrollmentStatusError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        UpdateEnrollmentStatusError::MissingAuthenticationToken(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEnrollmentStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEnrollmentStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEnrollmentStatusError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEnrollmentStatusError {}
/// Trait representing the capabilities of the AWS Compute Optimizer API. AWS Compute Optimizer clients implement this trait.
#[async_trait]
pub trait ComputeOptimizer {
    /// <p>Describes recommendation export jobs created in the last seven days.</p> <p>Use the <code>ExportAutoScalingGroupRecommendations</code> or <code>ExportEC2InstanceRecommendations</code> actions to request an export of your recommendations. Then use the <code>DescribeRecommendationExportJobs</code> action to view your export jobs.</p>
    async fn describe_recommendation_export_jobs(
        &self,
        input: DescribeRecommendationExportJobsRequest,
    ) -> Result<
        DescribeRecommendationExportJobsResponse,
        RusotoError<DescribeRecommendationExportJobsError>,
    >;

    /// <p>Exports optimization recommendations for Auto Scaling groups.</p> <p>Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (.json) file, to an existing Amazon Simple Storage Service (Amazon S3) bucket that you specify. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html">Exporting Recommendations</a> in the <i>Compute Optimizer User Guide</i>.</p> <p>You can have only one Auto Scaling group export job in progress per AWS Region.</p>
    async fn export_auto_scaling_group_recommendations(
        &self,
        input: ExportAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        ExportAutoScalingGroupRecommendationsResponse,
        RusotoError<ExportAutoScalingGroupRecommendationsError>,
    >;

    /// <p>Exports optimization recommendations for Amazon EC2 instances.</p> <p>Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (.json) file, to an existing Amazon Simple Storage Service (Amazon S3) bucket that you specify. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html">Exporting Recommendations</a> in the <i>Compute Optimizer User Guide</i>.</p> <p>You can have only one Amazon EC2 instance export job in progress per AWS Region.</p>
    async fn export_ec2_instance_recommendations(
        &self,
        input: ExportEC2InstanceRecommendationsRequest,
    ) -> Result<
        ExportEC2InstanceRecommendationsResponse,
        RusotoError<ExportEC2InstanceRecommendationsError>,
    >;

    /// <p>Returns Auto Scaling group recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon EC2 Auto Scaling groups that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_auto_scaling_group_recommendations(
        &self,
        input: GetAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        GetAutoScalingGroupRecommendationsResponse,
        RusotoError<GetAutoScalingGroupRecommendationsError>,
    >;

    /// <p>Returns Amazon Elastic Block Store (Amazon EBS) volume recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon EBS volumes that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_ebs_volume_recommendations(
        &self,
        input: GetEBSVolumeRecommendationsRequest,
    ) -> Result<GetEBSVolumeRecommendationsResponse, RusotoError<GetEBSVolumeRecommendationsError>>;

    /// <p>Returns Amazon EC2 instance recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon Elastic Compute Cloud (Amazon EC2) instances that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_ec2_instance_recommendations(
        &self,
        input: GetEC2InstanceRecommendationsRequest,
    ) -> Result<
        GetEC2InstanceRecommendationsResponse,
        RusotoError<GetEC2InstanceRecommendationsError>,
    >;

    /// <p><p>Returns the projected utilization metrics of Amazon EC2 instance recommendations.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned when you run this action. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    async fn get_ec2_recommendation_projected_metrics(
        &self,
        input: GetEC2RecommendationProjectedMetricsRequest,
    ) -> Result<
        GetEC2RecommendationProjectedMetricsResponse,
        RusotoError<GetEC2RecommendationProjectedMetricsError>,
    >;

    /// <p>Returns the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is the management account of an organization, this action also confirms the enrollment status of member accounts within the organization.</p>
    async fn get_enrollment_status(
        &self,
    ) -> Result<GetEnrollmentStatusResponse, RusotoError<GetEnrollmentStatusError>>;

    /// <p>Returns the optimization findings for an account.</p> <p>For example, it returns the number of Amazon EC2 instances in an account that are under-provisioned, over-provisioned, or optimized. It also returns the number of Auto Scaling groups in an account that are not optimized, or optimized.</p>
    async fn get_recommendation_summaries(
        &self,
        input: GetRecommendationSummariesRequest,
    ) -> Result<GetRecommendationSummariesResponse, RusotoError<GetRecommendationSummariesError>>;

    /// <p>Updates the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a management account of an organization, this action can also be used to enroll member accounts within the organization.</p>
    async fn update_enrollment_status(
        &self,
        input: UpdateEnrollmentStatusRequest,
    ) -> Result<UpdateEnrollmentStatusResponse, RusotoError<UpdateEnrollmentStatusError>>;
}
/// A client for the AWS Compute Optimizer API.
#[derive(Clone)]
pub struct ComputeOptimizerClient {
    client: Client,
    region: region::Region,
}

impl ComputeOptimizerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComputeOptimizerClient {
        ComputeOptimizerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComputeOptimizerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ComputeOptimizerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComputeOptimizerClient {
        ComputeOptimizerClient { client, region }
    }
}

#[async_trait]
impl ComputeOptimizer for ComputeOptimizerClient {
    /// <p>Describes recommendation export jobs created in the last seven days.</p> <p>Use the <code>ExportAutoScalingGroupRecommendations</code> or <code>ExportEC2InstanceRecommendations</code> actions to request an export of your recommendations. Then use the <code>DescribeRecommendationExportJobs</code> action to view your export jobs.</p>
    async fn describe_recommendation_export_jobs(
        &self,
        input: DescribeRecommendationExportJobsRequest,
    ) -> Result<
        DescribeRecommendationExportJobsResponse,
        RusotoError<DescribeRecommendationExportJobsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.DescribeRecommendationExportJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeRecommendationExportJobsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeRecommendationExportJobsResponse, _>()
    }

    /// <p>Exports optimization recommendations for Auto Scaling groups.</p> <p>Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (.json) file, to an existing Amazon Simple Storage Service (Amazon S3) bucket that you specify. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html">Exporting Recommendations</a> in the <i>Compute Optimizer User Guide</i>.</p> <p>You can have only one Auto Scaling group export job in progress per AWS Region.</p>
    async fn export_auto_scaling_group_recommendations(
        &self,
        input: ExportAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        ExportAutoScalingGroupRecommendationsResponse,
        RusotoError<ExportAutoScalingGroupRecommendationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.ExportAutoScalingGroupRecommendations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ExportAutoScalingGroupRecommendationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ExportAutoScalingGroupRecommendationsResponse, _>()
    }

    /// <p>Exports optimization recommendations for Amazon EC2 instances.</p> <p>Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (.json) file, to an existing Amazon Simple Storage Service (Amazon S3) bucket that you specify. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html">Exporting Recommendations</a> in the <i>Compute Optimizer User Guide</i>.</p> <p>You can have only one Amazon EC2 instance export job in progress per AWS Region.</p>
    async fn export_ec2_instance_recommendations(
        &self,
        input: ExportEC2InstanceRecommendationsRequest,
    ) -> Result<
        ExportEC2InstanceRecommendationsResponse,
        RusotoError<ExportEC2InstanceRecommendationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.ExportEC2InstanceRecommendations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ExportEC2InstanceRecommendationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ExportEC2InstanceRecommendationsResponse, _>()
    }

    /// <p>Returns Auto Scaling group recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon EC2 Auto Scaling groups that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_auto_scaling_group_recommendations(
        &self,
        input: GetAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        GetAutoScalingGroupRecommendationsResponse,
        RusotoError<GetAutoScalingGroupRecommendationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetAutoScalingGroupRecommendations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetAutoScalingGroupRecommendationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAutoScalingGroupRecommendationsResponse, _>()
    }

    /// <p>Returns Amazon Elastic Block Store (Amazon EBS) volume recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon EBS volumes that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_ebs_volume_recommendations(
        &self,
        input: GetEBSVolumeRecommendationsRequest,
    ) -> Result<GetEBSVolumeRecommendationsResponse, RusotoError<GetEBSVolumeRecommendationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEBSVolumeRecommendations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEBSVolumeRecommendationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetEBSVolumeRecommendationsResponse, _>()
    }

    /// <p>Returns Amazon EC2 instance recommendations.</p> <p>AWS Compute Optimizer generates recommendations for Amazon Elastic Compute Cloud (Amazon EC2) instances that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>AWS Compute Optimizer User Guide</i>.</p>
    async fn get_ec2_instance_recommendations(
        &self,
        input: GetEC2InstanceRecommendationsRequest,
    ) -> Result<
        GetEC2InstanceRecommendationsResponse,
        RusotoError<GetEC2InstanceRecommendationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEC2InstanceRecommendations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEC2InstanceRecommendationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetEC2InstanceRecommendationsResponse, _>()
    }

    /// <p><p>Returns the projected utilization metrics of Amazon EC2 instance recommendations.</p> <note> <p>The <code>Cpu</code> and <code>Memory</code> metrics are the only projected utilization metrics returned when you run this action. Additionally, the <code>Memory</code> metric is returned only for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html#cw-agent">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    async fn get_ec2_recommendation_projected_metrics(
        &self,
        input: GetEC2RecommendationProjectedMetricsRequest,
    ) -> Result<
        GetEC2RecommendationProjectedMetricsResponse,
        RusotoError<GetEC2RecommendationProjectedMetricsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEC2RecommendationProjectedMetrics",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetEC2RecommendationProjectedMetricsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetEC2RecommendationProjectedMetricsResponse, _>()
    }

    /// <p>Returns the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is the management account of an organization, this action also confirms the enrollment status of member accounts within the organization.</p>
    async fn get_enrollment_status(
        &self,
    ) -> Result<GetEnrollmentStatusResponse, RusotoError<GetEnrollmentStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEnrollmentStatus",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetEnrollmentStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEnrollmentStatusResponse, _>()
    }

    /// <p>Returns the optimization findings for an account.</p> <p>For example, it returns the number of Amazon EC2 instances in an account that are under-provisioned, over-provisioned, or optimized. It also returns the number of Auto Scaling groups in an account that are not optimized, or optimized.</p>
    async fn get_recommendation_summaries(
        &self,
        input: GetRecommendationSummariesRequest,
    ) -> Result<GetRecommendationSummariesResponse, RusotoError<GetRecommendationSummariesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetRecommendationSummaries",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRecommendationSummariesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetRecommendationSummariesResponse, _>()
    }

    /// <p>Updates the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a management account of an organization, this action can also be used to enroll member accounts within the organization.</p>
    async fn update_enrollment_status(
        &self,
        input: UpdateEnrollmentStatusRequest,
    ) -> Result<UpdateEnrollmentStatusResponse, RusotoError<UpdateEnrollmentStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.UpdateEnrollmentStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateEnrollmentStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateEnrollmentStatusResponse, _>()
    }
}
