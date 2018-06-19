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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents a CloudWatch alarm associated with a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Alarm {
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    #[serde(rename = "AlarmARN")]
    pub alarm_arn: String,
    /// <p>The name of the alarm.</p>
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,
}

/// <p>Configures a customized metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomizedMetricSpecification {
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
pub struct DeleteScalingPolicyRequest {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteScalingPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteScheduledActionRequest {
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteScheduledActionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterScalableTargetRequest {
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterScalableTargetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeScalableTargetsRequest {
    /// <p>The maximum number of scalable targets. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeScalableTargetsResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The scalable targets that match the request parameters.</p>
    #[serde(rename = "ScalableTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_targets: Option<Vec<ScalableTarget>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeScalingActivitiesRequest {
    /// <p>The maximum number of scalable targets. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeScalingActivitiesResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of scaling activity objects.</p>
    #[serde(rename = "ScalingActivities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_activities: Option<Vec<ScalingActivity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeScalingPoliciesRequest {
    /// <p>The maximum number of scalable targets. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the scaling policies to describe.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeScalingPoliciesResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the scaling policies.</p>
    #[serde(rename = "ScalingPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeScheduledActionsRequest {
    /// <p>The maximum number of scheduled action results. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The names of the scheduled actions to describe.</p>
    #[serde(rename = "ScheduledActionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_names: Option<Vec<String>>,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeScheduledActionsResponse {
    /// <p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the scheduled actions.</p>
    #[serde(rename = "ScheduledActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<Vec<ScheduledAction>>,
}

/// <p>Describes the dimension of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the dimension.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Configures a predefined metric for a target tracking policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredefinedMetricSpecification {
    /// <p>The metric type. The <code>ALBRequestCountPerTarget</code> metric type applies only to Spot fleet requests and ECS services.</p>
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group attached to the Spot fleet request or ECS service.</p> <p>The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul></p>
    #[serde(rename = "ResourceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutScalingPolicyRequest {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>The policy type. This parameter is required if you are creating a policy.</p> <p>For DynamoDB, only <code>TargetTrackingScaling</code> is supported. For Amazon ECS, Spot Fleet, and Amazon RDS, both <code>StepScaling</code> and <code>TargetTrackingScaling</code> are supported. For any other service, only <code>StepScaling</code> is supported.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>A step scaling policy.</p> <p>This parameter is required if you are creating a policy and the policy type is <code>StepScaling</code>.</p>
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    /// <p>A target tracking policy.</p> <p>This parameter is required if you are creating a policy and the policy type is <code>TargetTrackingScaling</code>.</p>
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutScalingPolicyResponse {
    /// <p>The CloudWatch alarms created for the target tracking policy.</p>
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Amazon Resource Name (ARN) of the resulting scaling policy.</p>
    #[serde(rename = "PolicyARN")]
    pub policy_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutScheduledActionRequest {
    /// <p>The date and time for the scheduled action to end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This parameter is required if you are creating a scheduled action. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The new minimum and maximum capacity. You can set both values or just one. During the scheduled time, if the current capacity is below the minimum capacity, Application Auto Scaling scales out to the minimum capacity. If the current capacity is above the maximum capacity, Application Auto Scaling scales in to the maximum capacity.</p>
    #[serde(rename = "ScalableTargetAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    /// <p>The schedule for this action. The following formats are supported:</p> <ul> <li> <p>At expressions - <code>at(<i>yyyy</i>-<i>mm</i>-<i>dd</i>T<i>hh</i>:<i>mm</i>:<i>ss</i>)</code> </p> </li> <li> <p>Rate expressions - <code>rate(<i>value</i> <i>unit</i>)</code> </p> </li> <li> <p>Cron expressions - <code>cron(<i>fields</i>)</code> </p> </li> </ul> <p>At expressions are useful for one-time schedules. Specify the time, in UTC.</p> <p>For rate expressions, <i>value</i> is a positive integer and <i>unit</i> is <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code>.</p> <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">Cron</a>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The date and time for the scheduled action to start.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutScheduledActionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterScalableTargetRequest {
    /// <p>The maximum value to scale to in response to a scale out event. This parameter is required if you are registering a scalable target.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i64>,
    /// <p>The minimum value to scale to in response to a scale in event. This parameter is required if you are registering a scalable target.</p>
    #[serde(rename = "MinCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i64>,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Application Auto Scaling creates a service-linked role that grants it permissions to modify the scalable target on your behalf. For more information, see <a href="http://docs.aws.amazon.com/ApplicationAutoScaling/latest/APIReference/application-autoscaling-service-linked-roles.html">Service-Linked Roles for Application Auto Scaling</a>.</p> <p>For resources that are not supported using a service-linked role, this parameter is required and must specify the ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterScalableTargetResponse {}

/// <p>Represents a scalable target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScalableTarget {
    /// <p>The Unix timestamp for when the scalable target was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The maximum value to scale to in response to a scale out event.</p>
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    /// <p>The minimum value to scale to in response to a scale in event.</p>
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

/// <p>Represents the minimum and maximum capacity for a scheduled action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalableTargetAction {
    /// <p>The maximum capacity.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i64>,
    /// <p>The minimum capacity.</p>
    #[serde(rename = "MinCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i64>,
}

/// <p>Represents a scaling activity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScalingActivity {
    /// <p>The unique identifier of the scaling activity.</p>
    #[serde(rename = "ActivityId")]
    pub activity_id: String,
    /// <p>A simple description of what caused the scaling activity to happen.</p>
    #[serde(rename = "Cause")]
    pub cause: String,
    /// <p>A simple description of what action the scaling activity intends to accomplish.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The details about the scaling activity.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The Unix timestamp for when the scaling activity ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The Unix timestamp for when the scaling activity began.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    /// <p>Indicates the status of the scaling activity.</p>
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    /// <p>A simple message about the current status of the scaling activity.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Represents a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScalingPolicy {
    /// <p>The CloudWatch alarms associated with the scaling policy.</p>
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Unix timestamp for when the scaling policy was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the scaling policy.</p>
    #[serde(rename = "PolicyARN")]
    pub policy_arn: String,
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>The scaling policy type.</p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>A step scaling policy.</p>
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    /// <p>A target tracking policy.</p>
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

/// <p>Represents a scheduled action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduledAction {
    /// <p>The date and time that the scheduled action was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The date and time that the action is scheduled to end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variants - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The new minimum and maximum capacity. You can set both values or just one. During the scheduled time, if the current capacity is below the minimum capacity, Application Auto Scaling scales out to the minimum capacity. If the current capacity is above the maximum capacity, Application Auto Scaling scales in to the maximum capacity.</p>
    #[serde(rename = "ScalableTargetAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    /// <p>The schedule for this action. The following formats are supported:</p> <ul> <li> <p>At expressions - <code>at(<i>yyyy</i>-<i>mm</i>-<i>dd</i>T<i>hh</i>:<i>mm</i>:<i>ss</i>)</code> </p> </li> <li> <p>Rate expressions - <code>rate(<i>value</i> <i>unit</i>)</code> </p> </li> <li> <p>Cron expressions - <code>cron(<i>fields</i>)</code> </p> </li> </ul> <p>At expressions are useful for one-time schedules. Specify the time, in UTC.</p> <p>For rate expressions, <i>value</i> is a positive integer and <i>unit</i> is <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code>.</p> <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">Cron</a>.</p>
    #[serde(rename = "Schedule")]
    pub schedule: String,
    /// <p>The Amazon Resource Name (ARN) of the scheduled action.</p>
    #[serde(rename = "ScheduledActionARN")]
    pub scheduled_action_arn: String,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The date and time that the action is scheduled to begin.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p><p>Represents a step adjustment for a <a>StepScalingPolicyConfiguration</a>. Describes an adjustment based on the difference between the value of the aggregated CloudWatch metric and the breach threshold that you&#39;ve defined for the alarm. </p> <p>For the following examples, suppose that you have an alarm with a breach threshold of 50:</p> <ul> <li> <p>To trigger the adjustment when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.</p> </li> <li> <p>To trigger the adjustment when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.</p> </li> </ul> <p>There are a few rules for the step adjustments for your step policy:</p> <ul> <li> <p>The ranges of your step adjustments can&#39;t overlap or have a gap.</p> </li> <li> <p>At most one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.</p> </li> <li> <p>At most one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.</p> </li> <li> <p>The upper and lower bound can&#39;t be null in the same step adjustment.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepAdjustment {
    /// <p>The lower bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the lower bound is inclusive (the metric must be greater than or equal to the threshold plus the lower bound). Otherwise, it is exclusive (the metric must be greater than the threshold plus the lower bound). A null value indicates negative infinity.</p>
    #[serde(rename = "MetricIntervalLowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_interval_lower_bound: Option<f64>,
    /// <p>The upper bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the upper bound is exclusive (the metric must be less than the threshold plus the upper bound). Otherwise, it is inclusive (the metric must be less than or equal to the threshold plus the upper bound). A null value indicates positive infinity.</p> <p>The upper bound must be greater than the lower bound.</p>
    #[serde(rename = "MetricIntervalUpperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_interval_upper_bound: Option<f64>,
    /// <p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current scalable dimension while a negative number removes from the current scalable dimension.</p>
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,
}

/// <p>Represents a step scaling policy configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepScalingPolicyConfiguration {
    /// <p>The adjustment type, which specifies how the <code>ScalingAdjustment</code> parameter in a <a>StepAdjustment</a> is interpreted.</p>
    #[serde(rename = "AdjustmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    /// <p>The amount of time, in seconds, after a scaling activity completes where previous trigger-related scaling activities can influence future scaling events.</p> <p>For scale out policies, while the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out. For example, an alarm triggers a step scaling policy to scale out an Amazon ECS service by 2 tasks, the scaling activity completes successfully, and a cooldown period of 5 minutes starts. During the Cooldown period, if the alarm triggers the same policy again but at a more aggressive step adjustment to scale out the service by 3 tasks, the 2 tasks that were added in the previous scale out event are considered part of that capacity and only 1 additional task is added to the desired count.</p> <p>For scale in policies, the cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "Cooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. Valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>.</p>
    #[serde(rename = "MetricAggregationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number to adjust your scalable dimension as a result of a scaling activity. If the adjustment type is <code>PercentChangeInCapacity</code>, the scaling policy changes the scalable dimension of the scalable target by this amount.</p>
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>
    #[serde(rename = "StepAdjustments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,
}

/// <p>Represents a target tracking scaling policy configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetTrackingScalingPolicyConfiguration {
    /// <p>A customized metric.</p>
    #[serde(rename = "CustomizedMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    /// <p>Indicates whether scale in by the target tracking policy is disabled. If the value is <code>true</code>, scale in is disabled and the target tracking policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking policy can remove capacity from the scalable resource. The default value is <code>false</code>.</p>
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    /// <p>A predefined metric.</p>
    #[serde(rename = "PredefinedMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    /// <p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start.</p> <p>The cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i64>,
    /// <p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start.</p> <p>While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out.</p>
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<i64>,
    /// <p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
}

/// Errors returned by DeleteScalingPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPolicyError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteScalingPolicyError {
    pub fn from_body(body: &str) -> DeleteScalingPolicyError {
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
                    "ConcurrentUpdateException" => {
                        DeleteScalingPolicyError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteScalingPolicyError::InternalService(String::from(error_message))
                    }
                    "ObjectNotFoundException" => {
                        DeleteScalingPolicyError::ObjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteScalingPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteScalingPolicyError {
    fn from(err: CredentialsError) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteScalingPolicyError {
    fn from(err: HttpDispatchError) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteScalingPolicyError {
    fn from(err: io::Error) -> DeleteScalingPolicyError {
        DeleteScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteScalingPolicyError::ConcurrentUpdate(ref cause) => cause,
            DeleteScalingPolicyError::InternalService(ref cause) => cause,
            DeleteScalingPolicyError::ObjectNotFound(ref cause) => cause,
            DeleteScalingPolicyError::Validation(ref cause) => cause,
            DeleteScalingPolicyError::Credentials(ref err) => err.description(),
            DeleteScalingPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteScheduledAction
#[derive(Debug, PartialEq)]
pub enum DeleteScheduledActionError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteScheduledActionError {
    pub fn from_body(body: &str) -> DeleteScheduledActionError {
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
                    "ConcurrentUpdateException" => {
                        DeleteScheduledActionError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeleteScheduledActionError::InternalService(String::from(error_message))
                    }
                    "ObjectNotFoundException" => {
                        DeleteScheduledActionError::ObjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteScheduledActionError::Validation(error_message.to_string())
                    }
                    _ => DeleteScheduledActionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteScheduledActionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteScheduledActionError {
    fn from(err: serde_json::error::Error) -> DeleteScheduledActionError {
        DeleteScheduledActionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteScheduledActionError {
    fn from(err: CredentialsError) -> DeleteScheduledActionError {
        DeleteScheduledActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteScheduledActionError {
    fn from(err: HttpDispatchError) -> DeleteScheduledActionError {
        DeleteScheduledActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteScheduledActionError {
    fn from(err: io::Error) -> DeleteScheduledActionError {
        DeleteScheduledActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteScheduledActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteScheduledActionError {
    fn description(&self) -> &str {
        match *self {
            DeleteScheduledActionError::ConcurrentUpdate(ref cause) => cause,
            DeleteScheduledActionError::InternalService(ref cause) => cause,
            DeleteScheduledActionError::ObjectNotFound(ref cause) => cause,
            DeleteScheduledActionError::Validation(ref cause) => cause,
            DeleteScheduledActionError::Credentials(ref err) => err.description(),
            DeleteScheduledActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteScheduledActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum DeregisterScalableTargetError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterScalableTargetError {
    pub fn from_body(body: &str) -> DeregisterScalableTargetError {
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
                    "ConcurrentUpdateException" => {
                        DeregisterScalableTargetError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DeregisterScalableTargetError::InternalService(String::from(error_message))
                    }
                    "ObjectNotFoundException" => {
                        DeregisterScalableTargetError::ObjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterScalableTargetError::Validation(error_message.to_string())
                    }
                    _ => DeregisterScalableTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterScalableTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterScalableTargetError {
    fn from(err: serde_json::error::Error) -> DeregisterScalableTargetError {
        DeregisterScalableTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterScalableTargetError {
    fn from(err: CredentialsError) -> DeregisterScalableTargetError {
        DeregisterScalableTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterScalableTargetError {
    fn from(err: HttpDispatchError) -> DeregisterScalableTargetError {
        DeregisterScalableTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterScalableTargetError {
    fn from(err: io::Error) -> DeregisterScalableTargetError {
        DeregisterScalableTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterScalableTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterScalableTargetError {
    fn description(&self) -> &str {
        match *self {
            DeregisterScalableTargetError::ConcurrentUpdate(ref cause) => cause,
            DeregisterScalableTargetError::InternalService(ref cause) => cause,
            DeregisterScalableTargetError::ObjectNotFound(ref cause) => cause,
            DeregisterScalableTargetError::Validation(ref cause) => cause,
            DeregisterScalableTargetError::Credentials(ref err) => err.description(),
            DeregisterScalableTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterScalableTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalableTargets
#[derive(Debug, PartialEq)]
pub enum DescribeScalableTargetsError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalableTargetsError {
    pub fn from_body(body: &str) -> DescribeScalableTargetsError {
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
                    "ConcurrentUpdateException" => {
                        DescribeScalableTargetsError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DescribeScalableTargetsError::InternalService(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeScalableTargetsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeScalableTargetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeScalableTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeScalableTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeScalableTargetsError {
    fn from(err: serde_json::error::Error) -> DescribeScalableTargetsError {
        DescribeScalableTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalableTargetsError {
    fn from(err: CredentialsError) -> DescribeScalableTargetsError {
        DescribeScalableTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalableTargetsError {
    fn from(err: HttpDispatchError) -> DescribeScalableTargetsError {
        DescribeScalableTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalableTargetsError {
    fn from(err: io::Error) -> DescribeScalableTargetsError {
        DescribeScalableTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalableTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalableTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalableTargetsError::ConcurrentUpdate(ref cause) => cause,
            DescribeScalableTargetsError::InternalService(ref cause) => cause,
            DescribeScalableTargetsError::InvalidNextToken(ref cause) => cause,
            DescribeScalableTargetsError::Validation(ref cause) => cause,
            DescribeScalableTargetsError::Credentials(ref err) => err.description(),
            DescribeScalableTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalableTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingActivities
#[derive(Debug, PartialEq)]
pub enum DescribeScalingActivitiesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalingActivitiesError {
    pub fn from_body(body: &str) -> DescribeScalingActivitiesError {
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
                    "ConcurrentUpdateException" => {
                        DescribeScalingActivitiesError::ConcurrentUpdate(String::from(
                            error_message,
                        ))
                    }
                    "InternalServiceException" => {
                        DescribeScalingActivitiesError::InternalService(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeScalingActivitiesError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeScalingActivitiesError::Validation(error_message.to_string())
                    }
                    _ => DescribeScalingActivitiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeScalingActivitiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeScalingActivitiesError {
    fn from(err: serde_json::error::Error) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalingActivitiesError {
    fn from(err: CredentialsError) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingActivitiesError {
    fn from(err: HttpDispatchError) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingActivitiesError {
    fn from(err: io::Error) -> DescribeScalingActivitiesError {
        DescribeScalingActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingActivitiesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingActivitiesError::ConcurrentUpdate(ref cause) => cause,
            DescribeScalingActivitiesError::InternalService(ref cause) => cause,
            DescribeScalingActivitiesError::InvalidNextToken(ref cause) => cause,
            DescribeScalingActivitiesError::Validation(ref cause) => cause,
            DescribeScalingActivitiesError::Credentials(ref err) => err.description(),
            DescribeScalingActivitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingActivitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScalingPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPoliciesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> on your behalf.</p>
    FailedResourceAccess(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScalingPoliciesError {
    pub fn from_body(body: &str) -> DescribeScalingPoliciesError {
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
                    "ConcurrentUpdateException" => {
                        DescribeScalingPoliciesError::ConcurrentUpdate(String::from(error_message))
                    }
                    "FailedResourceAccessException" => {
                        DescribeScalingPoliciesError::FailedResourceAccess(String::from(
                            error_message,
                        ))
                    }
                    "InternalServiceException" => {
                        DescribeScalingPoliciesError::InternalService(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeScalingPoliciesError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeScalingPoliciesError::Validation(error_message.to_string())
                    }
                    _ => DescribeScalingPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeScalingPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeScalingPoliciesError {
    fn from(err: serde_json::error::Error) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScalingPoliciesError {
    fn from(err: CredentialsError) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScalingPoliciesError {
    fn from(err: HttpDispatchError) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScalingPoliciesError {
    fn from(err: io::Error) -> DescribeScalingPoliciesError {
        DescribeScalingPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScalingPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScalingPoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeScalingPoliciesError::ConcurrentUpdate(ref cause) => cause,
            DescribeScalingPoliciesError::FailedResourceAccess(ref cause) => cause,
            DescribeScalingPoliciesError::InternalService(ref cause) => cause,
            DescribeScalingPoliciesError::InvalidNextToken(ref cause) => cause,
            DescribeScalingPoliciesError::Validation(ref cause) => cause,
            DescribeScalingPoliciesError::Credentials(ref err) => err.description(),
            DescribeScalingPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScalingPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeScheduledActions
#[derive(Debug, PartialEq)]
pub enum DescribeScheduledActionsError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeScheduledActionsError {
    pub fn from_body(body: &str) -> DescribeScheduledActionsError {
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
                    "ConcurrentUpdateException" => {
                        DescribeScheduledActionsError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        DescribeScheduledActionsError::InternalService(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeScheduledActionsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeScheduledActionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeScheduledActionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeScheduledActionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeScheduledActionsError {
    fn from(err: serde_json::error::Error) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeScheduledActionsError {
    fn from(err: CredentialsError) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeScheduledActionsError {
    fn from(err: HttpDispatchError) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeScheduledActionsError {
    fn from(err: io::Error) -> DescribeScheduledActionsError {
        DescribeScheduledActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeScheduledActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeScheduledActionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeScheduledActionsError::ConcurrentUpdate(ref cause) => cause,
            DescribeScheduledActionsError::InternalService(ref cause) => cause,
            DescribeScheduledActionsError::InvalidNextToken(ref cause) => cause,
            DescribeScheduledActionsError::Validation(ref cause) => cause,
            DescribeScheduledActionsError::Credentials(ref err) => err.description(),
            DescribeScheduledActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeScheduledActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> on your behalf.</p>
    FailedResourceAccess(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="http://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutScalingPolicyError {
    pub fn from_body(body: &str) -> PutScalingPolicyError {
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
                    "ConcurrentUpdateException" => {
                        PutScalingPolicyError::ConcurrentUpdate(String::from(error_message))
                    }
                    "FailedResourceAccessException" => {
                        PutScalingPolicyError::FailedResourceAccess(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        PutScalingPolicyError::InternalService(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutScalingPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ObjectNotFoundException" => {
                        PutScalingPolicyError::ObjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutScalingPolicyError {
    fn from(err: serde_json::error::Error) -> PutScalingPolicyError {
        PutScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutScalingPolicyError {
    fn from(err: CredentialsError) -> PutScalingPolicyError {
        PutScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutScalingPolicyError {
    fn from(err: HttpDispatchError) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutScalingPolicyError {
    fn from(err: io::Error) -> PutScalingPolicyError {
        PutScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutScalingPolicyError::ConcurrentUpdate(ref cause) => cause,
            PutScalingPolicyError::FailedResourceAccess(ref cause) => cause,
            PutScalingPolicyError::InternalService(ref cause) => cause,
            PutScalingPolicyError::LimitExceeded(ref cause) => cause,
            PutScalingPolicyError::ObjectNotFound(ref cause) => cause,
            PutScalingPolicyError::Validation(ref cause) => cause,
            PutScalingPolicyError::Credentials(ref err) => err.description(),
            PutScalingPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutScheduledAction
#[derive(Debug, PartialEq)]
pub enum PutScheduledActionError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="http://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutScheduledActionError {
    pub fn from_body(body: &str) -> PutScheduledActionError {
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
                    "ConcurrentUpdateException" => {
                        PutScheduledActionError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        PutScheduledActionError::InternalService(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutScheduledActionError::LimitExceeded(String::from(error_message))
                    }
                    "ObjectNotFoundException" => {
                        PutScheduledActionError::ObjectNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutScheduledActionError::Validation(error_message.to_string())
                    }
                    _ => PutScheduledActionError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutScheduledActionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutScheduledActionError {
    fn from(err: serde_json::error::Error) -> PutScheduledActionError {
        PutScheduledActionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutScheduledActionError {
    fn from(err: CredentialsError) -> PutScheduledActionError {
        PutScheduledActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutScheduledActionError {
    fn from(err: HttpDispatchError) -> PutScheduledActionError {
        PutScheduledActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutScheduledActionError {
    fn from(err: io::Error) -> PutScheduledActionError {
        PutScheduledActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutScheduledActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutScheduledActionError {
    fn description(&self) -> &str {
        match *self {
            PutScheduledActionError::ConcurrentUpdate(ref cause) => cause,
            PutScheduledActionError::InternalService(ref cause) => cause,
            PutScheduledActionError::LimitExceeded(ref cause) => cause,
            PutScheduledActionError::ObjectNotFound(ref cause) => cause,
            PutScheduledActionError::Validation(ref cause) => cause,
            PutScheduledActionError::Credentials(ref err) => err.description(),
            PutScheduledActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutScheduledActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum RegisterScalableTargetError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="http://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterScalableTargetError {
    pub fn from_body(body: &str) -> RegisterScalableTargetError {
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
                    "ConcurrentUpdateException" => {
                        RegisterScalableTargetError::ConcurrentUpdate(String::from(error_message))
                    }
                    "InternalServiceException" => {
                        RegisterScalableTargetError::InternalService(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RegisterScalableTargetError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterScalableTargetError::Validation(error_message.to_string())
                    }
                    _ => RegisterScalableTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterScalableTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterScalableTargetError {
    fn from(err: serde_json::error::Error) -> RegisterScalableTargetError {
        RegisterScalableTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterScalableTargetError {
    fn from(err: CredentialsError) -> RegisterScalableTargetError {
        RegisterScalableTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterScalableTargetError {
    fn from(err: HttpDispatchError) -> RegisterScalableTargetError {
        RegisterScalableTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterScalableTargetError {
    fn from(err: io::Error) -> RegisterScalableTargetError {
        RegisterScalableTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterScalableTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterScalableTargetError {
    fn description(&self) -> &str {
        match *self {
            RegisterScalableTargetError::ConcurrentUpdate(ref cause) => cause,
            RegisterScalableTargetError::InternalService(ref cause) => cause,
            RegisterScalableTargetError::LimitExceeded(ref cause) => cause,
            RegisterScalableTargetError::Validation(ref cause) => cause,
            RegisterScalableTargetError::Credentials(ref err) => err.description(),
            RegisterScalableTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterScalableTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Application Auto Scaling API. Application Auto Scaling clients implement this trait.
pub trait ApplicationAutoScaling {
    /// <p>Deletes the specified Application Auto Scaling scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyRequest,
    ) -> RusotoFuture<DeleteScalingPolicyResponse, DeleteScalingPolicyError>;

    /// <p>Deletes the specified Application Auto Scaling scheduled action.</p>
    fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionRequest,
    ) -> RusotoFuture<DeleteScheduledActionResponse, DeleteScheduledActionError>;

    /// <p>Deregisters a scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>.</p>
    fn deregister_scalable_target(
        &self,
        input: DeregisterScalableTargetRequest,
    ) -> RusotoFuture<DeregisterScalableTargetResponse, DeregisterScalableTargetError>;

    /// <p>Gets information about the scalable targets in the specified namespace.</p> <p>You can filter the results using the <code>ResourceIds</code> and <code>ScalableDimension</code> parameters.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    fn describe_scalable_targets(
        &self,
        input: DescribeScalableTargetsRequest,
    ) -> RusotoFuture<DescribeScalableTargetsResponse, DescribeScalableTargetsError>;

    /// <p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using the <code>ResourceId</code> and <code>ScalableDimension</code> parameters.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesRequest,
    ) -> RusotoFuture<DescribeScalingActivitiesResponse, DescribeScalingActivitiesError>;

    /// <p>Describes the scaling policies for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code> parameters.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesRequest,
    ) -> RusotoFuture<DescribeScalingPoliciesResponse, DescribeScalingPoliciesError>;

    /// <p>Describes the scheduled actions for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>ScheduledActionNames</code> parameters.</p> <p>To create a scheduled action or update an existing one, see <a>PutScheduledAction</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsRequest,
    ) -> RusotoFuture<DescribeScheduledActionsResponse, DescribeScheduledActionsError>;

    /// <p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy until you register the scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    fn put_scaling_policy(
        &self,
        input: PutScalingPolicyRequest,
    ) -> RusotoFuture<PutScalingPolicyResponse, PutScalingPolicyError>;

    /// <p>Creates or updates a scheduled action for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scheduled action applies to the scalable target identified by those three attributes. You cannot create a scheduled action until you register the scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update an action, specify its name and the parameters that you want to change. If you don't specify start and end times, the old values are deleted. Any other parameters that you don't specify are not changed by this update request.</p> <p>You can view the scheduled actions using <a>DescribeScheduledActions</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    fn put_scheduled_action(
        &self,
        input: PutScheduledActionRequest,
    ) -> RusotoFuture<PutScheduledActionResponse, PutScheduledActionError>;

    /// <p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out or scale in. After you have registered a scalable target, you can use this operation to update the minimum and maximum values for its scalable dimension.</p> <p>After you register a scalable target, you can create and apply scaling policies using <a>PutScalingPolicy</a>. You can view the scaling policies for a service namespace using <a>DescribeScalableTargets</a>. If you no longer need a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    fn register_scalable_target(
        &self,
        input: RegisterScalableTargetRequest,
    ) -> RusotoFuture<RegisterScalableTargetResponse, RegisterScalableTargetError>;
}
/// A client for the Application Auto Scaling API.
pub struct ApplicationAutoScalingClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ApplicationAutoScalingClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ApplicationAutoScalingClient {
        ApplicationAutoScalingClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ApplicationAutoScalingClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ApplicationAutoScalingClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ApplicationAutoScaling for ApplicationAutoScalingClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Deletes the specified Application Auto Scaling scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyRequest,
    ) -> RusotoFuture<DeleteScalingPolicyResponse, DeleteScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeleteScalingPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteScalingPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteScalingPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified Application Auto Scaling scheduled action.</p>
    fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionRequest,
    ) -> RusotoFuture<DeleteScheduledActionResponse, DeleteScheduledActionError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeleteScheduledAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteScheduledActionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteScheduledActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deregisters a scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>.</p>
    fn deregister_scalable_target(
        &self,
        input: DeregisterScalableTargetRequest,
    ) -> RusotoFuture<DeregisterScalableTargetResponse, DeregisterScalableTargetError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeregisterScalableTarget",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterScalableTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterScalableTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about the scalable targets in the specified namespace.</p> <p>You can filter the results using the <code>ResourceIds</code> and <code>ScalableDimension</code> parameters.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    fn describe_scalable_targets(
        &self,
        input: DescribeScalableTargetsRequest,
    ) -> RusotoFuture<DescribeScalableTargetsResponse, DescribeScalableTargetsError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalableTargets",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScalableTargetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalableTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using the <code>ResourceId</code> and <code>ScalableDimension</code> parameters.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesRequest,
    ) -> RusotoFuture<DescribeScalingActivitiesResponse, DescribeScalingActivitiesError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalingActivities",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScalingActivitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingActivitiesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the scaling policies for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code> parameters.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesRequest,
    ) -> RusotoFuture<DescribeScalingPoliciesResponse, DescribeScalingPoliciesError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalingPolicies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScalingPoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScalingPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the scheduled actions for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>ScheduledActionNames</code> parameters.</p> <p>To create a scheduled action or update an existing one, see <a>PutScheduledAction</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsRequest,
    ) -> RusotoFuture<DescribeScheduledActionsResponse, DescribeScheduledActionsError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScheduledActions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeScheduledActionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeScheduledActionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy until you register the scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    fn put_scaling_policy(
        &self,
        input: PutScalingPolicyRequest,
    ) -> RusotoFuture<PutScalingPolicyResponse, PutScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AnyScaleFrontendService.PutScalingPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutScalingPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutScalingPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a scheduled action for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scheduled action applies to the scalable target identified by those three attributes. You cannot create a scheduled action until you register the scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update an action, specify its name and the parameters that you want to change. If you don't specify start and end times, the old values are deleted. Any other parameters that you don't specify are not changed by this update request.</p> <p>You can view the scheduled actions using <a>DescribeScheduledActions</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    fn put_scheduled_action(
        &self,
        input: PutScheduledActionRequest,
    ) -> RusotoFuture<PutScheduledActionResponse, PutScheduledActionError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AnyScaleFrontendService.PutScheduledAction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutScheduledActionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutScheduledActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out or scale in. After you have registered a scalable target, you can use this operation to update the minimum and maximum values for its scalable dimension.</p> <p>After you register a scalable target, you can create and apply scaling policies using <a>PutScalingPolicy</a>. You can view the scaling policies for a service namespace using <a>DescribeScalableTargets</a>. If you no longer need a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    fn register_scalable_target(
        &self,
        input: RegisterScalableTargetRequest,
    ) -> RusotoFuture<RegisterScalableTargetResponse, RegisterScalableTargetError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.RegisterScalableTarget",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterScalableTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterScalableTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
