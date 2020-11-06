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
/// <p>The structure representing the AddNotificationChannelsRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddNotificationChannelsRequest {
    /// <p>One or 2 channels to report to when anomalies are detected.</p>
    #[serde(rename = "channels")]
    pub channels: Vec<Channel>,
    /// <p>The name of the profiling group that we are setting up notifications for.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the AddNotificationChannelsResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddNotificationChannelsResponse {
    /// <p>The new notification configuration for this profiling group.</p>
    #[serde(rename = "notificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
}

/// <p> The response of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a> that specifies if an agent profiles or not and for how long to return profiling data. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentConfiguration {
    /// <p><p> Parameters used by the profiler. The valid parameters are: </p> <ul> <li> <p> <code>MaxStackDepth</code> - The maximum depth of the stacks in the code that is represented in the profile. For example, if CodeGuru Profiler finds a method <code>A</code>, which calls method <code>B</code>, which calls method <code>C</code>, which calls method <code>D</code>, then the depth is 4. If the <code>maxDepth</code> is set to 2, then the profiler evaluates <code>A</code> and <code>B</code>. </p> </li> <li> <p> <code>MemoryUsageLimitPercent</code> - The percentage of memory that is used by the profiler.</p> </li> <li> <p> <code>MinimumTimeForReportingInMilliseconds</code> - The minimum time in milliseconds between sending reports. </p> </li> <li> <p> <code>ReportingIntervalInMilliseconds</code> - The reporting interval in milliseconds used to report profiles. </p> </li> <li> <p> <code>SamplingIntervalInMilliseconds</code> - The sampling interval in milliseconds that is used to profile samples. </p> </li> </ul></p>
    #[serde(rename = "agentParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p> How long a profiling agent should send profiling data using <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a>. For example, if this is set to 300, the profiling agent calls <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a> every 5 minutes to submit the profiled data collected during that period. </p>
    #[serde(rename = "periodInSeconds")]
    pub period_in_seconds: i64,
    /// <p> A <code>Boolean</code> that specifies whether the profiling agent collects profiling data or not. Set to <code>true</code> to enable profiling. </p>
    #[serde(rename = "shouldProfile")]
    pub should_profile: bool,
}

/// <p> Specifies whether profiling is enabled or disabled for a profiling group. It is used by <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a> to enable or disable profiling for a profiling group. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AgentOrchestrationConfig {
    /// <p> A <code>Boolean</code> that specifies whether the profiling agent collects profiling data or not. Set to <code>true</code> to enable profiling. </p>
    #[serde(rename = "profilingEnabled")]
    pub profiling_enabled: bool,
}

/// <p> Specifies the aggregation period and aggregation start time for an aggregated profile. An aggregated profile is used to collect posted agent profiles during an aggregation period. There are three possible aggregation periods (1 day, 1 hour, or 5 minutes). </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AggregatedProfileTime {
    /// <p><p> The aggregation period. This indicates the period during which an aggregation profile collects posted agent profiles for a profiling group. Use one of three valid durations that are specified using the ISO 8601 format. </p> <ul> <li> <p> <code>P1D</code> — 1 day </p> </li> <li> <p> <code>PT1H</code> — 1 hour </p> </li> <li> <p> <code>PT5M</code> — 5 minutes </p> </li> </ul></p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p> The time that aggregation of posted agent profiles for a profiling group starts. The aggregation profile contains profiles posted by the agent starting at this time for an aggregation period specified by the <code>period</code> property of the <code>AggregatedProfileTime</code> object. </p> <p> Specify <code>start</code> using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p> Details about an anomaly in a specific metric of application profile. The anomaly is detected using analysis of the metric data over a period of time. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Anomaly {
    /// <p> A list of the instances of the detected anomalies during the requested period. </p>
    #[serde(rename = "instances")]
    pub instances: Vec<AnomalyInstance>,
    /// <p> Details about the metric that the analysis used when it detected the anomaly. The metric includes the name of the frame that was analyzed with the type and thread states used to derive the metric value for that frame. </p>
    #[serde(rename = "metric")]
    pub metric: Metric,
    /// <p>The reason for which metric was flagged as anomalous.</p>
    #[serde(rename = "reason")]
    pub reason: String,
}

/// <p>The specific duration in which the metric is flagged as anomalous.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnomalyInstance {
    /// <p> The end time of the period during which the metric is flagged as anomalous. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The universally unique identifier (UUID) of an instance of an anomaly in a metric. </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p> The start time of the period during which the metric is flagged as anomalous. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>Feedback type on a specific instance of anomaly submitted by the user.</p>
    #[serde(rename = "userFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<UserFeedback>,
}

/// <p>The structure representing the BatchGetFrameMetricDataRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetFrameMetricDataRequest {
    /// <p> The end time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The details of the metrics that are used to request a time series of values. The metric includes the name of the frame, the aggregation type to calculate the metric value for the frame, and the thread states to use to get the count for the metric value of the frame.</p>
    #[serde(rename = "frameMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_metrics: Option<Vec<FrameMetric>>,
    /// <p> The duration of the frame metrics used to return the time series values. Specify using the ISO 8601 format. The maximum period duration is one day (<code>PT24H</code> or <code>P1D</code>). </p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p> The name of the profiling group associated with the the frame metrics used to return the time series values. </p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> The start time of the time period for the frame metrics used to return the time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The requested resolution of time steps for the returned time series of values. If the requested target resolution is not available due to data not being retained we provide a best effort result by falling back to the most granular available resolution after the target resolution. There are 3 valid values. </p> <ul> <li> <p> <code>P1D</code> — 1 day </p> </li> <li> <p> <code>PT1H</code> — 1 hour </p> </li> <li> <p> <code>PT5M</code> — 5 minutes </p> </li> </ul></p>
    #[serde(rename = "targetResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resolution: Option<String>,
}

/// <p>The structure representing the BatchGetFrameMetricDataResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetFrameMetricDataResponse {
    /// <p> The end time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p> List of instances, or time steps, in the time series. For example, if the <code>period</code> is one day (<code>PT24H)</code>), and the <code>resolution</code> is five minutes (<code>PT5M</code>), then there are 288 <code>endTimes</code> in the list that are each five minutes appart. </p>
    #[serde(rename = "endTimes")]
    pub end_times: Vec<TimestampStructure>,
    /// <p>Details of the metrics to request a time series of values. The metric includes the name of the frame, the aggregation type to calculate the metric value for the frame, and the thread states to use to get the count for the metric value of the frame.</p>
    #[serde(rename = "frameMetricData")]
    pub frame_metric_data: Vec<FrameMetricDatum>,
    /// <p><p>Resolution or granularity of the profile data used to generate the time series. This is the value used to jump through time steps in a time series. There are 3 valid values. </p> <ul> <li> <p> <code>P1D</code> — 1 day </p> </li> <li> <p> <code>PT1H</code> — 1 hour </p> </li> <li> <p> <code>PT5M</code> — 5 minutes </p> </li> </ul></p>
    #[serde(rename = "resolution")]
    pub resolution: String,
    /// <p> The start time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>List of instances which remained unprocessed. This will create a missing time step in the list of end times.</p>
    #[serde(rename = "unprocessedEndTimes")]
    pub unprocessed_end_times: ::std::collections::HashMap<String, Vec<TimestampStructure>>,
}

/// <p>Notification medium for users to get alerted for events that occur in application profile. We support SNS topic as a notification channel.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Channel {
    /// <p>List of publishers for different type of events that may be detected in an application from the profile. Anomaly detection is the only event publisher in Profiler.</p>
    #[serde(rename = "eventPublishers")]
    pub event_publishers: Vec<String>,
    /// <p>Unique identifier for each <code>Channel</code> in the notification configuration of a Profiling Group. A random UUID for channelId is used when adding a channel to the notification configuration if not specified in the request.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Unique arn of the resource to be used for notifications. We support a valid SNS topic arn as a channel uri.</p>
    #[serde(rename = "uri")]
    pub uri: String,
}

/// <p>The structure representing the configureAgentRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigureAgentRequest {
    /// <p> A universally unique identifier (UUID) for a profiling instance. For example, if the profiling instance is an Amazon EC2 instance, it is the instance ID. If it is an AWS Fargate container, it is the container's task ID. </p>
    #[serde(rename = "fleetInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_instance_id: Option<String>,
    /// <p><p> Metadata captured about the compute platform the agent is running on. It includes information about sampling and reporting. The valid fields are:</p> <ul> <li> <p> <code>COMPUTE<em>PLATFORM</code> - The compute platform on which the agent is running </p> </li> <li> <p> <code>AGENT</em>ID</code> - The ID for an agent instance. </p> </li> <li> <p> <code>AWS<em>REQUEST</em>ID</code> - The AWS request ID of a Lambda invocation. </p> </li> <li> <p> <code>EXECUTION<em>ENVIRONMENT</code> - The execution environment a Lambda function is running on. </p> </li> <li> <p> <code>LAMBDA</em>FUNCTION<em>ARN</code> - The Amazon Resource Name (ARN) that is used to invoke a Lambda function. </p> </li> <li> <p> <code>LAMBDA</em>MEMORY<em>LIMIT</em>IN<em>MB</code> - The memory allocated to a Lambda function. </p> </li> <li> <p> <code>LAMBDA</em>REMAINING<em>TIME</em>IN<em>MILLISECONDS</code> - The time in milliseconds before execution of a Lambda function times out. </p> </li> <li> <p> <code>LAMBDA</em>TIME<em>GAP</em>BETWEEN<em>INVOKES</em>IN<em>MILLISECONDS</code> - The time in milliseconds between two invocations of a Lambda function. </p> </li> <li> <p> <code>LAMBDA</em>PREVIOUS<em>EXECUTION</em>TIME<em>IN</em>MILLISECONDS</code> - The time in milliseconds for the previous Lambda invocation. </p> </li> </ul></p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p> The name of the profiling group for which the configured agent is collecting profiling data. </p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the configureAgentResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigureAgentResponse {
    /// <p> An <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_AgentConfiguration.html"> <code>AgentConfiguration</code> </a> object that specifies if an agent profiles or not and for how long to return profiling data. </p>
    #[serde(rename = "configuration")]
    pub configuration: AgentConfiguration,
}

/// <p>The structure representing the createProfiliingGroupRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProfilingGroupRequest {
    /// <p> Specifies whether profiling is enabled or disabled for the created profiling group. </p>
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    /// <p> Amazon CodeGuru Profiler uses this universally unique identifier (UUID) to prevent the accidental creation of duplicate profiling groups if there are failures and retries. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p> The compute platform of the profiling group. Use <code>AWSLambda</code> if your application runs on AWS Lambda. Use <code>Default</code> if your application runs on a compute platform that is not AWS Lambda, such an Amazon EC2 instance, an on-premises server, or a different platform. If not specified, <code>Default</code> is used. </p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The name of the profiling group to create.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> A list of tags to add to the created profiling group. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The structure representing the createProfilingGroupResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProfilingGroupResponse {
    /// <p> The returned <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> object that contains information about the created profiling group. </p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>The structure representing the deleteProfilingGroupRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProfilingGroupRequest {
    /// <p>The name of the profiling group to delete.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the deleteProfilingGroupResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProfilingGroupResponse {}

/// <p>The structure representing the describeProfilingGroupRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProfilingGroupRequest {
    /// <p> The name of the profiling group to get information about. </p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the describeProfilingGroupResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProfilingGroupResponse {
    /// <p> The returned <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> object that contains information about the requested profiling group. </p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p> Information about potential recommendations that might be created from the analysis of profiling data. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FindingsReportSummary {
    /// <p>The universally unique identifier (UUID) of the recommendation report.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p> The end time of the period during which the metric is flagged as anomalous. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "profileEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_end_time: Option<f64>,
    /// <p>The start time of the profile the analysis data is about. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "profileStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_start_time: Option<f64>,
    /// <p>The name of the profiling group that is associated with the analysis data.</p>
    #[serde(rename = "profilingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_group_name: Option<String>,
    /// <p>The total number of different recommendations that were found by the analysis.</p>
    #[serde(rename = "totalNumberOfFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_findings: Option<i64>,
}

/// <p> The frame name, metric type, and thread states. These are used to derive the value of the metric for the frame.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameMetric {
    /// <p> Name of the method common across the multiple occurrences of a frame in an application profile.</p>
    #[serde(rename = "frameName")]
    pub frame_name: String,
    /// <p>List of application runtime thread states used to get the counts for a frame a derive a metric value.</p>
    #[serde(rename = "threadStates")]
    pub thread_states: Vec<String>,
    /// <p> A type of aggregation that specifies how a metric for a frame is analyzed. The supported value <code>AggregatedRelativeTotalTime</code> is an aggregation of the metric value for one frame that is calculated across the occurrences of all frames in a profile. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p> Information about a frame metric and its values. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FrameMetricDatum {
    #[serde(rename = "frameMetric")]
    pub frame_metric: FrameMetric,
    /// <p> A list of values that are associated with a frame metric. </p>
    #[serde(rename = "values")]
    pub values: Vec<f64>,
}

/// <p>The structure representing the GetFindingsReportAccountSummaryRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsReportAccountSummaryRequest {
    /// <p>A <code>Boolean</code> value indicating whether to only return reports from daily profiles. If set to <code>True</code>, only analysis data from daily profiles is returned. If set to <code>False</code>, analysis data is returned from smaller time windows (for example, one hour).</p>
    #[serde(rename = "dailyReportsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_reports_only: Option<bool>,
    /// <p>The maximum number of results returned by <code> GetFindingsReportAccountSummary</code> in paginated output. When this parameter is used, <code>GetFindingsReportAccountSummary</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>GetFindingsReportAccountSummary</code> request with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>GetFindingsReportAccountSummary</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The structure representing the GetFindingsReportAccountSummaryResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingsReportAccountSummaryResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>GetFindingsReportAccountSummary</code> request. When the results of a <code>GetFindingsReportAccountSummary</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The return list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_FindingsReportSummary.html"> <code>FindingsReportSummary</code> </a> objects taht contain summaries of analysis results for all profiling groups in your AWS account.</p>
    #[serde(rename = "reportSummaries")]
    pub report_summaries: Vec<FindingsReportSummary>,
}

/// <p>The structure representing the GetNotificationConfigurationRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNotificationConfigurationRequest {
    /// <p>The name of the profiling group we want to get the notification configuration for.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the GetNotificationConfigurationResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNotificationConfigurationResponse {
    /// <p>The current notification configuration for this profiling group.</p>
    #[serde(rename = "notificationConfiguration")]
    pub notification_configuration: NotificationConfiguration,
}

/// <p> The structure representing the <code>getPolicyRequest</code>. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPolicyRequest {
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the <code>getPolicyResponse</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPolicyResponse {
    /// <p>The JSON-formatted resource-based policy attached to the <code>ProfilingGroup</code>.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>A unique identifier for the current revision of the returned policy.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the getProfileRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProfileRequest {
    /// <p><p> The format of the returned profiling data. The format maps to the <code>Accept</code> and <code>Content-Type</code> headers of the HTTP request. You can specify one of the following: or the default . </p> <pre><code> &lt;ul&gt; &lt;li&gt; &lt;p&gt; &lt;code&gt;application/json&lt;/code&gt; — standard JSON format &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; &lt;code&gt;application/x-amzn-ion&lt;/code&gt; — the Amazon Ion data format. For more information, see &lt;a href=&quot;http://amzn.github.io/ion-docs/&quot;&gt;Amazon Ion&lt;/a&gt;. &lt;/p&gt; &lt;/li&gt; &lt;/ul&gt; </code></pre></p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p> The end time of the requested profile. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p> <p> If you specify <code>endTime</code>, then you must also specify <code>period</code> or <code>startTime</code>, but not both. </p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The maximum depth of the stacks in the code that is represented in the aggregated profile. For example, if CodeGuru Profiler finds a method <code>A</code>, which calls method <code>B</code>, which calls method <code>C</code>, which calls method <code>D</code>, then the depth is 4. If the <code>maxDepth</code> is set to 2, then the aggregated profile contains representations of methods <code>A</code> and <code>B</code>. </p>
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    /// <p><p> Used with <code>startTime</code> or <code>endTime</code> to specify the time range for the returned aggregated profile. Specify using the ISO 8601 format. For example, <code>P1DT1H1M1S</code>. </p> <pre><code> &lt;p&gt; To get the latest aggregated profile, specify only &lt;code&gt;period&lt;/code&gt;. &lt;/p&gt; </code></pre></p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p>The name of the profiling group to get.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p><p>The start time of the profile to get. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p> <pre><code> &lt;p&gt; If you specify &lt;code&gt;startTime&lt;/code&gt;, then you must also specify &lt;code&gt;period&lt;/code&gt; or &lt;code&gt;endTime&lt;/code&gt;, but not both. &lt;/p&gt; </code></pre></p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>The structure representing the getProfileResponse.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetProfileResponse {
    /// <p>The content encoding of the profile.</p>
    pub content_encoding: Option<String>,
    /// <p>The content type of the profile in the payload. It is either <code>application/json</code> or the default <code>application/x-amzn-ion</code>.</p>
    pub content_type: String,
    /// <p>Information about the profile.</p>
    pub profile: bytes::Bytes,
}

/// <p>The structure representing the GetRecommendationsRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecommendationsRequest {
    /// <p> The start time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p><p> The language used to provide analysis. Specify using a string that is one of the following <code>BCP 47</code> language codes. </p> <ul> <li> <p> <code>de-DE</code> - German, Germany </p> </li> <li> <p> <code>en-GB</code> - English, United Kingdom </p> </li> <li> <p> <code>en-US</code> - English, United States </p> </li> <li> <p> <code>es-ES</code> - Spanish, Spain </p> </li> <li> <p> <code>fr-FR</code> - French, France </p> </li> <li> <p> <code>it-IT</code> - Italian, Italy </p> </li> <li> <p> <code>ja-JP</code> - Japanese, Japan </p> </li> <li> <p> <code>ko-KR</code> - Korean, Republic of Korea </p> </li> <li> <p> <code>pt-BR</code> - Portugese, Brazil </p> </li> <li> <p> <code>zh-CN</code> - Chinese, China </p> </li> <li> <p> <code>zh-TW</code> - Chinese, Taiwan </p> </li> </ul></p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p> The name of the profiling group to get analysis data about. </p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> The end time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

/// <p>The structure representing the GetRecommendationsResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationsResponse {
    /// <p> The list of anomalies that the analysis has found for this profile. </p>
    #[serde(rename = "anomalies")]
    pub anomalies: Vec<Anomaly>,
    /// <p> The end time of the profile the analysis data is about. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "profileEndTime")]
    pub profile_end_time: f64,
    /// <p> The start time of the profile the analysis data is about. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "profileStartTime")]
    pub profile_start_time: f64,
    /// <p>The name of the profiling group the analysis data is about.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The list of recommendations that the analysis found for this profile.</p>
    #[serde(rename = "recommendations")]
    pub recommendations: Vec<Recommendation>,
}

/// <p>The structure representing the ListFindingsReportsRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFindingsReportsRequest {
    /// <p>A <code>Boolean</code> value indicating whether to only return reports from daily profiles. If set to <code>True</code>, only analysis data from daily profiles is returned. If set to <code>False</code>, analysis data is returned from smaller time windows (for example, one hour).</p>
    #[serde(rename = "dailyReportsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_reports_only: Option<bool>,
    /// <p> The end time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The maximum number of report results returned by <code>ListFindingsReports</code> in paginated output. When this parameter is used, <code>ListFindingsReports</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListFindingsReports</code> request with the returned <code>nextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListFindingsReportsRequest</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the profiling group from which to search for analysis data.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> The start time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

/// <p>The structure representing the ListFindingsReportsResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFindingsReportsResponse {
    /// <p>The list of analysis results summaries.</p>
    #[serde(rename = "findingsReportSummaries")]
    pub findings_report_summaries: Vec<FindingsReportSummary>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListFindingsReports</code> request. When the results of a <code>ListFindingsReports</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The structure representing the listProfileTimesRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfileTimesRequest {
    /// <p>The end time of the time range from which to list the profiles.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The maximum number of profile time results returned by <code>ListProfileTimes</code> in paginated output. When this parameter is used, <code>ListProfileTimes</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListProfileTimes</code> request with the returned <code>nextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListProfileTimes</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order (ascending or descending by start time of the profile) to use when listing profiles. Defaults to <code>TIMESTAMP_DESCENDING</code>. </p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// <p><p> The aggregation period. This specifies the period during which an aggregation profile collects posted agent profiles for a profiling group. There are 3 valid values. </p> <ul> <li> <p> <code>P1D</code> — 1 day </p> </li> <li> <p> <code>PT1H</code> — 1 hour </p> </li> <li> <p> <code>PT5M</code> — 5 minutes </p> </li> </ul></p>
    #[serde(rename = "period")]
    pub period: String,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The start time of the time range from which to list the profiles.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

/// <p>The structure representing the listProfileTimesResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfileTimesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListProfileTimes</code> request. When the results of a <code>ListProfileTimes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of start times of the available profiles for the aggregation period in the specified time range. </p>
    #[serde(rename = "profileTimes")]
    pub profile_times: Vec<ProfileTime>,
}

/// <p>The structure representing the listProfilingGroupsRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfilingGroupsRequest {
    /// <p>A <code>Boolean</code> value indicating whether to include a description. If <code>true</code>, then a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects that contain detailed information about profiling groups is returned. If <code>false</code>, then a list of profiling group names is returned.</p>
    #[serde(rename = "includeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_description: Option<bool>,
    /// <p>The maximum number of profiling groups results returned by <code>ListProfilingGroups</code> in paginated output. When this parameter is used, <code>ListProfilingGroups</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListProfilingGroups</code> request with the returned <code>nextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListProfilingGroups</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The structure representing the listProfilingGroupsResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfilingGroupsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListProfilingGroups</code> request. When the results of a <code>ListProfilingGroups</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A returned list of profiling group names. A list of the names is returned only if <code>includeDescription</code> is <code>false</code>, otherwise a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects is returned. </p>
    #[serde(rename = "profilingGroupNames")]
    pub profiling_group_names: Vec<String>,
    /// <p> A returned list <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects. A list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects is returned only if <code>includeDescription</code> is <code>true</code>, otherwise a list of profiling group names is returned. </p>
    #[serde(rename = "profilingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_groups: Option<Vec<ProfilingGroupDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> The Amazon Resource Name (ARN) of the resource that contains the tags to return. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> The list of tags assigned to the specified resource. This is the list of tags returned in the response. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The part of a profile that contains a recommendation found during analysis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Match {
    /// <p>The location in the profiling graph that contains a recommendation found during analysis.</p>
    #[serde(rename = "frameAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_address: Option<String>,
    /// <p>The target frame that triggered a match.</p>
    #[serde(rename = "targetFramesIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_frames_index: Option<i64>,
    /// <p>The value in the profile data that exceeded the recommendation threshold.</p>
    #[serde(rename = "thresholdBreachValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_breach_value: Option<f64>,
}

/// <p> Details about the metric that the analysis used when it detected the anomaly. The metric what is analyzed to create recommendations. It includes the name of the frame that was analyzed and the type and thread states used to derive the metric value for that frame. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Metric {
    /// <p> The name of the method that appears as a frame in any stack in a profile. </p>
    #[serde(rename = "frameName")]
    pub frame_name: String,
    /// <p> The list of application runtime thread states that is used to calculate the metric value for the frame. </p>
    #[serde(rename = "threadStates")]
    pub thread_states: Vec<String>,
    /// <p> A type that specifies how a metric for a frame is analyzed. The supported value <code>AggregatedRelativeTotalTime</code> is an aggregation of the metric value for one frame that is calculated across the occurences of all frames in a profile.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>The configuration for notifications stored for each profiling group. This includes up to to two channels and a list of event publishers associated with each channel.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotificationConfiguration {
    /// <p>List of up to two channels to be used for sending notifications for events detected from the application profile.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
}

/// <p> A set of rules used to make a recommendation during an analysis. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Pattern {
    /// <p> A list of the different counters used to determine if there is a match. </p>
    #[serde(rename = "countersToAggregate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters_to_aggregate: Option<Vec<String>>,
    /// <p>The description of the recommendation. This explains a potential inefficiency in a profiled application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The universally unique identifier (UUID) of this pattern.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name for this pattern.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> A string that contains the steps recommended to address the potential inefficiency. </p>
    #[serde(rename = "resolutionSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_steps: Option<String>,
    /// <p>A list of frame names that were searched during the analysis that generated a recommendation.</p>
    #[serde(rename = "targetFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_frames: Option<Vec<Vec<String>>>,
    /// <p> The percentage of time an application spends in one method that triggers a recommendation. The percentage of time is the same as the percentage of the total gathered sample counts during analysis. </p>
    #[serde(rename = "thresholdPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_percent: Option<f64>,
}

/// <p>The structure representing the postAgentProfileRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostAgentProfileRequest {
    /// <p> The submitted profiling data. </p>
    #[serde(rename = "agentProfile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub agent_profile: bytes::Bytes,
    /// <p><p> The format of the submitted profiling data. The format maps to the <code>Accept</code> and <code>Content-Type</code> headers of the HTTP request. You can specify one of the following: or the default . </p> <pre><code> &lt;ul&gt; &lt;li&gt; &lt;p&gt; &lt;code&gt;application/json&lt;/code&gt; — standard JSON format &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; &lt;code&gt;application/x-amzn-ion&lt;/code&gt; — the Amazon Ion data format. For more information, see &lt;a href=&quot;http://amzn.github.io/ion-docs/&quot;&gt;Amazon Ion&lt;/a&gt;. &lt;/p&gt; &lt;/li&gt; &lt;/ul&gt; </code></pre></p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p> Amazon CodeGuru Profiler uses this universally unique identifier (UUID) to prevent the accidental submission of duplicate profiling data if there are failures and retries. </p>
    #[serde(rename = "profileToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_token: Option<String>,
    /// <p> The name of the profiling group with the aggregated profile that receives the submitted profiling data. </p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the postAgentProfileResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostAgentProfileResponse {}

/// <p> Contains the start time of a profile. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfileTime {
    /// <p>The start time of a profile. It is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p> Contains information about a profiling group. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfilingGroupDescription {
    /// <p> An <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_AgentOrchestrationConfig.html"> <code>AgentOrchestrationConfig</code> </a> object that indicates if the profiling group is enabled for profiled or not. </p>
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    /// <p>The Amazon Resource Name (ARN) identifying the profiling group resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p> The compute platform of the profiling group. If it is set to <code>AWSLambda</code>, then the profiled application runs on AWS Lambda. If it is set to <code>Default</code>, then the profiled application runs on a compute platform that is not AWS Lambda, such an Amazon EC2 instance, an on-premises server, or a different platform. The default is <code>Default</code>. </p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The time when the profiling group was created. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> A <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingStatus.html"> <code>ProfilingStatus</code> </a> object that includes information about the last time a profile agent pinged back, the last time a profile was received, and the aggregation period and start time for the most recent aggregated profile. </p>
    #[serde(rename = "profilingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_status: Option<ProfilingStatus>,
    /// <p> A list of the tags that belong to this profiling group. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p> The date and time when the profiling group was last updated. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p> Profiling status includes information about the last time a profile agent pinged back, the last time a profile was received, and the aggregation period and start time for the most recent aggregated profile. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfilingStatus {
    /// <p>The date and time when the profiling agent most recently pinged back. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "latestAgentOrchestratedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_orchestrated_at: Option<f64>,
    /// <p>The date and time when the most recent profile was received. Specify using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "latestAgentProfileReportedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_profile_reported_at: Option<f64>,
    /// <p> An <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_AggregatedProfileTime.html"> <code>AggregatedProfileTime</code> </a> object that contains the aggregation period and start time for an aggregated profile. </p>
    #[serde(rename = "latestAggregatedProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_aggregated_profile: Option<AggregatedProfileTime>,
}

/// <p>The structure representing the <code>putPermissionRequest</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPermissionRequest {
    /// <p> Specifies an action group that contains permissions to add to a profiling group resource. One action group is supported, <code>agentPermissions</code>, which grants permission to perform actions required by the profiling agent, <code>ConfigureAgent</code> and <code>PostAgentProfile</code> permissions. </p>
    #[serde(rename = "actionGroup")]
    pub action_group: String,
    /// <p> A list ARNs for the roles and users you want to grant access to the profiling group. Wildcards are not are supported in the ARNs. </p>
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// <p>The name of the profiling group to grant access to.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> A universally unique identifier (UUID) for the revision of the policy you are adding to the profiling group. Do not specify this when you add permissions to a profiling group for the first time. If a policy already exists on the profiling group, you must specify the <code>revisionId</code>. </p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// <p>The structure representing the <code>putPermissionResponse</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPermissionResponse {
    /// <p> The JSON-formatted resource-based policy on the profiling group that includes the added permissions. </p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p> A universally unique identifier (UUID) for the revision of the resource-based policy that includes the added permissions. The JSON-formatted policy is in the <code>policy</code> element of the response. </p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>A potential improvement that was found from analyzing the profiling data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Recommendation {
    /// <p>How many different places in the profile graph triggered a match.</p>
    #[serde(rename = "allMatchesCount")]
    pub all_matches_count: i64,
    /// <p>How much of the total sample count is potentially affected.</p>
    #[serde(rename = "allMatchesSum")]
    pub all_matches_sum: f64,
    /// <p>End time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The pattern that analysis recognized in the profile to make this recommendation.</p>
    #[serde(rename = "pattern")]
    pub pattern: Pattern,
    /// <p>The start time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>List of the matches with most impact. </p>
    #[serde(rename = "topMatches")]
    pub top_matches: Vec<Match>,
}

/// <p>The structure representing the RemoveNotificationChannelRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveNotificationChannelRequest {
    /// <p>The id of the channel that we want to stop receiving notifications.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// <p>The name of the profiling group we want to change notification configuration for.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the RemoveNotificationChannelResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveNotificationChannelResponse {
    /// <p>The new notification configuration for this profiling group.</p>
    #[serde(rename = "notificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
}

/// <p><p> <pre><code> The structure representing the &lt;code&gt;removePermissionRequest&lt;/code&gt;.&lt;/p&gt; </code></pre></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemovePermissionRequest {
    /// <p> Specifies an action group that contains the permissions to remove from a profiling group's resource-based policy. One action group is supported, <code>agentPermissions</code>, which grants <code>ConfigureAgent</code> and <code>PostAgentProfile</code> permissions. </p>
    #[serde(rename = "actionGroup")]
    pub action_group: String,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> A universally unique identifier (UUID) for the revision of the resource-based policy from which you want to remove permissions. </p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the <code>removePermissionResponse</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemovePermissionResponse {
    /// <p> The JSON-formatted resource-based policy on the profiling group after the specified permissions were removed. </p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p> A universally unique identifier (UUID) for the revision of the resource-based policy after the specified permissions were removed. The updated JSON-formatted policy is in the <code>policy</code> element of the response. </p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the SubmitFeedbackRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitFeedbackRequest {
    /// <p>The universally unique identifier (UUID) of the <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_AnomalyInstance.html"> <code>AnomalyInstance</code> </a> object that is included in the analysis data.</p>
    #[serde(rename = "anomalyInstanceId")]
    pub anomaly_instance_id: String,
    /// <p>Optional feedback about this anomaly.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the profiling group that is associated with the analysis data.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p> The feedback tpye. Thee are two valid values, <code>Positive</code> and <code>Negative</code>. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>The structure representing the SubmitFeedbackResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitFeedbackResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> The Amazon Resource Name (ARN) of the resource that the tags are added to. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> The list of tags that are added to the specified resource. </p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p> A data type that contains a <code>Timestamp</code> object. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TimestampStructure {
    /// <p> A <code>Timestamp</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    #[serde(rename = "value")]
    pub value: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> The Amazon Resource Name (ARN) of the resource that contains the tags to remove. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> A list of tag keys. Existing tags of resources with keys in this list are removed from the specified resource. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>The structure representing the updateProfilingGroupRequest.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProfilingGroupRequest {
    /// <p> Specifies whether profiling is enabled or disabled for a profiling group. </p>
    #[serde(rename = "agentOrchestrationConfig")]
    pub agent_orchestration_config: AgentOrchestrationConfig,
    /// <p>The name of the profiling group to update.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the updateProfilingGroupResponse.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProfilingGroupResponse {
    /// <p> A <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> that contains information about the returned updated profiling group. </p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>Feedback that can be submitted for each instance of an anomaly by the user. Feedback is be used for improvements in generating recommendations for the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserFeedback {
    /// <p>Optional <code>Positive</code> or <code>Negative</code> feedback submitted by the user about whether the recommendation is useful or not.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// Errors returned by AddNotificationChannels
#[derive(Debug, PartialEq)]
pub enum AddNotificationChannelsError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/intro.html">Service Quotas</a> to request a service quota increase. </p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AddNotificationChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddNotificationChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(AddNotificationChannelsError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AddNotificationChannelsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddNotificationChannelsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        AddNotificationChannelsError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AddNotificationChannelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddNotificationChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddNotificationChannelsError::Conflict(ref cause) => write!(f, "{}", cause),
            AddNotificationChannelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            AddNotificationChannelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddNotificationChannelsError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AddNotificationChannelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddNotificationChannelsError {}
/// Errors returned by BatchGetFrameMetricData
#[derive(Debug, PartialEq)]
pub enum BatchGetFrameMetricDataError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl BatchGetFrameMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetFrameMetricDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchGetFrameMetricDataError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BatchGetFrameMetricDataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchGetFrameMetricDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetFrameMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetFrameMetricDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchGetFrameMetricDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            BatchGetFrameMetricDataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetFrameMetricDataError {}
/// Errors returned by ConfigureAgent
#[derive(Debug, PartialEq)]
pub enum ConfigureAgentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ConfigureAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfigureAgentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ConfigureAgentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfigureAgentError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ConfigureAgentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfigureAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureAgentError::InternalServer(ref cause) => write!(f, "{}", cause),
            ConfigureAgentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ConfigureAgentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfigureAgentError {}
/// Errors returned by CreateProfilingGroup
#[derive(Debug, PartialEq)]
pub enum CreateProfilingGroupError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/intro.html">Service Quotas</a> to request a service quota increase. </p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateProfilingGroupError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateProfilingGroupError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateProfilingGroupError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProfilingGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProfilingGroupError {}
/// Errors returned by DeleteProfilingGroup
#[derive(Debug, PartialEq)]
pub enum DeleteProfilingGroupError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProfilingGroupError {}
/// Errors returned by DescribeProfilingGroup
#[derive(Debug, PartialEq)]
pub enum DescribeProfilingGroupError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProfilingGroupError {}
/// Errors returned by GetFindingsReportAccountSummary
#[derive(Debug, PartialEq)]
pub enum GetFindingsReportAccountSummaryError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetFindingsReportAccountSummaryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFindingsReportAccountSummaryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetFindingsReportAccountSummaryError::InternalServer(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFindingsReportAccountSummaryError::Throttling(
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
impl fmt::Display for GetFindingsReportAccountSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingsReportAccountSummaryError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFindingsReportAccountSummaryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingsReportAccountSummaryError {}
/// Errors returned by GetNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetNotificationConfigurationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetNotificationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetNotificationConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetNotificationConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetNotificationConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetNotificationConfigurationError::Throttling(
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
impl fmt::Display for GetNotificationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNotificationConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetNotificationConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetNotificationConfigurationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNotificationConfigurationError {}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetPolicyError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPolicyError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPolicyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetPolicyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPolicyError {}
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetProfileError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProfileError {}
/// Errors returned by GetRecommendations
#[derive(Debug, PartialEq)]
pub enum GetRecommendationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetRecommendationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRecommendationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetRecommendationsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRecommendationsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetRecommendationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecommendationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetRecommendationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecommendationsError {}
/// Errors returned by ListFindingsReports
#[derive(Debug, PartialEq)]
pub enum ListFindingsReportsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListFindingsReportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFindingsReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListFindingsReportsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFindingsReportsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFindingsReportsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFindingsReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFindingsReportsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListFindingsReportsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFindingsReportsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFindingsReportsError {}
/// Errors returned by ListProfileTimes
#[derive(Debug, PartialEq)]
pub enum ListProfileTimesError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListProfileTimesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProfileTimesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProfileTimesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProfileTimesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProfileTimesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProfileTimesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProfileTimesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProfileTimesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListProfileTimesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProfileTimesError {}
/// Errors returned by ListProfilingGroups
#[derive(Debug, PartialEq)]
pub enum ListProfilingGroupsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListProfilingGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProfilingGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProfilingGroupsError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProfilingGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProfilingGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProfilingGroupsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProfilingGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProfilingGroupsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PostAgentProfile
#[derive(Debug, PartialEq)]
pub enum PostAgentProfileError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PostAgentProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostAgentProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PostAgentProfileError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PostAgentProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PostAgentProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PostAgentProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostAgentProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            PostAgentProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PostAgentProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PostAgentProfileError {}
/// Errors returned by PutPermission
#[derive(Debug, PartialEq)]
pub enum PutPermissionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PutPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(PutPermissionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutPermissionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutPermissionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutPermissionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPermissionError::Conflict(ref cause) => write!(f, "{}", cause),
            PutPermissionError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutPermissionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPermissionError {}
/// Errors returned by RemoveNotificationChannel
#[derive(Debug, PartialEq)]
pub enum RemoveNotificationChannelError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl RemoveNotificationChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveNotificationChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(RemoveNotificationChannelError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveNotificationChannelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RemoveNotificationChannelError::Throttling(
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
impl fmt::Display for RemoveNotificationChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveNotificationChannelError::InternalServer(ref cause) => write!(f, "{}", cause),
            RemoveNotificationChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemoveNotificationChannelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveNotificationChannelError {}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(RemovePermissionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(RemovePermissionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemovePermissionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RemovePermissionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::Conflict(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::InternalServer(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
/// Errors returned by SubmitFeedback
#[derive(Debug, PartialEq)]
pub enum SubmitFeedbackError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SubmitFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitFeedbackError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(SubmitFeedbackError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SubmitFeedbackError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SubmitFeedbackError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitFeedbackError::InternalServer(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitFeedbackError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
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
        match *self {
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
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
        match *self {
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateProfilingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateProfilingGroupError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProfilingGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProfilingGroupError {}
/// Trait representing the capabilities of the Amazon CodeGuru Profiler API. Amazon CodeGuru Profiler clients implement this trait.
#[async_trait]
pub trait CodeGuruProfiler {
    /// <p>Add up to 2 anomaly notifications channels for a profiling group.</p>
    async fn add_notification_channels(
        &self,
        input: AddNotificationChannelsRequest,
    ) -> Result<AddNotificationChannelsResponse, RusotoError<AddNotificationChannelsError>>;

    /// <p> Returns the time series of values for a requested list of frame metrics from a time period.</p>
    async fn batch_get_frame_metric_data(
        &self,
        input: BatchGetFrameMetricDataRequest,
    ) -> Result<BatchGetFrameMetricDataResponse, RusotoError<BatchGetFrameMetricDataError>>;

    /// <p> Used by profiler agents to report their current state and to receive remote configuration updates. For example, <code>ConfigureAgent</code> can be used to tell and agent whether to profile or not and for how long to return profiling data. </p>
    async fn configure_agent(
        &self,
        input: ConfigureAgentRequest,
    ) -> Result<ConfigureAgentResponse, RusotoError<ConfigureAgentError>>;

    /// <p>Creates a profiling group.</p>
    async fn create_profiling_group(
        &self,
        input: CreateProfilingGroupRequest,
    ) -> Result<CreateProfilingGroupResponse, RusotoError<CreateProfilingGroupError>>;

    /// <p>Deletes a profiling group.</p>
    async fn delete_profiling_group(
        &self,
        input: DeleteProfilingGroupRequest,
    ) -> Result<DeleteProfilingGroupResponse, RusotoError<DeleteProfilingGroupError>>;

    /// <p> Returns a <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> object that contains information about the requested profiling group. </p>
    async fn describe_profiling_group(
        &self,
        input: DescribeProfilingGroupRequest,
    ) -> Result<DescribeProfilingGroupResponse, RusotoError<DescribeProfilingGroupError>>;

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_FindingsReportSummary.html"> <code>FindingsReportSummary</code> </a> objects that contain analysis results for all profiling groups in your AWS account. </p>
    async fn get_findings_report_account_summary(
        &self,
        input: GetFindingsReportAccountSummaryRequest,
    ) -> Result<
        GetFindingsReportAccountSummaryResponse,
        RusotoError<GetFindingsReportAccountSummaryError>,
    >;

    /// <p>Get the current configuration for anomaly notifications for a profiling group.</p>
    async fn get_notification_configuration(
        &self,
        input: GetNotificationConfigurationRequest,
    ) -> Result<GetNotificationConfigurationResponse, RusotoError<GetNotificationConfigurationError>>;

    /// <p> Returns the JSON-formatted resource-based policy on a profiling group. </p>
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>>;

    /// <p><p> Gets the aggregated profile of a profiling group for a specified time range. Amazon CodeGuru Profiler collects posted agent profiles for a profiling group into aggregated profiles. </p> <pre><code> &lt;note&gt; &lt;p&gt; Because aggregated profiles expire over time &lt;code&gt;GetProfile&lt;/code&gt; is not idempotent. &lt;/p&gt; &lt;/note&gt; &lt;p&gt; Specify the time range for the requested aggregated profile using 1 or 2 of the following parameters: &lt;code&gt;startTime&lt;/code&gt;, &lt;code&gt;endTime&lt;/code&gt;, &lt;code&gt;period&lt;/code&gt;. The maximum time range allowed is 7 days. If you specify all 3 parameters, an exception is thrown. If you specify only &lt;code&gt;period&lt;/code&gt;, the latest aggregated profile is returned. &lt;/p&gt; &lt;p&gt; Aggregated profiles are available with aggregation periods of 5 minutes, 1 hour, and 1 day, aligned to UTC. The aggregation period of an aggregated profile determines how long it is retained. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API<em>AggregatedProfileTime.html&quot;&gt; &lt;code&gt;AggregatedProfileTime&lt;/code&gt; &lt;/a&gt;. The aggregated profile&#39;s aggregation period determines how long it is retained by CodeGuru Profiler. &lt;/p&gt; &lt;ul&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 5 minutes, the aggregated profile is retained for 15 days. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 1 hour, the aggregated profile is retained for 60 days. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 1 day, the aggregated profile is retained for 3 years. &lt;/p&gt; &lt;/li&gt; &lt;/ul&gt; &lt;p&gt;There are two use cases for calling &lt;code&gt;GetProfile&lt;/code&gt;.&lt;/p&gt; &lt;ol&gt; &lt;li&gt; &lt;p&gt; If you want to return an aggregated profile that already exists, use &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API</em>ListProfileTimes.html&quot;&gt; &lt;code&gt;ListProfileTimes&lt;/code&gt; &lt;/a&gt; to view the time ranges of existing aggregated profiles. Use them in a &lt;code&gt;GetProfile&lt;/code&gt; request to return a specific, existing aggregated profile. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If you want to return an aggregated profile for a time range that doesn&#39;t align with an existing aggregated profile, then CodeGuru Profiler makes a best effort to combine existing aggregated profiles from the requested time range and return them as one aggregated profile. &lt;/p&gt; &lt;p&gt; If aggregated profiles do not exist for the full time range requested, then aggregated profiles for a smaller time range are returned. For example, if the requested time range is from 00:00 to 00:20, and the existing aggregated profiles are from 00:15 and 00:25, then the aggregated profiles from 00:15 to 00:20 are returned. &lt;/p&gt; &lt;/li&gt; &lt;/ol&gt; </code></pre></p>
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>>;

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_Recommendation.html"> <code>Recommendation</code> </a> objects that contain recommendations for a profiling group for a given time period. A list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_Anomaly.html"> <code>Anomaly</code> </a> objects that contains details about anomalies detected in the profiling group for the same time period is also returned. </p>
    async fn get_recommendations(
        &self,
        input: GetRecommendationsRequest,
    ) -> Result<GetRecommendationsResponse, RusotoError<GetRecommendationsError>>;

    /// <p>List the available reports for a given profiling group and time range.</p>
    async fn list_findings_reports(
        &self,
        input: ListFindingsReportsRequest,
    ) -> Result<ListFindingsReportsResponse, RusotoError<ListFindingsReportsError>>;

    /// <p>Lists the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
    async fn list_profile_times(
        &self,
        input: ListProfileTimesRequest,
    ) -> Result<ListProfileTimesResponse, RusotoError<ListProfileTimesError>>;

    /// <p> Returns a list of profiling groups. The profiling groups are returned as <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects. </p>
    async fn list_profiling_groups(
        &self,
        input: ListProfilingGroupsRequest,
    ) -> Result<ListProfilingGroupsResponse, RusotoError<ListProfilingGroupsError>>;

    /// <p> Returns a list of the tags that are assigned to a specified resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p> Submits profiling data to an aggregated profile of a profiling group. To get an aggregated profile that is created with this profiling data, use <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_GetProfile.html"> <code>GetProfile</code> </a>. </p>
    async fn post_agent_profile(
        &self,
        input: PostAgentProfileRequest,
    ) -> Result<PostAgentProfileResponse, RusotoError<PostAgentProfileError>>;

    /// <p><p> Adds permissions to a profiling group&#39;s resource-based policy that are provided using an action group. If a profiling group doesn&#39;t have a resource-based policy, one is created for it using the permissions in the action group and the roles and users in the <code>principals</code> parameter. </p> <pre><code> &lt;p&gt; The one supported action group that can be added is &lt;code&gt;agentPermission&lt;/code&gt; which grants &lt;code&gt;ConfigureAgent&lt;/code&gt; and &lt;code&gt;PostAgent&lt;/code&gt; permissions. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-ug/resource-based-policies.html&quot;&gt;Resource-based policies in CodeGuru Profiler&lt;/a&gt; in the &lt;i&gt;Amazon CodeGuru Profiler User Guide&lt;/i&gt;, &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API<em>ConfigureAgent.html&quot;&gt; &lt;code&gt;ConfigureAgent&lt;/code&gt; &lt;/a&gt;, and &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API</em>PostAgentProfile.html&quot;&gt; &lt;code&gt;PostAgentProfile&lt;/code&gt; &lt;/a&gt;. &lt;/p&gt; &lt;p&gt; The first time you call &lt;code&gt;PutPermission&lt;/code&gt; on a profiling group, do not specify a &lt;code&gt;revisionId&lt;/code&gt; because it doesn&#39;t have a resource-based policy. Subsequent calls must provide a &lt;code&gt;revisionId&lt;/code&gt; to specify which revision of the resource-based policy to add the permissions to. &lt;/p&gt; &lt;p&gt; The response contains the profiling group&#39;s JSON-formatted resource policy. &lt;/p&gt; </code></pre></p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<PutPermissionResponse, RusotoError<PutPermissionError>>;

    /// <p>Remove one anomaly notifications channel for a profiling group.</p>
    async fn remove_notification_channel(
        &self,
        input: RemoveNotificationChannelRequest,
    ) -> Result<RemoveNotificationChannelResponse, RusotoError<RemoveNotificationChannelError>>;

    /// <p> Removes permissions from a profiling group's resource-based policy that are provided using an action group. The one supported action group that can be removed is <code>agentPermission</code> which grants <code>ConfigureAgent</code> and <code>PostAgent</code> permissions. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-ug/resource-based-policies.html">Resource-based policies in CodeGuru Profiler</a> in the <i>Amazon CodeGuru Profiler User Guide</i>, <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a>, and <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_PostAgentProfile.html"> <code>PostAgentProfile</code> </a>. </p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<RemovePermissionResponse, RusotoError<RemovePermissionError>>;

    /// <p>Sends feedback to CodeGuru Profiler about whether the anomaly detected by the analysis is useful or not.</p>
    async fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Result<SubmitFeedbackResponse, RusotoError<SubmitFeedbackError>>;

    /// <p> Use to assign one or more tags to a resource. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p> Use to remove one or more tags from a resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates a profiling group.</p>
    async fn update_profiling_group(
        &self,
        input: UpdateProfilingGroupRequest,
    ) -> Result<UpdateProfilingGroupResponse, RusotoError<UpdateProfilingGroupError>>;
}
/// A client for the Amazon CodeGuru Profiler API.
#[derive(Clone)]
pub struct CodeGuruProfilerClient {
    client: Client,
    region: region::Region,
}

impl CodeGuruProfilerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeGuruProfilerClient {
        CodeGuruProfilerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeGuruProfilerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeGuruProfilerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeGuruProfilerClient {
        CodeGuruProfilerClient { client, region }
    }
}

#[async_trait]
impl CodeGuruProfiler for CodeGuruProfilerClient {
    /// <p>Add up to 2 anomaly notifications channels for a profiling group.</p>
    #[allow(unused_mut)]
    async fn add_notification_channels(
        &self,
        input: AddNotificationChannelsRequest,
    ) -> Result<AddNotificationChannelsResponse, RusotoError<AddNotificationChannelsError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/notificationConfiguration",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
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
                .deserialize::<AddNotificationChannelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddNotificationChannelsError::from_response(response))
        }
    }

    /// <p> Returns the time series of values for a requested list of frame metrics from a time period.</p>
    #[allow(unused_mut)]
    async fn batch_get_frame_metric_data(
        &self,
        input: BatchGetFrameMetricDataRequest,
    ) -> Result<BatchGetFrameMetricDataResponse, RusotoError<BatchGetFrameMetricDataError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/frames/-/metrics",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.period {
            params.put("period", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        if let Some(ref x) = input.target_resolution {
            params.put("targetResolution", x);
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
                .deserialize::<BatchGetFrameMetricDataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetFrameMetricDataError::from_response(response))
        }
    }

    /// <p> Used by profiler agents to report their current state and to receive remote configuration updates. For example, <code>ConfigureAgent</code> can be used to tell and agent whether to profile or not and for how long to return profiling data. </p>
    #[allow(unused_mut)]
    async fn configure_agent(
        &self,
        input: ConfigureAgentRequest,
    ) -> Result<ConfigureAgentResponse, RusotoError<ConfigureAgentError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/configureAgent",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
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
                .deserialize::<ConfigureAgentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ConfigureAgentError::from_response(response))
        }
    }

    /// <p>Creates a profiling group.</p>
    #[allow(unused_mut)]
    async fn create_profiling_group(
        &self,
        input: CreateProfilingGroupRequest,
    ) -> Result<CreateProfilingGroupResponse, RusotoError<CreateProfilingGroupError>> {
        let request_uri = "/profilingGroups";

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("clientToken", &input.client_token);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProfilingGroupError::from_response(response))
        }
    }

    /// <p>Deletes a profiling group.</p>
    #[allow(unused_mut)]
    async fn delete_profiling_group(
        &self,
        input: DeleteProfilingGroupRequest,
    ) -> Result<DeleteProfilingGroupResponse, RusotoError<DeleteProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProfilingGroupError::from_response(response))
        }
    }

    /// <p> Returns a <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> object that contains information about the requested profiling group. </p>
    #[allow(unused_mut)]
    async fn describe_profiling_group(
        &self,
        input: DescribeProfilingGroupRequest,
    ) -> Result<DescribeProfilingGroupResponse, RusotoError<DescribeProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProfilingGroupError::from_response(response))
        }
    }

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_FindingsReportSummary.html"> <code>FindingsReportSummary</code> </a> objects that contain analysis results for all profiling groups in your AWS account. </p>
    #[allow(unused_mut)]
    async fn get_findings_report_account_summary(
        &self,
        input: GetFindingsReportAccountSummaryRequest,
    ) -> Result<
        GetFindingsReportAccountSummaryResponse,
        RusotoError<GetFindingsReportAccountSummaryError>,
    > {
        let request_uri = "/internal/findingsReports";

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.daily_reports_only {
            params.put("dailyReportsOnly", x);
        }
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
                .deserialize::<GetFindingsReportAccountSummaryResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsReportAccountSummaryError::from_response(
                response,
            ))
        }
    }

    /// <p>Get the current configuration for anomaly notifications for a profiling group.</p>
    #[allow(unused_mut)]
    async fn get_notification_configuration(
        &self,
        input: GetNotificationConfigurationRequest,
    ) -> Result<GetNotificationConfigurationResponse, RusotoError<GetNotificationConfigurationError>>
    {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/notificationConfiguration",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetNotificationConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetNotificationConfigurationError::from_response(response))
        }
    }

    /// <p> Returns the JSON-formatted resource-based policy on a profiling group. </p>
    #[allow(unused_mut)]
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPolicyError::from_response(response))
        }
    }

    /// <p><p> Gets the aggregated profile of a profiling group for a specified time range. Amazon CodeGuru Profiler collects posted agent profiles for a profiling group into aggregated profiles. </p> <pre><code> &lt;note&gt; &lt;p&gt; Because aggregated profiles expire over time &lt;code&gt;GetProfile&lt;/code&gt; is not idempotent. &lt;/p&gt; &lt;/note&gt; &lt;p&gt; Specify the time range for the requested aggregated profile using 1 or 2 of the following parameters: &lt;code&gt;startTime&lt;/code&gt;, &lt;code&gt;endTime&lt;/code&gt;, &lt;code&gt;period&lt;/code&gt;. The maximum time range allowed is 7 days. If you specify all 3 parameters, an exception is thrown. If you specify only &lt;code&gt;period&lt;/code&gt;, the latest aggregated profile is returned. &lt;/p&gt; &lt;p&gt; Aggregated profiles are available with aggregation periods of 5 minutes, 1 hour, and 1 day, aligned to UTC. The aggregation period of an aggregated profile determines how long it is retained. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API<em>AggregatedProfileTime.html&quot;&gt; &lt;code&gt;AggregatedProfileTime&lt;/code&gt; &lt;/a&gt;. The aggregated profile&#39;s aggregation period determines how long it is retained by CodeGuru Profiler. &lt;/p&gt; &lt;ul&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 5 minutes, the aggregated profile is retained for 15 days. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 1 hour, the aggregated profile is retained for 60 days. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If the aggregation period is 1 day, the aggregated profile is retained for 3 years. &lt;/p&gt; &lt;/li&gt; &lt;/ul&gt; &lt;p&gt;There are two use cases for calling &lt;code&gt;GetProfile&lt;/code&gt;.&lt;/p&gt; &lt;ol&gt; &lt;li&gt; &lt;p&gt; If you want to return an aggregated profile that already exists, use &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API</em>ListProfileTimes.html&quot;&gt; &lt;code&gt;ListProfileTimes&lt;/code&gt; &lt;/a&gt; to view the time ranges of existing aggregated profiles. Use them in a &lt;code&gt;GetProfile&lt;/code&gt; request to return a specific, existing aggregated profile. &lt;/p&gt; &lt;/li&gt; &lt;li&gt; &lt;p&gt; If you want to return an aggregated profile for a time range that doesn&#39;t align with an existing aggregated profile, then CodeGuru Profiler makes a best effort to combine existing aggregated profiles from the requested time range and return them as one aggregated profile. &lt;/p&gt; &lt;p&gt; If aggregated profiles do not exist for the full time range requested, then aggregated profiles for a smaller time range are returned. For example, if the requested time range is from 00:00 to 00:20, and the existing aggregated profiles are from 00:15 and 00:25, then the aggregated profiles from 00:15 to 00:20 are returned. &lt;/p&gt; &lt;/li&gt; &lt;/ol&gt; </code></pre></p>
    #[allow(unused_mut)]
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/profile",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Accept", input.accept.as_ref());
        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.max_depth {
            params.put("maxDepth", x);
        }
        if let Some(ref x) = input.period {
            params.put("period", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetProfileResponse::default();
            result.profile = response.body;

            result.content_encoding = response.headers.remove("Content-Encoding");
            let value = response.headers.remove("Content-Type").unwrap();
            result.content_type = value;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetProfileError::from_response(response))
        }
    }

    /// <p> Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_Recommendation.html"> <code>Recommendation</code> </a> objects that contain recommendations for a profiling group for a given time period. A list of <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_Anomaly.html"> <code>Anomaly</code> </a> objects that contains details about anomalies detected in the profiling group for the same time period is also returned. </p>
    #[allow(unused_mut)]
    async fn get_recommendations(
        &self,
        input: GetRecommendationsRequest,
    ) -> Result<GetRecommendationsResponse, RusotoError<GetRecommendationsError>> {
        let request_uri = format!(
            "/internal/profilingGroups/{profiling_group_name}/recommendations",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("endTime", &input.end_time);
        if let Some(ref x) = input.locale {
            params.put("locale", x);
        }
        params.put("startTime", &input.start_time);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRecommendationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRecommendationsError::from_response(response))
        }
    }

    /// <p>List the available reports for a given profiling group and time range.</p>
    #[allow(unused_mut)]
    async fn list_findings_reports(
        &self,
        input: ListFindingsReportsRequest,
    ) -> Result<ListFindingsReportsResponse, RusotoError<ListFindingsReportsError>> {
        let request_uri = format!(
            "/internal/profilingGroups/{profiling_group_name}/findingsReports",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.daily_reports_only {
            params.put("dailyReportsOnly", x);
        }
        params.put("endTime", &input.end_time);
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("startTime", &input.start_time);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFindingsReportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFindingsReportsError::from_response(response))
        }
    }

    /// <p>Lists the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
    #[allow(unused_mut)]
    async fn list_profile_times(
        &self,
        input: ListProfileTimesRequest,
    ) -> Result<ListProfileTimesResponse, RusotoError<ListProfileTimesError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/profileTimes",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("endTime", &input.end_time);
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order_by {
            params.put("orderBy", x);
        }
        params.put("period", &input.period);
        params.put("startTime", &input.start_time);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProfileTimesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfileTimesError::from_response(response))
        }
    }

    /// <p> Returns a list of profiling groups. The profiling groups are returned as <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ProfilingGroupDescription.html"> <code>ProfilingGroupDescription</code> </a> objects. </p>
    #[allow(unused_mut)]
    async fn list_profiling_groups(
        &self,
        input: ListProfilingGroupsRequest,
    ) -> Result<ListProfilingGroupsResponse, RusotoError<ListProfilingGroupsError>> {
        let request_uri = "/profilingGroups";

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_description {
            params.put("includeDescription", x);
        }
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
                .deserialize::<ListProfilingGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfilingGroupsError::from_response(response))
        }
    }

    /// <p> Returns a list of the tags that are assigned to a specified resource. </p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
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

    /// <p> Submits profiling data to an aggregated profile of a profiling group. To get an aggregated profile that is created with this profiling data, use <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_GetProfile.html"> <code>GetProfile</code> </a>. </p>
    #[allow(unused_mut)]
    async fn post_agent_profile(
        &self,
        input: PostAgentProfileRequest,
    ) -> Result<PostAgentProfileResponse, RusotoError<PostAgentProfileError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/agentProfile",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.agent_profile.to_owned());
        request.set_payload(encoded);
        request.add_header("Content-Type", &input.content_type.to_string());
        let mut params = Params::new();
        if let Some(ref x) = input.profile_token {
            params.put("profileToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PostAgentProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PostAgentProfileError::from_response(response))
        }
    }

    /// <p><p> Adds permissions to a profiling group&#39;s resource-based policy that are provided using an action group. If a profiling group doesn&#39;t have a resource-based policy, one is created for it using the permissions in the action group and the roles and users in the <code>principals</code> parameter. </p> <pre><code> &lt;p&gt; The one supported action group that can be added is &lt;code&gt;agentPermission&lt;/code&gt; which grants &lt;code&gt;ConfigureAgent&lt;/code&gt; and &lt;code&gt;PostAgent&lt;/code&gt; permissions. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-ug/resource-based-policies.html&quot;&gt;Resource-based policies in CodeGuru Profiler&lt;/a&gt; in the &lt;i&gt;Amazon CodeGuru Profiler User Guide&lt;/i&gt;, &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API<em>ConfigureAgent.html&quot;&gt; &lt;code&gt;ConfigureAgent&lt;/code&gt; &lt;/a&gt;, and &lt;a href=&quot;https://docs.aws.amazon.com/codeguru/latest/profiler-api/API</em>PostAgentProfile.html&quot;&gt; &lt;code&gt;PostAgentProfile&lt;/code&gt; &lt;/a&gt;. &lt;/p&gt; &lt;p&gt; The first time you call &lt;code&gt;PutPermission&lt;/code&gt; on a profiling group, do not specify a &lt;code&gt;revisionId&lt;/code&gt; because it doesn&#39;t have a resource-based policy. Subsequent calls must provide a &lt;code&gt;revisionId&lt;/code&gt; to specify which revision of the resource-based policy to add the permissions to. &lt;/p&gt; &lt;p&gt; The response contains the profiling group&#39;s JSON-formatted resource policy. &lt;/p&gt; </code></pre></p>
    #[allow(unused_mut)]
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<PutPermissionResponse, RusotoError<PutPermissionError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy/{action_group}",
            action_group = input.action_group,
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("PUT", "codeguru-profiler", &self.region, &request_uri);
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
                .deserialize::<PutPermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutPermissionError::from_response(response))
        }
    }

    /// <p>Remove one anomaly notifications channel for a profiling group.</p>
    #[allow(unused_mut)]
    async fn remove_notification_channel(
        &self,
        input: RemoveNotificationChannelRequest,
    ) -> Result<RemoveNotificationChannelResponse, RusotoError<RemoveNotificationChannelError>>
    {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/notificationConfiguration/{channel_id}",
            channel_id = input.channel_id,
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveNotificationChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveNotificationChannelError::from_response(response))
        }
    }

    /// <p> Removes permissions from a profiling group's resource-based policy that are provided using an action group. The one supported action group that can be removed is <code>agentPermission</code> which grants <code>ConfigureAgent</code> and <code>PostAgent</code> permissions. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-ug/resource-based-policies.html">Resource-based policies in CodeGuru Profiler</a> in the <i>Amazon CodeGuru Profiler User Guide</i>, <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_ConfigureAgent.html"> <code>ConfigureAgent</code> </a>, and <a href="https://docs.aws.amazon.com/codeguru/latest/profiler-api/API_PostAgentProfile.html"> <code>PostAgentProfile</code> </a>. </p>
    #[allow(unused_mut)]
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<RemovePermissionResponse, RusotoError<RemovePermissionError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy/{action_group}",
            action_group = input.action_group,
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("revisionId", &input.revision_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemovePermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemovePermissionError::from_response(response))
        }
    }

    /// <p>Sends feedback to CodeGuru Profiler about whether the anomaly detected by the analysis is useful or not.</p>
    #[allow(unused_mut)]
    async fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Result<SubmitFeedbackResponse, RusotoError<SubmitFeedbackError>> {
        let request_uri = format!("/internal/profilingGroups/{profiling_group_name}/anomalies/{anomaly_instance_id}/feedback", anomaly_instance_id = input.anomaly_instance_id, profiling_group_name = input.profiling_group_name);

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
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
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SubmitFeedbackResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SubmitFeedbackError::from_response(response))
        }
    }

    /// <p> Use to assign one or more tags to a resource. </p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
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
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p> Use to remove one or more tags from a resource. </p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
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
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a profiling group.</p>
    #[allow(unused_mut)]
    async fn update_profiling_group(
        &self,
        input: UpdateProfilingGroupRequest,
    ) -> Result<UpdateProfilingGroupResponse, RusotoError<UpdateProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("PUT", "codeguru-profiler", &self.region, &request_uri);
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
                .deserialize::<UpdateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProfilingGroupError::from_response(response))
        }
    }
}
