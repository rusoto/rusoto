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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Represents an application source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSource {
    /// <p>The Amazon Resource Name (ARN) of a AWS CloudFormation stack.</p>
    #[serde(rename = "CloudFormationStackARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_arn: Option<String>,
    /// <p>A set of tags (up to 50).</p>
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateScalingPlanRequest {
    /// <p>A CloudFormation stack or set of tags. You can create one scaling plan per application source.</p>
    #[serde(rename = "ApplicationSource")]
    pub application_source: ApplicationSource,
    /// <p>The scaling instructions.</p>
    #[serde(rename = "ScalingInstructions")]
    pub scaling_instructions: Vec<ScalingInstruction>,
    /// <p>The name of the scaling plan. Names cannot contain vertical bars, colons, or forward slashes.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateScalingPlanResponse {
    /// <p>The version number of the scaling plan. This value is always 1.</p> <p>Currently, you cannot specify multiple scaling plan versions.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

/// <p>Represents a CloudWatch metric of your choosing that can be used for predictive scaling. </p> <p>For predictive scaling to work with a customized load metric specification, AWS Auto Scaling needs access to the <code>Sum</code> and <code>Average</code> statistics that CloudWatch computes from metric data. Statistics are calculations used to aggregate data over specified time periods.</p> <p>When you choose a load metric, make sure that the required <code>Sum</code> and <code>Average</code> statistics for your metric are available in CloudWatch and that they provide relevant data for predictive scaling. The <code>Sum</code> statistic must represent the total load on the resource, and the <code>Average</code> statistic must represent the average load per capacity unit of the resource. For example, there is a metric that counts the number of requests processed by your Auto Scaling group. If the <code>Sum</code> statistic represents the total request count processed by the group, then the <code>Average</code> statistic for the specified metric must represent the average request count processed by each instance of the group.</p> <p>For information about terminology, available metrics, or how to publish new metrics, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html">Amazon CloudWatch Concepts</a> in the <i>Amazon CloudWatch User Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomizedLoadMetricSpecification {
    /// <p>The dimensions of the metric.</p> <p>Conditional: If you published your metric with dimensions, you must specify the same dimensions in your customized load metric specification.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>The namespace of the metric.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The statistic of the metric. Currently, the value must always be <code>Sum</code>. </p>
    #[serde(rename = "Statistic")]
    pub statistic: String,
    /// <p>The unit of the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Represents a CloudWatch metric of your choosing that can be used for dynamic scaling as part of a target tracking scaling policy. </p> <p>To create your customized scaling metric specification:</p> <ul> <li> <p>Add values for each required parameter from CloudWatch. You can use an existing metric, or a new metric that you create. To use your own metric, you must first publish the metric to CloudWatch. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publish Custom Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> </li> <li> <p>Choose a metric that changes proportionally with capacity. The value of the metric should increase or decrease in inverse proportion to the number of capacity units. That is, the value of the metric should decrease when capacity increases. </p> </li> </ul> <p>For more information about CloudWatch, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html">Amazon CloudWatch Concepts</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomizedScalingMetricSpecification {
    /// <p>The dimensions of the metric.</p> <p>Conditional: If you published your metric with dimensions, you must specify the same dimensions in your customized scaling metric specification.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>The namespace of the metric.</p>
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// <p>The statistic of the metric.</p>
    #[serde(rename = "Statistic")]
    pub statistic: String,
    /// <p>The unit of the metric. </p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Represents a single value in the forecast data used for predictive scaling.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Datapoint {
    /// <p>The time stamp for the data point in UTC format.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The value of the data point.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScalingPlanRequest {
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteScalingPlanResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalingPlanResourcesRequest {
    /// <p>The maximum number of scalable resources to return. The value must be between 1 and 50. The default value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeScalingPlanResourcesResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the scalable resources.</p>
    #[serde(rename = "ScalingPlanResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_resources: Option<Vec<ScalingPlanResource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalingPlansRequest {
    /// <p>The sources for the applications (up to 10). If you specify scaling plan names, you cannot specify application sources.</p>
    #[serde(rename = "ApplicationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_sources: Option<Vec<ApplicationSource>>,
    /// <p>The maximum number of scalable resources to return. This value can be between 1 and 50. The default value is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the scaling plans (up to 10). If you specify application sources, you cannot specify scaling plan names.</p>
    #[serde(rename = "ScalingPlanNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_names: Option<Vec<String>>,
    /// <p>The version number of the scaling plan. If you specify a scaling plan version, you must also specify a scaling plan name.</p>
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeScalingPlansResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the scaling plans.</p>
    #[serde(rename = "ScalingPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plans: Option<Vec<ScalingPlan>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetScalingPlanResourceForecastDataRequest {
    /// <p>The exclusive end time of the time range for the forecast data to get. The maximum time duration between the start and end time is seven days. </p> <p>Although this parameter can accept a date and time that is more than two days in the future, the availability of forecast data has limits. AWS Auto Scaling only issues forecasts for periods of two days in advance.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p><p>The type of forecast data to get.</p> <ul> <li> <p> <code>LoadForecast</code>: The load metric forecast. </p> </li> <li> <p> <code>CapacityForecast</code>: The capacity forecast. </p> </li> <li> <p> <code>ScheduledActionMinCapacity</code>: The minimum capacity for each scheduled scaling action. This data is calculated as the larger of two values: the capacity forecast or the minimum capacity in the scaling instruction.</p> </li> <li> <p> <code>ScheduledActionMaxCapacity</code>: The maximum capacity for each scheduled scaling action. The calculation used is determined by the predictive scaling maximum capacity behavior setting in the scaling instruction.</p> </li> </ul></p>
    #[serde(rename = "ForecastDataType")]
    pub forecast_data_type: String,
    /// <p><p>The ID of the resource. This string consists of the resource type and unique identifier. </p> <ul> <li> <p>Auto Scaling group - The resource type is <code>autoScalingGroup</code> and the unique identifier is the name of the Auto Scaling group. Example: <code>autoScalingGroup/my-asg</code>.</p> </li> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The scalable dimension for the resource.</p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
    /// <p>The namespace of the AWS service.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The inclusive start time of the time range for the forecast data to get. The date and time can be at most 56 days before the current date and time. </p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetScalingPlanResourceForecastDataResponse {
    /// <p>The data points to return.</p>
    #[serde(rename = "Datapoints")]
    pub datapoints: Vec<Datapoint>,
}

/// <p>Represents a dimension for a customized metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the dimension.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Represents a predefined metric that can be used for predictive scaling. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredefinedLoadMetricSpecification {
    /// <p>The metric type.</p>
    #[serde(rename = "PredefinedLoadMetricType")]
    pub predefined_load_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group for an Application Load Balancer attached to the Auto Scaling group.</p> <p>The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN.</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul></p>
    #[serde(rename = "ResourceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

/// <p>Represents a predefined metric that can be used for dynamic scaling as part of a target tracking scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredefinedScalingMetricSpecification {
    /// <p>The metric type. The <code>ALBRequestCountPerTarget</code> metric type applies only to Auto Scaling groups, Spot Fleet requests, and ECS services.</p>
    #[serde(rename = "PredefinedScalingMetricType")]
    pub predefined_scaling_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group for an Application Load Balancer attached to the Auto Scaling group, Spot Fleet request, or ECS service.</p> <p>The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN.</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul></p>
    #[serde(rename = "ResourceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

/// <p>Describes a scaling instruction for a scalable resource.</p> <p>The scaling instruction is used in combination with a scaling plan, which is a set of instructions for configuring dynamic scaling and predictive scaling for the scalable resources in your application. Each scaling instruction applies to one resource.</p> <p>AWS Auto Scaling creates target tracking scaling policies based on the scaling instructions. Target tracking scaling policies adjust the capacity of your scalable resource as required to maintain resource utilization at the target value that you specified. </p> <p>AWS Auto Scaling also configures predictive scaling for your Amazon EC2 Auto Scaling groups using a subset of parameters, including the load metric, the scaling metric, the target value for the scaling metric, the predictive scaling mode (forecast and scale or forecast only), and the desired behavior when the forecast capacity exceeds the maximum capacity of the resource. With predictive scaling, AWS Auto Scaling generates forecasts with traffic predictions for the two days ahead and schedules scaling actions that proactively add and remove resource capacity to match the forecast. </p> <p>We recommend waiting a minimum of 24 hours after creating an Auto Scaling group to configure predictive scaling. At minimum, there must be 24 hours of historical data to generate a forecast.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/plans/userguide/auto-scaling-getting-started.html">Getting Started with AWS Auto Scaling</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingInstruction {
    /// <p>The customized load metric to use for predictive scaling. This parameter or a <b>PredefinedLoadMetricSpecification</b> is required when configuring predictive scaling, and cannot be used otherwise. </p>
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_load_metric_specification: Option<CustomizedLoadMetricSpecification>,
    /// <p>Controls whether dynamic scaling by AWS Auto Scaling is disabled. When dynamic scaling is enabled, AWS Auto Scaling creates target tracking scaling policies based on the specified target tracking configurations. </p> <p>The default is enabled (<code>false</code>). </p>
    #[serde(rename = "DisableDynamicScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_dynamic_scaling: Option<bool>,
    /// <p>The maximum capacity of the resource. The exception to this upper limit is if you specify a non-default setting for <b>PredictiveScalingMaxCapacityBehavior</b>. </p>
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    /// <p>The minimum capacity of the resource. </p>
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
    /// <p>The predefined load metric to use for predictive scaling. This parameter or a <b>CustomizedLoadMetricSpecification</b> is required when configuring predictive scaling, and cannot be used otherwise. </p>
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_load_metric_specification: Option<PredefinedLoadMetricSpecification>,
    /// <p>Defines the behavior that should be applied if the forecast capacity approaches or exceeds the maximum capacity specified for the resource. The default value is <code>SetForecastCapacityToMaxCapacity</code>.</p> <p>The following are possible values:</p> <ul> <li> <p> <code>SetForecastCapacityToMaxCapacity</code> - AWS Auto Scaling cannot scale resource capacity higher than the maximum capacity. The maximum capacity is enforced as a hard limit. </p> </li> <li> <p> <code>SetMaxCapacityToForecastCapacity</code> - AWS Auto Scaling may scale resource capacity higher than the maximum capacity to equal but not exceed forecast capacity.</p> </li> <li> <p> <code>SetMaxCapacityAboveForecastCapacity</code> - AWS Auto Scaling may scale resource capacity higher than the maximum capacity by a specified buffer value. The intention is to give the target tracking scaling policy extra capacity if unexpected traffic occurs. </p> </li> </ul> <p>Only valid when configuring predictive scaling.</p>
    #[serde(rename = "PredictiveScalingMaxCapacityBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_max_capacity_behavior: Option<String>,
    /// <p>The size of the capacity buffer to use when the forecast capacity is close to or exceeds the maximum capacity. The value is specified as a percentage relative to the forecast capacity. For example, if the buffer is 10, this means a 10 percent buffer, such that if the forecast capacity is 50, and the maximum capacity is 40, then the effective maximum capacity is 55.</p> <p>Only valid when configuring predictive scaling. Required if the <b>PredictiveScalingMaxCapacityBehavior</b> is set to <code>SetMaxCapacityAboveForecastCapacity</code>, and cannot be used otherwise.</p> <p>The range is 1-100.</p>
    #[serde(rename = "PredictiveScalingMaxCapacityBuffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_max_capacity_buffer: Option<i64>,
    /// <p>The predictive scaling mode. The default value is <code>ForecastAndScale</code>. Otherwise, AWS Auto Scaling forecasts capacity but does not create any scheduled scaling actions based on the capacity forecast. </p>
    #[serde(rename = "PredictiveScalingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_scaling_mode: Option<String>,
    /// <p><p>The ID of the resource. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>Auto Scaling group - The resource type is <code>autoScalingGroup</code> and the unique identifier is the name of the Auto Scaling group. Example: <code>autoScalingGroup/my-asg</code>.</p> </li> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension associated with the resource.</p> <ul> <li> <p> <code>autoscaling:autoScalingGroup:DesiredCapacity</code> - The desired capacity of an Auto Scaling group.</p> </li> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>Controls whether a resource's externally created scaling policies are kept or replaced. </p> <p>The default value is <code>KeepExternalPolicies</code>. If the parameter is set to <code>ReplaceExternalPolicies</code>, any scaling policies that are external to AWS Auto Scaling are deleted and new target tracking scaling policies created. </p> <p>Only valid when configuring dynamic scaling. </p> <p>Condition: The number of existing policies to be replaced must be less than or equal to 50. If there are more than 50 policies to be replaced, AWS Auto Scaling keeps all existing policies and does not create new ones.</p>
    #[serde(rename = "ScalingPolicyUpdateBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy_update_behavior: Option<String>,
    /// <p>The amount of time, in seconds, to buffer the run time of scheduled scaling actions when scaling out. For example, if the forecast says to add capacity at 10:00 AM, and the buffer time is 5 minutes, then the run time of the corresponding scheduled scaling action will be 9:55 AM. The intention is to give resources time to be provisioned. For example, it can take a few minutes to launch an EC2 instance. The actual amount of time required depends on several factors, such as the size of the instance and whether there are startup scripts to complete. </p> <p>The value must be less than the forecast interval duration of 3600 seconds (60 minutes). The default is 300 seconds. </p> <p>Only valid when configuring predictive scaling. </p>
    #[serde(rename = "ScheduledActionBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_buffer_time: Option<i64>,
    /// <p>The namespace of the AWS service.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The structure that defines new target tracking configurations (up to 10). Each of these structures includes a specific scaling metric and a target value for the metric, along with various parameters to use with dynamic scaling. </p> <p>With predictive scaling and dynamic scaling, the resource scales based on the target tracking configuration that provides the largest capacity for both scale in and scale out. </p> <p>Condition: The scaling metric must be unique across target tracking configurations.</p>
    #[serde(rename = "TargetTrackingConfigurations")]
    pub target_tracking_configurations: Vec<TargetTrackingConfiguration>,
}

/// <p>Represents a scaling plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScalingPlan {
    /// <p>The application source.</p>
    #[serde(rename = "ApplicationSource")]
    pub application_source: ApplicationSource,
    /// <p>The Unix time stamp when the scaling plan was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The scaling instructions.</p>
    #[serde(rename = "ScalingInstructions")]
    pub scaling_instructions: Vec<ScalingInstruction>,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
    /// <p><p>The status of the scaling plan.</p> <ul> <li> <p> <code>Active</code> - The scaling plan is active.</p> </li> <li> <p> <code>ActiveWithProblems</code> - The scaling plan is active, but the scaling configuration for one or more resources could not be applied.</p> </li> <li> <p> <code>CreationInProgress</code> - The scaling plan is being created.</p> </li> <li> <p> <code>CreationFailed</code> - The scaling plan could not be created.</p> </li> <li> <p> <code>DeletionInProgress</code> - The scaling plan is being deleted.</p> </li> <li> <p> <code>DeletionFailed</code> - The scaling plan could not be deleted.</p> </li> <li> <p> <code>UpdateInProgress</code> - The scaling plan is being updated.</p> </li> <li> <p> <code>UpdateFailed</code> - The scaling plan could not be updated.</p> </li> </ul></p>
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    /// <p>A simple message about the current status of the scaling plan.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The Unix time stamp when the scaling plan entered the current status.</p>
    #[serde(rename = "StatusStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_start_time: Option<f64>,
}

/// <p>Represents a scalable resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScalingPlanResource {
    /// <p><p>The ID of the resource. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>Auto Scaling group - The resource type is <code>autoScalingGroup</code> and the unique identifier is the name of the Auto Scaling group. Example: <code>autoScalingGroup/my-asg</code>.</p> </li> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension for the resource.</p> <ul> <li> <p> <code>autoscaling:autoScalingGroup:DesiredCapacity</code> - The desired capacity of an Auto Scaling group.</p> </li> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
    /// <p>The scaling policies.</p>
    #[serde(rename = "ScalingPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
    /// <p><p>The scaling status of the resource.</p> <ul> <li> <p> <code>Active</code> - The scaling configuration is active.</p> </li> <li> <p> <code>Inactive</code> - The scaling configuration is not active because the scaling plan is being created or the scaling configuration could not be applied. Check the status message for more information.</p> </li> <li> <p> <code>PartiallyActive</code> - The scaling configuration is partially active because the scaling plan is being created or deleted or the scaling configuration could not be fully applied. Check the status message for more information.</p> </li> </ul></p>
    #[serde(rename = "ScalingStatusCode")]
    pub scaling_status_code: String,
    /// <p>A simple message about the current scaling status of the resource.</p>
    #[serde(rename = "ScalingStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_status_message: Option<String>,
    /// <p>The namespace of the AWS service.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

/// <p>Represents a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScalingPolicy {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>The type of scaling policy.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The target tracking scaling policy. Includes support for predefined or customized metrics.</p>
    #[serde(rename = "TargetTrackingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
}

/// <p>Represents a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagFilter {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag values (0 to 20).</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Describes a target tracking configuration to use with AWS Auto Scaling. Used with <a>ScalingInstruction</a> and <a>ScalingPolicy</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetTrackingConfiguration {
    /// <p>A customized metric. You can specify either a predefined metric or a customized metric. </p>
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_scaling_metric_specification: Option<CustomizedScalingMetricSpecification>,
    /// <p>Indicates whether scale in by the target tracking scaling policy is disabled. If the value is <code>true</code>, scale in is disabled and the target tracking scaling policy doesn't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking scaling policy can remove capacity from the scalable resource. </p> <p>The default value is <code>false</code>.</p>
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. This value is used only if the resource is an Auto Scaling group.</p>
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i64>,
    /// <p>A predefined metric. You can specify either a predefined metric or a customized metric.</p>
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_scaling_metric_specification: Option<PredefinedScalingMetricSpecification>,
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. This value is not used if the scalable resource is an Auto Scaling group.</p> <p>The cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale-out policy during the cooldown period after a scale-in, AWS Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i64>,
    /// <p>The amount of time, in seconds, after a scale-out activity completes before another scale-out activity can start. This value is not used if the scalable resource is an Auto Scaling group.</p> <p>While the cooldown period is in effect, the capacity that has been added by the previous scale-out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out.</p>
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i64>,
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateScalingPlanRequest {
    /// <p>A CloudFormation stack or set of tags.</p>
    #[serde(rename = "ApplicationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_source: Option<ApplicationSource>,
    /// <p>The scaling instructions.</p>
    #[serde(rename = "ScalingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_instructions: Option<Vec<ScalingInstruction>>,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version number of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateScalingPlanResponse {}

/// Errors returned by CreateScalingPlan
#[derive(Debug, PartialEq)]
pub enum CreateScalingPlanError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>Your account exceeded a limit. This exception is thrown when a per-account resource limit is exceeded.</p>
    LimitExceeded(String),
}

impl CreateScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateScalingPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(CreateScalingPlanError::ConcurrentUpdate(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateScalingPlanError::InternalService(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateScalingPlanError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateScalingPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateScalingPlanError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            CreateScalingPlanError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateScalingPlanError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateScalingPlanError {}
/// Errors returned by DeleteScalingPlan
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPlanError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found.</p>
    ObjectNotFound(String),
}

impl DeleteScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScalingPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DeleteScalingPlanError::ConcurrentUpdate(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteScalingPlanError::InternalService(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DeleteScalingPlanError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteScalingPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScalingPlanError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DeleteScalingPlanError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteScalingPlanError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScalingPlanError {}
/// Errors returned by DescribeScalingPlanResources
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPlanResourcesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The token provided is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeScalingPlanResourcesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeScalingPlanResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(
                        DescribeScalingPlanResourcesError::ConcurrentUpdate(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeScalingPlanResourcesError::InternalService(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeScalingPlanResourcesError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeScalingPlanResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingPlanResourcesError::ConcurrentUpdate(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeScalingPlanResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalingPlanResourcesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeScalingPlanResourcesError {}
/// Errors returned by DescribeScalingPlans
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPlansError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The token provided is not valid.</p>
    InvalidNextToken(String),
}

impl DescribeScalingPlansError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingPlansError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DescribeScalingPlansError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScalingPlansError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeScalingPlansError::InvalidNextToken(
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
impl fmt::Display for DescribeScalingPlansError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingPlansError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DescribeScalingPlansError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalingPlansError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalingPlansError {}
/// Errors returned by GetScalingPlanResourceForecastData
#[derive(Debug, PartialEq)]
pub enum GetScalingPlanResourceForecastDataError {
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
}

impl GetScalingPlanResourceForecastDataError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetScalingPlanResourceForecastDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        GetScalingPlanResourceForecastDataError::InternalService(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetScalingPlanResourceForecastDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetScalingPlanResourceForecastDataError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetScalingPlanResourceForecastDataError {}
/// Errors returned by UpdateScalingPlan
#[derive(Debug, PartialEq)]
pub enum UpdateScalingPlanError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found.</p>
    ObjectNotFound(String),
}

impl UpdateScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateScalingPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(UpdateScalingPlanError::ConcurrentUpdate(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateScalingPlanError::InternalService(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(UpdateScalingPlanError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateScalingPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateScalingPlanError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            UpdateScalingPlanError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateScalingPlanError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateScalingPlanError {}
/// Trait representing the capabilities of the AWS Auto Scaling Plans API. AWS Auto Scaling Plans clients implement this trait.
#[async_trait]
pub trait AutoscalingPlans {
    /// <p>Creates a scaling plan.</p>
    async fn create_scaling_plan(
        &self,
        input: CreateScalingPlanRequest,
    ) -> Result<CreateScalingPlanResponse, RusotoError<CreateScalingPlanError>>;

    /// <p>Deletes the specified scaling plan.</p> <p>Deleting a scaling plan deletes the underlying <a>ScalingInstruction</a> for all of the scalable resources that are covered by the plan.</p> <p>If the plan has launched resources or has scaling activities in progress, you must delete those resources separately.</p>
    async fn delete_scaling_plan(
        &self,
        input: DeleteScalingPlanRequest,
    ) -> Result<DeleteScalingPlanResponse, RusotoError<DeleteScalingPlanError>>;

    /// <p>Describes the scalable resources in the specified scaling plan.</p>
    async fn describe_scaling_plan_resources(
        &self,
        input: DescribeScalingPlanResourcesRequest,
    ) -> Result<DescribeScalingPlanResourcesResponse, RusotoError<DescribeScalingPlanResourcesError>>;

    /// <p>Describes one or more of your scaling plans.</p>
    async fn describe_scaling_plans(
        &self,
        input: DescribeScalingPlansRequest,
    ) -> Result<DescribeScalingPlansResponse, RusotoError<DescribeScalingPlansError>>;

    /// <p>Retrieves the forecast data for a scalable resource.</p> <p>Capacity forecasts are represented as predicted values, or data points, that are calculated using historical data points from a specified CloudWatch load metric. Data points are available for up to 56 days. </p>
    async fn get_scaling_plan_resource_forecast_data(
        &self,
        input: GetScalingPlanResourceForecastDataRequest,
    ) -> Result<
        GetScalingPlanResourceForecastDataResponse,
        RusotoError<GetScalingPlanResourceForecastDataError>,
    >;

    /// <p>Updates the specified scaling plan.</p> <p>You cannot update a scaling plan if it is in the process of being created, updated, or deleted.</p>
    async fn update_scaling_plan(
        &self,
        input: UpdateScalingPlanRequest,
    ) -> Result<UpdateScalingPlanResponse, RusotoError<UpdateScalingPlanError>>;
}
/// A client for the AWS Auto Scaling Plans API.
#[derive(Clone)]
pub struct AutoscalingPlansClient {
    client: Client,
    region: region::Region,
}

impl AutoscalingPlansClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AutoscalingPlansClient {
        AutoscalingPlansClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AutoscalingPlansClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AutoscalingPlansClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AutoscalingPlansClient {
        AutoscalingPlansClient { client, region }
    }
}

#[async_trait]
impl AutoscalingPlans for AutoscalingPlansClient {
    /// <p>Creates a scaling plan.</p>
    async fn create_scaling_plan(
        &self,
        input: CreateScalingPlanRequest,
    ) -> Result<CreateScalingPlanResponse, RusotoError<CreateScalingPlanError>> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.CreateScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateScalingPlanResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateScalingPlanError::from_response(response))
        }
    }

    /// <p>Deletes the specified scaling plan.</p> <p>Deleting a scaling plan deletes the underlying <a>ScalingInstruction</a> for all of the scalable resources that are covered by the plan.</p> <p>If the plan has launched resources or has scaling activities in progress, you must delete those resources separately.</p>
    async fn delete_scaling_plan(
        &self,
        input: DeleteScalingPlanRequest,
    ) -> Result<DeleteScalingPlanResponse, RusotoError<DeleteScalingPlanError>> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DeleteScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteScalingPlanResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteScalingPlanError::from_response(response))
        }
    }

    /// <p>Describes the scalable resources in the specified scaling plan.</p>
    async fn describe_scaling_plan_resources(
        &self,
        input: DescribeScalingPlanResourcesRequest,
    ) -> Result<DescribeScalingPlanResourcesResponse, RusotoError<DescribeScalingPlanResourcesError>>
    {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DescribeScalingPlanResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeScalingPlanResourcesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScalingPlanResourcesError::from_response(response))
        }
    }

    /// <p>Describes one or more of your scaling plans.</p>
    async fn describe_scaling_plans(
        &self,
        input: DescribeScalingPlansRequest,
    ) -> Result<DescribeScalingPlansResponse, RusotoError<DescribeScalingPlansError>> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DescribeScalingPlans",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeScalingPlansResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScalingPlansError::from_response(response))
        }
    }

    /// <p>Retrieves the forecast data for a scalable resource.</p> <p>Capacity forecasts are represented as predicted values, or data points, that are calculated using historical data points from a specified CloudWatch load metric. Data points are available for up to 56 days. </p>
    async fn get_scaling_plan_resource_forecast_data(
        &self,
        input: GetScalingPlanResourceForecastDataRequest,
    ) -> Result<
        GetScalingPlanResourceForecastDataResponse,
        RusotoError<GetScalingPlanResourceForecastDataError>,
    > {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.GetScalingPlanResourceForecastData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetScalingPlanResourceForecastDataResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetScalingPlanResourceForecastDataError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the specified scaling plan.</p> <p>You cannot update a scaling plan if it is in the process of being created, updated, or deleted.</p>
    async fn update_scaling_plan(
        &self,
        input: UpdateScalingPlanRequest,
    ) -> Result<UpdateScalingPlanResponse, RusotoError<UpdateScalingPlanError>> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.UpdateScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateScalingPlanResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateScalingPlanError::from_response(response))
        }
    }
}
