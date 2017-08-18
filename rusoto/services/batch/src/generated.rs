
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="<p>An object representing the details of a container that is part of a job attempt.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttemptContainerDetail {
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>"]
    #[serde(rename="containerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[doc="<p>The exit code for the job attempt. A non-zero exit code is considered a failure.</p>"]
    #[serde(rename="exitCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc="<p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the job attempt.</p>"]
    #[serde(rename="taskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_arn: Option<String>,
}

#[doc="<p>An object representing a job attempt.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttemptDetail {
    #[doc="<p>Details about the container in this job attempt.</p>"]
    #[serde(rename="container")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<AttemptContainerDetail>,
    #[doc="<p>The Unix timestamp for when the attempt was started (when the task transitioned from the <code>PENDING</code> state to the <code>RUNNING</code> state).</p>"]
    #[serde(rename="startedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<i64>,
    #[doc="<p>A short, human-readable string to provide additional details about the current status of the job attempt.</p>"]
    #[serde(rename="statusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
    #[doc="<p>The Unix timestamp for when the attempt was stopped (when the task transitioned from the <code>RUNNING</code> state to the <code>STOPPED</code> state).</p>"]
    #[serde(rename="stoppedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped_at: Option<i64>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CEState {
    Disabled,
    Enabled,
}

impl Into<String> for CEState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CEState {
    fn into(self) -> &'static str {
        match self {
            CEState::Disabled => "DISABLED",
            CEState::Enabled => "ENABLED",
        }
    }
}

impl ::std::str::FromStr for CEState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DISABLED" => Ok(CEState::Disabled),
            "ENABLED" => Ok(CEState::Enabled),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CEStatus {
    Creating,
    Deleted,
    Deleting,
    Invalid,
    Updating,
    Valid,
}

impl Into<String> for CEStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CEStatus {
    fn into(self) -> &'static str {
        match self {
            CEStatus::Creating => "CREATING",
            CEStatus::Deleted => "DELETED",
            CEStatus::Deleting => "DELETING",
            CEStatus::Invalid => "INVALID",
            CEStatus::Updating => "UPDATING",
            CEStatus::Valid => "VALID",
        }
    }
}

impl ::std::str::FromStr for CEStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATING" => Ok(CEStatus::Creating),
            "DELETED" => Ok(CEStatus::Deleted),
            "DELETING" => Ok(CEStatus::Deleting),
            "INVALID" => Ok(CEStatus::Invalid),
            "UPDATING" => Ok(CEStatus::Updating),
            "VALID" => Ok(CEStatus::Valid),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CEType {
    Managed,
    Unmanaged,
}

impl Into<String> for CEType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CEType {
    fn into(self) -> &'static str {
        match self {
            CEType::Managed => "MANAGED",
            CEType::Unmanaged => "UNMANAGED",
        }
    }
}

impl ::std::str::FromStr for CEType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MANAGED" => Ok(CEType::Managed),
            "UNMANAGED" => Ok(CEType::Unmanaged),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CRType {
    Ec2,
    Spot,
}

impl Into<String> for CRType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CRType {
    fn into(self) -> &'static str {
        match self {
            CRType::Ec2 => "EC2",
            CRType::Spot => "SPOT",
        }
    }
}

impl ::std::str::FromStr for CRType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EC2" => Ok(CRType::Ec2),
            "SPOT" => Ok(CRType::Spot),
            _ => Err(()),
        }
    }
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelJobRequest {
    #[doc="<p>A list of up to 100 job IDs to cancel.</p>"]
    #[serde(rename="jobId")]
    pub job_id: String,
    #[doc="<p>A message to attach to the job that explains the reason for cancelling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs. </p>"]
    #[serde(rename="reason")]
    pub reason: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CancelJobResponse;

#[doc="<p>An object representing an AWS Batch compute environment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComputeEnvironmentDetail {
    #[doc="<p>The Amazon Resource Name (ARN) of the compute environment. </p>"]
    #[serde(rename="computeEnvironmentArn")]
    pub compute_environment_arn: String,
    #[doc="<p>The name of the compute environment. </p>"]
    #[serde(rename="computeEnvironmentName")]
    pub compute_environment_name: String,
    #[doc="<p>The compute resources defined for the compute environment. </p>"]
    #[serde(rename="computeResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    #[doc="<p>The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster used by the compute environment. </p>"]
    #[serde(rename="ecsClusterArn")]
    pub ecs_cluster_arn: String,
    #[doc="<p>The service role associated with the compute environment that allows AWS Batch to make calls to AWS API operations on your behalf.</p>"]
    #[serde(rename="serviceRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role: Option<String>,
    #[doc="<p>The state of the compute environment. The valid values are <code>ENABLED</code> or <code>DISABLED</code>. An <code>ENABLED</code> state indicates that you can register instances with the compute environment and that the associated instances can accept jobs. </p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The current status of the compute environment (for example, <code>CREATING</code> or <code>VALID</code>).</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>A short, human-readable string to provide additional details about the current status of the compute environment.</p>"]
    #[serde(rename="statusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
    #[doc="<p>The type of the compute environment.</p>"]
    #[serde(rename="type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>The order in which compute environments are tried for job placement within a queue. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower order integer value is tried for job placement first.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ComputeEnvironmentOrder {
    #[doc="<p>The Amazon Resource Name (ARN) of the compute environment.</p>"]
    #[serde(rename="computeEnvironment")]
    pub compute_environment: String,
    #[doc="<p>The order of the compute environment.</p>"]
    #[serde(rename="order")]
    pub order: i64,
}

#[doc="<p>An object representing an AWS Batch compute resource.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ComputeResource {
    #[doc="<p>The minimum percentage that a Spot Instance price must be when compared with the On-Demand price for that instance type before instances are launched. For example, if your bid percentage is 20%, then the Spot price must be below 20% of the current On-Demand price for that EC2 instance.</p>"]
    #[serde(rename="bidPercentage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_percentage: Option<i64>,
    #[doc="<p>The desired number of EC2 vCPUS in the compute environment. </p>"]
    #[serde(rename="desiredvCpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    #[doc="<p>The EC2 key pair that is used for instances launched in the compute environment.</p>"]
    #[serde(rename="ec2KeyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_key_pair: Option<String>,
    #[doc="<p>The Amazon Machine Image (AMI) ID used for instances launched in the compute environment.</p>"]
    #[serde(rename="imageId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_id: Option<String>,
    #[doc="<p>The Amazon ECS instance role applied to Amazon EC2 instances in a compute environment.</p>"]
    #[serde(rename="instanceRole")]
    pub instance_role: String,
    #[doc="<p>The instances types that may launched.</p>"]
    #[serde(rename="instanceTypes")]
    pub instance_types: Vec<String>,
    #[doc="<p>The maximum number of EC2 vCPUs that an environment can reach. </p>"]
    #[serde(rename="maxvCpus")]
    pub maxv_cpus: i64,
    #[doc="<p>The minimum number of EC2 vCPUs that an environment should maintain. </p>"]
    #[serde(rename="minvCpus")]
    pub minv_cpus: i64,
    #[doc="<p>The EC2 security group that is associated with instances launched in the compute environment. </p>"]
    #[serde(rename="securityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a <code>SPOT</code> compute environment.</p>"]
    #[serde(rename="spotIamFleetRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spot_iam_fleet_role: Option<String>,
    #[doc="<p>The VPC subnets into which the compute resources are launched. </p>"]
    #[serde(rename="subnets")]
    pub subnets: Vec<String>,
    #[doc="<p>Key-value pair tags to be applied to resources that are launched in the compute environment. </p>"]
    #[serde(rename="tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The type of compute environment.</p>"]
    #[serde(rename="type")]
    pub type_: String,
}

#[doc="<p>An object representing the attributes of a compute environment that can be updated.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ComputeResourceUpdate {
    #[doc="<p>The desired number of EC2 vCPUS in the compute environment.</p>"]
    #[serde(rename="desiredvCpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    #[doc="<p>The maximum number of EC2 vCPUs that an environment can reach.</p>"]
    #[serde(rename="maxvCpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maxv_cpus: Option<i64>,
    #[doc="<p>The minimum number of EC2 vCPUs that an environment should maintain.</p>"]
    #[serde(rename="minvCpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minv_cpus: Option<i64>,
}

#[doc="<p>An object representing the details of a container that is part of a job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ContainerDetail {
    #[doc="<p>The command that is passed to the container. </p>"]
    #[serde(rename="command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of the container instance on which the container is running.</p>"]
    #[serde(rename="containerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[doc="<p>The environment variables to pass to a container.</p>"]
    #[serde(rename="environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[doc="<p>The exit code to return upon completion.</p>"]
    #[serde(rename="exitCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc="<p>The image used to start the container.</p>"]
    #[serde(rename="image")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) associated with the job upon execution. </p>"]
    #[serde(rename="jobRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_role_arn: Option<String>,
    #[doc="<p>The number of MiB of memory reserved for the job.</p>"]
    #[serde(rename="memory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory: Option<i64>,
    #[doc="<p>The mount points for data volumes in your container.</p>"]
    #[serde(rename="mountPoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[doc="<p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user).</p>"]
    #[serde(rename="privileged")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privileged: Option<bool>,
    #[doc="<p>When this parameter is true, the container is given read-only access to its root file system.</p>"]
    #[serde(rename="readonlyRootFilesystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[doc="<p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon ECS task that is associated with the container job.</p>"]
    #[serde(rename="taskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_arn: Option<String>,
    #[doc="<p>A list of <code>ulimit</code> values to set in the container.</p>"]
    #[serde(rename="ulimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[doc="<p>The user name to use inside the container.</p>"]
    #[serde(rename="user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
    #[doc="<p>The number of VCPUs allocated for the job. </p>"]
    #[serde(rename="vcpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcpus: Option<i64>,
    #[doc="<p>A list of volumes associated with the job.</p>"]
    #[serde(rename="volumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[doc="<p>The overrides that should be sent to a container.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ContainerOverrides {
    #[doc="<p>The command to send to the container that overrides the default command from the Docker image or the job definition.</p>"]
    #[serde(rename="command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<Vec<String>>,
    #[doc="<p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the job definition.</p>"]
    #[serde(rename="environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[doc="<p>The number of MiB of memory reserved for the job. This value overrides the value set in the job definition.</p>"]
    #[serde(rename="memory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory: Option<i64>,
    #[doc="<p>The number of vCPUs to reserve for the container. This value overrides the value set in the job definition.</p>"]
    #[serde(rename="vcpus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcpus: Option<i64>,
}

#[doc="<p>Container properties are used in job definitions to describe the container that is launched as part of a job.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ContainerProperties {
    #[doc="<p>The command that is passed to the container. This parameter maps to <code>Cmd</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. For more information, see <a href=\"https://docs.docker.com/engine/reference/builder/#cmd\">https://docs.docker.com/engine/reference/builder/#cmd</a>.</p>"]
    #[serde(rename="command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<Vec<String>>,
    #[doc="<p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--env</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <important> <p>We do not recommend using plain text environment variables for sensitive information, such as credential data.</p> </important>"]
    #[serde(rename="environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[doc="<p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <ul> <li> <p>Images in Amazon ECR repositories use the full registry and repository URI (for example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;</code>). </p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul>"]
    #[serde(rename="image")]
    pub image: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions.</p>"]
    #[serde(rename="jobRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_role_arn: Option<String>,
    #[doc="<p>The hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--memory</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="memory")]
    pub memory: i64,
    #[doc="<p>The mount points for data volumes in your container. This parameter maps to <code>Volumes</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--volume</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="mountPoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[doc="<p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--privileged</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="privileged")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privileged: Option<bool>,
    #[doc="<p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--read-only</code> option to <code>docker run</code>.</p>"]
    #[serde(rename="readonlyRootFilesystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[doc="<p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--ulimit</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="ulimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[doc="<p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--user</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
    #[doc="<p>The number of vCPUs reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. Each vCPU is equivalent to 1,024 CPU shares.</p>"]
    #[serde(rename="vcpus")]
    pub vcpus: i64,
    #[doc="<p>A list of data volumes used in a job.</p>"]
    #[serde(rename="volumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateComputeEnvironmentRequest {
    #[doc="<p>The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, and underscores are allowed.</p>"]
    #[serde(rename="computeEnvironmentName")]
    pub compute_environment_name: String,
    #[doc="<p>Details of the compute resources managed by the compute environment. This parameter is required for managed compute environments.</p>"]
    #[serde(rename="computeResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    #[doc="<p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf. </p>"]
    #[serde(rename="serviceRole")]
    pub service_role: String,
    #[doc="<p>The state of the compute environment. If the state is <code>ENABLED</code>, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.</p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The type of the compute environment. </p>"]
    #[serde(rename="type")]
    pub type_: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateComputeEnvironmentResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the compute environment. </p>"]
    #[serde(rename="computeEnvironmentArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environment_arn: Option<String>,
    #[doc="<p>The name of the compute environment.</p>"]
    #[serde(rename="computeEnvironmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateJobQueueRequest {
    #[doc="<p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment should execute a given job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to 3 compute environments with a job queue.</p>"]
    #[serde(rename="computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    #[doc="<p>The name of the job queue.</p>"]
    #[serde(rename="jobQueueName")]
    pub job_queue_name: String,
    #[doc="<p>The priority of the job queue. Job queues with a higher priority (or a lower integer value for the <code>priority</code> parameter) are evaluated first when associated with same compute environment. Priority is determined in ascending order, for example, a job queue with a priority value of <code>1</code> is given scheduling preference over a job queue with a priority value of <code>10</code>.</p>"]
    #[serde(rename="priority")]
    pub priority: i64,
    #[doc="<p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs.</p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateJobQueueResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the job queue.</p>"]
    #[serde(rename="jobQueueArn")]
    pub job_queue_arn: String,
    #[doc="<p>The name of the job queue.</p>"]
    #[serde(rename="jobQueueName")]
    pub job_queue_name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteComputeEnvironmentRequest {
    #[doc="<p>The name or Amazon Resource Name (ARN) of the compute environment to delete. </p>"]
    #[serde(rename="computeEnvironment")]
    pub compute_environment: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteComputeEnvironmentResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteJobQueueRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the queue to delete. </p>"]
    #[serde(rename="jobQueue")]
    pub job_queue: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteJobQueueResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterJobDefinitionRequest {
    #[doc="<p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister. </p>"]
    #[serde(rename="jobDefinition")]
    pub job_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeregisterJobDefinitionResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeComputeEnvironmentsRequest {
    #[doc="<p>A list of up to 100 compute environment names or full Amazon Resource Name (ARN) entries. </p>"]
    #[serde(rename="computeEnvironments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environments: Option<Vec<String>>,
    #[doc="<p>The maximum number of cluster results returned by <code>DescribeComputeEnvironments</code> in paginated output. When this parameter is used, <code>DescribeComputeEnvironments</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeComputeEnvironments</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeComputeEnvironments</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeComputeEnvironments</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeComputeEnvironmentsResponse {
    #[doc="<p>The list of compute environments.</p>"]
    #[serde(rename="computeEnvironments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environments: Option<Vec<ComputeEnvironmentDetail>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>DescribeComputeEnvironments</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeJobDefinitionsRequest {
    #[doc="<p>The name of the job definition to describe.</p>"]
    #[serde(rename="jobDefinitionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_definition_name: Option<String>,
    #[doc="<p>A space-separated list of up to 100 job definition names or full Amazon Resource Name (ARN) entries.</p>"]
    #[serde(rename="jobDefinitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_definitions: Option<Vec<String>>,
    #[doc="<p>The maximum number of results returned by <code>DescribeJobDefinitions</code> in paginated output. When this parameter is used, <code>DescribeJobDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The status with which to filter job definitions.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeJobDefinitionsResponse {
    #[doc="<p>The list of job definitions. </p>"]
    #[serde(rename="jobDefinitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_definitions: Option<Vec<JobDefinition>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>DescribeJobDefinitions</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeJobQueuesRequest {
    #[doc="<p>A list of up to 100 queue names or full queue Amazon Resource Name (ARN) entries.</p>"]
    #[serde(rename="jobQueues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_queues: Option<Vec<String>>,
    #[doc="<p>The maximum number of results returned by <code>DescribeJobQueues</code> in paginated output. When this parameter is used, <code>DescribeJobQueues</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobQueues</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeJobQueues</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobQueues</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeJobQueuesResponse {
    #[doc="<p>The list of job queues. </p>"]
    #[serde(rename="jobQueues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_queues: Option<Vec<JobQueueDetail>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>DescribeJobQueues</code> request. When the results of a <code>DescribeJobQueues</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeJobsRequest {
    #[doc="<p>A space-separated list of up to 100 job IDs.</p>"]
    #[serde(rename="jobs")]
    pub jobs: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeJobsResponse {
    #[doc="<p>The list of jobs. </p>"]
    #[serde(rename="jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
}

#[doc="<p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data is not guaranteed to persist after the containers associated with it stop running.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Host {
    #[doc="<p>The path on the host container instance that is presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If the <code>host</code> parameter contains a <code>sourcePath</code> file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the <code>sourcePath</code> value does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p>"]
    #[serde(rename="sourcePath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_path: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum JQState {
    Disabled,
    Enabled,
}

impl Into<String> for JQState {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for JQState {
    fn into(self) -> &'static str {
        match self {
            JQState::Disabled => "DISABLED",
            JQState::Enabled => "ENABLED",
        }
    }
}

impl ::std::str::FromStr for JQState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DISABLED" => Ok(JQState::Disabled),
            "ENABLED" => Ok(JQState::Enabled),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum JQStatus {
    Creating,
    Deleted,
    Deleting,
    Invalid,
    Updating,
    Valid,
}

impl Into<String> for JQStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for JQStatus {
    fn into(self) -> &'static str {
        match self {
            JQStatus::Creating => "CREATING",
            JQStatus::Deleted => "DELETED",
            JQStatus::Deleting => "DELETING",
            JQStatus::Invalid => "INVALID",
            JQStatus::Updating => "UPDATING",
            JQStatus::Valid => "VALID",
        }
    }
}

impl ::std::str::FromStr for JQStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATING" => Ok(JQStatus::Creating),
            "DELETED" => Ok(JQStatus::Deleted),
            "DELETING" => Ok(JQStatus::Deleting),
            "INVALID" => Ok(JQStatus::Invalid),
            "UPDATING" => Ok(JQStatus::Updating),
            "VALID" => Ok(JQStatus::Valid),
            _ => Err(()),
        }
    }
}

#[doc="<p>An object representing an AWS Batch job definition.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobDefinition {
    #[doc="<p>An object with various properties specific to container-based jobs. </p>"]
    #[serde(rename="containerProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    #[doc="<p>The Amazon Resource Name (ARN) for the job definition. </p>"]
    #[serde(rename="jobDefinitionArn")]
    pub job_definition_arn: String,
    #[doc="<p>The name of the job definition. </p>"]
    #[serde(rename="jobDefinitionName")]
    pub job_definition_name: String,
    #[doc="<p>Default parameters or parameter substitution placeholders that are set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>"]
    #[serde(rename="parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The retry strategy to use for failed jobs that are submitted with this job definition.</p>"]
    #[serde(rename="retryStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[doc="<p>The revision of the job definition.</p>"]
    #[serde(rename="revision")]
    pub revision: i64,
    #[doc="<p>The status of the job definition.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The type of job definition.</p>"]
    #[serde(rename="type")]
    pub type_: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum JobDefinitionType {
    Container,
}

impl Into<String> for JobDefinitionType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for JobDefinitionType {
    fn into(self) -> &'static str {
        match self {
            JobDefinitionType::Container => "container",
        }
    }
}

impl ::std::str::FromStr for JobDefinitionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "container" => Ok(JobDefinitionType::Container),
            _ => Err(()),
        }
    }
}

#[doc="<p>An object representing an AWS Batch job dependency.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct JobDependency {
    #[doc="<p>The job ID of the AWS Batch job associated with this dependency.</p>"]
    #[serde(rename="jobId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_id: Option<String>,
}

#[doc="<p>An object representing an AWS Batch job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobDetail {
    #[doc="<p>A list of job attempts associated with this job.</p>"]
    #[serde(rename="attempts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attempts: Option<Vec<AttemptDetail>>,
    #[doc="<p>An object representing the details of the container that is associated with the job.</p>"]
    #[serde(rename="container")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<ContainerDetail>,
    #[doc="<p>The Unix timestamp for when the job was created (when the task entered the <code>PENDING</code> state). </p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,
    #[doc="<p>A list of job names or IDs on which this job depends.</p>"]
    #[serde(rename="dependsOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    #[doc="<p>The job definition that is used by this job.</p>"]
    #[serde(rename="jobDefinition")]
    pub job_definition: String,
    #[doc="<p>The ID for the job.</p>"]
    #[serde(rename="jobId")]
    pub job_id: String,
    #[doc="<p>The name of the job.</p>"]
    #[serde(rename="jobName")]
    pub job_name: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the job queue with which the job is associated.</p>"]
    #[serde(rename="jobQueue")]
    pub job_queue: String,
    #[doc="<p>Additional parameters passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition. </p>"]
    #[serde(rename="parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The retry strategy to use for this job if an attempt fails.</p>"]
    #[serde(rename="retryStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[doc="<p>The Unix timestamp for when the job was started (when the task transitioned from the <code>PENDING</code> state to the <code>RUNNING</code> state). </p>"]
    #[serde(rename="startedAt")]
    pub started_at: i64,
    #[doc="<p>The current status for the job.</p>"]
    #[serde(rename="status")]
    pub status: String,
    #[doc="<p>A short, human-readable string to provide additional details about the current status of the job. </p>"]
    #[serde(rename="statusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
    #[doc="<p>The Unix timestamp for when the job was stopped (when the task transitioned from the <code>RUNNING</code> state to the <code>STOPPED</code> state).</p>"]
    #[serde(rename="stoppedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[doc="<p>An object representing the details of an AWS Batch job queue.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobQueueDetail {
    #[doc="<p>The compute environments that are attached to the job queue and the order in which job placement is preferred. Compute environments are selected for job placement in ascending order.</p>"]
    #[serde(rename="computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    #[doc="<p>The Amazon Resource Name (ARN) of the job queue.</p>"]
    #[serde(rename="jobQueueArn")]
    pub job_queue_arn: String,
    #[doc="<p>The name of the job queue.</p>"]
    #[serde(rename="jobQueueName")]
    pub job_queue_name: String,
    #[doc="<p>The priority of the job queue. </p>"]
    #[serde(rename="priority")]
    pub priority: i64,
    #[doc="<p>Describes the ability of the queue to accept new jobs.</p>"]
    #[serde(rename="state")]
    pub state: String,
    #[doc="<p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>A short, human-readable string to provide additional details about the current status of the job queue.</p>"]
    #[serde(rename="statusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_reason: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum JobStatus {
    Failed,
    Pending,
    Runnable,
    Running,
    Starting,
    Submitted,
    Succeeded,
}

impl Into<String> for JobStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for JobStatus {
    fn into(self) -> &'static str {
        match self {
            JobStatus::Failed => "FAILED",
            JobStatus::Pending => "PENDING",
            JobStatus::Runnable => "RUNNABLE",
            JobStatus::Running => "RUNNING",
            JobStatus::Starting => "STARTING",
            JobStatus::Submitted => "SUBMITTED",
            JobStatus::Succeeded => "SUCCEEDED",
        }
    }
}

impl ::std::str::FromStr for JobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FAILED" => Ok(JobStatus::Failed),
            "PENDING" => Ok(JobStatus::Pending),
            "RUNNABLE" => Ok(JobStatus::Runnable),
            "RUNNING" => Ok(JobStatus::Running),
            "STARTING" => Ok(JobStatus::Starting),
            "SUBMITTED" => Ok(JobStatus::Submitted),
            "SUCCEEDED" => Ok(JobStatus::Succeeded),
            _ => Err(()),
        }
    }
}

#[doc="<p>An object representing summary details of a job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobSummary {
    #[doc="<p>The ID of the job.</p>"]
    #[serde(rename="jobId")]
    pub job_id: String,
    #[doc="<p>The name of the job.</p>"]
    #[serde(rename="jobName")]
    pub job_name: String,
}

#[doc="<p>A key-value pair object.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct KeyValuePair {
    #[doc="<p>The name of the key value pair. For environment variables, this is the name of the environment variable.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The value of the key value pair. For environment variables, this is the value of the environment variable.</p>"]
    #[serde(rename="value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListJobsRequest {
    #[doc="<p>The name or full Amazon Resource Name (ARN) of the job queue with which to list jobs.</p>"]
    #[serde(rename="jobQueue")]
    pub job_queue: String,
    #[doc="<p>The job status with which to filter jobs in the specified queue.</p>"]
    #[serde(rename="jobStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_status: Option<String>,
    #[doc="<p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListJobsResponse {
    #[doc="<p>A list of job summaries that match the request.</p>"]
    #[serde(rename="jobSummaryList")]
    pub job_summary_list: Vec<JobSummary>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Details on a Docker volume mount point that is used in a job's container properties.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MountPoint {
    #[doc="<p>The path on the container at which to mount the host volume.</p>"]
    #[serde(rename="containerPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_path: Option<String>,
    #[doc="<p>If this value is <code>true</code>, the container has read-only access to the volume; otherwise, the container can write to the volume. The default value is <code>false</code>.</p>"]
    #[serde(rename="readOnly")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,
    #[doc="<p>The name of the volume to mount.</p>"]
    #[serde(rename="sourceVolume")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterJobDefinitionRequest {
    #[doc="<p>An object with various properties specific for container-based jobs. This parameter is required if the <code>type</code> parameter is <code>container</code>.</p>"]
    #[serde(rename="containerProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    #[doc="<p>The name of the job definition to register. </p>"]
    #[serde(rename="jobDefinitionName")]
    pub job_definition_name: String,
    #[doc="<p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>"]
    #[serde(rename="parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that is specified during a <a>SubmitJob</a> operation overrides the retry strategy defined here.</p>"]
    #[serde(rename="retryStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[doc="<p>The type of job definition.</p>"]
    #[serde(rename="type")]
    pub type_: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterJobDefinitionResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the job definition. </p>"]
    #[serde(rename="jobDefinitionArn")]
    pub job_definition_arn: String,
    #[doc="<p>The name of the job definition. </p>"]
    #[serde(rename="jobDefinitionName")]
    pub job_definition_name: String,
    #[doc="<p>The revision of the job definition.</p>"]
    #[serde(rename="revision")]
    pub revision: i64,
}

#[doc="<p>The retry strategy associated with a job.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RetryStrategy {
    #[doc="<p>The number of times to move a job to the <code>RUNNABLE</code> status. You may specify between 1 and 10 attempts. If <code>attempts</code> is greater than one, the job is retried if it fails until it has moved to <code>RUNNABLE</code> that many times.</p>"]
    #[serde(rename="attempts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attempts: Option<i64>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SubmitJobRequest {
    #[doc="<p>A list of container overrides in JSON format that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container (that is specified in the job definition or the Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the job definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p>"]
    #[serde(rename="containerOverrides")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    #[doc="<p>A list of job IDs on which this job depends. A job can depend upon a maximum of 100 jobs. </p>"]
    #[serde(rename="dependsOn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    #[doc="<p>The job definition used by this job. This value can be either a <code>name:revision</code> or the Amazon Resource Name (ARN) for the job definition.</p>"]
    #[serde(rename="jobDefinition")]
    pub job_definition: String,
    #[doc="<p>The name of the job. A name must be 1 to 128 characters in length.</p> <p>Pattern: ^[a-zA-Z0-9_]+$</p>"]
    #[serde(rename="jobName")]
    pub job_name: String,
    #[doc="<p>The job queue into which the job will be submitted. You can specify either the name or the Amazon Resource Name (ARN) of the queue. </p>"]
    #[serde(rename="jobQueue")]
    pub job_queue: String,
    #[doc="<p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>"]
    #[serde(rename="parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The retry strategy to use for failed jobs from this <a>SubmitJob</a> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p>"]
    #[serde(rename="retryStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SubmitJobResponse {
    #[doc="<p>The unique identifier for the job.</p>"]
    #[serde(rename="jobId")]
    pub job_id: String,
    #[doc="<p>The name of the job. </p>"]
    #[serde(rename="jobName")]
    pub job_name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct TerminateJobRequest {
    #[doc="<p>Job IDs to be terminated. Up to 100 jobs can be specified.</p>"]
    #[serde(rename="jobId")]
    pub job_id: String,
    #[doc="<p>A message to attach to the job that explains the reason for cancelling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs. </p>"]
    #[serde(rename="reason")]
    pub reason: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TerminateJobResponse;

#[doc="<p>The <code>ulimit</code> settings to pass to the container.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Ulimit {
    #[doc="<p>The hard limit for the <code>ulimit</code> type.</p>"]
    #[serde(rename="hardLimit")]
    pub hard_limit: i64,
    #[doc="<p>The <code>type</code> of the <code>ulimit</code>.</p>"]
    #[serde(rename="name")]
    pub name: String,
    #[doc="<p>The soft limit for the <code>ulimit</code> type.</p>"]
    #[serde(rename="softLimit")]
    pub soft_limit: i64,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateComputeEnvironmentRequest {
    #[doc="<p>The name or full Amazon Resource Name (ARN) of the compute environment to update.</p>"]
    #[serde(rename="computeEnvironment")]
    pub compute_environment: String,
    #[doc="<p>Details of the compute resources managed by the compute environment. Required for a managed compute environment.</p>"]
    #[serde(rename="computeResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_resources: Option<ComputeResourceUpdate>,
    #[doc="<p>The name or full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to ECS, Auto Scaling, and EC2 on your behalf.</p>"]
    #[serde(rename="serviceRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role: Option<String>,
    #[doc="<p>The state of the compute environment. Compute environments in the <code>ENABLED</code> state can accept jobs from a queue and scale in or out automatically based on the workload demand of its associated queues.</p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateComputeEnvironmentResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the compute environment. </p>"]
    #[serde(rename="computeEnvironmentArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environment_arn: Option<String>,
    #[doc="<p>The name of compute environment.</p>"]
    #[serde(rename="computeEnvironmentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateJobQueueRequest {
    #[doc="<p>Details the set of compute environments mapped to a job queue and their order relative to each other. This is one of the parameters used by the job scheduler to determine which compute environment should execute a given job. </p>"]
    #[serde(rename="computeEnvironmentOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    #[doc="<p>The name or the Amazon Resource Name (ARN) of the job queue.</p>"]
    #[serde(rename="jobQueue")]
    pub job_queue: String,
    #[doc="<p>The priority of the job queue. Job queues with a higher priority (or a lower integer value for the <code>priority</code> parameter) are evaluated first when associated with same compute environment. Priority is determined in ascending order, for example, a job queue with a priority value of <code>1</code> is given scheduling preference over a job queue with a priority value of <code>10</code>.</p>"]
    #[serde(rename="priority")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub priority: Option<i64>,
    #[doc="<p>Describes the queue's ability to accept new jobs.</p>"]
    #[serde(rename="state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateJobQueueResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the job queue.</p>"]
    #[serde(rename="jobQueueArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_queue_arn: Option<String>,
    #[doc="<p>The name of the job queue.</p>"]
    #[serde(rename="jobQueueName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_queue_name: Option<String>,
}

#[doc="<p>A data volume used in a job's container properties.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Volume {
    #[doc="<p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data is not guaranteed to persist after the containers associated with it stop running.</p>"]
    #[serde(rename="host")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<Host>,
    #[doc="<p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelJobError {
    pub fn from_body(body: &str) -> CancelJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CancelJobError::Client(String::from(error_message)),
                    "ServerException" => CancelJobError::Server(String::from(error_message)),
                    "ValidationException" => CancelJobError::Validation(error_message.to_string()),
                    _ => CancelJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelJobError {
    fn from(err: serde_json::error::Error) -> CancelJobError {
        CancelJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::Client(ref cause) => cause,
            CancelJobError::Server(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateComputeEnvironmentError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateComputeEnvironmentError {
    pub fn from_body(body: &str) -> CreateComputeEnvironmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        CreateComputeEnvironmentError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        CreateComputeEnvironmentError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateComputeEnvironmentError::Validation(error_message.to_string())
                    }
                    _ => CreateComputeEnvironmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateComputeEnvironmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateComputeEnvironmentError {
    fn from(err: CredentialsError) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> CreateComputeEnvironmentError {
        CreateComputeEnvironmentError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            CreateComputeEnvironmentError::Client(ref cause) => cause,
            CreateComputeEnvironmentError::Server(ref cause) => cause,
            CreateComputeEnvironmentError::Validation(ref cause) => cause,
            CreateComputeEnvironmentError::Credentials(ref err) => err.description(),
            CreateComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateComputeEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJobQueue
#[derive(Debug, PartialEq)]
pub enum CreateJobQueueError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateJobQueueError {
    pub fn from_body(body: &str) -> CreateJobQueueError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateJobQueueError::Client(String::from(error_message)),
                    "ServerException" => CreateJobQueueError::Server(String::from(error_message)),
                    "ValidationException" => {
                        CreateJobQueueError::Validation(error_message.to_string())
                    }
                    _ => CreateJobQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateJobQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateJobQueueError {
    fn from(err: serde_json::error::Error) -> CreateJobQueueError {
        CreateJobQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobQueueError {
    fn from(err: CredentialsError) -> CreateJobQueueError {
        CreateJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobQueueError {
    fn from(err: HttpDispatchError) -> CreateJobQueueError {
        CreateJobQueueError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobQueueError {
    fn description(&self) -> &str {
        match *self {
            CreateJobQueueError::Client(ref cause) => cause,
            CreateJobQueueError::Server(ref cause) => cause,
            CreateJobQueueError::Validation(ref cause) => cause,
            CreateJobQueueError::Credentials(ref err) => err.description(),
            CreateJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteComputeEnvironmentError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteComputeEnvironmentError {
    pub fn from_body(body: &str) -> DeleteComputeEnvironmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DeleteComputeEnvironmentError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DeleteComputeEnvironmentError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteComputeEnvironmentError::Validation(error_message.to_string())
                    }
                    _ => DeleteComputeEnvironmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteComputeEnvironmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteComputeEnvironmentError {
    fn from(err: CredentialsError) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> DeleteComputeEnvironmentError {
        DeleteComputeEnvironmentError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteComputeEnvironmentError::Client(ref cause) => cause,
            DeleteComputeEnvironmentError::Server(ref cause) => cause,
            DeleteComputeEnvironmentError::Validation(ref cause) => cause,
            DeleteComputeEnvironmentError::Credentials(ref err) => err.description(),
            DeleteComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteComputeEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteJobQueue
#[derive(Debug, PartialEq)]
pub enum DeleteJobQueueError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteJobQueueError {
    pub fn from_body(body: &str) -> DeleteJobQueueError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteJobQueueError::Client(String::from(error_message)),
                    "ServerException" => DeleteJobQueueError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DeleteJobQueueError::Validation(error_message.to_string())
                    }
                    _ => DeleteJobQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteJobQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteJobQueueError {
    fn from(err: serde_json::error::Error) -> DeleteJobQueueError {
        DeleteJobQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteJobQueueError {
    fn from(err: CredentialsError) -> DeleteJobQueueError {
        DeleteJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteJobQueueError {
    fn from(err: HttpDispatchError) -> DeleteJobQueueError {
        DeleteJobQueueError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobQueueError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobQueueError::Client(ref cause) => cause,
            DeleteJobQueueError::Server(ref cause) => cause,
            DeleteJobQueueError::Validation(ref cause) => cause,
            DeleteJobQueueError::Credentials(ref err) => err.description(),
            DeleteJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteJobQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterJobDefinitionError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterJobDefinitionError {
    pub fn from_body(body: &str) -> DeregisterJobDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DeregisterJobDefinitionError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DeregisterJobDefinitionError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterJobDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeregisterJobDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterJobDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterJobDefinitionError {
    fn from(err: serde_json::error::Error) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterJobDefinitionError {
    fn from(err: CredentialsError) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterJobDefinitionError {
    fn from(err: HttpDispatchError) -> DeregisterJobDefinitionError {
        DeregisterJobDefinitionError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterJobDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterJobDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeregisterJobDefinitionError::Client(ref cause) => cause,
            DeregisterJobDefinitionError::Server(ref cause) => cause,
            DeregisterJobDefinitionError::Validation(ref cause) => cause,
            DeregisterJobDefinitionError::Credentials(ref err) => err.description(),
            DeregisterJobDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterJobDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComputeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeComputeEnvironmentsError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeComputeEnvironmentsError {
    pub fn from_body(body: &str) -> DescribeComputeEnvironmentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeComputeEnvironmentsError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DescribeComputeEnvironmentsError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeComputeEnvironmentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeComputeEnvironmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeComputeEnvironmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeComputeEnvironmentsError {
    fn from(err: serde_json::error::Error) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeComputeEnvironmentsError {
    fn from(err: CredentialsError) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeComputeEnvironmentsError {
    fn from(err: HttpDispatchError) -> DescribeComputeEnvironmentsError {
        DescribeComputeEnvironmentsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeComputeEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComputeEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeComputeEnvironmentsError::Client(ref cause) => cause,
            DescribeComputeEnvironmentsError::Server(ref cause) => cause,
            DescribeComputeEnvironmentsError::Validation(ref cause) => cause,
            DescribeComputeEnvironmentsError::Credentials(ref err) => err.description(),
            DescribeComputeEnvironmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeComputeEnvironmentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJobDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeJobDefinitionsError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeJobDefinitionsError {
    pub fn from_body(body: &str) -> DescribeJobDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeJobDefinitionsError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DescribeJobDefinitionsError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobDefinitionsError {
    fn from(err: serde_json::error::Error) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobDefinitionsError {
    fn from(err: CredentialsError) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobDefinitionsError {
    fn from(err: HttpDispatchError) -> DescribeJobDefinitionsError {
        DescribeJobDefinitionsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeJobDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobDefinitionsError::Client(ref cause) => cause,
            DescribeJobDefinitionsError::Server(ref cause) => cause,
            DescribeJobDefinitionsError::Validation(ref cause) => cause,
            DescribeJobDefinitionsError::Credentials(ref err) => err.description(),
            DescribeJobDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeJobDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJobQueues
#[derive(Debug, PartialEq)]
pub enum DescribeJobQueuesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeJobQueuesError {
    pub fn from_body(body: &str) -> DescribeJobQueuesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeJobQueuesError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DescribeJobQueuesError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobQueuesError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobQueuesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobQueuesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobQueuesError {
    fn from(err: serde_json::error::Error) -> DescribeJobQueuesError {
        DescribeJobQueuesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobQueuesError {
    fn from(err: CredentialsError) -> DescribeJobQueuesError {
        DescribeJobQueuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobQueuesError {
    fn from(err: HttpDispatchError) -> DescribeJobQueuesError {
        DescribeJobQueuesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeJobQueuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobQueuesError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobQueuesError::Client(ref cause) => cause,
            DescribeJobQueuesError::Server(ref cause) => cause,
            DescribeJobQueuesError::Validation(ref cause) => cause,
            DescribeJobQueuesError::Credentials(ref err) => err.description(),
            DescribeJobQueuesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeJobQueuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJobs
#[derive(Debug, PartialEq)]
pub enum DescribeJobsError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeJobsError {
    pub fn from_body(body: &str) -> DescribeJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DescribeJobsError::Client(String::from(error_message)),
                    "ServerException" => DescribeJobsError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DescribeJobsError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobsError {
    fn from(err: serde_json::error::Error) -> DescribeJobsError {
        DescribeJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobsError {
    fn from(err: CredentialsError) -> DescribeJobsError {
        DescribeJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobsError {
    fn from(err: HttpDispatchError) -> DescribeJobsError {
        DescribeJobsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobsError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobsError::Client(ref cause) => cause,
            DescribeJobsError::Server(ref cause) => cause,
            DescribeJobsError::Validation(ref cause) => cause,
            DescribeJobsError::Credentials(ref err) => err.description(),
            DescribeJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ListJobsError::Client(String::from(error_message)),
                    "ServerException" => ListJobsError::Server(String::from(error_message)),
                    "ValidationException" => ListJobsError::Validation(error_message.to_string()),
                    _ => ListJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::Client(ref cause) => cause,
            ListJobsError::Server(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterJobDefinitionError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterJobDefinitionError {
    pub fn from_body(body: &str) -> RegisterJobDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        RegisterJobDefinitionError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        RegisterJobDefinitionError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterJobDefinitionError::Validation(error_message.to_string())
                    }
                    _ => RegisterJobDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterJobDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterJobDefinitionError {
    fn from(err: serde_json::error::Error) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterJobDefinitionError {
    fn from(err: CredentialsError) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterJobDefinitionError {
    fn from(err: HttpDispatchError) -> RegisterJobDefinitionError {
        RegisterJobDefinitionError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterJobDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterJobDefinitionError {
    fn description(&self) -> &str {
        match *self {
            RegisterJobDefinitionError::Client(ref cause) => cause,
            RegisterJobDefinitionError::Server(ref cause) => cause,
            RegisterJobDefinitionError::Validation(ref cause) => cause,
            RegisterJobDefinitionError::Credentials(ref err) => err.description(),
            RegisterJobDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterJobDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SubmitJob
#[derive(Debug, PartialEq)]
pub enum SubmitJobError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SubmitJobError {
    pub fn from_body(body: &str) -> SubmitJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => SubmitJobError::Client(String::from(error_message)),
                    "ServerException" => SubmitJobError::Server(String::from(error_message)),
                    "ValidationException" => SubmitJobError::Validation(error_message.to_string()),
                    _ => SubmitJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => SubmitJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SubmitJobError {
    fn from(err: serde_json::error::Error) -> SubmitJobError {
        SubmitJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SubmitJobError {
    fn from(err: CredentialsError) -> SubmitJobError {
        SubmitJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubmitJobError {
    fn from(err: HttpDispatchError) -> SubmitJobError {
        SubmitJobError::HttpDispatch(err)
    }
}
impl fmt::Display for SubmitJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitJobError {
    fn description(&self) -> &str {
        match *self {
            SubmitJobError::Client(ref cause) => cause,
            SubmitJobError::Server(ref cause) => cause,
            SubmitJobError::Validation(ref cause) => cause,
            SubmitJobError::Credentials(ref err) => err.description(),
            SubmitJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SubmitJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateJob
#[derive(Debug, PartialEq)]
pub enum TerminateJobError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TerminateJobError {
    pub fn from_body(body: &str) -> TerminateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => TerminateJobError::Client(String::from(error_message)),
                    "ServerException" => TerminateJobError::Server(String::from(error_message)),
                    "ValidationException" => {
                        TerminateJobError::Validation(error_message.to_string())
                    }
                    _ => TerminateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => TerminateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TerminateJobError {
    fn from(err: serde_json::error::Error) -> TerminateJobError {
        TerminateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateJobError {
    fn from(err: CredentialsError) -> TerminateJobError {
        TerminateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateJobError {
    fn from(err: HttpDispatchError) -> TerminateJobError {
        TerminateJobError::HttpDispatch(err)
    }
}
impl fmt::Display for TerminateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateJobError {
    fn description(&self) -> &str {
        match *self {
            TerminateJobError::Client(ref cause) => cause,
            TerminateJobError::Server(ref cause) => cause,
            TerminateJobError::Validation(ref cause) => cause,
            TerminateJobError::Credentials(ref err) => err.description(),
            TerminateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TerminateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateComputeEnvironmentError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateComputeEnvironmentError {
    pub fn from_body(body: &str) -> UpdateComputeEnvironmentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        UpdateComputeEnvironmentError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        UpdateComputeEnvironmentError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateComputeEnvironmentError::Validation(error_message.to_string())
                    }
                    _ => UpdateComputeEnvironmentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateComputeEnvironmentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateComputeEnvironmentError {
    fn from(err: serde_json::error::Error) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateComputeEnvironmentError {
    fn from(err: CredentialsError) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateComputeEnvironmentError {
    fn from(err: HttpDispatchError) -> UpdateComputeEnvironmentError {
        UpdateComputeEnvironmentError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateComputeEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateComputeEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateComputeEnvironmentError::Client(ref cause) => cause,
            UpdateComputeEnvironmentError::Server(ref cause) => cause,
            UpdateComputeEnvironmentError::Validation(ref cause) => cause,
            UpdateComputeEnvironmentError::Credentials(ref err) => err.description(),
            UpdateComputeEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateComputeEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJobQueue
#[derive(Debug, PartialEq)]
pub enum UpdateJobQueueError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid. </p>
    Client(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateJobQueueError {
    pub fn from_body(body: &str) -> UpdateJobQueueError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => UpdateJobQueueError::Client(String::from(error_message)),
                    "ServerException" => UpdateJobQueueError::Server(String::from(error_message)),
                    "ValidationException" => {
                        UpdateJobQueueError::Validation(error_message.to_string())
                    }
                    _ => UpdateJobQueueError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateJobQueueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateJobQueueError {
    fn from(err: serde_json::error::Error) -> UpdateJobQueueError {
        UpdateJobQueueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateJobQueueError {
    fn from(err: CredentialsError) -> UpdateJobQueueError {
        UpdateJobQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobQueueError {
    fn from(err: HttpDispatchError) -> UpdateJobQueueError {
        UpdateJobQueueError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateJobQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobQueueError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobQueueError::Client(ref cause) => cause,
            UpdateJobQueueError::Server(ref cause) => cause,
            UpdateJobQueueError::Validation(ref cause) => cause,
            UpdateJobQueueError::Credentials(ref err) => err.description(),
            UpdateJobQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateJobQueueError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Batch API. AWS Batch clients implement this trait.
pub trait Batch {
    #[doc="<p>Cancels jobs in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no jobs are cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>"]
    fn cancel_job(&self, input: &CancelJobRequest) -> Result<CancelJobResponse, CancelJobError>;


    #[doc="<p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the compute resources within the environment, based on the compute resources that you specify. Instances launched into a managed compute environment use the latest Amazon ECS-optimized AMI. You can choose to use Amazon EC2 On-Demand instances in your managed compute environment, or you can use Amazon EC2 Spot instances that only launch when the Spot bid price is below a specified percentage of the On-Demand price.</p> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html\">Container Instance AMIs</a> in the <i>Amazon EC2 Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it and then manually launch your container instances into that Amazon ECS cluster. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html\">Launching an Amazon ECS Container Instance</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn create_compute_environment
        (&self,
         input: &CreateComputeEnvironmentRequest)
         -> Result<CreateComputeEnvironmentResponse, CreateComputeEnvironmentError>;


    #[doc="<p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>"]
    fn create_job_queue(&self,
                        input: &CreateJobQueueRequest)
                        -> Result<CreateJobQueueResponse, CreateJobQueueError>;


    #[doc="<p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>"]
    fn delete_compute_environment
        (&self,
         input: &DeleteComputeEnvironmentRequest)
         -> Result<DeleteComputeEnvironmentResponse, DeleteComputeEnvironmentError>;


    #[doc="<p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation and terminate any jobs that have not completed with the <a>TerminateJob</a>.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request. </p>"]
    fn delete_job_queue(&self,
                        input: &DeleteJobQueueRequest)
                        -> Result<DeleteJobQueueResponse, DeleteJobQueueError>;


    #[doc="<p>Deregisters an AWS Batch job definition.</p>"]
    fn deregister_job_definition
        (&self,
         input: &DeregisterJobDefinitionRequest)
         -> Result<DeregisterJobDefinitionResponse, DeregisterJobDefinitionError>;


    #[doc="<p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>"]
    fn describe_compute_environments
        (&self,
         input: &DescribeComputeEnvironmentsRequest)
         -> Result<DescribeComputeEnvironmentsResponse, DescribeComputeEnvironmentsError>;


    #[doc="<p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>"]
    fn describe_job_definitions
        (&self,
         input: &DescribeJobDefinitionsRequest)
         -> Result<DescribeJobDefinitionsResponse, DescribeJobDefinitionsError>;


    #[doc="<p>Describes one or more of your job queues.</p>"]
    fn describe_job_queues(&self,
                           input: &DescribeJobQueuesRequest)
                           -> Result<DescribeJobQueuesResponse, DescribeJobQueuesError>;


    #[doc="<p>Describes a list of AWS Batch jobs.</p>"]
    fn describe_jobs(&self,
                     input: &DescribeJobsRequest)
                     -> Result<DescribeJobsResponse, DescribeJobsError>;


    #[doc="<p>Returns a list of task jobs for a specified job queue. You can filter the results by job status with the <code>jobStatus</code> parameter.</p>"]
    fn list_jobs(&self, input: &ListJobsRequest) -> Result<ListJobsResponse, ListJobsError>;


    #[doc="<p>Registers an AWS Batch job definition. </p>"]
    fn register_job_definition
        (&self,
         input: &RegisterJobDefinitionRequest)
         -> Result<RegisterJobDefinitionResponse, RegisterJobDefinitionError>;


    #[doc="<p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition. </p>"]
    fn submit_job(&self, input: &SubmitJobRequest) -> Result<SubmitJobResponse, SubmitJobError>;


    #[doc="<p>Terminates jobs in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>"]
    fn terminate_job(&self,
                     input: &TerminateJobRequest)
                     -> Result<TerminateJobResponse, TerminateJobError>;


    #[doc="<p>Updates an AWS Batch compute environment.</p>"]
    fn update_compute_environment
        (&self,
         input: &UpdateComputeEnvironmentRequest)
         -> Result<UpdateComputeEnvironmentResponse, UpdateComputeEnvironmentError>;


    #[doc="<p>Updates a job queue.</p>"]
    fn update_job_queue(&self,
                        input: &UpdateJobQueueRequest)
                        -> Result<UpdateJobQueueResponse, UpdateJobQueueError>;
}
/// A client for the AWS Batch API.
pub struct BatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> BatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        BatchClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Batch for BatchClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Cancels jobs in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are cancelled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not cancelled (but the API operation still succeeds, even if no jobs are cancelled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>"]
    fn cancel_job(&self, input: &CancelJobRequest) -> Result<CancelJobResponse, CancelJobError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/canceljob";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CancelJobResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(CancelJobError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments.</p> <p>In a managed compute environment, AWS Batch manages the compute resources within the environment, based on the compute resources that you specify. Instances launched into a managed compute environment use the latest Amazon ECS-optimized AMI. You can choose to use Amazon EC2 On-Demand instances in your managed compute environment, or you can use Amazon EC2 Spot instances that only launch when the Spot bid price is below a specified percentage of the On-Demand price.</p> <p>In an unmanaged compute environment, you can manage your own compute resources. This provides more compute resource configuration options, such as using a custom AMI, but you must ensure that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html\">Container Instance AMIs</a> in the <i>Amazon EC2 Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that is associated with it and then manually launch your container instances into that Amazon ECS cluster. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html\">Launching an Amazon ECS Container Instance</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn create_compute_environment
        (&self,
         input: &CreateComputeEnvironmentRequest)
         -> Result<CreateComputeEnvironmentResponse, CreateComputeEnvironmentError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/createcomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateComputeEnvironmentResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(CreateComputeEnvironmentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>"]
    fn create_job_queue(&self,
                        input: &CreateJobQueueRequest)
                        -> Result<CreateJobQueueResponse, CreateJobQueueError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/createjobqueue";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateJobQueueResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(CreateJobQueueError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation.</p>"]
    fn delete_compute_environment
        (&self,
         input: &DeleteComputeEnvironmentRequest)
         -> Result<DeleteComputeEnvironmentResponse, DeleteComputeEnvironmentError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/deletecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteComputeEnvironmentResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(DeleteComputeEnvironmentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation and terminate any jobs that have not completed with the <a>TerminateJob</a>.</p> <p>It is not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request. </p>"]
    fn delete_job_queue(&self,
                        input: &DeleteJobQueueRequest)
                        -> Result<DeleteJobQueueResponse, DeleteJobQueueError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/deletejobqueue";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteJobQueueResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DeleteJobQueueError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters an AWS Batch job definition.</p>"]
    fn deregister_job_definition
        (&self,
         input: &DeregisterJobDefinitionRequest)
         -> Result<DeregisterJobDefinitionResponse, DeregisterJobDefinitionError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/deregisterjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeregisterJobDefinitionResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(DeregisterJobDefinitionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes one or more of your compute environments.</p> <p>If you are using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>"]
    fn describe_compute_environments
        (&self,
         input: &DescribeComputeEnvironmentsRequest)
         -> Result<DescribeComputeEnvironmentsResponse, DescribeComputeEnvironmentsError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/describecomputeenvironments";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeComputeEnvironmentsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(DescribeComputeEnvironmentsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>"]
    fn describe_job_definitions
        (&self,
         input: &DescribeJobDefinitionsRequest)
         -> Result<DescribeJobDefinitionsResponse, DescribeJobDefinitionsError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/describejobdefinitions";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeJobDefinitionsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeJobDefinitionsError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Describes one or more of your job queues.</p>"]
    fn describe_job_queues(&self,
                           input: &DescribeJobQueuesRequest)
                           -> Result<DescribeJobQueuesResponse, DescribeJobQueuesError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/describejobqueues";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeJobQueuesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeJobQueuesError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Describes a list of AWS Batch jobs.</p>"]
    fn describe_jobs(&self,
                     input: &DescribeJobsRequest)
                     -> Result<DescribeJobsResponse, DescribeJobsError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/describejobs";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeJobsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeJobsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of task jobs for a specified job queue. You can filter the results by job status with the <code>jobStatus</code> parameter.</p>"]
    fn list_jobs(&self, input: &ListJobsRequest) -> Result<ListJobsResponse, ListJobsError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/listjobs";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListJobsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(ListJobsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Registers an AWS Batch job definition. </p>"]
    fn register_job_definition
        (&self,
         input: &RegisterJobDefinitionRequest)
         -> Result<RegisterJobDefinitionResponse, RegisterJobDefinitionError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/registerjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterJobDefinitionResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(RegisterJobDefinitionError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition. </p>"]
    fn submit_job(&self, input: &SubmitJobRequest) -> Result<SubmitJobResponse, SubmitJobError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/submitjob";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SubmitJobResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(SubmitJobError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Terminates jobs in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>"]
    fn terminate_job(&self,
                     input: &TerminateJobRequest)
                     -> Result<TerminateJobResponse, TerminateJobError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/terminatejob";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TerminateJobResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(TerminateJobError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates an AWS Batch compute environment.</p>"]
    fn update_compute_environment
        (&self,
         input: &UpdateComputeEnvironmentRequest)
         -> Result<UpdateComputeEnvironmentResponse, UpdateComputeEnvironmentError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/updatecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateComputeEnvironmentResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(UpdateComputeEnvironmentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates a job queue.</p>"]
    fn update_job_queue(&self,
                        input: &UpdateJobQueueRequest)
                        -> Result<UpdateJobQueueResponse, UpdateJobQueueError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/v1/updatejobqueue";

        let mut request = SignedRequest::new("POST", "batch", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateJobQueueResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(UpdateJobQueueError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
