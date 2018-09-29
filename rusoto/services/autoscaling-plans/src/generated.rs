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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents an application source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSource {
    /// <p>The Amazon Resource Name (ARN) of a CloudFormation stack.</p>
    #[serde(rename = "CloudFormationStackARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_arn: Option<String>,
    /// <p>A set of tags (up to 50).</p>
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateScalingPlanResponse {
    /// <p>The version of the scaling plan. This value is always 1.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

/// <p>Represents a customized metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomizedScalingMetricSpecification {
    /// <p>The dimensions of the metric.</p>
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
    /// <p>The unit of the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteScalingPlanRequest {
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteScalingPlanResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeScalingPlanResourcesRequest {
    /// <p>The maximum number of scalable resources to return. This value can be between 1 and 50. The default value is 50.</p>
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
    /// <p>The version of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The version of the scaling plan. If you specify a scaling plan version, you must also specify a scaling plan name.</p>
    #[serde(rename = "ScalingPlanVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_plan_version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Represents a predefined metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredefinedScalingMetricSpecification {
    /// <p>The metric type. The <code>ALBRequestCountPerTarget</code> metric type applies only to Auto Scaling groups, Sport Fleet requests, and ECS services.</p>
    #[serde(rename = "PredefinedScalingMetricType")]
    pub predefined_scaling_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group for an Application Load Balancer attached to the Auto Scaling group, Spot Fleet request, or ECS service.</p> <p>The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul></p>
    #[serde(rename = "ResourceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

/// <p>Specifies the scaling configuration for a scalable resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingInstruction {
    /// <p>The maximum value to scale to in response to a scale out event.</p>
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    /// <p>The minimum value to scale to in response to a scale in event.</p>
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
    /// <p><p>The ID of the resource. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>Auto Scaling group - The resource type is <code>autoScalingGroup</code> and the unique identifier is the name of the Auto Scaling group. Example: <code>autoScalingGroup/my-asg</code>.</p> </li> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension associated with the resource.</p> <ul> <li> <p> <code>autoscaling:autoScalingGroup:DesiredCapacity</code> - The desired capacity of an Auto Scaling group.</p> </li> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The target tracking scaling policies (up to 10).</p>
    #[serde(rename = "TargetTrackingConfigurations")]
    pub target_tracking_configurations: Vec<TargetTrackingConfiguration>,
}

/// <p>Represents a scaling plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScalingPlan {
    /// <p>The application source.</p>
    #[serde(rename = "ApplicationSource")]
    pub application_source: ApplicationSource,
    /// <p>The Unix timestamp when the scaling plan was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The scaling instructions.</p>
    #[serde(rename = "ScalingInstructions")]
    pub scaling_instructions: Vec<ScalingInstruction>,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version of the scaling plan.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
    /// <p><p>The status of the scaling plan.</p> <ul> <li> <p> <code>Active</code> - The scaling plan is active.</p> </li> <li> <p> <code>ActiveWithProblems</code> - The scaling plan is active, but the scaling configuration for one or more resources could not be applied.</p> </li> <li> <p> <code>CreationInProgress</code> - The scaling plan is being created.</p> </li> <li> <p> <code>CreationFailed</code> - The scaling plan could not be created.</p> </li> <li> <p> <code>DeletionInProgress</code> - The scaling plan is being deleted.</p> </li> <li> <p> <code>DeletionFailed</code> - The scaling plan could not be deleted.</p> </li> </ul></p>
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    /// <p>A simple message about the current status of the scaling plan.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The Unix timestamp when the scaling plan entered the current status.</p>
    #[serde(rename = "StatusStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_start_time: Option<f64>,
}

/// <p>Represents a scalable resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScalingPlanResource {
    /// <p><p>The ID of the resource. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>Auto Scaling group - The resource type is <code>autoScalingGroup</code> and the unique identifier is the name of the Auto Scaling group. Example: <code>autoScalingGroup/my-asg</code>.</p> </li> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension for the resource.</p> <ul> <li> <p> <code>autoscaling:autoScalingGroup:DesiredCapacity</code> - The desired capacity of an Auto Scaling group.</p> </li> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The name of the scaling plan.</p>
    #[serde(rename = "ScalingPlanName")]
    pub scaling_plan_name: String,
    /// <p>The version of the scaling plan.</p>
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
pub struct ScalingPolicy {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>The type of scaling policy.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The target tracking scaling policy.</p>
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

/// <p>Represents a target tracking scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetTrackingConfiguration {
    /// <p>A customized metric.</p>
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_scaling_metric_specification: Option<CustomizedScalingMetricSpecification>,
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is <code>true</code>, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is <code>false</code>.</p>
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    /// <p>The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. This value is used only if the resource is an Auto Scaling group.</p>
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i64>,
    /// <p>A predefined metric.</p>
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_scaling_metric_specification: Option<PredefinedScalingMetricSpecification>,
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start. This value is not used if the scalable resource is an Auto Scaling group.</p> <p>The cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, AWS Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i64>,
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start. This value is not used if the scalable resource is an Auto Scaling group.</p> <p>While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out.</p>
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i64>,
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p>The version number.</p>
    #[serde(rename = "ScalingPlanVersion")]
    pub scaling_plan_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateScalingPlanError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentUpdateException" => {
                    return CreateScalingPlanError::ConcurrentUpdate(String::from(error_message))
                }
                "InternalServiceException" => {
                    return CreateScalingPlanError::InternalService(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateScalingPlanError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateScalingPlanError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateScalingPlanError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateScalingPlanError {
    fn from(err: serde_json::error::Error) -> CreateScalingPlanError {
        CreateScalingPlanError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateScalingPlanError {
    fn from(err: CredentialsError) -> CreateScalingPlanError {
        CreateScalingPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateScalingPlanError {
    fn from(err: HttpDispatchError) -> CreateScalingPlanError {
        CreateScalingPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateScalingPlanError {
    fn from(err: io::Error) -> CreateScalingPlanError {
        CreateScalingPlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateScalingPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateScalingPlanError {
    fn description(&self) -> &str {
        match *self {
            CreateScalingPlanError::ConcurrentUpdate(ref cause) => cause,
            CreateScalingPlanError::InternalService(ref cause) => cause,
            CreateScalingPlanError::LimitExceeded(ref cause) => cause,
            CreateScalingPlanError::Validation(ref cause) => cause,
            CreateScalingPlanError::Credentials(ref err) => err.description(),
            CreateScalingPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateScalingPlanError::ParseError(ref cause) => cause,
            CreateScalingPlanError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteScalingPlan
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPlanError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteScalingPlanError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentUpdateException" => {
                    return DeleteScalingPlanError::ConcurrentUpdate(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DeleteScalingPlanError::InternalService(String::from(error_message))
                }
                "ObjectNotFoundException" => {
                    return DeleteScalingPlanError::ObjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteScalingPlanError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteScalingPlanError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteScalingPlanError {
    fn from(err: serde_json::error::Error) -> DeleteScalingPlanError {
        DeleteScalingPlanError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteScalingPlanError {
    fn from(err: CredentialsError) -> DeleteScalingPlanError {
        DeleteScalingPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteScalingPlanError {
    fn from(err: HttpDispatchError) -> DeleteScalingPlanError {
        DeleteScalingPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteScalingPlanError {
    fn from(err: io::Error) -> DeleteScalingPlanError {
        DeleteScalingPlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteScalingPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteScalingPlanError {
    fn description(&self) -> &str {
        match *self {
            DeleteScalingPlanError::ConcurrentUpdate(ref cause) => cause,
            DeleteScalingPlanError::InternalService(ref cause) => cause,
            DeleteScalingPlanError::ObjectNotFound(ref cause) => cause,
            DeleteScalingPlanError::Validation(ref cause) => cause,
            DeleteScalingPlanError::Credentials(ref err) => err.description(),
            DeleteScalingPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteScalingPlanError::ParseError(ref cause) => cause,
            DeleteScalingPlanError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeScalingPlanResources
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPlanResourcesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The token provided is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeScalingPlanResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeScalingPlanResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentUpdateException" => {
                    return DescribeScalingPlanResourcesError::ConcurrentUpdate(String::from(
                        error_message,
                    ))
                }
                "InternalServiceException" => {
                    return DescribeScalingPlanResourcesError::InternalService(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return DescribeScalingPlanResourcesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeScalingPlanResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeScalingPlanResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeScalingPlanResourcesError {
    fn from(err: serde_json::error::Error) -> DescribeScalingPlanResourcesError {
        DescribeScalingPlanResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalingPlanResourcesError {
    fn from(err: CredentialsError) -> DescribeScalingPlanResourcesError {
        DescribeScalingPlanResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingPlanResourcesError {
    fn from(err: HttpDispatchError) -> DescribeScalingPlanResourcesError {
        DescribeScalingPlanResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingPlanResourcesError {
    fn from(err: io::Error) -> DescribeScalingPlanResourcesError {
        DescribeScalingPlanResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingPlanResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingPlanResourcesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingPlanResourcesError::ConcurrentUpdate(ref cause) => cause,
            DescribeScalingPlanResourcesError::InternalService(ref cause) => cause,
            DescribeScalingPlanResourcesError::InvalidNextToken(ref cause) => cause,
            DescribeScalingPlanResourcesError::Validation(ref cause) => cause,
            DescribeScalingPlanResourcesError::Credentials(ref err) => err.description(),
            DescribeScalingPlanResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingPlanResourcesError::ParseError(ref cause) => cause,
            DescribeScalingPlanResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeScalingPlans
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPlansError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The token provided is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeScalingPlansError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeScalingPlansError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentUpdateException" => {
                    return DescribeScalingPlansError::ConcurrentUpdate(String::from(error_message))
                }
                "InternalServiceException" => {
                    return DescribeScalingPlansError::InternalService(String::from(error_message))
                }
                "InvalidNextTokenException" => {
                    return DescribeScalingPlansError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeScalingPlansError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeScalingPlansError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeScalingPlansError {
    fn from(err: serde_json::error::Error) -> DescribeScalingPlansError {
        DescribeScalingPlansError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalingPlansError {
    fn from(err: CredentialsError) -> DescribeScalingPlansError {
        DescribeScalingPlansError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingPlansError {
    fn from(err: HttpDispatchError) -> DescribeScalingPlansError {
        DescribeScalingPlansError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingPlansError {
    fn from(err: io::Error) -> DescribeScalingPlansError {
        DescribeScalingPlansError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingPlansError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingPlansError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingPlansError::ConcurrentUpdate(ref cause) => cause,
            DescribeScalingPlansError::InternalService(ref cause) => cause,
            DescribeScalingPlansError::InvalidNextToken(ref cause) => cause,
            DescribeScalingPlansError::Validation(ref cause) => cause,
            DescribeScalingPlansError::Credentials(ref err) => err.description(),
            DescribeScalingPlansError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingPlansError::ParseError(ref cause) => cause,
            DescribeScalingPlansError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateScalingPlan
#[derive(Debug, PartialEq)]
pub enum UpdateScalingPlanError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateScalingPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateScalingPlanError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentUpdateException" => {
                    return UpdateScalingPlanError::ConcurrentUpdate(String::from(error_message))
                }
                "InternalServiceException" => {
                    return UpdateScalingPlanError::InternalService(String::from(error_message))
                }
                "ObjectNotFoundException" => {
                    return UpdateScalingPlanError::ObjectNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateScalingPlanError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateScalingPlanError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateScalingPlanError {
    fn from(err: serde_json::error::Error) -> UpdateScalingPlanError {
        UpdateScalingPlanError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateScalingPlanError {
    fn from(err: CredentialsError) -> UpdateScalingPlanError {
        UpdateScalingPlanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateScalingPlanError {
    fn from(err: HttpDispatchError) -> UpdateScalingPlanError {
        UpdateScalingPlanError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateScalingPlanError {
    fn from(err: io::Error) -> UpdateScalingPlanError {
        UpdateScalingPlanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateScalingPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateScalingPlanError {
    fn description(&self) -> &str {
        match *self {
            UpdateScalingPlanError::ConcurrentUpdate(ref cause) => cause,
            UpdateScalingPlanError::InternalService(ref cause) => cause,
            UpdateScalingPlanError::ObjectNotFound(ref cause) => cause,
            UpdateScalingPlanError::Validation(ref cause) => cause,
            UpdateScalingPlanError::Credentials(ref err) => err.description(),
            UpdateScalingPlanError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateScalingPlanError::ParseError(ref cause) => cause,
            UpdateScalingPlanError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Auto Scaling Plans API. AWS Auto Scaling Plans clients implement this trait.
pub trait AutoscalingPlans {
    /// <p>Creates a scaling plan.</p> <p>A scaling plan contains a set of instructions used to configure dynamic scaling for the scalable resources in your application. AWS Auto Scaling creates target tracking scaling policies based on the scaling instructions in your scaling plan.</p>
    fn create_scaling_plan(
        &self,
        input: CreateScalingPlanRequest,
    ) -> RusotoFuture<CreateScalingPlanResponse, CreateScalingPlanError>;

    /// <p>Deletes the specified scaling plan.</p>
    fn delete_scaling_plan(
        &self,
        input: DeleteScalingPlanRequest,
    ) -> RusotoFuture<DeleteScalingPlanResponse, DeleteScalingPlanError>;

    /// <p>Describes the scalable resources in the specified scaling plan.</p>
    fn describe_scaling_plan_resources(
        &self,
        input: DescribeScalingPlanResourcesRequest,
    ) -> RusotoFuture<DescribeScalingPlanResourcesResponse, DescribeScalingPlanResourcesError>;

    /// <p>Describes the specified scaling plans or all of your scaling plans.</p>
    fn describe_scaling_plans(
        &self,
        input: DescribeScalingPlansRequest,
    ) -> RusotoFuture<DescribeScalingPlansResponse, DescribeScalingPlansError>;

    /// <p>Updates the scaling plan for the specified scaling plan.</p> <p>You cannot update a scaling plan if it is in the process of being created, updated, or deleted.</p>
    fn update_scaling_plan(
        &self,
        input: UpdateScalingPlanRequest,
    ) -> RusotoFuture<UpdateScalingPlanResponse, UpdateScalingPlanError>;
}
/// A client for the AWS Auto Scaling Plans API.
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AutoscalingPlansClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AutoscalingPlansClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl AutoscalingPlans for AutoscalingPlansClient {
    /// <p>Creates a scaling plan.</p> <p>A scaling plan contains a set of instructions used to configure dynamic scaling for the scalable resources in your application. AWS Auto Scaling creates target tracking scaling policies based on the scaling instructions in your scaling plan.</p>
    fn create_scaling_plan(
        &self,
        input: CreateScalingPlanRequest,
    ) -> RusotoFuture<CreateScalingPlanResponse, CreateScalingPlanError> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.CreateScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateScalingPlanResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateScalingPlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified scaling plan.</p>
    fn delete_scaling_plan(
        &self,
        input: DeleteScalingPlanRequest,
    ) -> RusotoFuture<DeleteScalingPlanResponse, DeleteScalingPlanError> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DeleteScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteScalingPlanResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteScalingPlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the scalable resources in the specified scaling plan.</p>
    fn describe_scaling_plan_resources(
        &self,
        input: DescribeScalingPlanResourcesRequest,
    ) -> RusotoFuture<DescribeScalingPlanResourcesResponse, DescribeScalingPlanResourcesError> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DescribeScalingPlanResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScalingPlanResourcesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingPlanResourcesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the specified scaling plans or all of your scaling plans.</p>
    fn describe_scaling_plans(
        &self,
        input: DescribeScalingPlansRequest,
    ) -> RusotoFuture<DescribeScalingPlansResponse, DescribeScalingPlansError> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.DescribeScalingPlans",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScalingPlansResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeScalingPlansError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the scaling plan for the specified scaling plan.</p> <p>You cannot update a scaling plan if it is in the process of being created, updated, or deleted.</p>
    fn update_scaling_plan(
        &self,
        input: UpdateScalingPlanRequest,
    ) -> RusotoFuture<UpdateScalingPlanResponse, UpdateScalingPlanError> {
        let mut request = SignedRequest::new("POST", "autoscaling-plans", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleScalingPlannerFrontendService.UpdateScalingPlan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateScalingPlanResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateScalingPlanError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
