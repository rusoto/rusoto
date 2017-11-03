
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Represents a CloudWatch alarm associated with a scaling policy.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Alarm {
    #[doc="<p>The Amazon Resource Name (ARN) of the alarm.</p>"]
    #[serde(rename="AlarmARN")]
    pub alarm_arn: String,
    #[doc="<p>The name of the alarm.</p>"]
    #[serde(rename="AlarmName")]
    pub alarm_name: String,
}

#[doc="<p>Configures a customized metric for a target tracking policy.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CustomizedMetricSpecification {
    #[doc="<p>The dimensions of the metric.</p>"]
    #[serde(rename="Dimensions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[doc="<p>The name of the metric.</p>"]
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[doc="<p>The namespace of the metric.</p>"]
    #[serde(rename="Namespace")]
    pub namespace: String,
    #[doc="<p>The statistic of the metric.</p>"]
    #[serde(rename="Statistic")]
    pub statistic: String,
    #[doc="<p>The unit of the metric.</p>"]
    #[serde(rename="Unit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteScalingPolicyRequest {
    #[doc="<p>The name of the scaling policy.</p>"]
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    #[doc="<p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteScalingPolicyResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterScalableTargetRequest {
    #[doc="<p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeregisterScalableTargetResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeScalableTargetsRequest {
    #[doc="<p>The maximum number of scalable target results. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[doc="<p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeScalableTargetsResponse {
    #[doc="<p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of scalable targets that matches the request parameters.</p>"]
    #[serde(rename="ScalableTargets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scalable_targets: Option<Vec<ScalableTarget>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeScalingActivitiesRequest {
    #[doc="<p>The maximum number of scalable target results. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_id: Option<String>,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeScalingActivitiesResponse {
    #[doc="<p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of scaling activity objects.</p>"]
    #[serde(rename="ScalingActivities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scaling_activities: Option<Vec<ScalingActivity>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeScalingPoliciesRequest {
    #[doc="<p>The maximum number of scalable target results. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The names of the scaling policies to describe.</p>"]
    #[serde(rename="PolicyNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    #[doc="<p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_id: Option<String>,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scalable_dimension: Option<String>,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeScalingPoliciesResponse {
    #[doc="<p>The token required to get the next set of results. This value is <code>null</code> if there are no more results to return.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of scaling policy objects.</p>"]
    #[serde(rename="ScalingPolicies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[doc="<p>Describes the dimension of a metric.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MetricDimension {
    #[doc="<p>The name of the dimension.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The value of the dimension.</p>"]
    #[serde(rename="Value")]
    pub value: String,
}

#[doc="<p>Configures a predefined metric for a target tracking policy.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PredefinedMetricSpecification {
    #[doc="<p>The metric type.</p>"]
    #[serde(rename="PredefinedMetricType")]
    pub predefined_metric_type: String,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="ResourceLabel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutScalingPolicyRequest {
    #[doc="<p>The name of the scaling policy.</p>"]
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    #[doc="<p>The policy type. If you are creating a new policy, this parameter is required. If you are updating a policy, this parameter is not required.</p> <p>For DynamoDB, only <code>TargetTrackingScaling</code> is supported. For any other service, only <code>StepScaling</code> is supported.</p>"]
    #[serde(rename="PolicyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_type: Option<String>,
    #[doc="<p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
    #[doc="<p>A step scaling policy.</p> <p>This parameter is required if you are creating a policy and the policy type is <code>StepScaling</code>.</p>"]
    #[serde(rename="StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    #[doc="<p>A target tracking policy.</p> <p>This parameter is required if you are creating a new policy and the policy type is <code>TargetTrackingScaling</code>.</p>"]
    #[serde(rename="TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutScalingPolicyResponse {
    #[doc="<p>The CloudWatch alarms created for the target tracking policy.</p>"]
    #[serde(rename="Alarms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    #[doc="<p>The Amazon Resource Name (ARN) of the resulting scaling policy.</p>"]
    #[serde(rename="PolicyARN")]
    pub policy_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterScalableTargetRequest {
    #[doc="<p>The maximum value to scale to in response to a scale out event. This parameter is required if you are registering a scalable target and optional if you are updating one.</p>"]
    #[serde(rename="MaxCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_capacity: Option<i64>,
    #[doc="<p>The minimum value to scale to in response to a scale in event. This parameter is required if you are registering a scalable target and optional if you are updating one.</p>"]
    #[serde(rename="MinCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_capacity: Option<i64>,
    #[doc="<p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf. This parameter is required when you register a scalable target and optional when you update one.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterScalableTargetResponse;

#[doc="<p>Represents a scalable target.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScalableTarget {
    #[doc="<p>The Unix timestamp for when the scalable target was created.</p>"]
    #[serde(rename="CreationTime")]
    pub creation_time: f64,
    #[doc="<p>The maximum value to scale to in response to a scale out event.</p>"]
    #[serde(rename="MaxCapacity")]
    pub max_capacity: i64,
    #[doc="<p>The minimum value to scale to in response to a scale in event.</p>"]
    #[serde(rename="MinCapacity")]
    pub min_capacity: i64,
    #[doc="<p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
}

#[doc="<p>Represents a scaling activity.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScalingActivity {
    #[doc="<p>The unique identifier of the scaling activity.</p>"]
    #[serde(rename="ActivityId")]
    pub activity_id: String,
    #[doc="<p>A simple description of what caused the scaling activity to happen.</p>"]
    #[serde(rename="Cause")]
    pub cause: String,
    #[doc="<p>A simple description of what action the scaling activity intends to accomplish.</p>"]
    #[serde(rename="Description")]
    pub description: String,
    #[doc="<p>The details about the scaling activity.</p>"]
    #[serde(rename="Details")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub details: Option<String>,
    #[doc="<p>The Unix timestamp for when the scaling activity ended.</p>"]
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<f64>,
    #[doc="<p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
    #[doc="<p>The Unix timestamp for when the scaling activity began.</p>"]
    #[serde(rename="StartTime")]
    pub start_time: f64,
    #[doc="<p>Indicates the status of the scaling activity.</p>"]
    #[serde(rename="StatusCode")]
    pub status_code: String,
    #[doc="<p>A simple message about the current status of the scaling activity.</p>"]
    #[serde(rename="StatusMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_message: Option<String>,
}

#[doc="<p>Represents a scaling policy.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScalingPolicy {
    #[doc="<p>The CloudWatch alarms associated with the scaling policy.</p>"]
    #[serde(rename="Alarms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    #[doc="<p>The Unix timestamp for when the scaling policy was created.</p>"]
    #[serde(rename="CreationTime")]
    pub creation_time: f64,
    #[doc="<p>The Amazon Resource Name (ARN) of the scaling policy.</p>"]
    #[serde(rename="PolicyARN")]
    pub policy_arn: String,
    #[doc="<p>The name of the scaling policy.</p>"]
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    #[doc="<p>The scaling policy type.</p>"]
    #[serde(rename="PolicyType")]
    pub policy_type: String,
    #[doc="<p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the resource ID. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the resource ID. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> </ul>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> </ul>"]
    #[serde(rename="ScalableDimension")]
    pub scalable_dimension: String,
    #[doc="<p>The namespace of the AWS service. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces\">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="ServiceNamespace")]
    pub service_namespace: String,
    #[doc="<p>A step scaling policy.</p>"]
    #[serde(rename="StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    #[doc="<p>A target tracking policy.</p>"]
    #[serde(rename="TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[doc="<p>Represents a step adjustment for a <a>StepScalingPolicyConfiguration</a>. Describes an adjustment based on the difference between the value of the aggregated CloudWatch metric and the breach threshold that you've defined for the alarm. </p> <p>For the following examples, suppose that you have an alarm with a breach threshold of 50:</p> <ul> <li> <p>To trigger the adjustment when the metric is greater than or equal to 50 and less than 60, specify a lower bound of 0 and an upper bound of 10.</p> </li> <li> <p>To trigger the adjustment when the metric is greater than 40 and less than or equal to 50, specify a lower bound of -10 and an upper bound of 0.</p> </li> </ul> <p>There are a few rules for the step adjustments for your step policy:</p> <ul> <li> <p>The ranges of your step adjustments can't overlap or have a gap.</p> </li> <li> <p>At most one step adjustment can have a null lower bound. If one step adjustment has a negative lower bound, then there must be a step adjustment with a null lower bound.</p> </li> <li> <p>At most one step adjustment can have a null upper bound. If one step adjustment has a positive upper bound, then there must be a step adjustment with a null upper bound.</p> </li> <li> <p>The upper and lower bound can't be null in the same step adjustment.</p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StepAdjustment {
    #[doc="<p>The lower bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the lower bound is inclusive (the metric must be greater than or equal to the threshold plus the lower bound). Otherwise, it is exclusive (the metric must be greater than the threshold plus the lower bound). A null value indicates negative infinity.</p>"]
    #[serde(rename="MetricIntervalLowerBound")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_interval_lower_bound: Option<f64>,
    #[doc="<p>The upper bound for the difference between the alarm threshold and the CloudWatch metric. If the metric value is above the breach threshold, the upper bound is exclusive (the metric must be less than the threshold plus the upper bound). Otherwise, it is inclusive (the metric must be less than or equal to the threshold plus the upper bound). A null value indicates positive infinity.</p> <p>The upper bound must be greater than the lower bound.</p>"]
    #[serde(rename="MetricIntervalUpperBound")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_interval_upper_bound: Option<f64>,
    #[doc="<p>The amount by which to scale, based on the specified adjustment type. A positive value adds to the current scalable dimension while a negative number removes from the current scalable dimension.</p>"]
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: i64,
}

#[doc="<p>Represents a step scaling policy configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StepScalingPolicyConfiguration {
    #[doc="<p>The adjustment type, which specifies how the <code>ScalingAdjustment</code> parameter in a <a>StepAdjustment</a> is interpreted.</p>"]
    #[serde(rename="AdjustmentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub adjustment_type: Option<String>,
    #[doc="<p>The amount of time, in seconds, after a scaling activity completes where previous trigger-related scaling activities can influence future scaling events.</p> <p>For scale out policies, while the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out. For example, an alarm triggers a step scaling policy to scale out an Amazon ECS service by 2 tasks, the scaling activity completes successfully, and a cooldown period of 5 minutes starts. During the Cooldown period, if the alarm triggers the same policy again but at a more aggressive step adjustment to scale out the service by 3 tasks, the 2 tasks that were added in the previous scale out event are considered part of that capacity and only 1 additional task is added to the desired count.</p> <p>For scale in policies, the cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>"]
    #[serde(rename="Cooldown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cooldown: Option<i64>,
    #[doc="<p>The aggregation type for the CloudWatch metrics. Valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>.</p>"]
    #[serde(rename="MetricAggregationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    #[doc="<p>The minimum number to adjust your scalable dimension as a result of a scaling activity. If the adjustment type is <code>PercentChangeInCapacity</code>, the scaling policy changes the scalable dimension of the scalable target by this amount.</p>"]
    #[serde(rename="MinAdjustmentMagnitude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_adjustment_magnitude: Option<i64>,
    #[doc="<p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>"]
    #[serde(rename="StepAdjustments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,
}

#[doc="<p>Represents a target tracking scaling policy configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TargetTrackingScalingPolicyConfiguration {
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="CustomizedMetricSpecification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    #[doc="<p>A predefined metric.</p>"]
    #[serde(rename="PredefinedMetricSpecification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    #[doc="<p>The amount of time, in seconds, after a scale in activity completes before another scale in activity can start.</p> <p>The cooldown period is used to block subsequent scale in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>"]
    #[serde(rename="ScaleInCooldown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scale_in_cooldown: Option<i64>,
    #[doc="<p>The amount of time, in seconds, after a scale out activity completes before another scale out activity can start.</p> <p>While the cooldown period is in effect, the capacity that has been added by the previous scale out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out.</p>"]
    #[serde(rename="ScaleOutCooldown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scale_out_cooldown: Option<i64>,
    #[doc="<p>The target value for the metric. The range is 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2).</p>"]
    #[serde(rename="TargetValue")]
    pub target_value: f64,
}

/// Errors returned by DeleteScalingPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPolicyError {
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>The specified object could not be found. For any <code>Put</code> or <code>Register</code> API operation, which depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any <code>Delete</code> or <code>Deregister</code> API operation, this exception is thrown if the resource that is to be deleted or deregistered cannot be found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DeregisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum DeregisterScalableTargetError {
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>The specified object could not be found. For any <code>Put</code> or <code>Register</code> API operation, which depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any <code>Delete</code> or <code>Deregister</code> API operation, this exception is thrown if the resource that is to be deleted or deregistered cannot be found.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>The next token supplied was invalid.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>The next token supplied was invalid.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentUpdateException" => DescribeScalingActivitiesError::ConcurrentUpdate(String::from(error_message)),
                    "InternalServiceException" => {
                        DescribeScalingActivitiesError::InternalService(String::from(error_message))
                    }
                    "InvalidNextTokenException" => DescribeScalingActivitiesError::InvalidNextToken(String::from(error_message)),
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
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> API operation on behalf of your account.</p>
    FailedResourceAccess(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>The next token supplied was invalid.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentUpdateException" => {
                        DescribeScalingPoliciesError::ConcurrentUpdate(String::from(error_message))
                    }
                    "FailedResourceAccessException" => DescribeScalingPoliciesError::FailedResourceAccess(String::from(error_message)),
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
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> API operation on behalf of your account.</p>
    FailedResourceAccess(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>Your account exceeded a limit. This exception is thrown when a per-account resource limit is exceeded. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_as-app">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    ///<p>The specified object could not be found. For any <code>Put</code> or <code>Register</code> API operation, which depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any <code>Delete</code> or <code>Deregister</code> API operation, this exception is thrown if the resource that is to be deleted or deregistered cannot be found.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by RegisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum RegisterScalableTargetError {
    ///<p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    ///<p>The service encountered an internal error.</p>
    InternalService(String),
    ///<p>Your account exceeded a limit. This exception is thrown when a per-account resource limit is exceeded. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_as-app">Application Auto Scaling Limits</a>.</p>
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
                let raw_error_type = json.get("__type")
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
    #[doc="<p>Deletes the specified Application Auto Scaling scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>"]
    fn delete_scaling_policy(&self,
                             input: &DeleteScalingPolicyRequest)
                             -> Result<DeleteScalingPolicyResponse, DeleteScalingPolicyError>;


    #[doc="<p>Deregisters a scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>.</p>"]
    fn deregister_scalable_target
        (&self,
         input: &DeregisterScalableTargetRequest)
         -> Result<DeregisterScalableTargetResponse, DeregisterScalableTargetError>;


    #[doc="<p>Provides descriptive information about the scalable targets in the specified namespace.</p> <p>You can filter the results using the <code>ResourceIds</code> and <code>ScalableDimension</code> parameters.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>"]
    fn describe_scalable_targets
        (&self,
         input: &DescribeScalableTargetsRequest)
         -> Result<DescribeScalableTargetsResponse, DescribeScalableTargetsError>;


    #[doc="<p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using the <code>ResourceId</code> and <code>ScalableDimension</code> parameters.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>"]
    fn describe_scaling_activities
        (&self,
         input: &DescribeScalingActivitiesRequest)
         -> Result<DescribeScalingActivitiesResponse, DescribeScalingActivitiesError>;


    #[doc="<p>Provides descriptive information about the scaling policies in the specified namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code> parameters.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>"]
    fn describe_scaling_policies
        (&self,
         input: &DescribeScalingPoliciesRequest)
         -> Result<DescribeScalingPoliciesResponse, DescribeScalingPoliciesError>;


    #[doc="<p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy without first registering a scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>"]
    fn put_scaling_policy(&self,
                          input: &PutScalingPolicyRequest)
                          -> Result<PutScalingPolicyResponse, PutScalingPolicyError>;


    #[doc="<p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out or scale in. After you have registered a scalable target, you can use this operation to update the minimum and maximum values for your scalable dimension.</p> <p>After you register a scalable target, you can create and apply scaling policies using <a>PutScalingPolicy</a>. You can view the scaling policies for a service namespace using <a>DescribeScalableTargets</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>"]
    fn register_scalable_target
        (&self,
         input: &RegisterScalableTargetRequest)
         -> Result<RegisterScalableTargetResponse, RegisterScalableTargetError>;
}
/// A client for the Application Auto Scaling API.
pub struct ApplicationAutoScalingClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> ApplicationAutoScalingClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ApplicationAutoScalingClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> ApplicationAutoScaling for ApplicationAutoScalingClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deletes the specified Application Auto Scaling scaling policy.</p> <p>Deleting a policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>"]
    fn delete_scaling_policy(&self,
                             input: &DeleteScalingPolicyRequest)
                             -> Result<DeleteScalingPolicyResponse, DeleteScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.DeleteScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteScalingPolicyResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteScalingPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters a scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>.</p>"]
    fn deregister_scalable_target
        (&self,
         input: &DeregisterScalableTargetRequest)
         -> Result<DeregisterScalableTargetResponse, DeregisterScalableTargetError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.DeregisterScalableTarget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeregisterScalableTargetResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeregisterScalableTargetError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Provides descriptive information about the scalable targets in the specified namespace.</p> <p>You can filter the results using the <code>ResourceIds</code> and <code>ScalableDimension</code> parameters.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>"]
    fn describe_scalable_targets
        (&self,
         input: &DescribeScalableTargetsRequest)
         -> Result<DescribeScalableTargetsResponse, DescribeScalableTargetsError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.DescribeScalableTargets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeScalableTargetsResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeScalableTargetsError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using the <code>ResourceId</code> and <code>ScalableDimension</code> parameters.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>"]
    fn describe_scaling_activities
        (&self,
         input: &DescribeScalingActivitiesRequest)
         -> Result<DescribeScalingActivitiesResponse, DescribeScalingActivitiesError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.DescribeScalingActivities");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeScalingActivitiesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeScalingActivitiesError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Provides descriptive information about the scaling policies in the specified namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code> parameters.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>"]
    fn describe_scaling_policies
        (&self,
         input: &DescribeScalingPoliciesRequest)
         -> Result<DescribeScalingPoliciesResponse, DescribeScalingPoliciesError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.DescribeScalingPolicies");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeScalingPoliciesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeScalingPoliciesError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy without first registering a scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>"]
    fn put_scaling_policy(&self,
                          input: &PutScalingPolicyRequest)
                          -> Result<PutScalingPolicyResponse, PutScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AnyScaleFrontendService.PutScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutScalingPolicyResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutScalingPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out or scale in. After you have registered a scalable target, you can use this operation to update the minimum and maximum values for your scalable dimension.</p> <p>After you register a scalable target, you can create and apply scaling policies using <a>PutScalingPolicy</a>. You can view the scaling policies for a service namespace using <a>DescribeScalableTargets</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>"]
    fn register_scalable_target
        (&self,
         input: &RegisterScalableTargetRequest)
         -> Result<RegisterScalableTargetResponse, RegisterScalableTargetError> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");
        request.set_endpoint_prefix("autoscaling".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AnyScaleFrontendService.RegisterScalableTarget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RegisterScalableTargetResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterScalableTargetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
