
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
#[doc="<p>An attribute is a name-value pair associated with an Amazon ECS object. Attributes enable you to extend the Amazon ECS data model by adding custom metadata to your resources. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes\">Attributes</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Attribute {
    #[doc="<p>The name of the attribute. Up to 128 letters (uppercase and lowercase), numbers, hyphens, underscores, and periods are allowed.</p>"]
    #[serde(rename="name")]
    pub name: String,
    #[doc="<p>The ID of the target. You can specify the short form ID for a resource or the full Amazon Resource Name (ARN).</p>"]
    #[serde(rename="targetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<String>,
    #[doc="<p>The type of the target with which to attach the attribute. This parameter is required if you use the short form ID for a resource instead of the full Amazon Resource Name (ARN).</p>"]
    #[serde(rename="targetType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_type: Option<String>,
    #[doc="<p>The value of the attribute. Up to 128 letters (uppercase and lowercase), numbers, hyphens, underscores, periods, at signs (@), forward slashes, colons, and spaces are allowed.</p>"]
    #[serde(rename="value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[doc="<p>A regional grouping of one or more container instances on which you can run task requests. Each account receives a default cluster the first time you use the Amazon ECS service, but you may also create other clusters. Clusters may contain more than one instance type simultaneously.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Cluster {
    #[doc="<p>The number of services that are running on the cluster in an <code>ACTIVE</code> state. You can view these services with <a>ListServices</a>.</p>"]
    #[serde(rename="activeServicesCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_services_count: Option<i64>,
    #[doc="<p>The Amazon Resource Name (ARN) that identifies the cluster. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the region of the cluster, the AWS account ID of the cluster owner, the <code>cluster</code> namespace, and then the cluster name. For example, <code>arn:aws:ecs:<i>region</i>:<i>012345678910</i>:cluster/<i>test</i> </code>..</p>"]
    #[serde(rename="clusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_arn: Option<String>,
    #[doc="<p>A user-generated string that you use to identify your cluster.</p>"]
    #[serde(rename="clusterName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc="<p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>"]
    #[serde(rename="pendingTasksCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    #[doc="<p>The number of container instances registered into the cluster.</p>"]
    #[serde(rename="registeredContainerInstancesCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered_container_instances_count: Option<i64>,
    #[doc="<p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>"]
    #[serde(rename="runningTasksCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_tasks_count: Option<i64>,
    #[doc="<p>The status of the cluster. The valid values are <code>ACTIVE</code> or <code>INACTIVE</code>. <code>ACTIVE</code> indicates that you can register container instances with the cluster and the associated instances can accept tasks.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>A Docker container that is part of a task.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Container {
    #[doc="<p>The Amazon Resource Name (ARN) of the container.</p>"]
    #[serde(rename="containerArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_arn: Option<String>,
    #[doc="<p>The exit code returned from the container.</p>"]
    #[serde(rename="exitCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc="<p>The last known status of the container.</p>"]
    #[serde(rename="lastStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_status: Option<String>,
    #[doc="<p>The name of the container.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The network bindings associated with the container.</p>"]
    #[serde(rename="networkBindings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    #[doc="<p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the task.</p>"]
    #[serde(rename="taskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_arn: Option<String>,
}

#[doc="<p>Container definitions are used in task definitions to describe the different containers that are launched as part of a task.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ContainerDefinition {
    #[doc="<p>The command that is passed to the container. This parameter maps to <code>Cmd</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. For more information, see <a href=\"https://docs.docker.com/engine/reference/builder/#cmd\">https://docs.docker.com/engine/reference/builder/#cmd</a>.</p>"]
    #[serde(rename="command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<Vec<String>>,
    #[doc="<p>The number of <code>cpu</code> units reserved for the container. A container instance has 1,024 <code>cpu</code> units for every CPU core. This parameter specifies the minimum amount of CPU to reserve for a container, and containers share unallocated CPU units with other containers on the instance with the same ratio as their allocated amount. This parameter maps to <code>CpuShares</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <note> <p>You can determine the number of CPU units that are available per EC2 instance type by multiplying the vCPUs listed for that instance type on the <a href=\"http://aws.amazon.com/ec2/instance-types/\">Amazon EC2 Instances</a> detail page by 1,024.</p> </note> <p>For example, if you run a single-container task on a single-core instance type with 512 CPU units specified for that container, and that is the only task running on the container instance, that container could use the full 1,024 CPU unit share at any given time. However, if you launched another copy of the same task on that container instance, each task would be guaranteed a minimum of 512 CPU units when needed, and each container could float to higher CPU usage if the other container was not using it, but if both tasks were 100% active all of the time, they would be limited to 512 CPU units.</p> <p>The Docker daemon on the container instance uses the CPU value to calculate the relative CPU share ratios for running containers. For more information, see <a href=\"https://docs.docker.com/engine/reference/run/#cpu-share-constraint\">CPU share constraint</a> in the Docker documentation. The minimum valid CPU share value that the Linux kernel allows is 2; however, the CPU parameter is not required, and you can use CPU values below 2 in your container definitions. For CPU values below 2 (including null), the behavior varies based on your Amazon ECS container agent version:</p> <ul> <li> <p> <b>Agent versions less than or equal to 1.1.0:</b> Null and zero CPU values are passed to Docker as 0, which Docker then converts to 1,024 CPU shares. CPU values of 1 are passed to Docker as 1, which the Linux kernel converts to 2 CPU shares.</p> </li> <li> <p> <b>Agent versions greater than or equal to 1.2.0:</b> Null, zero, and CPU values of 1 are passed to Docker as 2.</p> </li> </ul>"]
    #[serde(rename="cpu")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu: Option<i64>,
    #[doc="<p>When this parameter is true, networking is disabled within the container. This parameter maps to <code>NetworkDisabled</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a>.</p>"]
    #[serde(rename="disableNetworking")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disable_networking: Option<bool>,
    #[doc="<p>A list of DNS search domains that are presented to the container. This parameter maps to <code>DnsSearch</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--dns-search</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="dnsSearchDomains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    #[doc="<p>A list of DNS servers that are presented to the container. This parameter maps to <code>Dns</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--dns</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="dnsServers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    #[doc="<p>A key/value map of labels to add to the container. This parameter maps to <code>Labels</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--label</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep \"Server API version\"</code> </p>"]
    #[serde(rename="dockerLabels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_labels: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of strings to provide custom labels for SELinux and AppArmor multi-level security systems. This parameter maps to <code>SecurityOpt</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--security-opt</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <note> <p>The Amazon ECS container agent running on a container instance must register with the <code>ECS_SELINUX_CAPABLE=true</code> or <code>ECS_APPARMOR_CAPABLE=true</code> environment variables before containers placed on that instance can use these security options. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html\">Amazon ECS Container Agent Configuration</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> </note>"]
    #[serde(rename="dockerSecurityOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    #[doc="<important> <p>Early versions of the Amazon ECS container agent do not properly handle <code>entryPoint</code> parameters. If you have problems using <code>entryPoint</code>, update your container agent or enter your commands and arguments as <code>command</code> array items instead.</p> </important> <p>The entry point that is passed to the container. This parameter maps to <code>Entrypoint</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--entrypoint</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. For more information, see <a href=\"https://docs.docker.com/engine/reference/builder/#entrypoint\">https://docs.docker.com/engine/reference/builder/#entrypoint</a>.</p>"]
    #[serde(rename="entryPoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[doc="<p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--env</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <important> <p>We do not recommend using plain text environment variables for sensitive information, such as credential data.</p> </important>"]
    #[serde(rename="environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[doc="<p>If the <code>essential</code> parameter of a container is marked as <code>true</code>, and that container fails or stops for any reason, all other containers that are part of the task are stopped. If the <code>essential</code> parameter of a container is marked as <code>false</code>, then its failure does not affect the rest of the containers in a task. If this parameter is omitted, a container is assumed to be essential.</p> <p>All tasks must have at least one essential container. If you have an application that is composed of multiple containers, you should group containers that are used for a common purpose into components, and separate the different components into multiple task definitions. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/application_architecture.html\">Application Architecture</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="essential")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub essential: Option<bool>,
    #[doc="<p>A list of hostnames and IP address mappings to append to the <code>/etc/hosts</code> file on the container. This parameter maps to <code>ExtraHosts</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--add-host</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="extraHosts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_hosts: Option<Vec<HostEntry>>,
    #[doc="<p>The hostname to use for your container. This parameter maps to <code>Hostname</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--hostname</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <ul> <li> <p>Images in Amazon ECR repositories use the full registry and repository URI (for example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;</code>). </p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul>"]
    #[serde(rename="image")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image: Option<String>,
    #[doc="<p>The <code>link</code> parameter allows containers to communicate with each other without the need for port mappings, using the <code>name</code> parameter and optionally, an <code>alias</code> for the link. This construct is analogous to <code>name:alias</code> in Docker links. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed for each <code>name</code> and <code>alias</code>. For more information on linking Docker containers, see <a href=\"https://docs.docker.com/engine/userguide/networking/default_network/dockerlinks/\">https://docs.docker.com/engine/userguide/networking/default_network/dockerlinks/</a>. This parameter maps to <code>Links</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--link</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <important> <p>Containers that are collocated on a single container instance may be able to communicate with each other without requiring links or host port mappings. Network isolation is achieved on the container instance using security groups and VPC settings.</p> </important>"]
    #[serde(rename="links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub links: Option<Vec<String>>,
    #[doc="<p>The log configuration specification for the container. This parameter maps to <code>LogConfig</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--log-driver</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. By default, containers use the same logging driver that the Docker daemon uses; however the container may use a different logging driver than the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different logging driver for a container, the log system must be configured properly on the container instance (or on a different log server for remote logging options). For more information on the options for different supported log drivers, see <a href=\"https://docs.docker.com/engine/admin/logging/overview/\">Configure logging drivers</a> in the Docker documentation.</p> <note> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the <a>LogConfiguration</a> data type). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep \"Server API version\"</code> </p> <note> <p>The Amazon ECS container agent running on a container instance must register the logging drivers available on that instance with the <code>ECS_AVAILABLE_LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html\">Amazon ECS Container Agent Configuration</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> </note>"]
    #[serde(rename="logConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[doc="<p>The hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--memory</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <p>You must specify a non-zero integer for one or both of <code>memory</code> or <code>memoryReservation</code> in container definitions. If you specify both, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance on which the container is placed; otherwise, the value of <code>memory</code> is used.</p> <p>The Docker daemon reserves a minimum of 4 MiB of memory for a container, so you should not specify fewer than 4 MiB of memory for your containers. </p>"]
    #[serde(rename="memory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory: Option<i64>,
    #[doc="<p>The soft limit (in MiB) of memory to reserve for the container. When system memory is under heavy contention, Docker attempts to keep the container memory to this soft limit; however, your container can consume more memory when it needs to, up to either the hard limit specified with the <code>memory</code> parameter (if applicable), or all of the available memory on the container instance, whichever comes first. This parameter maps to <code>MemoryReservation</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--memory-reservation</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p> <p>You must specify a non-zero integer for one or both of <code>memory</code> or <code>memoryReservation</code> in container definitions. If you specify both, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance on which the container is placed; otherwise, the value of <code>memory</code> is used.</p> <p>For example, if your container normally uses 128 MiB of memory, but occasionally bursts to 256 MiB of memory for short periods of time, you can set a <code>memoryReservation</code> of 128 MiB, and a <code>memory</code> hard limit of 300 MiB. This configuration would allow the container to only reserve 128 MiB of memory from the remaining resources on the container instance, but also allow the container to consume more memory resources when needed.</p>"]
    #[serde(rename="memoryReservation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[doc="<p>The mount points for data volumes in your container. This parameter maps to <code>Volumes</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--volume</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="mountPoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[doc="<p>The name of a container. If you are linking multiple containers together in a task definition, the <code>name</code> of one container can be entered in the <code>links</code> of another container to connect the containers. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This parameter maps to <code>name</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--name</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. </p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The list of port mappings for the container. Port mappings allow containers to access ports on the host container instance to send or receive traffic. This parameter maps to <code>PortBindings</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--publish</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. If the network mode of a task definition is set to <code>none</code>, then you cannot specify port mappings. If the network mode of a task definition is set to <code>host</code>, then host ports must either be undefined or they must match the container port in the port mapping.</p> <note> <p>After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <b>Network Bindings</b> section of a container description of a selected task in the Amazon ECS console, or the <code>networkBindings</code> section <a>DescribeTasks</a> responses.</p> </note>"]
    #[serde(rename="portMappings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
    #[doc="<p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--privileged</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="privileged")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privileged: Option<bool>,
    #[doc="<p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--read-only</code> option to <code>docker run</code>.</p>"]
    #[serde(rename="readonlyRootFilesystem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[doc="<p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--ulimit</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>. Valid naming values are displayed in the <a>Ulimit</a> data type. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep \"Server API version\"</code> </p>"]
    #[serde(rename="ulimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[doc="<p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--user</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
    #[doc="<p>Data volumes to mount from another container. This parameter maps to <code>VolumesFrom</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--volumes-from</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="volumesFrom")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes_from: Option<Vec<VolumeFrom>>,
    #[doc="<p>The working directory in which to run commands inside the container. This parameter maps to <code>WorkingDir</code> in the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/#create-a-container\">Create a container</a> section of the <a href=\"https://docs.docker.com/engine/reference/api/docker_remote_api_v1.23/\">Docker Remote API</a> and the <code>--workdir</code> option to <a href=\"https://docs.docker.com/engine/reference/run/\">docker run</a>.</p>"]
    #[serde(rename="workingDirectory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub working_directory: Option<String>,
}

#[doc="<p>An EC2 instance that is running the Amazon ECS agent and has been registered with a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ContainerInstance {
    #[doc="<p>This parameter returns <code>true</code> if the agent is actually connected to Amazon ECS. Registered instances with an agent that may be unhealthy or stopped return <code>false</code>, and instances without a connected agent cannot accept placement requests.</p>"]
    #[serde(rename="agentConnected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_connected: Option<bool>,
    #[doc="<p>The status of the most recent agent update. If an update has never been requested, this value is <code>NULL</code>.</p>"]
    #[serde(rename="agentUpdateStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_update_status: Option<String>,
    #[doc="<p>The attributes set for the container instance, either by the Amazon ECS container agent at instance registration or manually with the <a>PutAttributes</a> operation.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[doc="<p>The Amazon Resource Name (ARN) of the container instance. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>"]
    #[serde(rename="containerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[doc="<p>The EC2 instance ID of the container instance.</p>"]
    #[serde(rename="ec2InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    #[doc="<p>The number of tasks on the container instance that are in the <code>PENDING</code> status.</p>"]
    #[serde(rename="pendingTasksCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    #[doc="<p>The Unix timestamp for when the container instance was registered.</p>"]
    #[serde(rename="registeredAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered_at: Option<f64>,
    #[doc="<p>For most resource types, this parameter describes the registered resources on the container instance that are in use by current tasks. For port resource types, this parameter describes the ports that were reserved by the Amazon ECS container agent when it registered the container instance with Amazon ECS.</p>"]
    #[serde(rename="registeredResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered_resources: Option<Vec<Resource>>,
    #[doc="<p>For most resource types, this parameter describes the remaining resources of the container instance that are available for new tasks. For port resource types, this parameter describes the ports that are reserved by the Amazon ECS container agent and any containers that have reserved port mappings; any port that is not specified here is available for new tasks.</p>"]
    #[serde(rename="remainingResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remaining_resources: Option<Vec<Resource>>,
    #[doc="<p>The number of tasks on the container instance that are in the <code>RUNNING</code> status.</p>"]
    #[serde(rename="runningTasksCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_tasks_count: Option<i64>,
    #[doc="<p>The status of the container instance. The valid values are <code>ACTIVE</code>, <code>INACTIVE</code>, or <code>DRAINING</code>. <code>ACTIVE</code> indicates that the container instance can accept tasks. <code>DRAINING</code> indicates that new tasks are not placed on the container instance and any service tasks running on the container instance are removed if possible. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-draining.html\">Container Instance Draining</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The version counter for the container instance. Every time a container instance experiences a change that triggers a CloudWatch event, the version counter is incremented. If you are replicating your Amazon ECS container instance state with CloudWatch events, you can compare the version of a container instance reported by the Amazon ECS APIs with the version reported in CloudWatch events for the container instance (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>"]
    #[serde(rename="version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
    #[doc="<p>The version information for the Amazon ECS container agent and Docker daemon running on the container instance.</p>"]
    #[serde(rename="versionInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[doc="<p>The overrides that should be sent to a container.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ContainerOverride {
    #[doc="<p>The command to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.</p>"]
    #[serde(rename="command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<Vec<String>>,
    #[doc="<p>The number of <code>cpu</code> units reserved for the container, instead of the default value from the task definition. You must also specify a container name.</p>"]
    #[serde(rename="cpu")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu: Option<i64>,
    #[doc="<p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name.</p>"]
    #[serde(rename="environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[doc="<p>The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.</p>"]
    #[serde(rename="memory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory: Option<i64>,
    #[doc="<p>The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition. You must also specify a container name.</p>"]
    #[serde(rename="memoryReservation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory_reservation: Option<i64>,
    #[doc="<p>The name of the container that receives the override. This parameter is required if any override is specified.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateClusterRequest {
    #[doc="<p>The name of your cluster. If you do not specify a name for your cluster, you create a cluster named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>"]
    #[serde(rename="clusterName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_name: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateClusterResponse {
    #[doc="<p>The full description of your new cluster.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateServiceRequest {
    #[doc="<p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. Up to 32 ASCII characters are allowed.</p>"]
    #[serde(rename="clientToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_token: Option<String>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster on which to run your service. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>"]
    #[serde(rename="deploymentConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[doc="<p>The number of instantiations of the specified task definition to place and keep running on your cluster.</p>"]
    #[serde(rename="desiredCount")]
    pub desired_count: i64,
    #[doc="<p>A load balancer object representing the load balancer to use with your service. Currently, you are limited to one load balancer or target group per service. After you create a service, the load balancer name or target group ARN, container name, and container port specified in the service definition are immutable.</p> <p>For Elastic Load Balancing Classic load balancers, this object must contain the load balancer name, the container name (as it appears in a container definition), and the container port to access from the load balancer. When a task from this service is placed on a container instance, the container instance is registered with the load balancer specified here.</p> <p>For Elastic Load Balancing Application load balancers, this object must contain the load balancer target group ARN, the container name (as it appears in a container definition), and the container port to access from the load balancer. When a task from this service is placed on a container instance, the container instance and port combination is registered as a target in the target group specified here.</p>"]
    #[serde(rename="loadBalancers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[doc="<p>An array of placement constraint objects to use for tasks in your service. You can specify a maximum of 10 constraints per task (this limit includes constraints in the task definition and those specified at run time). </p>"]
    #[serde(rename="placementConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[doc="<p>The placement strategy objects to use for tasks in your service. You can specify a maximum of 5 strategy rules per service.</p>"]
    #[serde(rename="placementStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[doc="<p>The name or full Amazon Resource Name (ARN) of the IAM role that allows Amazon ECS to make calls to your load balancer on your behalf. This parameter is required if you are using a load balancer with your service. If you specify the <code>role</code> parameter, you must also specify a load balancer object with the <code>loadBalancers</code> parameter.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path. For example, if a role with the name <code>bar</code> has a path of <code>/foo/</code> then you would specify <code>/foo/bar</code> as the role name. For more information, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names\">Friendly Names and Paths</a> in the <i>IAM User Guide</i>.</p>"]
    #[serde(rename="role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
    #[doc="<p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. Service names must be unique within a cluster, but you can have similarly named services in multiple clusters within a region or across multiple regions.</p>"]
    #[serde(rename="serviceName")]
    pub service_name: String,
    #[doc="<p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to run in your service. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>"]
    #[serde(rename="taskDefinition")]
    pub task_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateServiceResponse {
    #[doc="<p>The full description of your service following the create call.</p>"]
    #[serde(rename="service")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteAttributesRequest {
    #[doc="<p>The attributes to delete from your resource. You can specify up to 10 attributes per request. For custom attributes, specify the attribute name and target ID, but do not specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>"]
    #[serde(rename="attributes")]
    pub attributes: Vec<Attribute>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteAttributesResponse {
    #[doc="<p>A list of attribute objects that were successfully deleted from your resource.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteClusterRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster to delete.</p>"]
    #[serde(rename="cluster")]
    pub cluster: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteClusterResponse {
    #[doc="<p>The full description of the deleted cluster.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteServiceRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to delete. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The name of the service to delete.</p>"]
    #[serde(rename="service")]
    pub service: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteServiceResponse {
    #[doc="<p>The full description of the deleted service.</p>"]
    #[serde(rename="service")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service: Option<Service>,
}

#[doc="<p>The details of an Amazon ECS service deployment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Deployment {
    #[doc="<p>The Unix timestamp for when the service was created.</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The most recent desired count of tasks that was specified for the service to deploy or maintain.</p>"]
    #[serde(rename="desiredCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_count: Option<i64>,
    #[doc="<p>The ID of the deployment.</p>"]
    #[serde(rename="id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The number of tasks in the deployment that are in the <code>PENDING</code> status.</p>"]
    #[serde(rename="pendingCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_count: Option<i64>,
    #[doc="<p>The number of tasks in the deployment that are in the <code>RUNNING</code> status.</p>"]
    #[serde(rename="runningCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_count: Option<i64>,
    #[doc="<p>The status of the deployment. Valid values are <code>PRIMARY</code> (for the most recent deployment), <code>ACTIVE</code> (for previous deployments that still have tasks running, but are being replaced with the <code>PRIMARY</code> deployment), and <code>INACTIVE</code> (for deployments that have been completely replaced).</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The most recent task definition that was specified for the service to use.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<String>,
    #[doc="<p>The Unix timestamp for when the service was last updated.</p>"]
    #[serde(rename="updatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<f64>,
}

#[doc="<p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeploymentConfiguration {
    #[doc="<p>The upper limit (as a percentage of the service's <code>desiredCount</code>) of the number of tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state in a service during a deployment. The maximum number of tasks during a deployment is the <code>desiredCount</code> multiplied by <code>maximumPercent</code>/100, rounded down to the nearest integer value.</p>"]
    #[serde(rename="maximumPercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_percent: Option<i64>,
    #[doc="<p>The lower limit (as a percentage of the service's <code>desiredCount</code>) of the number of running tasks that must remain in the <code>RUNNING</code> state in a service during a deployment. The minimum healthy tasks during a deployment is the <code>desiredCount</code> multiplied by <code>minimumHealthyPercent</code>/100, rounded up to the nearest integer value.</p>"]
    #[serde(rename="minimumHealthyPercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_healthy_percent: Option<i64>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterContainerInstanceRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to deregister. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The container instance ID or full Amazon Resource Name (ARN) of the container instance to deregister. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>"]
    #[serde(rename="containerInstance")]
    pub container_instance: String,
    #[doc="<p>Forces the deregistration of the container instance. If you have tasks running on the container instance when you deregister it with the <code>force</code> option, these tasks remain running until you terminate the instance or the tasks stop through some other means, but they are orphaned (no longer monitored or accounted for by Amazon ECS). If an orphaned task on your container instance is part of an Amazon ECS service, then the service scheduler starts another copy of that task, on a different container instance if possible. </p> <p>Any containers in orphaned service tasks that are registered with a Classic load balancer or an Application load balancer target group are deregistered, and they will begin connection draining according to the settings on the load balancer or target group.</p>"]
    #[serde(rename="force")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeregisterContainerInstanceResponse {
    #[doc="<p>The container instance that was deregistered.</p>"]
    #[serde(rename="containerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterTaskDefinitionRequest {
    #[doc="<p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to deregister. You must specify a <code>revision</code>.</p>"]
    #[serde(rename="taskDefinition")]
    pub task_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeregisterTaskDefinitionResponse {
    #[doc="<p>The full description of the deregistered task.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeClustersRequest {
    #[doc="<p>A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="clusters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clusters: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeClustersResponse {
    #[doc="<p>The list of clusters.</p>"]
    #[serde(rename="clusters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeContainerInstancesRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to describe. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>A list of container instance IDs or full Amazon Resource Name (ARN) entries.</p>"]
    #[serde(rename="containerInstances")]
    pub container_instances: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeContainerInstancesResponse {
    #[doc="<p>The list of container instances.</p>"]
    #[serde(rename="containerInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeServicesRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN)the cluster that hosts the service to describe. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>A list of services to describe. You may specify up to 10 services to describe in a single operation.</p>"]
    #[serde(rename="services")]
    pub services: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeServicesResponse {
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[doc="<p>The list of services described.</p>"]
    #[serde(rename="services")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTaskDefinitionRequest {
    #[doc="<p>The <code>family</code> for the latest <code>ACTIVE</code> revision, <code>family</code> and <code>revision</code> (<code>family:revision</code>) for a specific revision in the family, or full Amazon Resource Name (ARN) of the task definition to describe.</p>"]
    #[serde(rename="taskDefinition")]
    pub task_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTaskDefinitionResponse {
    #[doc="<p>The full task definition description.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTasksRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to describe. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>A list of up to 100 task IDs or full Amazon Resource Name (ARN) entries.</p>"]
    #[serde(rename="tasks")]
    pub tasks: Vec<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTasksResponse {
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[doc="<p>The list of tasks.</p>"]
    #[serde(rename="tasks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DiscoverPollEndpointRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The container instance ID or full Amazon Resource Name (ARN) of the container instance. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>"]
    #[serde(rename="containerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DiscoverPollEndpointResponse {
    #[doc="<p>The endpoint for the Amazon ECS agent to poll.</p>"]
    #[serde(rename="endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint: Option<String>,
    #[doc="<p>The telemetry endpoint for the Amazon ECS agent.</p>"]
    #[serde(rename="telemetryEndpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub telemetry_endpoint: Option<String>,
}

#[doc="<p>A failed resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Failure {
    #[doc="<p>The Amazon Resource Name (ARN) of the failed resource.</p>"]
    #[serde(rename="arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>The reason for the failure.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
}

#[doc="<p>Hostnames and IP address entries that are added to the <code>/etc/hosts</code> file of a container via the <code>extraHosts</code> parameter of its <a>ContainerDefinition</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct HostEntry {
    #[doc="<p>The hostname to use in the <code>/etc/hosts</code> entry.</p>"]
    #[serde(rename="hostname")]
    pub hostname: String,
    #[doc="<p>The IP address to use in the <code>/etc/hosts</code> entry.</p>"]
    #[serde(rename="ipAddress")]
    pub ip_address: String,
}

#[doc="<p>Details on a container instance host volume.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct HostVolumeProperties {
    #[doc="<p>The path on the host container instance that is presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If the <code>host</code> parameter contains a <code>sourcePath</code> file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the <code>sourcePath</code> value does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p>"]
    #[serde(rename="sourcePath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_path: Option<String>,
}

#[doc="<p>A key and value pair object.</p>"]
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
pub struct ListAttributesRequest {
    #[doc="<p>The name of the attribute with which to filter the results. </p>"]
    #[serde(rename="attributeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc="<p>The value of the attribute with which to filter results. You must also specify an attribute name to use this parameter.</p>"]
    #[serde(rename="attributeValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster to list attributes. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The maximum number of cluster results returned by <code>ListAttributes</code> in paginated output. When this parameter is used, <code>ListAttributes</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListAttributes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListAttributes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListAttributes</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The type of the target with which to list attributes.</p>"]
    #[serde(rename="targetType")]
    pub target_type: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAttributesResponse {
    #[doc="<p>A list of attribute objects that meet the criteria of the request.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListClustersRequest {
    #[doc="<p>The maximum number of cluster results returned by <code>ListClusters</code> in paginated output. When this parameter is used, <code>ListClusters</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListClusters</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListClusters</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListClusters</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListClustersResponse {
    #[doc="<p>The list of full Amazon Resource Name (ARN) entries for each cluster associated with your account.</p>"]
    #[serde(rename="clusterArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_arns: Option<Vec<String>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListClusters</code> request. When the results of a <code>ListClusters</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListContainerInstancesRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html\">Cluster Query Language</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<String>,
    #[doc="<p>The maximum number of container instance results returned by <code>ListContainerInstances</code> in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListContainerInstances</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <a>UpdateContainerInstancesState</a>. If you do not specify this parameter, the default is to include container instances set to <code>ACTIVE</code> and <code>DRAINING</code>.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListContainerInstancesResponse {
    #[doc="<p>The list of container instances with full Amazon Resource Name (ARN) entries for each container instance associated with the specified cluster.</p>"]
    #[serde(rename="containerInstanceArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arns: Option<Vec<String>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListContainerInstances</code> request. When the results of a <code>ListContainerInstances</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListServicesRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the services to list. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The maximum number of container instance results returned by <code>ListServices</code> in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListServices</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListServicesResponse {
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListServices</code> request. When the results of a <code>ListServices</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of full Amazon Resource Name (ARN) entries for each service associated with the specified cluster.</p>"]
    #[serde(rename="serviceArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_arns: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTaskDefinitionFamiliesRequest {
    #[doc="<p>The <code>familyPrefix</code> is a string that is used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>"]
    #[serde(rename="familyPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_prefix: Option<String>,
    #[doc="<p>The maximum number of task definition family results returned by <code>ListTaskDefinitionFamilies</code> in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListTaskDefinitionFamilies</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The task definition family status with which to filter the <code>ListTaskDefinitionFamilies</code> results. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTaskDefinitionFamiliesResponse {
    #[doc="<p>The list of task definition family names that match the <code>ListTaskDefinitionFamilies</code> request.</p>"]
    #[serde(rename="families")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub families: Option<Vec<String>>,
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitionFamilies</code> request. When the results of a <code>ListTaskDefinitionFamilies</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTaskDefinitionsRequest {
    #[doc="<p>The full family name with which to filter the <code>ListTaskDefinitions</code> results. Specifying a <code>familyPrefix</code> limits the listed task definitions to task definition revisions that belong to that family.</p>"]
    #[serde(rename="familyPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_prefix: Option<String>,
    #[doc="<p>The maximum number of task definition results returned by <code>ListTaskDefinitions</code> in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTaskDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListTaskDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The order in which to sort the results. Valid values are <code>ASC</code> and <code>DESC</code>. By default (<code>ASC</code>), task definitions are listed lexicographically by family name and in ascending numerical order by revision so that the newest task definitions in a family are listed last. Setting this parameter to <code>DESC</code> reverses the sort order on family name and revision so that the newest task definitions in a family are listed first.</p>"]
    #[serde(rename="sort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort: Option<String>,
    #[doc="<p>The task definition status with which to filter the <code>ListTaskDefinitions</code> results. By default, only <code>ACTIVE</code> task definitions are listed. By setting this parameter to <code>INACTIVE</code>, you can view task definitions that are <code>INACTIVE</code> as long as an active task or service still references them. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTaskDefinitionsResponse {
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitions</code> request. When the results of a <code>ListTaskDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of task definition Amazon Resource Name (ARN) entries for the <code>ListTaskDefinitions</code> request.</p>"]
    #[serde(rename="taskDefinitionArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition_arns: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTasksRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the tasks to list. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The container instance ID or full Amazon Resource Name (ARN) of the container instance with which to filter the <code>ListTasks</code> results. Specifying a <code>containerInstance</code> limits the results to tasks that belong to that container instance.</p>"]
    #[serde(rename="containerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance: Option<String>,
    #[doc="<p>The task desired status with which to filter the <code>ListTasks</code> results. Specifying a <code>desiredStatus</code> of <code>STOPPED</code> limits the results to tasks that ECS has set the desired status to <code>STOPPED</code>, which can be useful for debugging tasks that are not starting properly or have died or finished. The default status filter is <code>RUNNING</code>, which shows tasks that ECS has set the desired status to <code>RUNNING</code>.</p> <note> <p>Although you can filter results based on a desired status of <code>PENDING</code>, this will not return any results because ECS never sets the desired status of a task to that value (only a task's <code>lastStatus</code> may have a value of <code>PENDING</code>).</p> </note>"]
    #[serde(rename="desiredStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_status: Option<String>,
    #[doc="<p>The name of the family with which to filter the <code>ListTasks</code> results. Specifying a <code>family</code> limits the results to tasks that belong to that family.</p>"]
    #[serde(rename="family")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family: Option<String>,
    #[doc="<p>The maximum number of task results returned by <code>ListTasks</code> in paginated output. When this parameter is used, <code>ListTasks</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTasks</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTasks</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The <code>nextToken</code> value returned from a previous paginated <code>ListTasks</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the service with which to filter the <code>ListTasks</code> results. Specifying a <code>serviceName</code> limits the results to tasks that belong to that service.</p>"]
    #[serde(rename="serviceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_name: Option<String>,
    #[doc="<p>The <code>startedBy</code> value with which to filter the task results. Specifying a <code>startedBy</code> value limits the results to tasks that were started with that value.</p>"]
    #[serde(rename="startedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_by: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTasksResponse {
    #[doc="<p>The <code>nextToken</code> value to include in a future <code>ListTasks</code> request. When the results of a <code>ListTasks</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The list of task Amazon Resource Name (ARN) entries for the <code>ListTasks</code> request.</p>"]
    #[serde(rename="taskArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_arns: Option<Vec<String>>,
}

#[doc="<p>Details on a load balancer that is used with a service.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct LoadBalancer {
    #[doc="<p>The name of the container (as it appears in a container definition) to associate with the load balancer.</p>"]
    #[serde(rename="containerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_name: Option<String>,
    #[doc="<p>The port on the container to associate with the load balancer. This port must correspond to a <code>containerPort</code> in the service's task definition. Your container instances must allow ingress traffic on the <code>hostPort</code> of the port mapping.</p>"]
    #[serde(rename="containerPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_port: Option<i64>,
    #[doc="<p>The name of a Classic load balancer.</p>"]
    #[serde(rename="loadBalancerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[doc="<p>The full Amazon Resource Name (ARN) of the Elastic Load Balancing target group associated with a service.</p>"]
    #[serde(rename="targetGroupArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_group_arn: Option<String>,
}

#[doc="<p>Log configuration options to send to a custom log driver for the container.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct LogConfiguration {
    #[doc="<p>The log driver to use for the container. The valid values listed for this parameter are log drivers that the Amazon ECS container agent can communicate with by default. </p> <note> <p>If you have a custom driver that is not listed above that you would like to work with the Amazon ECS container agent, you can fork the Amazon ECS container agent project that is <a href=\"https://github.com/aws/amazon-ecs-agent\">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you would like to have included. However, Amazon Web Services does not currently provide support for running modified copies of this software.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep \"Server API version\"</code> </p>"]
    #[serde(rename="logDriver")]
    pub log_driver: String,
    #[doc="<p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep \"Server API version\"</code> </p>"]
    #[serde(rename="options")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>Details on a volume mount point that is used in a container definition.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MountPoint {
    #[doc="<p>The path on the container to mount the host volume at.</p>"]
    #[serde(rename="containerPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_path: Option<String>,
    #[doc="<p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>"]
    #[serde(rename="readOnly")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,
    #[doc="<p>The name of the volume to mount.</p>"]
    #[serde(rename="sourceVolume")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_volume: Option<String>,
}

#[doc="<p>Details on the network bindings between a container and its host container instance. After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct NetworkBinding {
    #[doc="<p>The IP address that the container is bound to on the container instance.</p>"]
    #[serde(rename="bindIP")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bind_ip: Option<String>,
    #[doc="<p>The port number on the container that is be used with the network binding.</p>"]
    #[serde(rename="containerPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_port: Option<i64>,
    #[doc="<p>The port number on the host that is used with the network binding.</p>"]
    #[serde(rename="hostPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_port: Option<i64>,
    #[doc="<p>The protocol used for the network binding.</p>"]
    #[serde(rename="protocol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protocol: Option<String>,
}

#[doc="<p>An object representing a constraint on task placement. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html\">Task Placement Constraints</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PlacementConstraint {
    #[doc="<p>A cluster query language expression to apply to the constraint. Note you cannot specify an expression if the constraint type is <code>distinctInstance</code>. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html\">Cluster Query Language</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<String>,
    #[doc="<p>The type of constraint. Use <code>distinctInstance</code> to ensure that each task in a particular group is running on a different container instance. Use <code>memberOf</code> to restrict selection to a group of valid candidates. Note that <code>distinctInstance</code> is not supported in task definitions.</p>"]
    #[serde(rename="type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>The task placement strategy for a task or service. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-strategies.html\">Task Placement Strategies</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PlacementStrategy {
    #[doc="<p>The field to apply the placement strategy against. For the <code>spread</code> placement strategy, valid values are <code>instanceId</code> (or <code>host</code>, which has the same effect), or any platform or custom attribute that is applied to a container instance, such as <code>attribute:ecs.availability-zone</code>. For the <code>binpack</code> placement strategy, valid values are <code>cpu</code> and <code>memory</code>. For the <code>random</code> placement strategy, this field is not used.</p>"]
    #[serde(rename="field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,
    #[doc="<p>The type of placement strategy. The <code>random</code> placement strategy randomly places tasks on available candidates. The <code>spread</code> placement strategy spreads placement across available candidates evenly based on the <code>field</code> parameter. The <code>binpack</code> strategy places tasks on available candidates that have the least available amount of the resource that is specified with the <code>field</code> parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task).</p>"]
    #[serde(rename="type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>Port mappings allow containers to access ports on the host container instance to send or receive traffic. Port mappings are specified as part of the container definition. After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PortMapping {
    #[doc="<p>The port number on the container that is bound to the user-specified or automatically assigned host port. If you specify a container port and not a host port, your container automatically receives a host port in the ephemeral port range (for more information, see <code>hostPort</code>). Port mappings that are automatically assigned in this way do not count toward the 100 reserved ports limit of a container instance.</p>"]
    #[serde(rename="containerPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_port: Option<i64>,
    #[doc="<p>The port number on the container instance to reserve for your container. You can specify a non-reserved host port for your container port mapping, or you can omit the <code>hostPort</code> (or set it to <code>0</code>) while specifying a <code>containerPort</code> and your container automatically receives a port in the ephemeral port range for your container instance operating system and Docker version.</p> <p>The default ephemeral port range for Docker version 1.6.0 and later is listed on the instance under <code>/proc/sys/net/ipv4/ip_local_port_range</code>; if this kernel parameter is unavailable, the default ephemeral port range of 49153 to 65535 is used. You should not attempt to specify a host port in the ephemeral port range as these are reserved for automatic assignment. In general, ports below 32768 are outside of the ephemeral port range.</p> <note> <p>The default ephemeral port range of 49153 to 65535 will always be used for Docker versions prior to 1.6.0.</p> </note> <p>The default reserved ports are 22 for SSH, the Docker ports 2375 and 2376, and the Amazon ECS container agent ports 51678 and 51679. Any host port that was previously specified in a running task is also reserved while the task is running (after a task stops, the host port is released).The current reserved ports are displayed in the <code>remainingResources</code> of <a>DescribeContainerInstances</a> output, and a container instance may have up to 100 reserved ports at a time, including the default reserved ports (automatically assigned ports do not count toward the 100 reserved ports limit).</p>"]
    #[serde(rename="hostPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_port: Option<i64>,
    #[doc="<p>The protocol used for the port mapping. Valid values are <code>tcp</code> and <code>udp</code>. The default is <code>tcp</code>.</p>"]
    #[serde(rename="protocol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutAttributesRequest {
    #[doc="<p>The attributes to apply to your resource. You can specify up to 10 custom attributes per resource. You can specify up to 10 attributes in a single call.</p>"]
    #[serde(rename="attributes")]
    pub attributes: Vec<Attribute>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to apply attributes. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutAttributesResponse {
    #[doc="<p>The attributes applied to your resource.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterContainerInstanceRequest {
    #[doc="<p>The container instance attributes that this container instance supports.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster with which to register your container instance. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the container instance (if it was previously registered).</p>"]
    #[serde(rename="containerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[doc="<p>The instance identity document for the EC2 instance to register. This document can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/document/</code> </p>"]
    #[serde(rename="instanceIdentityDocument")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_identity_document: Option<String>,
    #[doc="<p>The instance identity document signature for the EC2 instance to register. This signature can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/signature/</code> </p>"]
    #[serde(rename="instanceIdentityDocumentSignature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_identity_document_signature: Option<String>,
    #[doc="<p>The resources available on the instance.</p>"]
    #[serde(rename="totalResources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_resources: Option<Vec<Resource>>,
    #[doc="<p>The version information for the Amazon ECS container agent and Docker daemon running on the container instance.</p>"]
    #[serde(rename="versionInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterContainerInstanceResponse {
    #[doc="<p>The container instance that was registered.</p>"]
    #[serde(rename="containerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterTaskDefinitionRequest {
    #[doc="<p>A list of container definitions in JSON format that describe the different containers that make up your task.</p>"]
    #[serde(rename="containerDefinitions")]
    pub container_definitions: Vec<ContainerDefinition>,
    #[doc="<p>You must specify a <code>family</code> for a task definition, which allows you to track multiple versions of the same task definition. The <code>family</code> is used as a name for your task definition. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>"]
    #[serde(rename="family")]
    pub family: String,
    #[doc="<p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, and <code>host</code>. </p> <p>The default Docker network mode is <code>bridge</code>. If the network mode is set to <code>none</code>, you cannot specify port mappings in your container definitions, and the task's containers do not have external connectivity. The <code>host</code> network mode offers the highest networking performance for containers because they use the host network stack instead of the virtualized network stack provided by the <code>bridge</code> mode; however, exposed container ports are mapped directly to the corresponding host port, so you cannot take advantage of dynamic host port mappings or run multiple instantiations of the same task on a single container instance if port mappings are used.</p> <p>For more information, see <a href=\"https://docs.docker.com/engine/reference/run/#network-settings\">Network settings</a> in the <i>Docker run reference</i>.</p>"]
    #[serde(rename="networkMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_mode: Option<String>,
    #[doc="<p>An array of placement constraint objects to use for the task. You can specify a maximum of 10 constraints per task (this limit includes constraints in the task definition and those specified at run time).</p>"]
    #[serde(rename="placementConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html\">IAM Roles for Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="taskRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_role_arn: Option<String>,
    #[doc="<p>A list of volume definitions in JSON format that containers in your task may use.</p>"]
    #[serde(rename="volumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterTaskDefinitionResponse {
    #[doc="<p>The full description of the registered task definition.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[doc="<p>Describes the resources available for a container instance.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Resource {
    #[doc="<p>When the <code>doubleValue</code> type is set, the value of the resource must be a double precision floating-point type.</p>"]
    #[serde(rename="doubleValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub double_value: Option<f64>,
    #[doc="<p>When the <code>integerValue</code> type is set, the value of the resource must be an integer.</p>"]
    #[serde(rename="integerValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub integer_value: Option<i64>,
    #[doc="<p>When the <code>longValue</code> type is set, the value of the resource must be an extended precision floating-point type.</p>"]
    #[serde(rename="longValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub long_value: Option<i64>,
    #[doc="<p>The name of the resource, such as <code>cpu</code>, <code>memory</code>, <code>ports</code>, or a user-defined resource.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>When the <code>stringSetValue</code> type is set, the value of the resource must be a string type.</p>"]
    #[serde(rename="stringSetValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub string_set_value: Option<Vec<String>>,
    #[doc="<p>The type of the resource, such as <code>INTEGER</code>, <code>DOUBLE</code>, <code>LONG</code>, or <code>STRINGSET</code>.</p>"]
    #[serde(rename="type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RunTaskRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster on which to run your task. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The number of instantiations of the specified task to place on your cluster. You can specify up to 10 tasks per call.</p>"]
    #[serde(rename="count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,
    #[doc="<p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, family:my-family-name).</p>"]
    #[serde(rename="group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,
    #[doc="<p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it should receive. You can override the default command for a container (that is specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <note> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p> </note>"]
    #[serde(rename="overrides")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[doc="<p>An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at run time).</p>"]
    #[serde(rename="placementConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[doc="<p>The placement strategy objects to use for the task. You can specify a maximum of 5 strategy rules per task.</p>"]
    #[serde(rename="placementStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[doc="<p>An optional tag specified when a task is started. For example if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p> <p>If a task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>"]
    #[serde(rename="startedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_by: Option<String>,
    #[doc="<p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to run. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>"]
    #[serde(rename="taskDefinition")]
    pub task_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RunTaskResponse {
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[doc="<p>A full description of the tasks that were run. Each task that was successfully placed on your cluster are described here.</p>"]
    #[serde(rename="tasks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[doc="<p>Details on a service within a cluster</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Service {
    #[doc="<p>The Amazon Resource Name (ARN) of the cluster that hosts the service.</p>"]
    #[serde(rename="clusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_arn: Option<String>,
    #[doc="<p>The Unix timestamp for when the service was created.</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>"]
    #[serde(rename="deploymentConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[doc="<p>The current state of deployments for the service.</p>"]
    #[serde(rename="deployments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    #[doc="<p>The desired number of instantiations of the task definition to keep running on the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>"]
    #[serde(rename="desiredCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_count: Option<i64>,
    #[doc="<p>The event stream for your service. A maximum of 100 of the latest events are displayed.</p>"]
    #[serde(rename="events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<ServiceEvent>>,
    #[doc="<p>A list of Elastic Load Balancing load balancer objects, containing the load balancer name, the container name (as it appears in a container definition), and the container port to access from the load balancer.</p>"]
    #[serde(rename="loadBalancers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    #[doc="<p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>"]
    #[serde(rename="pendingCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_count: Option<i64>,
    #[doc="<p>The placement constraints for the tasks in the service.</p>"]
    #[serde(rename="placementConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[doc="<p>The placement strategy that determines how tasks for the service are placed.</p>"]
    #[serde(rename="placementStrategy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role associated with the service that allows the Amazon ECS container agent to register container instances with an Elastic Load Balancing load balancer.</p>"]
    #[serde(rename="roleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>"]
    #[serde(rename="runningCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_count: Option<i64>,
    #[doc="<p>The Amazon Resource Name (ARN) that identifies the service. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the region of the service, the AWS account ID of the service owner, the <code>service</code> namespace, and then the service name. For example, <code>arn:aws:ecs:<i>region</i>:<i>012345678910</i>:service/<i>my-service</i> </code>.</p>"]
    #[serde(rename="serviceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_arn: Option<String>,
    #[doc="<p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. Service names must be unique within a cluster, but you can have similarly named services in multiple clusters within a region or across multiple regions.</p>"]
    #[serde(rename="serviceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_name: Option<String>,
    #[doc="<p>The status of the service. The valid values are <code>ACTIVE</code>, <code>DRAINING</code>, or <code>INACTIVE</code>.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The task definition to use for tasks in the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<String>,
}

#[doc="<p>Details on an event associated with a service.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ServiceEvent {
    #[doc="<p>The Unix timestamp for when the event was triggered.</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The ID string of the event.</p>"]
    #[serde(rename="id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The event message.</p>"]
    #[serde(rename="message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartTaskRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster on which to start your task. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The container instance IDs or full Amazon Resource Name (ARN) entries for the container instances on which you would like to place your task. You can specify up to 10 container instances.</p>"]
    #[serde(rename="containerInstances")]
    pub container_instances: Vec<String>,
    #[doc="<p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, family:my-family-name).</p>"]
    #[serde(rename="group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,
    #[doc="<p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it should receive. You can override the default command for a container (that is specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <note> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p> </note>"]
    #[serde(rename="overrides")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[doc="<p>An optional tag specified when a task is started. For example if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p> <p>If a task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>"]
    #[serde(rename="startedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_by: Option<String>,
    #[doc="<p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to start. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>"]
    #[serde(rename="taskDefinition")]
    pub task_definition: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartTaskResponse {
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    #[doc="<p>A full description of the tasks that were started. Each task that was successfully placed on your container instances are described here.</p>"]
    #[serde(rename="tasks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopTaskRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to stop. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>An optional message specified when a task is stopped. For example, if you are using a custom scheduler, you can use this parameter to specify the reason for stopping the task here, and the message will appear in subsequent <a>DescribeTasks</a> API operations on this task. Up to 255 characters are allowed in this message.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The task ID or full Amazon Resource Name (ARN) entry of the task to stop.</p>"]
    #[serde(rename="task")]
    pub task: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopTaskResponse {
    #[doc="<p>The task that was stopped.</p>"]
    #[serde(rename="task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<Task>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SubmitContainerStateChangeRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The name of the container.</p>"]
    #[serde(rename="containerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_name: Option<String>,
    #[doc="<p>The exit code returned for the state change request.</p>"]
    #[serde(rename="exitCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc="<p>The network bindings of the container.</p>"]
    #[serde(rename="networkBindings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    #[doc="<p>The reason for the state change request.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The status of the state change request.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>"]
    #[serde(rename="task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SubmitContainerStateChangeResponse {
    #[doc="<p>Acknowledgement of the state change.</p>"]
    #[serde(rename="acknowledgment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SubmitTaskStateChangeRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The reason for the state change request.</p>"]
    #[serde(rename="reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The status of the state change request.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The task ID or full Amazon Resource Name (ARN) of the task in the state change request.</p>"]
    #[serde(rename="task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SubmitTaskStateChangeResponse {
    #[doc="<p>Acknowledgement of the state change.</p>"]
    #[serde(rename="acknowledgment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[doc="<p>Details on a task in a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Task {
    #[doc="<p>The Amazon Resource Name (ARN) of the cluster that hosts the task.</p>"]
    #[serde(rename="clusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_arn: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the container instances that host the task.</p>"]
    #[serde(rename="containerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[doc="<p>The containers associated with the task.</p>"]
    #[serde(rename="containers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[doc="<p>The Unix timestamp for when the task was created (the task entered the <code>PENDING</code> state).</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The desired status of the task.</p>"]
    #[serde(rename="desiredStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_status: Option<String>,
    #[doc="<p>The name of the task group associated with the task.</p>"]
    #[serde(rename="group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,
    #[doc="<p>The last known status of the task.</p>"]
    #[serde(rename="lastStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_status: Option<String>,
    #[doc="<p>One or more container overrides.</p>"]
    #[serde(rename="overrides")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub overrides: Option<TaskOverride>,
    #[doc="<p>The Unix timestamp for when the task was started (the task transitioned from the <code>PENDING</code> state to the <code>RUNNING</code> state).</p>"]
    #[serde(rename="startedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<f64>,
    #[doc="<p>The tag specified when a task is started. If the task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>"]
    #[serde(rename="startedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_by: Option<String>,
    #[doc="<p>The Unix timestamp for when the task was stopped (the task transitioned from the <code>RUNNING</code> state to the <code>STOPPED</code> state).</p>"]
    #[serde(rename="stoppedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped_at: Option<f64>,
    #[doc="<p>The reason the task was stopped.</p>"]
    #[serde(rename="stoppedReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped_reason: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the task.</p>"]
    #[serde(rename="taskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_arn: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the task definition that creates the task.</p>"]
    #[serde(rename="taskDefinitionArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition_arn: Option<String>,
    #[doc="<p>The version counter for the task. Every time a task experiences a change that triggers a CloudWatch event, the version counter is incremented. If you are replicating your Amazon ECS task state with CloudWatch events, you can compare the version of a task reported by the Amazon ECS APIs with the version reported in CloudWatch events for the task (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>"]
    #[serde(rename="version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="<p>Details of a task definition.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TaskDefinition {
    #[doc="<p>A list of container definitions in JSON format that describe the different containers that make up your task. For more information about container definition parameters and defaults, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html\">Amazon ECS Task Definitions</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="containerDefinitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,
    #[doc="<p>The family of your task definition, used as the definition name.</p>"]
    #[serde(rename="family")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family: Option<String>,
    #[doc="<p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, and <code>host</code>. </p> <p>If the network mode is <code>none</code>, the containers do not have external connectivity. The default Docker network mode is <code>bridge</code>. The <code>host</code> network mode offers the highest networking performance for containers because it uses the host network stack instead of the virtualized network stack provided by the <code>bridge</code> mode.</p> <p>For more information, see <a href=\"https://docs.docker.com/engine/reference/run/#network-settings\">Network settings</a> in the <i>Docker run reference</i>.</p>"]
    #[serde(rename="networkMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_mode: Option<String>,
    #[doc="<p>An array of placement constraint objects to use for tasks. </p>"]
    #[serde(rename="placementConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    #[doc="<p>The container instance attributes required by your task.</p>"]
    #[serde(rename="requiresAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requires_attributes: Option<Vec<Attribute>>,
    #[doc="<p>The revision of the task in a particular family. The revision is a version number of a task definition in a family. When you register a task definition for the first time, the revision is <code>1</code>; each time you register a new revision of a task definition in the same family, the revision value always increases by one (even if you have deregistered previous revisions in this family).</p>"]
    #[serde(rename="revision")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub revision: Option<i64>,
    #[doc="<p>The status of the task definition.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The full Amazon Resource Name (ARN) of the task definition.</p>"]
    #[serde(rename="taskDefinitionArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition_arn: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.</p>"]
    #[serde(rename="taskRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_role_arn: Option<String>,
    #[doc="<p>The list of volumes in a task. For more information about volume definition parameters and defaults, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html\">Amazon ECS Task Definitions</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="volumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[doc="<p>An object representing a constraint on task placement in the task definition. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html\">Task Placement Constraints</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TaskDefinitionPlacementConstraint {
    #[doc="<p>A cluster query language expression to apply to the constraint. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html\">Cluster Query Language</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<String>,
    #[doc="<p>The type of constraint. The <code>DistinctInstance</code> constraint ensures that each task in a particular group is running on a different container instance. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>"]
    #[serde(rename="type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[doc="<p>The overrides associated with a task.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TaskOverride {
    #[doc="<p>One or more container overrides sent to a task.</p>"]
    #[serde(rename="containerOverrides")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_overrides: Option<Vec<ContainerOverride>>,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.</p>"]
    #[serde(rename="taskRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[doc="<p>The <code>ulimit</code> settings to pass to the container.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Ulimit {
    #[doc="<p>The hard limit for the ulimit type.</p>"]
    #[serde(rename="hardLimit")]
    pub hard_limit: i64,
    #[doc="<p>The <code>type</code> of the <code>ulimit</code>.</p>"]
    #[serde(rename="name")]
    pub name: String,
    #[doc="<p>The soft limit for the ulimit type.</p>"]
    #[serde(rename="softLimit")]
    pub soft_limit: i64,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateContainerAgentRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that your container instance is running on. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>The container instance ID or full Amazon Resource Name (ARN) entries for the container instance on which you would like to update the Amazon ECS container agent.</p>"]
    #[serde(rename="containerInstance")]
    pub container_instance: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateContainerAgentResponse {
    #[doc="<p>The container instance for which the container agent was updated.</p>"]
    #[serde(rename="containerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateContainerInstancesStateRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to update. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>A list of container instance IDs or full Amazon Resource Name (ARN) entries.</p>"]
    #[serde(rename="containerInstances")]
    pub container_instances: Vec<String>,
    #[doc="<p>The container instance state with which to update the container instance.</p>"]
    #[serde(rename="status")]
    pub status: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateContainerInstancesStateResponse {
    #[doc="<p>The list of container instances.</p>"]
    #[serde(rename="containerInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    #[doc="<p>Any failures associated with the call.</p>"]
    #[serde(rename="failures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateServiceRequest {
    #[doc="<p>The short name or full Amazon Resource Name (ARN) of the cluster that your service is running on. If you do not specify a cluster, the default cluster is assumed.</p>"]
    #[serde(rename="cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<String>,
    #[doc="<p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>"]
    #[serde(rename="deploymentConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[doc="<p>The number of instantiations of the task to place and keep running in your service.</p>"]
    #[serde(rename="desiredCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_count: Option<i64>,
    #[doc="<p>The name of the service to update.</p>"]
    #[serde(rename="service")]
    pub service: String,
    #[doc="<p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to run in your service. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used. If you modify the task definition with <code>UpdateService</code>, Amazon ECS spawns a task with the new version of the task definition and then stops an old task after the new version is running.</p>"]
    #[serde(rename="taskDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_definition: Option<String>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateServiceResponse {
    #[doc="<p>The full description of your service following the update call.</p>"]
    #[serde(rename="service")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service: Option<Service>,
}

#[doc="<p>The Docker and Amazon ECS container agent version information about a container instance.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct VersionInfo {
    #[doc="<p>The Git commit hash for the Amazon ECS container agent build on the <a href=\"https://github.com/aws/amazon-ecs-agent/commits/master\">amazon-ecs-agent </a> GitHub repository.</p>"]
    #[serde(rename="agentHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_hash: Option<String>,
    #[doc="<p>The version number of the Amazon ECS container agent.</p>"]
    #[serde(rename="agentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>The Docker version running on the container instance.</p>"]
    #[serde(rename="dockerVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_version: Option<String>,
}

#[doc="<p>A data volume used in a task definition.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Volume {
    #[doc="<p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data is not guaranteed to persist after the containers associated with it stop running.</p>"]
    #[serde(rename="host")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<HostVolumeProperties>,
    #[doc="<p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>"]
    #[serde(rename="name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[doc="<p>Details on a data volume from another container in the same task definition.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct VolumeFrom {
    #[doc="<p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>"]
    #[serde(rename="readOnly")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,
    #[doc="<p>The name of another container within the same task definition to mount volumes from.</p>"]
    #[serde(rename="sourceContainer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_container: Option<String>,
}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl CreateClusterError {
    pub fn from_body(body: &str) -> CreateClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateClusterError::Client(String::from(error_message)),
                    "InvalidParameterException" => {
                        CreateClusterError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => CreateClusterError::Server(String::from(error_message)),
                    "ValidationException" => {
                        CreateClusterError::Validation(error_message.to_string())
                    }
                    _ => CreateClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateClusterError {
    fn from(err: serde_json::error::Error) -> CreateClusterError {
        CreateClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateClusterError {
    fn from(err: CredentialsError) -> CreateClusterError {
        CreateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterError {
    fn from(err: HttpDispatchError) -> CreateClusterError {
        CreateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterError {
    fn from(err: io::Error) -> CreateClusterError {
        CreateClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::Client(ref cause) => cause,
            CreateClusterError::InvalidParameter(ref cause) => cause,
            CreateClusterError::Server(ref cause) => cause,
            CreateClusterError::Validation(ref cause) => cause,
            CreateClusterError::Credentials(ref err) => err.description(),
            CreateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateService
#[derive(Debug, PartialEq)]
pub enum CreateServiceError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl CreateServiceError {
    pub fn from_body(body: &str) -> CreateServiceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateServiceError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        CreateServiceError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateServiceError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => CreateServiceError::Server(String::from(error_message)),
                    "ValidationException" => {
                        CreateServiceError::Validation(error_message.to_string())
                    }
                    _ => CreateServiceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateServiceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateServiceError {
    fn from(err: serde_json::error::Error) -> CreateServiceError {
        CreateServiceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateServiceError {
    fn from(err: CredentialsError) -> CreateServiceError {
        CreateServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateServiceError {
    fn from(err: HttpDispatchError) -> CreateServiceError {
        CreateServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateServiceError {
    fn from(err: io::Error) -> CreateServiceError {
        CreateServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateServiceError {
    fn description(&self) -> &str {
        match *self {
            CreateServiceError::Client(ref cause) => cause,
            CreateServiceError::ClusterNotFound(ref cause) => cause,
            CreateServiceError::InvalidParameter(ref cause) => cause,
            CreateServiceError::Server(ref cause) => cause,
            CreateServiceError::Validation(ref cause) => cause,
            CreateServiceError::Credentials(ref err) => err.description(),
            CreateServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateServiceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteAttributesError {
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>The specified target could not be found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and region-specific.</p>
    TargetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteAttributesError {
    pub fn from_body(body: &str) -> DeleteAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClusterNotFoundException" => {
                        DeleteAttributesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "TargetNotFoundException" => {
                        DeleteAttributesError::TargetNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAttributesError::Validation(error_message.to_string())
                    }
                    _ => DeleteAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAttributesError {
    fn from(err: serde_json::error::Error) -> DeleteAttributesError {
        DeleteAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAttributesError {
    fn from(err: CredentialsError) -> DeleteAttributesError {
        DeleteAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAttributesError {
    fn from(err: HttpDispatchError) -> DeleteAttributesError {
        DeleteAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAttributesError {
    fn from(err: io::Error) -> DeleteAttributesError {
        DeleteAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAttributesError {
    fn description(&self) -> &str {
        match *self {
            DeleteAttributesError::ClusterNotFound(ref cause) => cause,
            DeleteAttributesError::InvalidParameter(ref cause) => cause,
            DeleteAttributesError::TargetNotFound(ref cause) => cause,
            DeleteAttributesError::Validation(ref cause) => cause,
            DeleteAttributesError::Credentials(ref err) => err.description(),
            DeleteAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>You cannot delete a cluster that has registered container instances. You must first deregister the container instances before you can delete the cluster. For more information, see <a>DeregisterContainerInstance</a>.</p>
    ClusterContainsContainerInstances(String),
    ///<p>You cannot delete a cluster that contains services. You must first update the service to reduce its desired task count to 0 and then delete the service. For more information, see <a>UpdateService</a> and <a>DeleteService</a>.</p>
    ClusterContainsServices(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DeleteClusterError {
    pub fn from_body(body: &str) -> DeleteClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteClusterError::Client(String::from(error_message)),
                    "ClusterContainsContainerInstancesException" => DeleteClusterError::ClusterContainsContainerInstances(String::from(error_message)),
                    "ClusterContainsServicesException" => {
                        DeleteClusterError::ClusterContainsServices(String::from(error_message))
                    }
                    "ClusterNotFoundException" => {
                        DeleteClusterError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteClusterError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => DeleteClusterError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DeleteClusterError::Validation(error_message.to_string())
                    }
                    _ => DeleteClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteClusterError {
    fn from(err: serde_json::error::Error) -> DeleteClusterError {
        DeleteClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteClusterError {
    fn from(err: CredentialsError) -> DeleteClusterError {
        DeleteClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterError {
    fn from(err: HttpDispatchError) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterError {
    fn from(err: io::Error) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::Client(ref cause) => cause,
            DeleteClusterError::ClusterContainsContainerInstances(ref cause) => cause,
            DeleteClusterError::ClusterContainsServices(ref cause) => cause,
            DeleteClusterError::ClusterNotFound(ref cause) => cause,
            DeleteClusterError::InvalidParameter(ref cause) => cause,
            DeleteClusterError::Server(ref cause) => cause,
            DeleteClusterError::Validation(ref cause) => cause,
            DeleteClusterError::Credentials(ref err) => err.description(),
            DeleteClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteService
#[derive(Debug, PartialEq)]
pub enum DeleteServiceError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    ///<p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and region-specific.</p>
    ServiceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteServiceError {
    pub fn from_body(body: &str) -> DeleteServiceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteServiceError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        DeleteServiceError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteServiceError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => DeleteServiceError::Server(String::from(error_message)),
                    "ServiceNotFoundException" => {
                        DeleteServiceError::ServiceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteServiceError::Validation(error_message.to_string())
                    }
                    _ => DeleteServiceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteServiceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteServiceError {
    fn from(err: serde_json::error::Error) -> DeleteServiceError {
        DeleteServiceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteServiceError {
    fn from(err: CredentialsError) -> DeleteServiceError {
        DeleteServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServiceError {
    fn from(err: HttpDispatchError) -> DeleteServiceError {
        DeleteServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServiceError {
    fn from(err: io::Error) -> DeleteServiceError {
        DeleteServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServiceError {
    fn description(&self) -> &str {
        match *self {
            DeleteServiceError::Client(ref cause) => cause,
            DeleteServiceError::ClusterNotFound(ref cause) => cause,
            DeleteServiceError::InvalidParameter(ref cause) => cause,
            DeleteServiceError::Server(ref cause) => cause,
            DeleteServiceError::ServiceNotFound(ref cause) => cause,
            DeleteServiceError::Validation(ref cause) => cause,
            DeleteServiceError::Credentials(ref err) => err.description(),
            DeleteServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteServiceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterContainerInstanceError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DeregisterContainerInstanceError {
    pub fn from_body(body: &str) -> DeregisterContainerInstanceError {
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
                        DeregisterContainerInstanceError::Client(String::from(error_message))
                    }
                    "ClusterNotFoundException" => DeregisterContainerInstanceError::ClusterNotFound(String::from(error_message)),
                    "InvalidParameterException" => DeregisterContainerInstanceError::InvalidParameter(String::from(error_message)),
                    "ServerException" => {
                        DeregisterContainerInstanceError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterContainerInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeregisterContainerInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterContainerInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterContainerInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterContainerInstanceError {
        DeregisterContainerInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterContainerInstanceError {
    fn from(err: CredentialsError) -> DeregisterContainerInstanceError {
        DeregisterContainerInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterContainerInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterContainerInstanceError {
        DeregisterContainerInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterContainerInstanceError {
    fn from(err: io::Error) -> DeregisterContainerInstanceError {
        DeregisterContainerInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterContainerInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterContainerInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterContainerInstanceError::Client(ref cause) => cause,
            DeregisterContainerInstanceError::ClusterNotFound(ref cause) => cause,
            DeregisterContainerInstanceError::InvalidParameter(ref cause) => cause,
            DeregisterContainerInstanceError::Server(ref cause) => cause,
            DeregisterContainerInstanceError::Validation(ref cause) => cause,
            DeregisterContainerInstanceError::Credentials(ref err) => err.description(),
            DeregisterContainerInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterContainerInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterTaskDefinitionError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DeregisterTaskDefinitionError {
    pub fn from_body(body: &str) -> DeregisterTaskDefinitionError {
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
                        DeregisterTaskDefinitionError::Client(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeregisterTaskDefinitionError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        DeregisterTaskDefinitionError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterTaskDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeregisterTaskDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterTaskDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterTaskDefinitionError {
    fn from(err: serde_json::error::Error) -> DeregisterTaskDefinitionError {
        DeregisterTaskDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterTaskDefinitionError {
    fn from(err: CredentialsError) -> DeregisterTaskDefinitionError {
        DeregisterTaskDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterTaskDefinitionError {
    fn from(err: HttpDispatchError) -> DeregisterTaskDefinitionError {
        DeregisterTaskDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterTaskDefinitionError {
    fn from(err: io::Error) -> DeregisterTaskDefinitionError {
        DeregisterTaskDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTaskDefinitionError::Client(ref cause) => cause,
            DeregisterTaskDefinitionError::InvalidParameter(ref cause) => cause,
            DeregisterTaskDefinitionError::Server(ref cause) => cause,
            DeregisterTaskDefinitionError::Validation(ref cause) => cause,
            DeregisterTaskDefinitionError::Credentials(ref err) => err.description(),
            DeregisterTaskDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterTaskDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DescribeClustersError {
    pub fn from_body(body: &str) -> DescribeClustersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DescribeClustersError::Client(String::from(error_message)),
                    "InvalidParameterException" => {
                        DescribeClustersError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => DescribeClustersError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DescribeClustersError::Validation(error_message.to_string())
                    }
                    _ => DescribeClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeClustersError {
    fn from(err: serde_json::error::Error) -> DescribeClustersError {
        DescribeClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeClustersError {
    fn from(err: CredentialsError) -> DescribeClustersError {
        DescribeClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClustersError {
    fn from(err: HttpDispatchError) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClustersError {
    fn from(err: io::Error) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::Client(ref cause) => cause,
            DescribeClustersError::InvalidParameter(ref cause) => cause,
            DescribeClustersError::Server(ref cause) => cause,
            DescribeClustersError::Validation(ref cause) => cause,
            DescribeClustersError::Credentials(ref err) => err.description(),
            DescribeClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeContainerInstances
#[derive(Debug, PartialEq)]
pub enum DescribeContainerInstancesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DescribeContainerInstancesError {
    pub fn from_body(body: &str) -> DescribeContainerInstancesError {
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
                        DescribeContainerInstancesError::Client(String::from(error_message))
                    }
                    "ClusterNotFoundException" => DescribeContainerInstancesError::ClusterNotFound(String::from(error_message)),
                    "InvalidParameterException" => DescribeContainerInstancesError::InvalidParameter(String::from(error_message)),
                    "ServerException" => {
                        DescribeContainerInstancesError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeContainerInstancesError::Validation(error_message.to_string())
                    }
                    _ => DescribeContainerInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeContainerInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeContainerInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeContainerInstancesError {
        DescribeContainerInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeContainerInstancesError {
    fn from(err: CredentialsError) -> DescribeContainerInstancesError {
        DescribeContainerInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeContainerInstancesError {
    fn from(err: HttpDispatchError) -> DescribeContainerInstancesError {
        DescribeContainerInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeContainerInstancesError {
    fn from(err: io::Error) -> DescribeContainerInstancesError {
        DescribeContainerInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeContainerInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeContainerInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeContainerInstancesError::Client(ref cause) => cause,
            DescribeContainerInstancesError::ClusterNotFound(ref cause) => cause,
            DescribeContainerInstancesError::InvalidParameter(ref cause) => cause,
            DescribeContainerInstancesError::Server(ref cause) => cause,
            DescribeContainerInstancesError::Validation(ref cause) => cause,
            DescribeContainerInstancesError::Credentials(ref err) => err.description(),
            DescribeContainerInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeContainerInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DescribeServicesError {
    pub fn from_body(body: &str) -> DescribeServicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DescribeServicesError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        DescribeServicesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeServicesError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => DescribeServicesError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DescribeServicesError::Validation(error_message.to_string())
                    }
                    _ => DescribeServicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeServicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeServicesError {
    fn from(err: serde_json::error::Error) -> DescribeServicesError {
        DescribeServicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeServicesError {
    fn from(err: CredentialsError) -> DescribeServicesError {
        DescribeServicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeServicesError {
    fn from(err: HttpDispatchError) -> DescribeServicesError {
        DescribeServicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeServicesError {
    fn from(err: io::Error) -> DescribeServicesError {
        DescribeServicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeServicesError::Client(ref cause) => cause,
            DescribeServicesError::ClusterNotFound(ref cause) => cause,
            DescribeServicesError::InvalidParameter(ref cause) => cause,
            DescribeServicesError::Server(ref cause) => cause,
            DescribeServicesError::Validation(ref cause) => cause,
            DescribeServicesError::Credentials(ref err) => err.description(),
            DescribeServicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeServicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DescribeTaskDefinitionError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DescribeTaskDefinitionError {
    pub fn from_body(body: &str) -> DescribeTaskDefinitionError {
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
                        DescribeTaskDefinitionError::Client(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeTaskDefinitionError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        DescribeTaskDefinitionError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTaskDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DescribeTaskDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTaskDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTaskDefinitionError {
    fn from(err: serde_json::error::Error) -> DescribeTaskDefinitionError {
        DescribeTaskDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTaskDefinitionError {
    fn from(err: CredentialsError) -> DescribeTaskDefinitionError {
        DescribeTaskDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTaskDefinitionError {
    fn from(err: HttpDispatchError) -> DescribeTaskDefinitionError {
        DescribeTaskDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTaskDefinitionError {
    fn from(err: io::Error) -> DescribeTaskDefinitionError {
        DescribeTaskDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DescribeTaskDefinitionError::Client(ref cause) => cause,
            DescribeTaskDefinitionError::InvalidParameter(ref cause) => cause,
            DescribeTaskDefinitionError::Server(ref cause) => cause,
            DescribeTaskDefinitionError::Validation(ref cause) => cause,
            DescribeTaskDefinitionError::Credentials(ref err) => err.description(),
            DescribeTaskDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTaskDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTasks
#[derive(Debug, PartialEq)]
pub enum DescribeTasksError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl DescribeTasksError {
    pub fn from_body(body: &str) -> DescribeTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DescribeTasksError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        DescribeTasksError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeTasksError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => DescribeTasksError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DescribeTasksError::Validation(error_message.to_string())
                    }
                    _ => DescribeTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTasksError {
    fn from(err: serde_json::error::Error) -> DescribeTasksError {
        DescribeTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTasksError {
    fn from(err: CredentialsError) -> DescribeTasksError {
        DescribeTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTasksError {
    fn from(err: HttpDispatchError) -> DescribeTasksError {
        DescribeTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTasksError {
    fn from(err: io::Error) -> DescribeTasksError {
        DescribeTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeTasksError::Client(ref cause) => cause,
            DescribeTasksError::ClusterNotFound(ref cause) => cause,
            DescribeTasksError::InvalidParameter(ref cause) => cause,
            DescribeTasksError::Server(ref cause) => cause,
            DescribeTasksError::Validation(ref cause) => cause,
            DescribeTasksError::Credentials(ref err) => err.description(),
            DescribeTasksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DiscoverPollEndpoint
#[derive(Debug, PartialEq)]
pub enum DiscoverPollEndpointError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
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


impl DiscoverPollEndpointError {
    pub fn from_body(body: &str) -> DiscoverPollEndpointError {
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
                        DiscoverPollEndpointError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        DiscoverPollEndpointError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DiscoverPollEndpointError::Validation(error_message.to_string())
                    }
                    _ => DiscoverPollEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DiscoverPollEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DiscoverPollEndpointError {
    fn from(err: serde_json::error::Error) -> DiscoverPollEndpointError {
        DiscoverPollEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DiscoverPollEndpointError {
    fn from(err: CredentialsError) -> DiscoverPollEndpointError {
        DiscoverPollEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DiscoverPollEndpointError {
    fn from(err: HttpDispatchError) -> DiscoverPollEndpointError {
        DiscoverPollEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DiscoverPollEndpointError {
    fn from(err: io::Error) -> DiscoverPollEndpointError {
        DiscoverPollEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DiscoverPollEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DiscoverPollEndpointError {
    fn description(&self) -> &str {
        match *self {
            DiscoverPollEndpointError::Client(ref cause) => cause,
            DiscoverPollEndpointError::Server(ref cause) => cause,
            DiscoverPollEndpointError::Validation(ref cause) => cause,
            DiscoverPollEndpointError::Credentials(ref err) => err.description(),
            DiscoverPollEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DiscoverPollEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAttributes
#[derive(Debug, PartialEq)]
pub enum ListAttributesError {
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAttributesError {
    pub fn from_body(body: &str) -> ListAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClusterNotFoundException" => {
                        ListAttributesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAttributesError::Validation(error_message.to_string())
                    }
                    _ => ListAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAttributesError {
    fn from(err: serde_json::error::Error) -> ListAttributesError {
        ListAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAttributesError {
    fn from(err: CredentialsError) -> ListAttributesError {
        ListAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAttributesError {
    fn from(err: HttpDispatchError) -> ListAttributesError {
        ListAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAttributesError {
    fn from(err: io::Error) -> ListAttributesError {
        ListAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListAttributesError::ClusterNotFound(ref cause) => cause,
            ListAttributesError::InvalidParameter(ref cause) => cause,
            ListAttributesError::Validation(ref cause) => cause,
            ListAttributesError::Credentials(ref err) => err.description(),
            ListAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl ListClustersError {
    pub fn from_body(body: &str) -> ListClustersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ListClustersError::Client(String::from(error_message)),
                    "InvalidParameterException" => {
                        ListClustersError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => ListClustersError::Server(String::from(error_message)),
                    "ValidationException" => {
                        ListClustersError::Validation(error_message.to_string())
                    }
                    _ => ListClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListClustersError {
    fn from(err: serde_json::error::Error) -> ListClustersError {
        ListClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListClustersError {
    fn from(err: CredentialsError) -> ListClustersError {
        ListClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListClustersError {
    fn from(err: HttpDispatchError) -> ListClustersError {
        ListClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListClustersError {
    fn from(err: io::Error) -> ListClustersError {
        ListClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::Client(ref cause) => cause,
            ListClustersError::InvalidParameter(ref cause) => cause,
            ListClustersError::Server(ref cause) => cause,
            ListClustersError::Validation(ref cause) => cause,
            ListClustersError::Credentials(ref err) => err.description(),
            ListClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListContainerInstances
#[derive(Debug, PartialEq)]
pub enum ListContainerInstancesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl ListContainerInstancesError {
    pub fn from_body(body: &str) -> ListContainerInstancesError {
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
                        ListContainerInstancesError::Client(String::from(error_message))
                    }
                    "ClusterNotFoundException" => {
                        ListContainerInstancesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListContainerInstancesError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        ListContainerInstancesError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListContainerInstancesError::Validation(error_message.to_string())
                    }
                    _ => ListContainerInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListContainerInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListContainerInstancesError {
    fn from(err: serde_json::error::Error) -> ListContainerInstancesError {
        ListContainerInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListContainerInstancesError {
    fn from(err: CredentialsError) -> ListContainerInstancesError {
        ListContainerInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListContainerInstancesError {
    fn from(err: HttpDispatchError) -> ListContainerInstancesError {
        ListContainerInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListContainerInstancesError {
    fn from(err: io::Error) -> ListContainerInstancesError {
        ListContainerInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListContainerInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListContainerInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListContainerInstancesError::Client(ref cause) => cause,
            ListContainerInstancesError::ClusterNotFound(ref cause) => cause,
            ListContainerInstancesError::InvalidParameter(ref cause) => cause,
            ListContainerInstancesError::Server(ref cause) => cause,
            ListContainerInstancesError::Validation(ref cause) => cause,
            ListContainerInstancesError::Credentials(ref err) => err.description(),
            ListContainerInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListContainerInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl ListServicesError {
    pub fn from_body(body: &str) -> ListServicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ListServicesError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        ListServicesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListServicesError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => ListServicesError::Server(String::from(error_message)),
                    "ValidationException" => {
                        ListServicesError::Validation(error_message.to_string())
                    }
                    _ => ListServicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListServicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListServicesError {
    fn from(err: serde_json::error::Error) -> ListServicesError {
        ListServicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListServicesError {
    fn from(err: CredentialsError) -> ListServicesError {
        ListServicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListServicesError {
    fn from(err: HttpDispatchError) -> ListServicesError {
        ListServicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListServicesError {
    fn from(err: io::Error) -> ListServicesError {
        ListServicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServicesError {
    fn description(&self) -> &str {
        match *self {
            ListServicesError::Client(ref cause) => cause,
            ListServicesError::ClusterNotFound(ref cause) => cause,
            ListServicesError::InvalidParameter(ref cause) => cause,
            ListServicesError::Server(ref cause) => cause,
            ListServicesError::Validation(ref cause) => cause,
            ListServicesError::Credentials(ref err) => err.description(),
            ListServicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListServicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTaskDefinitionFamilies
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionFamiliesError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl ListTaskDefinitionFamiliesError {
    pub fn from_body(body: &str) -> ListTaskDefinitionFamiliesError {
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
                        ListTaskDefinitionFamiliesError::Client(String::from(error_message))
                    }
                    "InvalidParameterException" => ListTaskDefinitionFamiliesError::InvalidParameter(String::from(error_message)),
                    "ServerException" => {
                        ListTaskDefinitionFamiliesError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTaskDefinitionFamiliesError::Validation(error_message.to_string())
                    }
                    _ => ListTaskDefinitionFamiliesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTaskDefinitionFamiliesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTaskDefinitionFamiliesError {
    fn from(err: serde_json::error::Error) -> ListTaskDefinitionFamiliesError {
        ListTaskDefinitionFamiliesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTaskDefinitionFamiliesError {
    fn from(err: CredentialsError) -> ListTaskDefinitionFamiliesError {
        ListTaskDefinitionFamiliesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTaskDefinitionFamiliesError {
    fn from(err: HttpDispatchError) -> ListTaskDefinitionFamiliesError {
        ListTaskDefinitionFamiliesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTaskDefinitionFamiliesError {
    fn from(err: io::Error) -> ListTaskDefinitionFamiliesError {
        ListTaskDefinitionFamiliesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTaskDefinitionFamiliesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTaskDefinitionFamiliesError {
    fn description(&self) -> &str {
        match *self {
            ListTaskDefinitionFamiliesError::Client(ref cause) => cause,
            ListTaskDefinitionFamiliesError::InvalidParameter(ref cause) => cause,
            ListTaskDefinitionFamiliesError::Server(ref cause) => cause,
            ListTaskDefinitionFamiliesError::Validation(ref cause) => cause,
            ListTaskDefinitionFamiliesError::Credentials(ref err) => err.description(),
            ListTaskDefinitionFamiliesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTaskDefinitionFamiliesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTaskDefinitions
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionsError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl ListTaskDefinitionsError {
    pub fn from_body(body: &str) -> ListTaskDefinitionsError {
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
                        ListTaskDefinitionsError::Client(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListTaskDefinitionsError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        ListTaskDefinitionsError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTaskDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => ListTaskDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTaskDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTaskDefinitionsError {
    fn from(err: serde_json::error::Error) -> ListTaskDefinitionsError {
        ListTaskDefinitionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTaskDefinitionsError {
    fn from(err: CredentialsError) -> ListTaskDefinitionsError {
        ListTaskDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTaskDefinitionsError {
    fn from(err: HttpDispatchError) -> ListTaskDefinitionsError {
        ListTaskDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTaskDefinitionsError {
    fn from(err: io::Error) -> ListTaskDefinitionsError {
        ListTaskDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTaskDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTaskDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListTaskDefinitionsError::Client(ref cause) => cause,
            ListTaskDefinitionsError::InvalidParameter(ref cause) => cause,
            ListTaskDefinitionsError::Server(ref cause) => cause,
            ListTaskDefinitionsError::Validation(ref cause) => cause,
            ListTaskDefinitionsError::Credentials(ref err) => err.description(),
            ListTaskDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTaskDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTasks
#[derive(Debug, PartialEq)]
pub enum ListTasksError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    ///<p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and region-specific.</p>
    ServiceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTasksError {
    pub fn from_body(body: &str) -> ListTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ListTasksError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        ListTasksError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListTasksError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => ListTasksError::Server(String::from(error_message)),
                    "ServiceNotFoundException" => {
                        ListTasksError::ServiceNotFound(String::from(error_message))
                    }
                    "ValidationException" => ListTasksError::Validation(error_message.to_string()),
                    _ => ListTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTasksError {
    fn from(err: serde_json::error::Error) -> ListTasksError {
        ListTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTasksError {
    fn from(err: CredentialsError) -> ListTasksError {
        ListTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTasksError {
    fn from(err: HttpDispatchError) -> ListTasksError {
        ListTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTasksError {
    fn from(err: io::Error) -> ListTasksError {
        ListTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTasksError {
    fn description(&self) -> &str {
        match *self {
            ListTasksError::Client(ref cause) => cause,
            ListTasksError::ClusterNotFound(ref cause) => cause,
            ListTasksError::InvalidParameter(ref cause) => cause,
            ListTasksError::Server(ref cause) => cause,
            ListTasksError::ServiceNotFound(ref cause) => cause,
            ListTasksError::Validation(ref cause) => cause,
            ListTasksError::Credentials(ref err) => err.description(),
            ListTasksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAttributes
#[derive(Debug, PartialEq)]
pub enum PutAttributesError {
    ///<p>You can apply up to 10 custom attributes per resource. You can view the attributes of a resource with <a>ListAttributes</a>. You can remove existing attributes on a resource with <a>DeleteAttributes</a>.</p>
    AttributeLimitExceeded(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>The specified target could not be found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and region-specific.</p>
    TargetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutAttributesError {
    pub fn from_body(body: &str) -> PutAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AttributeLimitExceededException" => {
                        PutAttributesError::AttributeLimitExceeded(String::from(error_message))
                    }
                    "ClusterNotFoundException" => {
                        PutAttributesError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        PutAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "TargetNotFoundException" => {
                        PutAttributesError::TargetNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutAttributesError::Validation(error_message.to_string())
                    }
                    _ => PutAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutAttributesError {
    fn from(err: serde_json::error::Error) -> PutAttributesError {
        PutAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutAttributesError {
    fn from(err: CredentialsError) -> PutAttributesError {
        PutAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAttributesError {
    fn from(err: HttpDispatchError) -> PutAttributesError {
        PutAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAttributesError {
    fn from(err: io::Error) -> PutAttributesError {
        PutAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAttributesError {
    fn description(&self) -> &str {
        match *self {
            PutAttributesError::AttributeLimitExceeded(ref cause) => cause,
            PutAttributesError::ClusterNotFound(ref cause) => cause,
            PutAttributesError::InvalidParameter(ref cause) => cause,
            PutAttributesError::TargetNotFound(ref cause) => cause,
            PutAttributesError::Validation(ref cause) => cause,
            PutAttributesError::Credentials(ref err) => err.description(),
            PutAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum RegisterContainerInstanceError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
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


impl RegisterContainerInstanceError {
    pub fn from_body(body: &str) -> RegisterContainerInstanceError {
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
                        RegisterContainerInstanceError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        RegisterContainerInstanceError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterContainerInstanceError::Validation(error_message.to_string())
                    }
                    _ => RegisterContainerInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterContainerInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterContainerInstanceError {
    fn from(err: serde_json::error::Error) -> RegisterContainerInstanceError {
        RegisterContainerInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterContainerInstanceError {
    fn from(err: CredentialsError) -> RegisterContainerInstanceError {
        RegisterContainerInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterContainerInstanceError {
    fn from(err: HttpDispatchError) -> RegisterContainerInstanceError {
        RegisterContainerInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterContainerInstanceError {
    fn from(err: io::Error) -> RegisterContainerInstanceError {
        RegisterContainerInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterContainerInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterContainerInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterContainerInstanceError::Client(ref cause) => cause,
            RegisterContainerInstanceError::Server(ref cause) => cause,
            RegisterContainerInstanceError::Validation(ref cause) => cause,
            RegisterContainerInstanceError::Credentials(ref err) => err.description(),
            RegisterContainerInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterContainerInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterTaskDefinitionError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl RegisterTaskDefinitionError {
    pub fn from_body(body: &str) -> RegisterTaskDefinitionError {
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
                        RegisterTaskDefinitionError::Client(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RegisterTaskDefinitionError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        RegisterTaskDefinitionError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterTaskDefinitionError::Validation(error_message.to_string())
                    }
                    _ => RegisterTaskDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterTaskDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterTaskDefinitionError {
    fn from(err: serde_json::error::Error) -> RegisterTaskDefinitionError {
        RegisterTaskDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterTaskDefinitionError {
    fn from(err: CredentialsError) -> RegisterTaskDefinitionError {
        RegisterTaskDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterTaskDefinitionError {
    fn from(err: HttpDispatchError) -> RegisterTaskDefinitionError {
        RegisterTaskDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterTaskDefinitionError {
    fn from(err: io::Error) -> RegisterTaskDefinitionError {
        RegisterTaskDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            RegisterTaskDefinitionError::Client(ref cause) => cause,
            RegisterTaskDefinitionError::InvalidParameter(ref cause) => cause,
            RegisterTaskDefinitionError::Server(ref cause) => cause,
            RegisterTaskDefinitionError::Validation(ref cause) => cause,
            RegisterTaskDefinitionError::Credentials(ref err) => err.description(),
            RegisterTaskDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterTaskDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RunTask
#[derive(Debug, PartialEq)]
pub enum RunTaskError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl RunTaskError {
    pub fn from_body(body: &str) -> RunTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => RunTaskError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        RunTaskError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RunTaskError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => RunTaskError::Server(String::from(error_message)),
                    "ValidationException" => RunTaskError::Validation(error_message.to_string()),
                    _ => RunTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => RunTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RunTaskError {
    fn from(err: serde_json::error::Error) -> RunTaskError {
        RunTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RunTaskError {
    fn from(err: CredentialsError) -> RunTaskError {
        RunTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RunTaskError {
    fn from(err: HttpDispatchError) -> RunTaskError {
        RunTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for RunTaskError {
    fn from(err: io::Error) -> RunTaskError {
        RunTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RunTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RunTaskError {
    fn description(&self) -> &str {
        match *self {
            RunTaskError::Client(ref cause) => cause,
            RunTaskError::ClusterNotFound(ref cause) => cause,
            RunTaskError::InvalidParameter(ref cause) => cause,
            RunTaskError::Server(ref cause) => cause,
            RunTaskError::Validation(ref cause) => cause,
            RunTaskError::Credentials(ref err) => err.description(),
            RunTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RunTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartTask
#[derive(Debug, PartialEq)]
pub enum StartTaskError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl StartTaskError {
    pub fn from_body(body: &str) -> StartTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => StartTaskError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        StartTaskError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StartTaskError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => StartTaskError::Server(String::from(error_message)),
                    "ValidationException" => StartTaskError::Validation(error_message.to_string()),
                    _ => StartTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartTaskError {
    fn from(err: serde_json::error::Error) -> StartTaskError {
        StartTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartTaskError {
    fn from(err: CredentialsError) -> StartTaskError {
        StartTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartTaskError {
    fn from(err: HttpDispatchError) -> StartTaskError {
        StartTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartTaskError {
    fn from(err: io::Error) -> StartTaskError {
        StartTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTaskError {
    fn description(&self) -> &str {
        match *self {
            StartTaskError::Client(ref cause) => cause,
            StartTaskError::ClusterNotFound(ref cause) => cause,
            StartTaskError::InvalidParameter(ref cause) => cause,
            StartTaskError::Server(ref cause) => cause,
            StartTaskError::Validation(ref cause) => cause,
            StartTaskError::Credentials(ref err) => err.description(),
            StartTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopTask
#[derive(Debug, PartialEq)]
pub enum StopTaskError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl StopTaskError {
    pub fn from_body(body: &str) -> StopTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => StopTaskError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        StopTaskError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StopTaskError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => StopTaskError::Server(String::from(error_message)),
                    "ValidationException" => StopTaskError::Validation(error_message.to_string()),
                    _ => StopTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopTaskError {
    fn from(err: serde_json::error::Error) -> StopTaskError {
        StopTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTaskError {
    fn from(err: CredentialsError) -> StopTaskError {
        StopTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTaskError {
    fn from(err: HttpDispatchError) -> StopTaskError {
        StopTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTaskError {
    fn from(err: io::Error) -> StopTaskError {
        StopTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTaskError {
    fn description(&self) -> &str {
        match *self {
            StopTaskError::Client(ref cause) => cause,
            StopTaskError::ClusterNotFound(ref cause) => cause,
            StopTaskError::InvalidParameter(ref cause) => cause,
            StopTaskError::Server(ref cause) => cause,
            StopTaskError::Validation(ref cause) => cause,
            StopTaskError::Credentials(ref err) => err.description(),
            StopTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SubmitContainerStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitContainerStateChangeError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
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


impl SubmitContainerStateChangeError {
    pub fn from_body(body: &str) -> SubmitContainerStateChangeError {
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
                        SubmitContainerStateChangeError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        SubmitContainerStateChangeError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        SubmitContainerStateChangeError::Validation(error_message.to_string())
                    }
                    _ => SubmitContainerStateChangeError::Unknown(String::from(body)),
                }
            }
            Err(_) => SubmitContainerStateChangeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SubmitContainerStateChangeError {
    fn from(err: serde_json::error::Error) -> SubmitContainerStateChangeError {
        SubmitContainerStateChangeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SubmitContainerStateChangeError {
    fn from(err: CredentialsError) -> SubmitContainerStateChangeError {
        SubmitContainerStateChangeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubmitContainerStateChangeError {
    fn from(err: HttpDispatchError) -> SubmitContainerStateChangeError {
        SubmitContainerStateChangeError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubmitContainerStateChangeError {
    fn from(err: io::Error) -> SubmitContainerStateChangeError {
        SubmitContainerStateChangeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubmitContainerStateChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitContainerStateChangeError {
    fn description(&self) -> &str {
        match *self {
            SubmitContainerStateChangeError::Client(ref cause) => cause,
            SubmitContainerStateChangeError::Server(ref cause) => cause,
            SubmitContainerStateChangeError::Validation(ref cause) => cause,
            SubmitContainerStateChangeError::Credentials(ref err) => err.description(),
            SubmitContainerStateChangeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SubmitContainerStateChangeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SubmitTaskStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitTaskStateChangeError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
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


impl SubmitTaskStateChangeError {
    pub fn from_body(body: &str) -> SubmitTaskStateChangeError {
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
                        SubmitTaskStateChangeError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        SubmitTaskStateChangeError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        SubmitTaskStateChangeError::Validation(error_message.to_string())
                    }
                    _ => SubmitTaskStateChangeError::Unknown(String::from(body)),
                }
            }
            Err(_) => SubmitTaskStateChangeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SubmitTaskStateChangeError {
    fn from(err: serde_json::error::Error) -> SubmitTaskStateChangeError {
        SubmitTaskStateChangeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SubmitTaskStateChangeError {
    fn from(err: CredentialsError) -> SubmitTaskStateChangeError {
        SubmitTaskStateChangeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubmitTaskStateChangeError {
    fn from(err: HttpDispatchError) -> SubmitTaskStateChangeError {
        SubmitTaskStateChangeError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubmitTaskStateChangeError {
    fn from(err: io::Error) -> SubmitTaskStateChangeError {
        SubmitTaskStateChangeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubmitTaskStateChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitTaskStateChangeError {
    fn description(&self) -> &str {
        match *self {
            SubmitTaskStateChangeError::Client(ref cause) => cause,
            SubmitTaskStateChangeError::Server(ref cause) => cause,
            SubmitTaskStateChangeError::Validation(ref cause) => cause,
            SubmitTaskStateChangeError::Credentials(ref err) => err.description(),
            SubmitTaskStateChangeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SubmitTaskStateChangeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContainerAgent
#[derive(Debug, PartialEq)]
pub enum UpdateContainerAgentError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>Amazon ECS is unable to determine the current version of the Amazon ECS container agent on the container instance and does not have enough information to proceed with an update. This could be because the agent running on the container instance is an older or custom version that does not use our version information.</p>
    MissingVersion(String),
    ///<p>There is no update available for this Amazon ECS container agent. This could be because the agent is already running the latest version, or it is so old that there is no update path to the current version.</p>
    NoUpdateAvailable(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    ///<p>There is already a current Amazon ECS container agent update in progress on the specified container instance. If the container agent becomes disconnected while it is in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateContainerAgentError {
    pub fn from_body(body: &str) -> UpdateContainerAgentError {
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
                        UpdateContainerAgentError::Client(String::from(error_message))
                    }
                    "ClusterNotFoundException" => {
                        UpdateContainerAgentError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateContainerAgentError::InvalidParameter(String::from(error_message))
                    }
                    "MissingVersionException" => {
                        UpdateContainerAgentError::MissingVersion(String::from(error_message))
                    }
                    "NoUpdateAvailableException" => {
                        UpdateContainerAgentError::NoUpdateAvailable(String::from(error_message))
                    }
                    "ServerException" => {
                        UpdateContainerAgentError::Server(String::from(error_message))
                    }
                    "UpdateInProgressException" => {
                        UpdateContainerAgentError::UpdateInProgress(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateContainerAgentError::Validation(error_message.to_string())
                    }
                    _ => UpdateContainerAgentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateContainerAgentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateContainerAgentError {
    fn from(err: serde_json::error::Error) -> UpdateContainerAgentError {
        UpdateContainerAgentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateContainerAgentError {
    fn from(err: CredentialsError) -> UpdateContainerAgentError {
        UpdateContainerAgentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateContainerAgentError {
    fn from(err: HttpDispatchError) -> UpdateContainerAgentError {
        UpdateContainerAgentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateContainerAgentError {
    fn from(err: io::Error) -> UpdateContainerAgentError {
        UpdateContainerAgentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateContainerAgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContainerAgentError {
    fn description(&self) -> &str {
        match *self {
            UpdateContainerAgentError::Client(ref cause) => cause,
            UpdateContainerAgentError::ClusterNotFound(ref cause) => cause,
            UpdateContainerAgentError::InvalidParameter(ref cause) => cause,
            UpdateContainerAgentError::MissingVersion(ref cause) => cause,
            UpdateContainerAgentError::NoUpdateAvailable(ref cause) => cause,
            UpdateContainerAgentError::Server(ref cause) => cause,
            UpdateContainerAgentError::UpdateInProgress(ref cause) => cause,
            UpdateContainerAgentError::Validation(ref cause) => cause,
            UpdateContainerAgentError::Credentials(ref err) => err.description(),
            UpdateContainerAgentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateContainerAgentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContainerInstancesState
#[derive(Debug, PartialEq)]
pub enum UpdateContainerInstancesStateError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
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


impl UpdateContainerInstancesStateError {
    pub fn from_body(body: &str) -> UpdateContainerInstancesStateError {
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
                        UpdateContainerInstancesStateError::Client(String::from(error_message))
                    }
                    "ClusterNotFoundException" => UpdateContainerInstancesStateError::ClusterNotFound(String::from(error_message)),
                    "InvalidParameterException" => UpdateContainerInstancesStateError::InvalidParameter(String::from(error_message)),
                    "ServerException" => {
                        UpdateContainerInstancesStateError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateContainerInstancesStateError::Validation(error_message.to_string())
                    }
                    _ => UpdateContainerInstancesStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateContainerInstancesStateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateContainerInstancesStateError {
    fn from(err: serde_json::error::Error) -> UpdateContainerInstancesStateError {
        UpdateContainerInstancesStateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateContainerInstancesStateError {
    fn from(err: CredentialsError) -> UpdateContainerInstancesStateError {
        UpdateContainerInstancesStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateContainerInstancesStateError {
    fn from(err: HttpDispatchError) -> UpdateContainerInstancesStateError {
        UpdateContainerInstancesStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateContainerInstancesStateError {
    fn from(err: io::Error) -> UpdateContainerInstancesStateError {
        UpdateContainerInstancesStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateContainerInstancesStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContainerInstancesStateError {
    fn description(&self) -> &str {
        match *self {
            UpdateContainerInstancesStateError::Client(ref cause) => cause,
            UpdateContainerInstancesStateError::ClusterNotFound(ref cause) => cause,
            UpdateContainerInstancesStateError::InvalidParameter(ref cause) => cause,
            UpdateContainerInstancesStateError::Server(ref cause) => cause,
            UpdateContainerInstancesStateError::Validation(ref cause) => cause,
            UpdateContainerInstancesStateError::Credentials(ref err) => err.description(),
            UpdateContainerInstancesStateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateContainerInstancesStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateService
#[derive(Debug, PartialEq)]
pub enum UpdateServiceError {
    ///<p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permission to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    ///<p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are region-specific.</p>
    ClusterNotFound(String),
    ///<p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    ///<p>These errors are usually caused by a server issue.</p>
    Server(String),
    ///<p>The specified service is not active. You cannot update a service that is not active. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    ///<p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and region-specific.</p>
    ServiceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateServiceError {
    pub fn from_body(body: &str) -> UpdateServiceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => UpdateServiceError::Client(String::from(error_message)),
                    "ClusterNotFoundException" => {
                        UpdateServiceError::ClusterNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateServiceError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => UpdateServiceError::Server(String::from(error_message)),
                    "ServiceNotActiveException" => {
                        UpdateServiceError::ServiceNotActive(String::from(error_message))
                    }
                    "ServiceNotFoundException" => {
                        UpdateServiceError::ServiceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateServiceError::Validation(error_message.to_string())
                    }
                    _ => UpdateServiceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateServiceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateServiceError {
    fn from(err: serde_json::error::Error) -> UpdateServiceError {
        UpdateServiceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateServiceError {
    fn from(err: CredentialsError) -> UpdateServiceError {
        UpdateServiceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateServiceError {
    fn from(err: HttpDispatchError) -> UpdateServiceError {
        UpdateServiceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateServiceError {
    fn from(err: io::Error) -> UpdateServiceError {
        UpdateServiceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceError::Client(ref cause) => cause,
            UpdateServiceError::ClusterNotFound(ref cause) => cause,
            UpdateServiceError::InvalidParameter(ref cause) => cause,
            UpdateServiceError::Server(ref cause) => cause,
            UpdateServiceError::ServiceNotActive(ref cause) => cause,
            UpdateServiceError::ServiceNotFound(ref cause) => cause,
            UpdateServiceError::Validation(ref cause) => cause,
            UpdateServiceError::Credentials(ref err) => err.description(),
            UpdateServiceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateServiceError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon ECS API. Amazon ECS clients implement this trait.
pub trait Ecs {
    #[doc="<p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p>"]
    fn create_cluster(&self,
                      input: &CreateClusterRequest)
                      -> Result<CreateClusterResponse, CreateClusterError>;


    #[doc="<p>Runs and maintains a desired number of tasks from a specified task definition. If the number of tasks running in a service drops below <code>desiredCount</code>, Amazon ECS spawns another copy of the task in the specified cluster. To update an existing service, see <a>UpdateService</a>.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind a load balancer. The load balancer distributes traffic across the tasks that are associated with the service. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html\">Service Load Balancing</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can optionally specify a deployment configuration for your service. During a deployment (which is triggered by changing the task definition or the desired count of a service with an <a>UpdateService</a> operation), the service scheduler uses the <code>minimumHealthyPercent</code> and <code>maximumPercent</code> parameters to determine the deployment strategy.</p> <p>The <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the scheduler can stop two existing tasks to free up cluster capacity before starting two new tasks. Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer. The default value for <code>minimumHealthyPercent</code> is 50% in the console and 100% for the AWS CLI, the AWS SDKs, and the APIs.</p> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler can start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for <code>maximumPercent</code> is 200%.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service's task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy) with the <code>placementStrategy</code> parameter):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul>"]
    fn create_service(&self,
                      input: &CreateServiceRequest)
                      -> Result<CreateServiceResponse, CreateServiceError>;


    #[doc="<p>Deletes one or more custom attributes from an Amazon ECS resource.</p>"]
    fn delete_attributes(&self,
                         input: &DeleteAttributesRequest)
                         -> Result<DeleteAttributesResponse, DeleteAttributesError>;


    #[doc="<p>Deletes the specified cluster. You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>"]
    fn delete_cluster(&self,
                      input: &DeleteClusterRequest)
                      -> Result<DeleteClusterResponse, DeleteClusterError>;


    #[doc="<p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you cannot delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in <a>ListServices</a> API operations. After the tasks have stopped, then the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with <a>DescribeServices</a> API operations; however, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> API operations on those services will return a <code>ServiceNotFoundException</code> error.</p> </note>"]
    fn delete_service(&self,
                      input: &DeleteServiceRequest)
                      -> Result<DeleteServiceResponse, DeleteServiceError>;


    #[doc="<p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, you should stop all of the tasks running on the container instance before deregistration to avoid any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it does not terminate the EC2 instance; if you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents are not automatically deregistered when terminated).</p> </note>"]
    fn deregister_container_instance
        (&self,
         input: &DeregisterContainerInstanceRequest)
         -> Result<DeregisterContainerInstanceResponse, DeregisterContainerInstanceError>;


    #[doc="<p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service's desired count.</p> <p>You cannot use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you cannot update an existing service to reference an <code>INACTIVE</code> task definition (although there may be up to a 10 minute window following deregistration where these restrictions have not yet taken effect).</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely; however, this behavior is subject to change in the future, so you should not rely on <code>INACTIVE</code> task definitions persisting beyond the life cycle of any associated tasks and services.</p> </note>"]
    fn deregister_task_definition
        (&self,
         input: &DeregisterTaskDefinitionRequest)
         -> Result<DeregisterTaskDefinitionResponse, DeregisterTaskDefinitionError>;


    #[doc="<p>Describes one or more of your clusters.</p>"]
    fn describe_clusters(&self,
                         input: &DescribeClustersRequest)
                         -> Result<DescribeClustersResponse, DescribeClustersError>;


    #[doc="<p>Describes Amazon EC2 Container Service container instances. Returns metadata about registered and remaining resources on each container instance requested.</p>"]
    fn describe_container_instances
        (&self,
         input: &DescribeContainerInstancesRequest)
         -> Result<DescribeContainerInstancesResponse, DescribeContainerInstancesError>;


    #[doc="<p>Describes the specified services running in your cluster.</p>"]
    fn describe_services(&self,
                         input: &DescribeServicesRequest)
                         -> Result<DescribeServicesResponse, DescribeServicesError>;


    #[doc="<p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note>"]
    fn describe_task_definition
        (&self,
         input: &DescribeTaskDefinitionRequest)
         -> Result<DescribeTaskDefinitionResponse, DescribeTaskDefinitionError>;


    #[doc="<p>Describes a specified task or tasks.</p>"]
    fn describe_tasks(&self,
                      input: &DescribeTasksRequest)
                      -> Result<DescribeTasksResponse, DescribeTasksError>;


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon EC2 Container Service agent to poll for updates.</p>"]
    fn discover_poll_endpoint
        (&self,
         input: &DiscoverPollEndpointRequest)
         -> Result<DiscoverPollEndpointResponse, DiscoverPollEndpointError>;


    #[doc="<p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>"]
    fn list_attributes(&self,
                       input: &ListAttributesRequest)
                       -> Result<ListAttributesResponse, ListAttributesError>;


    #[doc="<p>Returns a list of existing clusters.</p>"]
    fn list_clusters(&self,
                     input: &ListClustersRequest)
                     -> Result<ListClustersResponse, ListClustersError>;


    #[doc="<p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html\">Cluster Query Language</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn list_container_instances
        (&self,
         input: &ListContainerInstancesRequest)
         -> Result<ListContainerInstancesResponse, ListContainerInstancesError>;


    #[doc="<p>Lists the services that are running in a specified cluster.</p>"]
    fn list_services(&self,
                     input: &ListServicesRequest)
                     -> Result<ListServicesResponse, ListServicesError>;


    #[doc="<p>Returns a list of task definition families that are registered to your account (which may include task definition families that no longer have any <code>ACTIVE</code> task definition revisions).</p> <p>You can filter out task definition families that do not contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>"]
    fn list_task_definition_families
        (&self,
         input: &ListTaskDefinitionFamiliesRequest)
         -> Result<ListTaskDefinitionFamiliesResponse, ListTaskDefinitionFamiliesError>;


    #[doc="<p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>"]
    fn list_task_definitions(&self,
                             input: &ListTaskDefinitionsRequest)
                             -> Result<ListTaskDefinitionsResponse, ListTaskDefinitionsError>;


    #[doc="<p>Returns a list of tasks for a specified cluster. You can filter the results by family name, by a particular container instance, or by the desired status of the task with the <code>family</code>, <code>containerInstance</code>, and <code>desiredStatus</code> parameters.</p> <p>Recently-stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour. </p>"]
    fn list_tasks(&self, input: &ListTasksRequest) -> Result<ListTasksResponse, ListTasksError>;


    #[doc="<p>Create or update an attribute on an Amazon ECS resource. If the attribute does not exist, it is created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes\">Attributes</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn put_attributes(&self,
                      input: &PutAttributesRequest)
                      -> Result<PutAttributesResponse, PutAttributesError>;


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p>"]
    fn register_container_instance
        (&self,
         input: &RegisterContainerInstanceRequest)
         -> Result<RegisterContainerInstanceResponse, RegisterContainerInstanceError>;


    #[doc="<p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html\">Amazon ECS Task Definitions</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the AWS CLI or SDKs to make API requests to the AWS services that are specified in the IAM policy associated with the role. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html\">IAM Roles for Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href=\"https://docs.docker.com/engine/reference/run/#/network-settings\">Network settings</a> in the Docker run reference.</p>"]
    fn register_task_definition
        (&self,
         input: &RegisterTaskDefinitionRequest)
         -> Result<RegisterTaskDefinitionResponse, RegisterTaskDefinitionError>;


    #[doc="<p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html\">Scheduling Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p>"]
    fn run_task(&self, input: &RunTaskRequest) -> Result<RunTaskResponse, RunTaskError>;


    #[doc="<p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html\">Scheduling Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn start_task(&self, input: &StartTaskRequest) -> Result<StartTaskResponse, StartTaskError>;


    #[doc="<p>Stops a running task.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a default 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS_CONTAINER_STOP_TIMEOUT</code> variable. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html\">Amazon ECS Container Agent Configuration</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> </note>"]
    fn stop_task(&self, input: &StopTaskRequest) -> Result<StopTaskResponse, StopTaskError>;


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p>"]
    fn submit_container_state_change
        (&self,
         input: &SubmitContainerStateChangeRequest)
         -> Result<SubmitContainerStateChangeResponse, SubmitContainerStateChangeError>;


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p>"]
    fn submit_task_state_change
        (&self,
         input: &SubmitTaskStateChangeRequest)
         -> Result<SubmitTaskStateChangeResponse, SubmitTaskStateChangeError>;


    #[doc="<p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent does not interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <p> <code>UpdateContainerAgent</code> requires the Amazon ECS-optimized AMI or Amazon Linux with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent\">Manually Updating the Amazon ECS Container Agent</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn update_container_agent
        (&self,
         input: &UpdateContainerAgentRequest)
         -> Result<UpdateContainerAgentResponse, UpdateContainerAgentError>;


    #[doc="<p>Modifies the status of an Amazon ECS container instance.</p> <p>You can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size. </p> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. Note that you can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement, which enables you to define the replacement batch size. For example, if <code>desiredCount</code> of four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained (provided that the cluster resources required to do this are available). If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service are not affected; you must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When you set a container instance to <code>ACTIVE</code>, the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>"]
    fn update_container_instances_state
        (&self,
         input: &UpdateContainerInstancesStateRequest)
         -> Result<UpdateContainerInstancesStateResponse, UpdateContainerInstancesStateError>;


    #[doc="<p>Modifies the desired count, deployment configuration, or task definition used in a service.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>You can use <a>UpdateService</a> to modify your task definition and deploy a new version of your service.</p> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment, which enables you to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service's task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul>"]
    fn update_service(&self,
                      input: &UpdateServiceRequest)
                      -> Result<UpdateServiceResponse, UpdateServiceError>;
}
/// A client for the Amazon ECS API.
pub struct EcsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> EcsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        EcsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Ecs for EcsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p>"]
    fn create_cluster(&self,
                      input: &CreateClusterRequest)
                      -> Result<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.CreateCluster");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateClusterResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateClusterError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Runs and maintains a desired number of tasks from a specified task definition. If the number of tasks running in a service drops below <code>desiredCount</code>, Amazon ECS spawns another copy of the task in the specified cluster. To update an existing service, see <a>UpdateService</a>.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind a load balancer. The load balancer distributes traffic across the tasks that are associated with the service. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html\">Service Load Balancing</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can optionally specify a deployment configuration for your service. During a deployment (which is triggered by changing the task definition or the desired count of a service with an <a>UpdateService</a> operation), the service scheduler uses the <code>minimumHealthyPercent</code> and <code>maximumPercent</code> parameters to determine the deployment strategy.</p> <p>The <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the scheduler can stop two existing tasks to free up cluster capacity before starting two new tasks. Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer. The default value for <code>minimumHealthyPercent</code> is 50% in the console and 100% for the AWS CLI, the AWS SDKs, and the APIs.</p> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler can start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for <code>maximumPercent</code> is 200%.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service's task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy) with the <code>placementStrategy</code> parameter):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul>"]
    fn create_service(&self,
                      input: &CreateServiceRequest)
                      -> Result<CreateServiceResponse, CreateServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.CreateService");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateServiceResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateServiceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes one or more custom attributes from an Amazon ECS resource.</p>"]
    fn delete_attributes(&self,
                         input: &DeleteAttributesRequest)
                         -> Result<DeleteAttributesResponse, DeleteAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DeleteAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteAttributesResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified cluster. You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>"]
    fn delete_cluster(&self,
                      input: &DeleteClusterRequest)
                      -> Result<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DeleteCluster");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteClusterResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteClusterError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you cannot delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in <a>ListServices</a> API operations. After the tasks have stopped, then the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with <a>DescribeServices</a> API operations; however, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> API operations on those services will return a <code>ServiceNotFoundException</code> error.</p> </note>"]
    fn delete_service(&self,
                      input: &DeleteServiceRequest)
                      -> Result<DeleteServiceResponse, DeleteServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DeleteService");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteServiceResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteServiceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, you should stop all of the tasks running on the container instance before deregistration to avoid any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it does not terminate the EC2 instance; if you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents are not automatically deregistered when terminated).</p> </note>"]
    fn deregister_container_instance
        (&self,
         input: &DeregisterContainerInstanceRequest)
         -> Result<DeregisterContainerInstanceResponse, DeregisterContainerInstanceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DeregisterContainerInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeregisterContainerInstanceResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeregisterContainerInstanceError::from_body(String::from_utf8_lossy(&body)
                                                                    .as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service's desired count.</p> <p>You cannot use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you cannot update an existing service to reference an <code>INACTIVE</code> task definition (although there may be up to a 10 minute window following deregistration where these restrictions have not yet taken effect).</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely; however, this behavior is subject to change in the future, so you should not rely on <code>INACTIVE</code> task definitions persisting beyond the life cycle of any associated tasks and services.</p> </note>"]
    fn deregister_task_definition
        (&self,
         input: &DeregisterTaskDefinitionRequest)
         -> Result<DeregisterTaskDefinitionResponse, DeregisterTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DeregisterTaskDefinition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeregisterTaskDefinitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeregisterTaskDefinitionError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Describes one or more of your clusters.</p>"]
    fn describe_clusters(&self,
                         input: &DescribeClustersRequest)
                         -> Result<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DescribeClusters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeClustersResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeClustersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes Amazon EC2 Container Service container instances. Returns metadata about registered and remaining resources on each container instance requested.</p>"]
    fn describe_container_instances
        (&self,
         input: &DescribeContainerInstancesRequest)
         -> Result<DescribeContainerInstancesResponse, DescribeContainerInstancesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DescribeContainerInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeContainerInstancesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeContainerInstancesError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the specified services running in your cluster.</p>"]
    fn describe_services(&self,
                         input: &DescribeServicesRequest)
                         -> Result<DescribeServicesResponse, DescribeServicesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DescribeServices");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeServicesResponse>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeServicesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note>"]
    fn describe_task_definition
        (&self,
         input: &DescribeTaskDefinitionRequest)
         -> Result<DescribeTaskDefinitionResponse, DescribeTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DescribeTaskDefinition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeTaskDefinitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeTaskDefinitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes a specified task or tasks.</p>"]
    fn describe_tasks(&self,
                      input: &DescribeTasksRequest)
                      -> Result<DescribeTasksResponse, DescribeTasksError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DescribeTasks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeTasksResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeTasksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon EC2 Container Service agent to poll for updates.</p>"]
    fn discover_poll_endpoint
        (&self,
         input: &DiscoverPollEndpointRequest)
         -> Result<DiscoverPollEndpointResponse, DiscoverPollEndpointError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.DiscoverPollEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DiscoverPollEndpointResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DiscoverPollEndpointError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>"]
    fn list_attributes(&self,
                       input: &ListAttributesRequest)
                       -> Result<ListAttributesResponse, ListAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListAttributesResponse>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of existing clusters.</p>"]
    fn list_clusters(&self,
                     input: &ListClustersRequest)
                     -> Result<ListClustersResponse, ListClustersError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListClusters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListClustersResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListClustersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html\">Cluster Query Language</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn list_container_instances
        (&self,
         input: &ListContainerInstancesRequest)
         -> Result<ListContainerInstancesResponse, ListContainerInstancesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListContainerInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListContainerInstancesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListContainerInstancesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the services that are running in a specified cluster.</p>"]
    fn list_services(&self,
                     input: &ListServicesRequest)
                     -> Result<ListServicesResponse, ListServicesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListServices");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListServicesResponse>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListServicesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of task definition families that are registered to your account (which may include task definition families that no longer have any <code>ACTIVE</code> task definition revisions).</p> <p>You can filter out task definition families that do not contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>"]
    fn list_task_definition_families
        (&self,
         input: &ListTaskDefinitionFamiliesRequest)
         -> Result<ListTaskDefinitionFamiliesResponse, ListTaskDefinitionFamiliesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListTaskDefinitionFamilies");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListTaskDefinitionFamiliesResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTaskDefinitionFamiliesError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>"]
    fn list_task_definitions(&self,
                             input: &ListTaskDefinitionsRequest)
                             -> Result<ListTaskDefinitionsResponse, ListTaskDefinitionsError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListTaskDefinitions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListTaskDefinitionsResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTaskDefinitionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of tasks for a specified cluster. You can filter the results by family name, by a particular container instance, or by the desired status of the task with the <code>family</code>, <code>containerInstance</code>, and <code>desiredStatus</code> parameters.</p> <p>Recently-stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour. </p>"]
    fn list_tasks(&self, input: &ListTasksRequest) -> Result<ListTasksResponse, ListTasksError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.ListTasks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListTasksResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTasksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Create or update an attribute on an Amazon ECS resource. If the attribute does not exist, it is created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes\">Attributes</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn put_attributes(&self,
                      input: &PutAttributesRequest)
                      -> Result<PutAttributesResponse, PutAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.PutAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutAttributesResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutAttributesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p>"]
    fn register_container_instance
        (&self,
         input: &RegisterContainerInstanceRequest)
         -> Result<RegisterContainerInstanceResponse, RegisterContainerInstanceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.RegisterContainerInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RegisterContainerInstanceResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterContainerInstanceError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html\">Amazon ECS Task Definitions</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the AWS CLI or SDKs to make API requests to the AWS services that are specified in the IAM policy associated with the role. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html\">IAM Roles for Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href=\"https://docs.docker.com/engine/reference/run/#/network-settings\">Network settings</a> in the Docker run reference.</p>"]
    fn register_task_definition
        (&self,
         input: &RegisterTaskDefinitionRequest)
         -> Result<RegisterTaskDefinitionResponse, RegisterTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.RegisterTaskDefinition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RegisterTaskDefinitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterTaskDefinitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html\">Scheduling Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p>"]
    fn run_task(&self, input: &RunTaskRequest) -> Result<RunTaskResponse, RunTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonEC2ContainerServiceV20141113.RunTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RunTaskResponse>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RunTaskError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html\">Scheduling Tasks</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn start_task(&self, input: &StartTaskRequest) -> Result<StartTaskResponse, StartTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.StartTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartTaskResponse>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartTaskError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Stops a running task.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a default 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS_CONTAINER_STOP_TIMEOUT</code> variable. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html\">Amazon ECS Container Agent Configuration</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p> </note>"]
    fn stop_task(&self, input: &StopTaskRequest) -> Result<StopTaskResponse, StopTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.StopTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopTaskResponse>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopTaskError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p>"]
    fn submit_container_state_change
        (&self,
         input: &SubmitContainerStateChangeRequest)
         -> Result<SubmitContainerStateChangeResponse, SubmitContainerStateChangeError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.SubmitContainerStateChange");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SubmitContainerStateChangeResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SubmitContainerStateChangeError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<note> <p>This action is only used by the Amazon EC2 Container Service agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p>"]
    fn submit_task_state_change
        (&self,
         input: &SubmitTaskStateChangeRequest)
         -> Result<SubmitTaskStateChangeResponse, SubmitTaskStateChangeError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.SubmitTaskStateChange");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SubmitTaskStateChangeResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SubmitTaskStateChangeError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent does not interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <p> <code>UpdateContainerAgent</code> requires the Amazon ECS-optimized AMI or Amazon Linux with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent\">Manually Updating the Amazon ECS Container Agent</a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    fn update_container_agent
        (&self,
         input: &UpdateContainerAgentRequest)
         -> Result<UpdateContainerAgentResponse, UpdateContainerAgentError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.UpdateContainerAgent");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateContainerAgentResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateContainerAgentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the status of an Amazon ECS container instance.</p> <p>You can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size. </p> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. Note that you can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement, which enables you to define the replacement batch size. For example, if <code>desiredCount</code> of four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained (provided that the cluster resources required to do this are available). If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service are not affected; you must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When you set a container instance to <code>ACTIVE</code>, the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>"]
    fn update_container_instances_state
        (&self,
         input: &UpdateContainerInstancesStateRequest)
         -> Result<UpdateContainerInstancesStateResponse, UpdateContainerInstancesStateError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.UpdateContainerInstancesState");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateContainerInstancesStateResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateContainerInstancesStateError::from_body(String::from_utf8_lossy(&body)
                                                                      .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the desired count, deployment configuration, or task definition used in a service.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>You can use <a>UpdateService</a> to modify your task definition and deploy a new version of your service.</p> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment, which enables you to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service's task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul>"]
    fn update_service(&self,
                      input: &UpdateServiceRequest)
                      -> Result<UpdateServiceResponse, UpdateServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonEC2ContainerServiceV20141113.UpdateService");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateServiceResponse>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateServiceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
