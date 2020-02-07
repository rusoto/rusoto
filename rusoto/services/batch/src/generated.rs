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
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An object representing an AWS Batch array job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ArrayProperties {
    /// <p>The size of the array job.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArrayPropertiesDetail {
    /// <p>The job index within the array that is associated with this job. This parameter is returned for array job children.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>A summary of the number of array job children in each available job status. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "statusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArrayPropertiesSummary {
    /// <p>The job index within the array that is associated with this job. This parameter is returned for children of array jobs.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>An object representing the details of a container that is part of a job attempt.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttemptContainerDetail {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The exit code for the job attempt. A non-zero exit code is considered a failure.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The network interfaces associated with the job attempt.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>An object representing a job attempt.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttemptDetail {
    /// <p>Details about the container in this job attempt.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<AttemptContainerDetail>,
    /// <p>The Unix timestamp (in seconds and milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job attempt.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp (in seconds and milliseconds) for when the attempt was stopped (when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRequest {
    /// <p>The AWS Batch job ID of the job to cancel.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs.</p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>An object representing an AWS Batch compute environment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComputeEnvironmentDetail {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    pub compute_environment_arn: String,
    /// <p>The name of the compute environment.</p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>The compute resources defined for the compute environment.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p>The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster used by the compute environment.</p>
    #[serde(rename = "ecsClusterArn")]
    pub ecs_cluster_arn: String,
    /// <p>The service role associated with the compute environment that allows AWS Batch to make calls to AWS API operations on your behalf.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. The valid values are <code>ENABLED</code> or <code>DISABLED</code>.</p> <p>If the state is <code>ENABLED</code>, then the AWS Batch scheduler can attempt to place jobs from an associated job queue on the compute resources within the environment. If the compute environment is managed, then it can scale its instances out or in automatically, based on the job queue demand.</p> <p>If the state is <code>DISABLED</code>, then the AWS Batch scheduler does not attempt to place jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state continue to progress normally. Managed compute environments in the <code>DISABLED</code> state do not scale out. However, they scale in to <code>minvCpus</code> value after instances become idle.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The current status of the compute environment (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the compute environment.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The type of the compute environment.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The order in which compute environments are tried for job placement within a queue. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower order integer value is tried for job placement first.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeEnvironmentOrder {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>The order of the compute environment.</p>
    #[serde(rename = "order")]
    pub order: i64,
}

/// <p>An object representing an AWS Batch compute resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeResource {
    /// <p>The allocation strategy to use for the compute resource in case not enough instances of the best fitting instance type can be allocated. This could be due to availability of the instance type in the region or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 service limits</a>. If this is not specified, the default is <code>BEST_FIT</code>, which will use only the best fitting instance type, waiting for additional capacity if it's not available. This allocation strategy keeps costs lower but can limit scaling. If you are using Spot Fleets with <code>BEST_FIT</code> then the Spot Fleet IAM Role must be specified. <code>BEST_FIT_PROGRESSIVE</code> will select additional instance types that are large enough to meet the requirements of the jobs in the queue, with a preference for instance types with a lower cost per vCPU. <code>SPOT_CAPACITY_OPTIMIZED</code> is only available for Spot Instance compute resources and will select additional instance types that are large enough to meet the requirements of the jobs in the queue, with a preference for instance types that are less likely to be interrupted. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/allocation-strategies.html ">Allocation Strategies</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "allocationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    /// <p>The maximum percentage that a Spot Instance price can be when compared with the On-Demand price for that instance type before instances are launched. For example, if your maximum percentage is 20%, then the Spot price must be below 20% of the current On-Demand price for that Amazon EC2 instance. You always pay the lowest (market) price and never more than your maximum percentage. If you leave this field empty, the default value is 100% of the On-Demand price.</p>
    #[serde(rename = "bidPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_percentage: Option<i64>,
    /// <p>The desired number of Amazon EC2 vCPUS in the compute environment.</p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p>The Amazon EC2 key pair that is used for instances launched in the compute environment.</p>
    #[serde(rename = "ec2KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_pair: Option<String>,
    /// <p>The Amazon Machine Image (AMI) ID used for instances launched in the compute environment.</p>
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The Amazon ECS instance profile applied to Amazon EC2 instances in a compute environment. You can specify the short name or full Amazon Resource Name (ARN) of an instance profile. For example, <code> <i>ecsInstanceRole</i> </code> or <code>arn:aws:iam::<i>&lt;aws_account_id&gt;</i>:instance-profile/<i>ecsInstanceRole</i> </code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/instance_IAM_role.html">Amazon ECS Instance Role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "instanceRole")]
    pub instance_role: String,
    /// <p>The instances types that may be launched. You can specify instance families to launch any instance type within those families (for example, <code>c5</code> or <code>p3</code>), or you can specify specific sizes within a family (such as <code>c5.8xlarge</code>). You can also choose <code>optimal</code> to pick instance types (from the C, M, and R instance families) on the fly that match the demand of your job queues.</p>
    #[serde(rename = "instanceTypes")]
    pub instance_types: Vec<String>,
    /// <p>The launch template to use for your compute resources. Any other compute resource parameters that you specify in a <a>CreateComputeEnvironment</a> API operation override the same parameters in the launch template. You must specify either the launch template ID or launch template name in the request, but not both. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/launch-templates.html">Launch Template Support</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "launchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The maximum number of Amazon EC2 vCPUs that an environment can reach.</p>
    #[serde(rename = "maxvCpus")]
    pub maxv_cpus: i64,
    /// <p>The minimum number of Amazon EC2 vCPUs that an environment should maintain (even if the compute environment is <code>DISABLED</code>).</p>
    #[serde(rename = "minvCpus")]
    pub minv_cpus: i64,
    /// <p>The Amazon EC2 placement group to associate with your compute resources. If you intend to submit multi-node parallel jobs to your compute environment, you should consider creating a cluster placement group and associate it with your compute resources. This keeps your multi-node parallel job on a logical grouping of instances within a single Availability Zone with high network flow potential. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    #[serde(rename = "placementGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    /// <p>The Amazon EC2 security groups associated with instances launched in the compute environment. One or more security groups must be specified, either in <code>securityGroupIds</code> or using a launch template referenced in <code>launchTemplate</code>. If security groups are specified using both <code>securityGroupIds</code> and <code>launchTemplate</code>, the values in <code>securityGroupIds</code> will be used.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a <code>SPOT</code> compute environment. This role is required if the allocation strategy set to <code>BEST_FIT</code> or if the allocation strategy is not specified. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/spot_fleet_IAM_role.html">Amazon EC2 Spot Fleet Role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "spotIamFleetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_iam_fleet_role: Option<String>,
    /// <p>The VPC subnets into which the compute resources are launched. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">VPCs and Subnets</a> in the <i>Amazon VPC User Guide</i>.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// <p>Key-value pair tags to be applied to resources that are launched in the compute environment. For AWS Batch, these take the form of "String1": "String2", where String1 is the tag key and String2 is the tag value—for example, { "Name": "AWS Batch Instance - C4OnDemand" }.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of compute environment: <code>EC2</code> or <code>SPOT</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object representing the attributes of a compute environment that can be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ComputeResourceUpdate {
    /// <p>The desired number of Amazon EC2 vCPUS in the compute environment.</p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p>The maximum number of Amazon EC2 vCPUs that an environment can reach.</p>
    #[serde(rename = "maxvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxv_cpus: Option<i64>,
    /// <p>The minimum number of Amazon EC2 vCPUs that an environment should maintain.</p>
    #[serde(rename = "minvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i64>,
}

/// <p>An object representing the details of a container that is part of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerDetail {
    /// <p>The command that is passed to the container.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the container instance on which the container is running.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p><p>The environment variables to pass to a container.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The image used to start the container.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The instance type of the underlying host infrastructure of a multi-node parallel job.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) associated with the job upon execution.</p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The number of MiB of memory reserved for the job.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The mount points for data volumes in your container.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>The network interfaces associated with the job.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user).</p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The type and amount of a resource to assign to a container. Currently, the only supported resource is <code>GPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the container job. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>A list of <code>ulimit</code> values to set in the container.</p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>The number of VCPUs allocated for the job.</p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
    /// <p>A list of volumes associated with the job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>The overrides that should be sent to a container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContainerOverrides {
    /// <p>The command to send to the container that overrides the default command from the Docker image or the job definition.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the job definition.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The instance type to use for a multi-node parallel job. This parameter is not valid for single-node container jobs.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The number of MiB of memory reserved for the job. This value overrides the value set in the job definition.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The type and amount of a resource to assign to a container. This value overrides the value set in the job definition. Currently, the only supported resource is <code>GPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The number of vCPUs to reserve for the container. This value overrides the value set in the job definition.</p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
}

/// <p>Container properties are used in job definitions to describe the container that is launched as part of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    /// <p>The command that is passed to the container. This parameter maps to <code>Cmd</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#cmd">https://docs.docker.com/engine/reference/builder/#cmd</a>.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--env</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <important> <p>We do not recommend using plaintext environment variables for sensitive information, such as credential data.</p> </important> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p><p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <ul> <li> <p>Images in Amazon ECR repositories use the full registry and repository URI (for example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;</code>).</p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul></p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The instance type to use for a multi-node parallel job. Currently all node groups in a multi-node parallel job must use the same instance type. This parameter is not valid for single-node container jobs.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions.</p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p><p>The hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. You must specify at least 4 MiB of memory for a job.</p> <note> <p>If you are trying to maximize your resource utilization by providing your jobs as much memory as possible for a particular instance type, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/memory-management.html">Memory Management</a> in the <i>AWS Batch User Guide</i>.</p> </note></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The mount points for data volumes in your container. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--privileged</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--read-only</code> option to <code>docker run</code>.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>The type and amount of a resource to assign to a container. Currently, the only supported resource is <code>GPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>The number of vCPUs reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. Each vCPU is equivalent to 1,024 CPU shares. You must specify at least one vCPU.</p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
    /// <p>A list of data volumes used in a job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>An object representing summary details of a container within a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerSummary {
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComputeEnvironmentRequest {
    /// <p>The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>Details of the compute resources managed by the compute environment. This parameter is required for managed compute environments. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN may contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN does not use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    pub service_role: String,
    /// <p>The state of the compute environment. If the state is <code>ENABLED</code>, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the compute environment. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of the compute environment.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobQueueRequest {
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment should execute a given job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteComputeEnvironmentRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the compute environment to delete.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteComputeEnvironmentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJobQueueRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the queue to delete.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobQueueResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterJobDefinitionRequest {
    /// <p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterJobDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComputeEnvironmentsRequest {
    /// <p>A list of up to 100 compute environment names or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<String>>,
    /// <p>The maximum number of cluster results returned by <code>DescribeComputeEnvironments</code> in paginated output. When this parameter is used, <code>DescribeComputeEnvironments</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeComputeEnvironments</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeComputeEnvironments</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeComputeEnvironments</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComputeEnvironmentsResponse {
    /// <p>The list of compute environments.</p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<ComputeEnvironmentDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeComputeEnvironments</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobDefinitionsRequest {
    /// <p>The name of the job definition to describe.</p>
    #[serde(rename = "jobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    /// <p>A list of up to 100 job definition names or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobDefinitions</code> in paginated output. When this parameter is used, <code>DescribeJobDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status with which to filter job definitions.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobDefinitionsResponse {
    /// <p>The list of job definitions.</p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<JobDefinition>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobDefinitions</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobQueuesRequest {
    /// <p>A list of up to 100 queue names or full queue Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobQueues</code> in paginated output. When this parameter is used, <code>DescribeJobQueues</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobQueues</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobQueues</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobQueues</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobQueuesResponse {
    /// <p>The list of job queues.</p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<JobQueueDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobQueues</code> request. When the results of a <code>DescribeJobQueues</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobsRequest {
    /// <p>A list of up to 100 job IDs.</p>
    #[serde(rename = "jobs")]
    pub jobs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobsResponse {
    /// <p>The list of jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
}

/// <p>An object representing a container instance host device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// <p>The path inside the container at which to expose the host device. By default the <code>hostPath</code> value is used.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The path for the device on the host container instance.</p>
    #[serde(rename = "hostPath")]
    pub host_path: String,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// <p>Determine whether your data volume persists on the host container instance and where it is stored. If this parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data is not guaranteed to persist after the containers associated with it stop running.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    /// <p>The path on the host container instance that is presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If this parameter contains a file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the source path location does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p>
    #[serde(rename = "sourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>An object representing an AWS Batch job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDefinition {
    /// <p>An object with various properties specific to container-based jobs.</p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The Amazon Resource Name (ARN) for the job definition.</p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>An object with various properties specific to multi-node parallel jobs.</p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Default parameters or parameter substitution placeholders that are set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition. For more information about specifying parameters, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_definition_parameters.html">Job Definition Parameters</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
    /// <p>The status of the job definition.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition. You can specify a timeout duration after which AWS Batch terminates your jobs if they have not finished.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p>The type of job definition.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object representing an AWS Batch job dependency.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobDependency {
    /// <p>The job ID of the AWS Batch job associated with this dependency.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The type of the job dependency.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing an AWS Batch job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDetail {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesDetail>,
    /// <p>A list of job attempts associated with this job.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<AttemptDetail>>,
    /// <p>An object representing the details of the container that is associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetail>,
    /// <p>The Unix timestamp (in seconds and milliseconds) for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>A list of job IDs on which this job depends.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The job definition that is used by this job.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The ID for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The Amazon Resource Name (ARN) of the job queue with which the job is associated.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>An object representing the details of a node that is associated with a multi-node parallel job.</p>
    #[serde(rename = "nodeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_details: Option<NodeDetails>,
    /// <p>An object representing the node properties of a multi-node parallel job.</p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for this job if an attempt fails.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The Unix timestamp (in seconds and milliseconds) for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    /// <p><p>The current status for the job.</p> <note> <p>If your jobs do not progress to <code>STARTING</code>, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html#job_stuck_in_runnable">Jobs Stuck in RUNNABLE Status</a> in the troubleshooting section of the <i>AWS Batch User Guide</i>.</p> </note></p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>A short, human-readable string to provide additional details about the current status of the job.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp (in seconds and milliseconds) for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    /// <p>The timeout configuration for the job.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

/// <p>An object representing the details of an AWS Batch job queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobQueueDetail {
    /// <p>The compute environments that are attached to the job queue and the order in which job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>Describes the ability of the queue to accept new jobs.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job queue.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>An object representing summary details of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobSummary {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesSummary>,
    /// <p>An object representing the details of the container that is associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerSummary>,
    /// <p>The Unix timestamp for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>The ID of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The node properties for a single node in a job summary list.</p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodePropertiesSummary>,
    /// <p>The Unix timestamp for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>The current status for the job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

/// <p>An object representing a job timeout configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobTimeout {
    /// <p>The time duration in seconds (measured from the job attempt's <code>startedAt</code> timestamp) after which AWS Batch terminates your jobs if they have not finished.</p>
    #[serde(rename = "attemptDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i64>,
}

/// <p>A key-value pair object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePair {
    /// <p>The name of the key-value pair. For environment variables, this is the name of the environment variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the key-value pair. For environment variables, this is the value of the environment variable.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An object representing a launch template associated with a compute resource. You must specify either the launch template ID or launch template name in the request, but not both.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template.</p>
    #[serde(rename = "launchTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    /// <p>The name of the launch template.</p>
    #[serde(rename = "launchTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    /// <p>The version number of the launch template.</p> <p>Default: The default version of the launch template.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinuxParameters {
    /// <p>Any host devices to expose to the container. This parameter maps to <code>Devices</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--device</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    #[serde(rename = "arrayJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_job_id: Option<String>,
    /// <p>The name or full Amazon Resource Name (ARN) of the job queue with which to list jobs.</p>
    #[serde(rename = "jobQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    /// <p>The job status with which to filter jobs in the specified queue. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The job ID for a multi-node parallel job. Specifying a multi-node parallel job ID with this parameter lists all nodes that are associated with the specified job.</p>
    #[serde(rename = "multiNodeJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_node_job_id: Option<String>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>A list of job summaries that match the request.</p>
    #[serde(rename = "jobSummaryList")]
    pub job_summary_list: Vec<JobSummary>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Details on a Docker volume mount point that is used in a job's container properties. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.19/#create-a-container">Create a container</a> section of the Docker Remote API and the <code>--volume</code> option to docker run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountPoint {
    /// <p>The path on the container at which to mount the host volume.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume; otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount.</p>
    #[serde(rename = "sourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

/// <p>An object representing the elastic network interface for a multi-node parallel job node.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    #[serde(rename = "attachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>The private IPv6 address for the network interface.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>The private IPv4 address for the network interface.</p>
    #[serde(rename = "privateIpv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv_4_address: Option<String>,
}

/// <p>An object representing the details of a multi-node parallel job node.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeDetails {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    #[serde(rename = "isMainNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    #[serde(rename = "nodeIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i64>,
}

/// <p>Object representing any node overrides to a job definition that is used in a <a>SubmitJob</a> API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodeOverrides {
    /// <p>The node property overrides for the job.</p>
    #[serde(rename = "nodePropertyOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_property_overrides: Option<Vec<NodePropertyOverride>>,
    /// <p><p>The number of nodes to use with a multi-node parallel job. This value overrides the number of nodes that are specified in the job definition. To use this override:</p> <ul> <li> <p>There must be at least one node range in your job definition that has an open upper boundary (such as <code>:</code> or <code>n:</code>).</p> </li> <li> <p>The lower boundary of the node range specified in the job definition must be fewer than the number of nodes specified in the override.</p> </li> <li> <p>The main node index specified in the job definition must be fewer than the number of nodes specified in the override.</p> </li> </ul></p>
    #[serde(rename = "numNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i64>,
}

/// <p>An object representing the node properties of a multi-node parallel job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeProperties {
    /// <p>Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.</p>
    #[serde(rename = "mainNode")]
    pub main_node: i64,
    /// <p>A list of node ranges and their properties associated with a multi-node parallel job.</p>
    #[serde(rename = "nodeRangeProperties")]
    pub node_range_properties: Vec<NodeRangeProperty>,
    /// <p>The number of nodes associated with a multi-node parallel job.</p>
    #[serde(rename = "numNodes")]
    pub num_nodes: i64,
}

/// <p>An object representing the properties of a node that is associated with a multi-node parallel job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodePropertiesSummary {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    #[serde(rename = "isMainNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    #[serde(rename = "nodeIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i64>,
    /// <p>The number of nodes associated with a multi-node parallel job.</p>
    #[serde(rename = "numNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i64>,
}

/// <p>Object representing any node overrides to a job definition that is used in a <a>SubmitJob</a> API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodePropertyOverride {
    /// <p>The overrides that should be sent to a node range.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    /// <p>The range of nodes, using node index values, with which to override. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range.</p>
    #[serde(rename = "targetNodes")]
    pub target_nodes: String,
}

/// <p>An object representing the properties of the node range for a multi-node parallel job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeRangeProperty {
    /// <p>The container details for the node range.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerProperties>,
    /// <p>The range of nodes, using node index values. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range. Your accumulative node ranges must account for all nodes (0:n). You may nest node ranges, for example 0:10 and 4:5, in which case the 4:5 range properties override the 0:10 properties.</p>
    #[serde(rename = "targetNodes")]
    pub target_nodes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterJobDefinitionRequest {
    /// <p>An object with various properties specific to single-node container-based jobs. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The name of the job definition to register. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-parallel-jobs.html">Multi-node Parallel Jobs</a> in the <i>AWS Batch User Guide</i>. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that is specified during a <a>SubmitJob</a> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it is not retried.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition, after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it is not retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that is specified during a <a>SubmitJob</a> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p>The type of job definition.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterJobDefinitionResponse {
    /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
}

/// <p>The type and amount of a resource to assign to a container. Currently, the only supported resource type is <code>GPU</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirement {
    /// <p>The type of resource to assign to a container. Currently, the only supported resource type is <code>GPU</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The number of physical GPUs to reserve for the container. The number of GPUs reserved for all containers in a job should not exceed the number of available GPUs on the compute resource that the job is launched on.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>The retry strategy associated with a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryStrategy {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You may specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitJobRequest {
    /// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/array_jobs.html">Array Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayProperties>,
    /// <p>A list of container overrides in JSON format that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container (that is specified in the job definition or the Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the job definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    /// <p>A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a <code>SEQUENTIAL</code> type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an <code>N_TO_N</code> type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The job definition used by this job. This value can be one of <code>name</code>, <code>name:revision</code>, or the Amazon Resource Name (ARN) for the job definition. If <code>name</code> is specified without a revision then the latest active revision is used.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The name of the job. The first character must be alphanumeric, and up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The job queue into which the job is submitted. You can specify either the name or the Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>A list of node overrides in JSON format that specify the node range to target and the container overrides for that node range.</p>
    #[serde(rename = "nodeOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_overrides: Option<NodeOverrides>,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The retry strategy to use for failed jobs from this <a>SubmitJob</a> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The timeout configuration for this <a>SubmitJob</a> operation. You can specify a timeout duration after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it is not retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitJobResponse {
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateJobRequest {
    /// <p>The AWS Batch job ID of the job to terminate.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs.</p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateJobResponse {}

/// <p>The <code>ulimit</code> settings to pass to the container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ulimit {
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "hardLimit")]
    pub hard_limit: i64,
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "softLimit")]
    pub soft_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComputeEnvironmentRequest {
    /// <p>The name or full Amazon Resource Name (ARN) of the compute environment to update.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>Details of the compute resources managed by the compute environment. Required for a managed compute environment.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResourceUpdate>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN may contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN does not use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. Compute environments in the <code>ENABLED</code> state can accept jobs from a queue and scale in or out automatically based on the workload demand of its associated queues.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of the compute environment.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJobQueueRequest {
    /// <p>Details the set of compute environments mapped to a job queue and their order relative to each other. This is one of the parameters used by the job scheduler to determine which compute environment should execute a given job.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    /// <p>The name or the Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Describes the queue's ability to accept new jobs.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
}

/// <p>A data volume used in a job's container properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    /// <p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume. However, the data is not guaranteed to persist after the containers associated with it stop running.</p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,
    /// <p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(CancelJobError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(CancelJobError::Server(err.msg)),
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
            CancelJobError::Client(ref cause) => write!(f, "{}", cause),
            CancelJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            CreateComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComputeEnvironmentError {}
/// Errors returned by CreateJobQueue
#[derive(Debug, PartialEq)]
pub enum CreateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            CreateJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobQueueError {}
/// Errors returned by DeleteComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            DeleteComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteComputeEnvironmentError {}
/// Errors returned by DeleteJobQueue
#[derive(Debug, PartialEq)]
pub enum DeleteJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            DeleteJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJobQueueError {}
/// Errors returned by DeregisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterJobDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterJobDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterJobDefinitionError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterJobDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterJobDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterJobDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterJobDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterJobDefinitionError {}
/// Errors returned by DescribeComputeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeComputeEnvironmentsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeComputeEnvironmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComputeEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeComputeEnvironmentsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeComputeEnvironmentsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComputeEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComputeEnvironmentsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeComputeEnvironmentsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeComputeEnvironmentsError {}
/// Errors returned by DescribeJobDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeJobDefinitionsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobDefinitionsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobDefinitionsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobDefinitionsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobDefinitionsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobDefinitionsError {}
/// Errors returned by DescribeJobQueues
#[derive(Debug, PartialEq)]
pub enum DescribeJobQueuesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobQueuesError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobQueuesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobQueuesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobQueuesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobQueuesError {}
/// Errors returned by DescribeJobs
#[derive(Debug, PartialEq)]
pub enum DescribeJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobsError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(ListJobsError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(ListJobsError::Server(err.msg)),
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
            ListJobsError::Client(ref cause) => write!(f, "{}", cause),
            ListJobsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by RegisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterJobDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterJobDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterJobDefinitionError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterJobDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterJobDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterJobDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            RegisterJobDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterJobDefinitionError {}
/// Errors returned by SubmitJob
#[derive(Debug, PartialEq)]
pub enum SubmitJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(SubmitJobError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(SubmitJobError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitJobError::Client(ref cause) => write!(f, "{}", cause),
            SubmitJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitJobError {}
/// Errors returned by TerminateJob
#[derive(Debug, PartialEq)]
pub enum TerminateJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl TerminateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(TerminateJobError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TerminateJobError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateJobError::Client(ref cause) => write!(f, "{}", cause),
            TerminateJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TerminateJobError {}
/// Errors returned by UpdateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            UpdateComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateComputeEnvironmentError {}
/// Errors returned by UpdateJobQueue
#[derive(Debug, PartialEq)]
pub enum UpdateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            UpdateJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJobQueueError {}
/// Trait representing the capabilities of the AWS Batch API. AWS Batch clients implement this trait.
#[async_trait]
pub trait Batch {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no job is cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>>;

    /// <p><p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the capacity and instance types of the compute resources within the environment. This is based on the compute resource specification that you define or the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html">launch template</a> that you specify when you create the compute environment. You can choose to use Amazon EC2 On-Demand Instances or Spot Instances in your managed compute environment. You can optionally set a maximum price so that Spot Instances only launch when the Spot Instance price is below a specified percentage of the On-Demand price.</p> <note> <p>Multi-node parallel jobs are not supported on Spot Instances.</p> </note> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">Container Instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it. Then, manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS Container Instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>AWS Batch does not upgrade the AMIs in a compute environment after it is created (for example, when a newer version of the Amazon ECS-optimized AMI is available). You are responsible for the management of the guest operating system (including updates and security patches) and any additional application software or utilities that you install on the compute resources. To use a new AMI for your AWS Batch jobs:</p> <ol> <li> <p>Create a new compute environment with the new AMI.</p> </li> <li> <p>Add the compute environment to an existing job queue.</p> </li> <li> <p>Remove the old compute environment from your job queue.</p> </li> <li> <p>Delete the old compute environment.</p> </li> </ol> </note></p>
    async fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> Result<CreateComputeEnvironmentResponse, RusotoError<CreateComputeEnvironmentError>>;

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    async fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> Result<CreateJobQueueResponse, RusotoError<CreateJobQueueError>>;

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>
    async fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> Result<DeleteComputeEnvironmentResponse, RusotoError<DeleteComputeEnvironmentError>>;

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are terminated when you delete a job queue.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request.</p>
    async fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> Result<DeleteJobQueueResponse, RusotoError<DeleteJobQueueError>>;

    /// <p>Deregisters an AWS Batch job definition. Job definitions will be permanently deleted after 180 days.</p>
    async fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> Result<DeregisterJobDefinitionResponse, RusotoError<DeregisterJobDefinitionError>>;

    /// <p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    async fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> Result<DescribeComputeEnvironmentsResponse, RusotoError<DescribeComputeEnvironmentsError>>;

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    async fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> Result<DescribeJobDefinitionsResponse, RusotoError<DescribeJobDefinitionsError>>;

    /// <p>Describes one or more of your job queues.</p>
    async fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> Result<DescribeJobQueuesResponse, RusotoError<DescribeJobQueuesError>>;

    /// <p>Describes a list of AWS Batch jobs.</p>
    async fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> Result<DescribeJobsResponse, RusotoError<DescribeJobsError>>;

    /// <p>Returns a list of AWS Batch jobs.</p> <p>You must specify only one of the following:</p> <ul> <li> <p>a job queue ID to return a list of jobs in that job queue</p> </li> <li> <p>a multi-node parallel job ID to return a list of that job's nodes</p> </li> <li> <p>an array job ID to return a list of that job's children</p> </li> </ul> <p>You can filter the results by job status with the <code>jobStatus</code> parameter. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>>;

    /// <p>Registers an AWS Batch job definition.</p>
    async fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> Result<RegisterJobDefinitionResponse, RusotoError<RegisterJobDefinitionError>>;

    /// <p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition.</p>
    async fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> Result<SubmitJobResponse, RusotoError<SubmitJobError>>;

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    async fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> Result<TerminateJobResponse, RusotoError<TerminateJobError>>;

    /// <p>Updates an AWS Batch compute environment.</p>
    async fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> Result<UpdateComputeEnvironmentResponse, RusotoError<UpdateComputeEnvironmentError>>;

    /// <p>Updates a job queue.</p>
    async fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> Result<UpdateJobQueueResponse, RusotoError<UpdateJobQueueError>>;
}
/// A client for the AWS Batch API.
#[derive(Clone)]
pub struct BatchClient {
    client: Client,
    region: region::Region,
}

impl BatchClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> BatchClient {
        BatchClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> BatchClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        BatchClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> BatchClient {
        BatchClient { client, region }
    }
}

#[async_trait]
impl Batch for BatchClient {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no job is cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>> {
        let request_uri = "/v1/canceljob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobError::from_response(response))
        }
    }

    /// <p><p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the capacity and instance types of the compute resources within the environment. This is based on the compute resource specification that you define or the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html">launch template</a> that you specify when you create the compute environment. You can choose to use Amazon EC2 On-Demand Instances or Spot Instances in your managed compute environment. You can optionally set a maximum price so that Spot Instances only launch when the Spot Instance price is below a specified percentage of the On-Demand price.</p> <note> <p>Multi-node parallel jobs are not supported on Spot Instances.</p> </note> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">Container Instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it. Then, manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS Container Instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>AWS Batch does not upgrade the AMIs in a compute environment after it is created (for example, when a newer version of the Amazon ECS-optimized AMI is available). You are responsible for the management of the guest operating system (including updates and security patches) and any additional application software or utilities that you install on the compute resources. To use a new AMI for your AWS Batch jobs:</p> <ol> <li> <p>Create a new compute environment with the new AMI.</p> </li> <li> <p>Add the compute environment to an existing job queue.</p> </li> <li> <p>Remove the old compute environment from your job queue.</p> </li> <li> <p>Delete the old compute environment.</p> </li> </ol> </note></p>
    async fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> Result<CreateComputeEnvironmentResponse, RusotoError<CreateComputeEnvironmentError>> {
        let request_uri = "/v1/createcomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    async fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> Result<CreateJobQueueResponse, RusotoError<CreateJobQueueError>> {
        let request_uri = "/v1/createjobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobQueueError::from_response(response))
        }
    }

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>
    async fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> Result<DeleteComputeEnvironmentResponse, RusotoError<DeleteComputeEnvironmentError>> {
        let request_uri = "/v1/deletecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are terminated when you delete a job queue.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request.</p>
    async fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> Result<DeleteJobQueueResponse, RusotoError<DeleteJobQueueError>> {
        let request_uri = "/v1/deletejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJobQueueError::from_response(response))
        }
    }

    /// <p>Deregisters an AWS Batch job definition. Job definitions will be permanently deleted after 180 days.</p>
    async fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> Result<DeregisterJobDefinitionResponse, RusotoError<DeregisterJobDefinitionError>> {
        let request_uri = "/v1/deregisterjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeregisterJobDefinitionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterJobDefinitionError::from_response(response))
        }
    }

    /// <p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    async fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> Result<DescribeComputeEnvironmentsResponse, RusotoError<DescribeComputeEnvironmentsError>>
    {
        let request_uri = "/v1/describecomputeenvironments";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeComputeEnvironmentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeComputeEnvironmentsError::from_response(response))
        }
    }

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    async fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> Result<DescribeJobDefinitionsResponse, RusotoError<DescribeJobDefinitionsError>> {
        let request_uri = "/v1/describejobdefinitions";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobDefinitionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobDefinitionsError::from_response(response))
        }
    }

    /// <p>Describes one or more of your job queues.</p>
    async fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> Result<DescribeJobQueuesResponse, RusotoError<DescribeJobQueuesError>> {
        let request_uri = "/v1/describejobqueues";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobQueuesError::from_response(response))
        }
    }

    /// <p>Describes a list of AWS Batch jobs.</p>
    async fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> Result<DescribeJobsResponse, RusotoError<DescribeJobsError>> {
        let request_uri = "/v1/describejobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of AWS Batch jobs.</p> <p>You must specify only one of the following:</p> <ul> <li> <p>a job queue ID to return a list of jobs in that job queue</p> </li> <li> <p>a multi-node parallel job ID to return a list of that job's nodes</p> </li> <li> <p>an array job ID to return a list of that job's children</p> </li> </ul> <p>You can filter the results by job status with the <code>jobStatus</code> parameter. If you do not specify a status, only <code>RUNNING</code> jobs are returned.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>> {
        let request_uri = "/v1/listjobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>Registers an AWS Batch job definition.</p>
    async fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> Result<RegisterJobDefinitionResponse, RusotoError<RegisterJobDefinitionError>> {
        let request_uri = "/v1/registerjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterJobDefinitionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterJobDefinitionError::from_response(response))
        }
    }

    /// <p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition.</p>
    async fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> Result<SubmitJobResponse, RusotoError<SubmitJobError>> {
        let request_uri = "/v1/submitjob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SubmitJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SubmitJobError::from_response(response))
        }
    }

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    async fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> Result<TerminateJobResponse, RusotoError<TerminateJobError>> {
        let request_uri = "/v1/terminatejob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TerminateJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TerminateJobError::from_response(response))
        }
    }

    /// <p>Updates an AWS Batch compute environment.</p>
    async fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> Result<UpdateComputeEnvironmentResponse, RusotoError<UpdateComputeEnvironmentError>> {
        let request_uri = "/v1/updatecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Updates a job queue.</p>
    async fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> Result<UpdateJobQueueResponse, RusotoError<UpdateJobQueueError>> {
        let request_uri = "/v1/updatejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJobQueueError::from_response(response))
        }
    }
}
