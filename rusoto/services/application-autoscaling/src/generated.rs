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
/// <p>Represents a CloudWatch alarm associated with a scaling policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Alarm {
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    #[serde(rename = "AlarmARN")]
    pub alarm_arn: String,
    /// <p>The name of the alarm.</p>
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,
}

/// <p>Represents a CloudWatch metric of your choosing for a target tracking scaling policy to use with Application Auto Scaling.</p> <p>To create your customized metric specification:</p> <ul> <li> <p>Add values for each required parameter from CloudWatch. You can use an existing metric, or a new metric that you create. To use your own metric, you must first publish the metric to CloudWatch. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publish Custom Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> </li> <li> <p>Choose a metric that changes proportionally with capacity. The value of the metric should increase or decrease in inverse proportion to the number of capacity units. That is, the value of the metric should decrease when capacity increases. </p> </li> </ul> <p>For more information about CloudWatch, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html">Amazon CloudWatch Concepts</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomizedMetricSpecification {
    /// <p>The dimensions of the metric. </p> <p>Conditional: If you published your metric with dimensions, you must specify the same dimensions in your scaling policy.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    /// <p>The name of the metric. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScalingPolicyRequest {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteScalingPolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScheduledActionRequest {
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteScheduledActionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterScalableTargetRequest {
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterScalableTargetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalableTargetsRequest {
    /// <p>The maximum number of scalable targets. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalingActivitiesRequest {
    /// <p>The maximum number of scalable targets. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScheduledActionsRequest {
    /// <p>The maximum number of scheduled action results. This value can be between 1 and 50. The default value is 50.</p> <p>If this parameter is used, the operation returns up to <code>MaxResults</code> results at a time, along with a <code>NextToken</code> value. To get the next set of results, include the <code>NextToken</code> value in a subsequent call. If this parameter is not used, the operation returns up to 50 results and a <code>NextToken</code> value, if applicable.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property. If you specify a scalable dimension, you must also specify a resource ID.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The names of the scheduled actions to describe.</p>
    #[serde(rename = "ScheduledActionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_names: Option<Vec<String>>,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Describes the dimension names and values associated with a metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The name of the dimension.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the dimension.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Represents a predefined metric for a target tracking scaling policy to use with Application Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredefinedMetricSpecification {
    /// <p>The metric type. The <code>ALBRequestCountPerTarget</code> metric type applies only to Spot Fleet requests and ECS services.</p>
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,
    /// <p><p>Identifies the resource associated with the metric type. You can&#39;t specify a resource label unless the metric type is <code>ALBRequestCountPerTarget</code> and there is a target group attached to the Spot Fleet request or ECS service.</p> <p>The format is app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt;/targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt;, where:</p> <ul> <li> <p>app/&lt;load-balancer-name&gt;/&lt;load-balancer-id&gt; is the final portion of the load balancer ARN</p> </li> <li> <p>targetgroup/&lt;target-group-name&gt;/&lt;target-group-id&gt; is the final portion of the target group ARN.</p> </li> </ul></p>
    #[serde(rename = "ResourceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScalingPolicyRequest {
    /// <p>The name of the scaling policy.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// <p>The policy type. This parameter is required if you are creating a scaling policy.</p> <p>The following policy types are supported: </p> <p> <code>TargetTrackingScaling</code>—Not supported for Amazon EMR</p> <p> <code>StepScaling</code>—Not supported for DynamoDB, Amazon Comprehend, or AWS Lambda</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html">Target Tracking Scaling Policies</a> and <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html">Step Scaling Policies</a> in the <i>Application Auto Scaling User Guide</i>.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>A step scaling policy.</p> <p>This parameter is required if you are creating a policy and the policy type is <code>StepScaling</code>.</p>
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    /// <p>A target tracking scaling policy. Includes support for predefined or customized metrics.</p> <p>This parameter is required if you are creating a policy and the policy type is <code>TargetTrackingScaling</code>.</p>
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutScalingPolicyResponse {
    /// <p>The CloudWatch alarms created for the target tracking scaling policy.</p>
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    /// <p>The Amazon Resource Name (ARN) of the resulting scaling policy.</p>
    #[serde(rename = "PolicyARN")]
    pub policy_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScheduledActionRequest {
    /// <p>The date and time for the scheduled action to end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The identifier of the resource associated with the scheduled action. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The new minimum and maximum capacity. You can set both values or just one. During the scheduled time, if the current capacity is below the minimum capacity, Application Auto Scaling scales out to the minimum capacity. If the current capacity is above the maximum capacity, Application Auto Scaling scales in to the maximum capacity.</p>
    #[serde(rename = "ScalableTargetAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    /// <p>The schedule for this action. The following formats are supported:</p> <ul> <li> <p>At expressions - "<code>at(<i>yyyy</i>-<i>mm</i>-<i>dd</i>T<i>hh</i>:<i>mm</i>:<i>ss</i>)</code>"</p> </li> <li> <p>Rate expressions - "<code>rate(<i>value</i> <i>unit</i>)</code>"</p> </li> <li> <p>Cron expressions - "<code>cron(<i>fields</i>)</code>"</p> </li> </ul> <p>At expressions are useful for one-time schedules. Specify the time, in UTC.</p> <p>For rate expressions, <i>value</i> is a positive integer and <i>unit</i> is <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code>.</p> <p>For more information about cron expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html#CronExpressions">Cron Expressions</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>The date and time for the scheduled action to start.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutScheduledActionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterScalableTargetRequest {
    /// <p>The maximum value to scale to in response to a scale-out event. <code>MaxCapacity</code> is required to register a scalable target.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i64>,
    /// <p>The minimum value to scale to in response to a scale-in event. <code>MinCapacity</code> is required to register a scalable target.</p>
    #[serde(rename = "MinCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i64>,
    /// <p><p>The identifier of the resource that is associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Application Auto Scaling creates a service-linked role that grants it permissions to modify the scalable target on your behalf. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-service-linked-roles.html">Service-Linked Roles for Application Auto Scaling</a>.</p> <p>For Amazon EMR, this parameter is required, and it must specify the ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>. </p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>An embedded object that contains attributes and attribute values that are used to suspend and resume automatic scaling. Setting the value of an attribute to <code>true</code> suspends the specified scaling activities. Setting it to <code>false</code> (default) resumes the specified scaling activities. </p> <p> <b>Suspension Outcomes</b> </p> <ul> <li> <p>For <code>DynamicScalingInSuspended</code>, while a suspension is in effect, all scale-in activities that are triggered by a scaling policy are suspended.</p> </li> <li> <p>For <code>DynamicScalingOutSuspended</code>, while a suspension is in effect, all scale-out activities that are triggered by a scaling policy are suspended.</p> </li> <li> <p>For <code>ScheduledScalingSuspended</code>, while a suspension is in effect, all scaling activities that involve scheduled actions are suspended. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-suspend-resume-scaling.html">Suspending and Resuming Scaling</a> in the <i>Application Auto Scaling User Guide</i>.</p>
    #[serde(rename = "SuspendedState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_state: Option<SuspendedState>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterScalableTargetResponse {}

/// <p>Represents a scalable target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScalableTarget {
    /// <p>The Unix timestamp for when the scalable target was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The maximum value to scale to in response to a scale-out event.</p>
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    /// <p>The minimum value to scale to in response to a scale-in event.</p>
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
    /// <p><p>The identifier of the resource associated with the scalable target. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The ARN of an IAM role that allows Application Auto Scaling to modify the scalable target on your behalf.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p><p>The scalable dimension associated with the scalable target. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    #[serde(rename = "SuspendedState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_state: Option<SuspendedState>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The identifier of the resource associated with the scaling activity. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
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

/// <p>Represents a scaling policy to use with Application Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// <p>A step scaling policy.</p>
    #[serde(rename = "StepScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    /// <p>A target tracking scaling policy.</p>
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration:
        Option<TargetTrackingScalingPolicyConfiguration>,
}

/// <p>Represents a scheduled action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduledAction {
    /// <p>The date and time that the scheduled action was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The date and time that the action is scheduled to end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p><p>The identifier of the resource associated with the scaling policy. This string consists of the resource type and unique identifier.</p> <ul> <li> <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name and service name. Example: <code>service/default/sample-webapp</code>.</p> </li> <li> <p>Spot Fleet request - The resource type is <code>spot-fleet-request</code> and the unique identifier is the Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p> </li> <li> <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID. Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p> </li> <li> <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name. Example: <code>fleet/sample-fleet</code>.</p> </li> <li> <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. Example: <code>table/my-table</code>.</p> </li> <li> <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. Example: <code>table/my-table/index/my-table-index</code>.</p> </li> <li> <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:my-db-cluster</code>.</p> </li> <li> <p>Amazon SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID. Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p> </li> <li> <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub repository</a>.</p> </li> <li> <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p> </li> <li> <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p> <ul> <li> <p> <code>ecs:service:DesiredCount</code> - The desired task count of an ECS service.</p> </li> <li> <p> <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet request.</p> </li> <li> <p> <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p> </li> <li> <p> <code>appstream:fleet:DesiredCapacity</code> - The desired capacity of an AppStream 2.0 fleet.</p> </li> <li> <p> <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p> </li> <li> <p> <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p> </li> <li> <p> <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p> </li> <li> <p> <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for an Amazon SageMaker model endpoint variant.</p> </li> <li> <p> <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p> </li> <li> <p> <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p> </li> <li> <p> <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p> </li> </ul></p>
    #[serde(rename = "ScalableDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_dimension: Option<String>,
    /// <p>The new minimum and maximum capacity. You can set both values or just one. During the scheduled time, if the current capacity is below the minimum capacity, Application Auto Scaling scales out to the minimum capacity. If the current capacity is above the maximum capacity, Application Auto Scaling scales in to the maximum capacity.</p>
    #[serde(rename = "ScalableTargetAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    /// <p>The schedule for this action. The following formats are supported:</p> <ul> <li> <p>At expressions - "<code>at(<i>yyyy</i>-<i>mm</i>-<i>dd</i>T<i>hh</i>:<i>mm</i>:<i>ss</i>)</code>"</p> </li> <li> <p>Rate expressions - "<code>rate(<i>value</i> <i>unit</i>)</code>"</p> </li> <li> <p>Cron expressions - "<code>cron(<i>fields</i>)</code>"</p> </li> </ul> <p>At expressions are useful for one-time schedules. Specify the time, in UTC.</p> <p>For rate expressions, <i>value</i> is a positive integer and <i>unit</i> is <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code>.</p> <p>For more information about cron expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html#CronExpressions">Cron Expressions</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "Schedule")]
    pub schedule: String,
    /// <p>The Amazon Resource Name (ARN) of the scheduled action.</p>
    #[serde(rename = "ScheduledActionARN")]
    pub scheduled_action_arn: String,
    /// <p>The name of the scheduled action.</p>
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    /// <p>The namespace of the AWS service that provides the resource or <code>custom-resource</code> for a resource provided by your own application or service. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
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

/// <p>Represents a step scaling policy configuration to use with Application Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepScalingPolicyConfiguration {
    /// <p>Specifies whether the <code>ScalingAdjustment</code> value in a <a>StepAdjustment</a> is an absolute number or a percentage of the current capacity. </p>
    #[serde(rename = "AdjustmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    /// <p>The amount of time, in seconds, after a scaling activity completes where previous trigger-related scaling activities can influence future scaling events.</p> <p>For scale-out policies, while the cooldown period is in effect, the capacity that has been added by the previous scale-out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out. For example, an alarm triggers a step scaling policy to scale out an Amazon ECS service by 2 tasks, the scaling activity completes successfully, and a cooldown period of 5 minutes starts. During the cooldown period, if the alarm triggers the same policy again but at a more aggressive step adjustment to scale out the service by 3 tasks, the 2 tasks that were added in the previous scale-out event are considered part of that capacity and only 1 additional task is added to the desired count.</p> <p>For scale-in policies, the cooldown period is used to block subsequent scale-in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale-out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "Cooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i64>,
    /// <p>The aggregation type for the CloudWatch metrics. Valid values are <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code>. If the aggregation type is null, the value is treated as <code>Average</code>.</p>
    #[serde(rename = "MetricAggregationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_aggregation_type: Option<String>,
    /// <p>The minimum number to adjust your scalable dimension as a result of a scaling activity. If the adjustment type is <code>PercentChangeInCapacity</code>, the scaling policy changes the scalable dimension of the scalable target by this amount.</p> <p>For example, suppose that you create a step scaling policy to scale out an Amazon ECS service by 25 percent and you specify a <code>MinAdjustmentMagnitude</code> of 2. If the service has 4 tasks and the scaling policy is performed, 25 percent of 4 is 1. However, because you specified a <code>MinAdjustmentMagnitude</code> of 2, Application Auto Scaling scales out the service by 2 tasks.</p>
    #[serde(rename = "MinAdjustmentMagnitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_adjustment_magnitude: Option<i64>,
    /// <p>A set of adjustments that enable you to scale based on the size of the alarm breach.</p>
    #[serde(rename = "StepAdjustments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,
}

/// <p>Specifies whether the scaling activities for a scalable target are in a suspended state. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuspendedState {
    /// <p>Whether scale in by a target tracking scaling policy or a step scaling policy is suspended. Set the value to <code>true</code> if you don't want Application Auto Scaling to remove capacity when a scaling policy is triggered. The default is <code>false</code>. </p>
    #[serde(rename = "DynamicScalingInSuspended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_scaling_in_suspended: Option<bool>,
    /// <p>Whether scale out by a target tracking scaling policy or a step scaling policy is suspended. Set the value to <code>true</code> if you don't want Application Auto Scaling to add capacity when a scaling policy is triggered. The default is <code>false</code>. </p>
    #[serde(rename = "DynamicScalingOutSuspended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_scaling_out_suspended: Option<bool>,
    /// <p>Whether scheduled scaling is suspended. Set the value to <code>true</code> if you don't want Application Auto Scaling to add or remove capacity by initiating scheduled actions. The default is <code>false</code>. </p>
    #[serde(rename = "ScheduledScalingSuspended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_scaling_suspended: Option<bool>,
}

/// <p>Represents a target tracking scaling policy configuration to use with Application Auto Scaling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetTrackingScalingPolicyConfiguration {
    /// <p>A customized metric. You can specify either a predefined metric or a customized metric.</p>
    #[serde(rename = "CustomizedMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    /// <p>Indicates whether scale in by the target tracking scaling policy is disabled. If the value is <code>true</code>, scale in is disabled and the target tracking scaling policy won't remove capacity from the scalable resource. Otherwise, scale in is enabled and the target tracking scaling policy can remove capacity from the scalable resource. The default value is <code>false</code>.</p>
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<bool>,
    /// <p>A predefined metric. You can specify either a predefined metric or a customized metric.</p>
    #[serde(rename = "PredefinedMetricSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    /// <p>The amount of time, in seconds, after a scale-in activity completes before another scale in activity can start.</p> <p>The cooldown period is used to block subsequent scale-in requests until it has expired. The intention is to scale in conservatively to protect your application's availability. However, if another alarm triggers a scale-out policy during the cooldown period after a scale-in, Application Auto Scaling scales out your scalable target immediately.</p>
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<i64>,
    /// <p>The amount of time, in seconds, after a scale-out activity completes before another scale-out activity can start.</p> <p>While the cooldown period is in effect, the capacity that has been added by the previous scale-out event that initiated the cooldown is calculated as part of the desired capacity for the next scale out. The intention is to continuously (but not excessively) scale out.</p>
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
}

impl DeleteScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::InternalService(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScalingPolicyError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DeleteScalingPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteScalingPolicyError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScalingPolicyError {}
/// Errors returned by DeleteScheduledAction
#[derive(Debug, PartialEq)]
pub enum DeleteScheduledActionError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
}

impl DeleteScheduledActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScheduledActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DeleteScheduledActionError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteScheduledActionError::InternalService(
                        err.msg,
                    ))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DeleteScheduledActionError::ObjectNotFound(
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
impl fmt::Display for DeleteScheduledActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScheduledActionError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DeleteScheduledActionError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteScheduledActionError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScheduledActionError {}
/// Errors returned by DeregisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum DeregisterScalableTargetError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
}

impl DeregisterScalableTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterScalableTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DeregisterScalableTargetError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeregisterScalableTargetError::InternalService(
                        err.msg,
                    ))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DeregisterScalableTargetError::ObjectNotFound(
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
impl fmt::Display for DeregisterScalableTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterScalableTargetError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DeregisterScalableTargetError::InternalService(ref cause) => write!(f, "{}", cause),
            DeregisterScalableTargetError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterScalableTargetError {}
/// Errors returned by DescribeScalableTargets
#[derive(Debug, PartialEq)]
pub enum DescribeScalableTargetsError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
}

impl DescribeScalableTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalableTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DescribeScalableTargetsError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScalableTargetsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeScalableTargetsError::InvalidNextToken(
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
impl fmt::Display for DescribeScalableTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalableTargetsError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DescribeScalableTargetsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalableTargetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalableTargetsError {}
/// Errors returned by DescribeScalingActivities
#[derive(Debug, PartialEq)]
pub enum DescribeScalingActivitiesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
}

impl DescribeScalingActivitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingActivitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DescribeScalingActivitiesError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScalingActivitiesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeScalingActivitiesError::InvalidNextToken(
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
impl fmt::Display for DescribeScalingActivitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingActivitiesError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DescribeScalingActivitiesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalingActivitiesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalingActivitiesError {}
/// Errors returned by DescribeScalingPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPoliciesError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> on your behalf.</p>
    FailedResourceAccess(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
}

impl DescribeScalingPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "FailedResourceAccessException" => {
                    return RusotoError::Service(
                        DescribeScalingPoliciesError::FailedResourceAccess(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::InvalidNextToken(
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
impl fmt::Display for DescribeScalingPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingPoliciesError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::FailedResourceAccess(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalingPoliciesError {}
/// Errors returned by DescribeScheduledActions
#[derive(Debug, PartialEq)]
pub enum DescribeScheduledActionsError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>The next token supplied was invalid.</p>
    InvalidNextToken(String),
}

impl DescribeScheduledActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScheduledActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(DescribeScheduledActionsError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScheduledActionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeScheduledActionsError::InvalidNextToken(
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
impl fmt::Display for DescribeScheduledActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScheduledActionsError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            DescribeScheduledActionsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScheduledActionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScheduledActionsError {}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a scaling policy due to a client error, for example, if the role ARN specified for a scalable target does not have permission to call the CloudWatch <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html">DescribeAlarms</a> on your behalf.</p>
    FailedResourceAccess(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="https://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
}

impl PutScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(PutScalingPolicyError::ConcurrentUpdate(err.msg))
                }
                "FailedResourceAccessException" => {
                    return RusotoError::Service(PutScalingPolicyError::FailedResourceAccess(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutScalingPolicyError::InternalService(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutScalingPolicyError::LimitExceeded(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(PutScalingPolicyError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutScalingPolicyError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::FailedResourceAccess(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutScalingPolicyError {}
/// Errors returned by PutScheduledAction
#[derive(Debug, PartialEq)]
pub enum PutScheduledActionError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="https://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
    /// <p>The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable target with the specified service namespace, resource ID, and scalable dimension does not exist. For any operation that deletes or deregisters a resource, this exception is thrown if the resource cannot be found.</p>
    ObjectNotFound(String),
}

impl PutScheduledActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutScheduledActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(PutScheduledActionError::ConcurrentUpdate(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutScheduledActionError::InternalService(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutScheduledActionError::LimitExceeded(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(PutScheduledActionError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutScheduledActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutScheduledActionError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            PutScheduledActionError::InternalService(ref cause) => write!(f, "{}", cause),
            PutScheduledActionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutScheduledActionError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutScheduledActionError {}
/// Errors returned by RegisterScalableTarget
#[derive(Debug, PartialEq)]
pub enum RegisterScalableTargetError {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update.</p>
    ConcurrentUpdate(String),
    /// <p>The service encountered an internal error.</p>
    InternalService(String),
    /// <p>A per-account resource limit is exceeded. For more information, see <a href="https://docs.aws.amazon.com/ApplicationAutoScaling/latest/userguide/application-auto-scaling-limits.html">Application Auto Scaling Limits</a>.</p>
    LimitExceeded(String),
}

impl RegisterScalableTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterScalableTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentUpdateException" => {
                    return RusotoError::Service(RegisterScalableTargetError::ConcurrentUpdate(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(RegisterScalableTargetError::InternalService(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterScalableTargetError::LimitExceeded(
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
impl fmt::Display for RegisterScalableTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterScalableTargetError::ConcurrentUpdate(ref cause) => write!(f, "{}", cause),
            RegisterScalableTargetError::InternalService(ref cause) => write!(f, "{}", cause),
            RegisterScalableTargetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterScalableTargetError {}
/// Trait representing the capabilities of the Application Auto Scaling API. Application Auto Scaling clients implement this trait.
#[async_trait]
pub trait ApplicationAutoScaling {
    /// <p>Deletes the specified scaling policy for an Application Auto Scaling scalable target.</p> <p>Deleting a step scaling policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html#delete-step-scaling-policy">Delete a Step Scaling Policy</a> and <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html#delete-target-tracking-policy">Delete a Target Tracking Scaling Policy</a> in the <i>Application Auto Scaling User Guide</i>.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    async fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyRequest,
    ) -> Result<DeleteScalingPolicyResponse, RusotoError<DeleteScalingPolicyError>>;

    /// <p>Deletes the specified scheduled action for an Application Auto Scaling scalable target.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-scheduled-scaling.html#delete-scheduled-action">Delete a Scheduled Action</a> in the <i>Application Auto Scaling User Guide</i>.</p>
    async fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionRequest,
    ) -> Result<DeleteScheduledActionResponse, RusotoError<DeleteScheduledActionError>>;

    /// <p>Deregisters an Application Auto Scaling scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. </p>
    async fn deregister_scalable_target(
        &self,
        input: DeregisterScalableTargetRequest,
    ) -> Result<DeregisterScalableTargetResponse, RusotoError<DeregisterScalableTargetError>>;

    /// <p>Gets information about the scalable targets in the specified namespace.</p> <p>You can filter the results using <code>ResourceIds</code> and <code>ScalableDimension</code>.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    async fn describe_scalable_targets(
        &self,
        input: DescribeScalableTargetsRequest,
    ) -> Result<DescribeScalableTargetsResponse, RusotoError<DescribeScalableTargetsError>>;

    /// <p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using <code>ResourceId</code> and <code>ScalableDimension</code>.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    async fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesRequest,
    ) -> Result<DescribeScalingActivitiesResponse, RusotoError<DescribeScalingActivitiesError>>;

    /// <p>Describes the Application Auto Scaling scaling policies for the specified service namespace.</p> <p>You can filter the results using <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code>.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    async fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesRequest,
    ) -> Result<DescribeScalingPoliciesResponse, RusotoError<DescribeScalingPoliciesError>>;

    /// <p>Describes the Application Auto Scaling scheduled actions for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>ScheduledActionNames</code> parameters.</p> <p>To create a scheduled action or update an existing one, see <a>PutScheduledAction</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsRequest,
    ) -> Result<DescribeScheduledActionsResponse, RusotoError<DescribeScheduledActionsError>>;

    /// <p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy until you have registered the resource as a scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p> <p>Multiple scaling policies can be in force at the same time for the same scalable target. You can have one or more target tracking scaling policies, one or more step scaling policies, or both. However, there is a chance that multiple policies could conflict, instructing the scalable target to scale out or in at the same time. Application Auto Scaling gives precedence to the policy that provides the largest capacity for both scale out and scale in. For example, if one policy increases capacity by 3, another policy increases capacity by 200 percent, and the current capacity is 10, Application Auto Scaling uses the policy with the highest calculated capacity (200% of 10 = 20) and scales out to 30. </p> <p>Learn more about how to work with scaling policies in the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User Guide</a>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyRequest,
    ) -> Result<PutScalingPolicyResponse, RusotoError<PutScalingPolicyError>>;

    /// <p>Creates or updates a scheduled action for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scheduled action applies to the scalable target identified by those three attributes. You cannot create a scheduled action until you have registered the resource as a scalable target using <a>RegisterScalableTarget</a>. </p> <p>To update an action, specify its name and the parameters that you want to change. If you don't specify start and end times, the old values are deleted. Any other parameters that you don't specify are not changed by this update request.</p> <p>You can view the scheduled actions using <a>DescribeScheduledActions</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p> <p>Learn more about how to work with scheduled actions in the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User Guide</a>.</p>
    async fn put_scheduled_action(
        &self,
        input: PutScheduledActionRequest,
    ) -> Result<PutScheduledActionResponse, RusotoError<PutScheduledActionError>>;

    /// <p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out and scale in. Scalable targets are uniquely identified by the combination of resource ID, scalable dimension, and namespace. </p> <p>When you register a new scalable target, you must specify values for minimum and maximum capacity. Application Auto Scaling will not scale capacity to values that are outside of this range. </p> <p>To update a scalable target, specify the parameter that you want to change as well as the following parameters that identify the scalable target: resource ID, scalable dimension, and namespace. Any parameters that you don't specify are not changed by this update request. </p> <p>After you register a scalable target, you do not need to register it again to use other Application Auto Scaling operations. To see which resources have been registered, use <a>DescribeScalableTargets</a>. You can also view the scaling policies for a service namespace by using <a>DescribeScalableTargets</a>. </p> <p>If you no longer need a scalable target, you can deregister it by using <a>DeregisterScalableTarget</a>.</p>
    async fn register_scalable_target(
        &self,
        input: RegisterScalableTargetRequest,
    ) -> Result<RegisterScalableTargetResponse, RusotoError<RegisterScalableTargetError>>;
}
/// A client for the Application Auto Scaling API.
#[derive(Clone)]
pub struct ApplicationAutoScalingClient {
    client: Client,
    region: region::Region,
}

impl ApplicationAutoScalingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApplicationAutoScalingClient {
        ApplicationAutoScalingClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApplicationAutoScalingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ApplicationAutoScalingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ApplicationAutoScalingClient {
        ApplicationAutoScalingClient { client, region }
    }
}

#[async_trait]
impl ApplicationAutoScaling for ApplicationAutoScalingClient {
    /// <p>Deletes the specified scaling policy for an Application Auto Scaling scalable target.</p> <p>Deleting a step scaling policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the scaling policy, even if it no longer has an associated action.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html#delete-step-scaling-policy">Delete a Step Scaling Policy</a> and <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html#delete-target-tracking-policy">Delete a Target Tracking Scaling Policy</a> in the <i>Application Auto Scaling User Guide</i>.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    async fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyRequest,
    ) -> Result<DeleteScalingPolicyResponse, RusotoError<DeleteScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeleteScalingPolicy",
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
                .deserialize::<DeleteScalingPolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteScalingPolicyError::from_response(response))
        }
    }

    /// <p>Deletes the specified scheduled action for an Application Auto Scaling scalable target.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-scheduled-scaling.html#delete-scheduled-action">Delete a Scheduled Action</a> in the <i>Application Auto Scaling User Guide</i>.</p>
    async fn delete_scheduled_action(
        &self,
        input: DeleteScheduledActionRequest,
    ) -> Result<DeleteScheduledActionResponse, RusotoError<DeleteScheduledActionError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeleteScheduledAction",
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
                .deserialize::<DeleteScheduledActionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteScheduledActionError::from_response(response))
        }
    }

    /// <p>Deregisters an Application Auto Scaling scalable target.</p> <p>Deregistering a scalable target deletes the scaling policies that are associated with it.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. </p>
    async fn deregister_scalable_target(
        &self,
        input: DeregisterScalableTargetRequest,
    ) -> Result<DeregisterScalableTargetResponse, RusotoError<DeregisterScalableTargetError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DeregisterScalableTarget",
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
                .deserialize::<DeregisterScalableTargetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterScalableTargetError::from_response(response))
        }
    }

    /// <p>Gets information about the scalable targets in the specified namespace.</p> <p>You can filter the results using <code>ResourceIds</code> and <code>ScalableDimension</code>.</p> <p>To create a scalable target or update an existing one, see <a>RegisterScalableTarget</a>. If you are no longer using a scalable target, you can deregister it using <a>DeregisterScalableTarget</a>.</p>
    async fn describe_scalable_targets(
        &self,
        input: DescribeScalableTargetsRequest,
    ) -> Result<DescribeScalableTargetsResponse, RusotoError<DescribeScalableTargetsError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalableTargets",
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
                .deserialize::<DescribeScalableTargetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScalableTargetsError::from_response(response))
        }
    }

    /// <p>Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks.</p> <p>You can filter the results using <code>ResourceId</code> and <code>ScalableDimension</code>.</p> <p>Scaling activities are triggered by CloudWatch alarms that are associated with scaling policies. To view the scaling policies for a service namespace, see <a>DescribeScalingPolicies</a>. To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>.</p>
    async fn describe_scaling_activities(
        &self,
        input: DescribeScalingActivitiesRequest,
    ) -> Result<DescribeScalingActivitiesResponse, RusotoError<DescribeScalingActivitiesError>>
    {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalingActivities",
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
                .deserialize::<DescribeScalingActivitiesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScalingActivitiesError::from_response(response))
        }
    }

    /// <p>Describes the Application Auto Scaling scaling policies for the specified service namespace.</p> <p>You can filter the results using <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>PolicyNames</code>.</p> <p>To create a scaling policy or update an existing one, see <a>PutScalingPolicy</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p>
    async fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesRequest,
    ) -> Result<DescribeScalingPoliciesResponse, RusotoError<DescribeScalingPoliciesError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScalingPolicies",
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
                .deserialize::<DescribeScalingPoliciesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScalingPoliciesError::from_response(response))
        }
    }

    /// <p>Describes the Application Auto Scaling scheduled actions for the specified service namespace.</p> <p>You can filter the results using the <code>ResourceId</code>, <code>ScalableDimension</code>, and <code>ScheduledActionNames</code> parameters.</p> <p>To create a scheduled action or update an existing one, see <a>PutScheduledAction</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p>
    async fn describe_scheduled_actions(
        &self,
        input: DescribeScheduledActionsRequest,
    ) -> Result<DescribeScheduledActionsResponse, RusotoError<DescribeScheduledActionsError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.DescribeScheduledActions",
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
                .deserialize::<DescribeScheduledActionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScheduledActionsError::from_response(response))
        }
    }

    /// <p>Creates or updates a policy for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the scalable target identified by those three attributes. You cannot create a scaling policy until you have registered the resource as a scalable target using <a>RegisterScalableTarget</a>.</p> <p>To update a policy, specify its policy name and the parameters that you want to change. Any parameters that you don't specify are not changed by this update request.</p> <p>You can view the scaling policies for a service namespace using <a>DescribeScalingPolicies</a>. If you are no longer using a scaling policy, you can delete it using <a>DeleteScalingPolicy</a>.</p> <p>Multiple scaling policies can be in force at the same time for the same scalable target. You can have one or more target tracking scaling policies, one or more step scaling policies, or both. However, there is a chance that multiple policies could conflict, instructing the scalable target to scale out or in at the same time. Application Auto Scaling gives precedence to the policy that provides the largest capacity for both scale out and scale in. For example, if one policy increases capacity by 3, another policy increases capacity by 200 percent, and the current capacity is 10, Application Auto Scaling uses the policy with the highest calculated capacity (200% of 10 = 20) and scales out to 30. </p> <p>Learn more about how to work with scaling policies in the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User Guide</a>.</p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyRequest,
    ) -> Result<PutScalingPolicyResponse, RusotoError<PutScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AnyScaleFrontendService.PutScalingPolicy");
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
                .deserialize::<PutScalingPolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutScalingPolicyError::from_response(response))
        }
    }

    /// <p>Creates or updates a scheduled action for an Application Auto Scaling scalable target.</p> <p>Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scheduled action applies to the scalable target identified by those three attributes. You cannot create a scheduled action until you have registered the resource as a scalable target using <a>RegisterScalableTarget</a>. </p> <p>To update an action, specify its name and the parameters that you want to change. If you don't specify start and end times, the old values are deleted. Any other parameters that you don't specify are not changed by this update request.</p> <p>You can view the scheduled actions using <a>DescribeScheduledActions</a>. If you are no longer using a scheduled action, you can delete it using <a>DeleteScheduledAction</a>.</p> <p>Learn more about how to work with scheduled actions in the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User Guide</a>.</p>
    async fn put_scheduled_action(
        &self,
        input: PutScheduledActionRequest,
    ) -> Result<PutScheduledActionResponse, RusotoError<PutScheduledActionError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AnyScaleFrontendService.PutScheduledAction");
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
                .deserialize::<PutScheduledActionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutScheduledActionError::from_response(response))
        }
    }

    /// <p>Registers or updates a scalable target. A scalable target is a resource that Application Auto Scaling can scale out and scale in. Scalable targets are uniquely identified by the combination of resource ID, scalable dimension, and namespace. </p> <p>When you register a new scalable target, you must specify values for minimum and maximum capacity. Application Auto Scaling will not scale capacity to values that are outside of this range. </p> <p>To update a scalable target, specify the parameter that you want to change as well as the following parameters that identify the scalable target: resource ID, scalable dimension, and namespace. Any parameters that you don't specify are not changed by this update request. </p> <p>After you register a scalable target, you do not need to register it again to use other Application Auto Scaling operations. To see which resources have been registered, use <a>DescribeScalableTargets</a>. You can also view the scaling policies for a service namespace by using <a>DescribeScalableTargets</a>. </p> <p>If you no longer need a scalable target, you can deregister it by using <a>DeregisterScalableTarget</a>.</p>
    async fn register_scalable_target(
        &self,
        input: RegisterScalableTargetRequest,
    ) -> Result<RegisterScalableTargetResponse, RusotoError<RegisterScalableTargetError>> {
        let mut request = SignedRequest::new("POST", "application-autoscaling", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AnyScaleFrontendService.RegisterScalableTarget",
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
                .deserialize::<RegisterScalableTargetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterScalableTargetError::from_response(response))
        }
    }
}
