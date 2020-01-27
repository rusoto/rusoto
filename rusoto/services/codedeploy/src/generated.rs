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
/// <p>Represents the input of, and adds tags to, an on-premises instance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToOnPremisesInstancesInput {
    /// <p>The names of the on-premises instances to which to add tags.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The tag key-value pairs to add to the on-premises instances.</p> <p>Keys and values are both required. Keys cannot be null or empty strings. Value-only tags are not allowed.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>Information about an alarm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alarm {
    /// <p>The name of the alarm. Maximum length is 255 characters. Each alarm name can be used only once in a list of alarms.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about alarms associated with the deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlarmConfiguration {
    /// <p>A list of alarms configured for the deployment group. A maximum of 10 alarms can be added to a deployment group.</p>
    #[serde(rename = "alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    /// <p>Indicates whether the alarm configuration is enabled.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p><p>Indicates whether a deployment should continue if information about the current state of alarms cannot be retrieved from Amazon CloudWatch. The default value is false.</p> <ul> <li> <p>true: The deployment proceeds even if alarm status information can&#39;t be retrieved from Amazon CloudWatch.</p> </li> <li> <p>false: The deployment stops if alarm status information can&#39;t be retrieved from Amazon CloudWatch.</p> </li> </ul></p>
    #[serde(rename = "ignorePollAlarmFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_poll_alarm_failure: Option<bool>,
}

/// <p> A revision for an AWS Lambda or Amazon ECS deployment that is a YAML-formatted or JSON-formatted string. For AWS Lambda and Amazon ECS deployments, the revision is the same as the AppSpec file. This method replaces the deprecated <code>RawString</code> data type. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSpecContent {
    /// <p> The YAML-formatted or JSON-formatted revision string. </p> <p> For an AWS Lambda deployment, the content includes a Lambda function name, the alias for its original version, and the alias for its replacement version. The deployment shifts traffic from the original version of the Lambda function to the replacement version. </p> <p> For an Amazon ECS deployment, the content includes the task name, information about the load balancer that serves traffic to the container, and more. </p> <p> For both types of deployments, the content can specify Lambda functions that run at specified hooks, such as <code>BeforeInstall</code>, during a deployment. </p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p> The SHA256 hash value of the revision content. </p>
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_256: Option<String>,
}

/// <p>Information about an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationInfo {
    /// <p>The application ID.</p>
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>The destination platform type for deployment of the application (<code>Lambda</code> or <code>Server</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The time at which the application was created.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The name for a connection to a GitHub account.</p>
    #[serde(rename = "gitHubAccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_account_name: Option<String>,
    /// <p>True if the user has authenticated with GitHub for the specified application. Otherwise, false.</p>
    #[serde(rename = "linkedToGitHub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_to_git_hub: Option<bool>,
}

/// <p>Information about a configuration for automatically rolling back to a previous version of an application revision when a deployment is not completed successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoRollbackConfiguration {
    /// <p>Indicates whether a defined automatic rollback configuration is currently enabled.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The event type or types that trigger a rollback.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

/// <p>Information about an Auto Scaling group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroup {
    /// <p>An Auto Scaling lifecycle event hook name.</p>
    #[serde(rename = "hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    /// <p>The Auto Scaling group name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents the input of a BatchGetApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetApplicationRevisionsInput {
    /// <p>The name of an AWS CodeDeploy application about which to get revision information.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>An array of <code>RevisionLocation</code> objects that specify information to get about the application revisions, including type and location. The maximum number of <code>RevisionLocation</code> objects you can specify is 25.</p>
    #[serde(rename = "revisions")]
    pub revisions: Vec<RevisionLocation>,
}

/// <p>Represents the output of a BatchGetApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetApplicationRevisionsOutput {
    /// <p>The name of the application that corresponds to the revisions.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>Information about errors that might have occurred during the API call.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Additional information about the revisions, including the type and location.</p>
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionInfo>>,
}

/// <p>Represents the input of a BatchGetApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetApplicationsInput {
    /// <p>A list of application names separated by spaces. The maximum number of application names you can specify is 25.</p>
    #[serde(rename = "applicationNames")]
    pub application_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetApplicationsOutput {
    /// <p>Information about the applications.</p>
    #[serde(rename = "applicationsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_info: Option<Vec<ApplicationInfo>>,
}

/// <p>Represents the input of a BatchGetDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetDeploymentGroupsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The names of the deployment groups.</p>
    #[serde(rename = "deploymentGroupNames")]
    pub deployment_group_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetDeploymentGroupsOutput {
    /// <p>Information about the deployment groups.</p>
    #[serde(rename = "deploymentGroupsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups_info: Option<Vec<DeploymentGroupInfo>>,
    /// <p>Information about errors that might have occurred during the API call.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p> Represents the input of a BatchGetDeploymentInstances operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetDeploymentInstancesInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>The unique IDs of instances used in the deployment. The maximum number of instance IDs you can specify is 25.</p>
    #[serde(rename = "instanceIds")]
    pub instance_ids: Vec<String>,
}

/// <p>Represents the output of a BatchGetDeploymentInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetDeploymentInstancesOutput {
    /// <p>Information about errors that might have occurred during the API call.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the instance.</p>
    #[serde(rename = "instancesSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_summary: Option<Vec<InstanceSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetDeploymentTargetsInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p><p> The unique IDs of the deployment targets. The compute platform of the deployment determines the type of the targets and their formats. The maximum number of deployment target IDs you can specify is 25.</p> <ul> <li> <p> For deployments that use the EC2/On-premises compute platform, the target IDs are EC2 or on-premises instances IDs, and their target type is <code>instanceTarget</code>. </p> </li> <li> <p> For deployments that use the AWS Lambda compute platform, the target IDs are the names of Lambda functions, and their target type is <code>instanceTarget</code>. </p> </li> <li> <p> For deployments that use the Amazon ECS compute platform, the target IDs are pairs of Amazon ECS clusters and services specified using the format <code>&lt;clustername&gt;:&lt;servicename&gt;</code>. Their target type is <code>ecsTarget</code>. </p> </li> </ul></p>
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetDeploymentTargetsOutput {
    /// <p><p> A list of target objects for a deployment. Each target object contains details about the target, such as its status and lifecycle events. The type of the target objects depends on the deployment&#39; compute platform. </p> <ul> <li> <p> <b>EC2/On-premises</b>: Each target object is an EC2 or on-premises instance. </p> </li> <li> <p> <b>AWS Lambda</b>: The target object is a specific version of an AWS Lambda function. </p> </li> <li> <p> <b>Amazon ECS</b>: The target object is an Amazon ECS service. </p> </li> </ul></p>
    #[serde(rename = "deploymentTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<Vec<DeploymentTarget>>,
}

/// <p> Represents the input of a BatchGetDeployments operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetDeploymentsInput {
    /// <p> A list of deployment IDs, separated by spaces. The maximum number of deployment IDs you can specify is 25.</p>
    #[serde(rename = "deploymentIds")]
    pub deployment_ids: Vec<String>,
}

/// <p> Represents the output of a BatchGetDeployments operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetDeploymentsOutput {
    /// <p> Information about the deployments. </p>
    #[serde(rename = "deploymentsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_info: Option<Vec<DeploymentInfo>>,
}

/// <p>Represents the input of a BatchGetOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetOnPremisesInstancesInput {
    /// <p>The names of the on-premises instances about which to get information. The maximum number of instance names you can specify is 25.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetOnPremisesInstancesOutput {
    /// <p>Information about the on-premises instances.</p>
    #[serde(rename = "instanceInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_infos: Option<Vec<InstanceInfo>>,
}

/// <p>Information about blue/green deployment options for a deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlueGreenDeploymentConfiguration {
    /// <p>Information about the action to take when newly provisioned instances are ready to receive traffic in a blue/green deployment.</p>
    #[serde(rename = "deploymentReadyOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_ready_option: Option<DeploymentReadyOption>,
    /// <p>Information about how instances are provisioned for a replacement environment in a blue/green deployment.</p>
    #[serde(rename = "greenFleetProvisioningOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_fleet_provisioning_option: Option<GreenFleetProvisioningOption>,
    /// <p>Information about whether to terminate instances in the original fleet during a blue/green deployment.</p>
    #[serde(rename = "terminateBlueInstancesOnDeploymentSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_blue_instances_on_deployment_success: Option<BlueInstanceTerminationOption>,
}

/// <p>Information about whether instances in the original environment are terminated when a blue/green deployment is successful. <code>BlueInstanceTerminationOption</code> does not apply to Lambda deployments. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlueInstanceTerminationOption {
    /// <p><p>The action to take on instances in the original environment after a successful blue/green deployment.</p> <ul> <li> <p>TERMINATE: Instances are terminated after a specified wait time.</p> </li> <li> <p>KEEP_ALIVE: Instances are left running after they are deregistered from the load balancer and removed from the deployment group.</p> </li> </ul></p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>For an Amazon EC2 deployment, the number of minutes to wait after a successful blue/green deployment before terminating instances from the original environment.</p> <p> For an Amazon ECS deployment, the number of minutes before deleting the original (blue) task set. During an Amazon ECS deployment, CodeDeploy shifts traffic from the original (blue) task set to a replacement (green) task set. </p> <p> The maximum setting is 2880 minutes (2 days). </p>
    #[serde(rename = "terminationWaitTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_wait_time_in_minutes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContinueDeploymentInput {
    /// <p> The unique ID of a blue/green deployment for which you want to start rerouting traffic to the replacement environment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> The status of the deployment's waiting period. READY_WAIT indicates the deployment is ready to start shifting traffic. TERMINATION_WAIT indicates the traffic is shifted, but the original target is not terminated. </p>
    #[serde(rename = "deploymentWaitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_wait_type: Option<String>,
}

/// <p>Represents the input of a CreateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationInput {
    /// <p>The name of the application. This name must be unique with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p> The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p> The metadata that you apply to CodeDeploy applications to help you organize and categorize them. Each tag consists of a key and an optional value, both of which you define. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the output of a CreateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationOutput {
    /// <p>A unique application ID.</p>
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

/// <p>Represents the input of a CreateDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentConfigInput {
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The name of the deployment configuration to create.</p>
    #[serde(rename = "deploymentConfigName")]
    pub deployment_config_name: String,
    /// <p>The minimum number of healthy instances that should be available at any time during the deployment. There are two parameters expected in the input: type and value.</p> <p>The type parameter takes either of the following values:</p> <ul> <li> <p>HOST_COUNT: The value parameter represents the minimum number of healthy instances as an absolute value.</p> </li> <li> <p>FLEET_PERCENT: The value parameter represents the minimum number of healthy instances as a percentage of the total number of instances in the deployment. If you specify FLEET_PERCENT, at the start of the deployment, AWS CodeDeploy converts the percentage to the equivalent number of instance and rounds up fractional instances.</p> </li> </ul> <p>The value parameter takes an integer.</p> <p>For example, to set a minimum of 95% healthy instance, specify a type of FLEET_PERCENT and a value of 95.</p>
    #[serde(rename = "minimumHealthyHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
    /// <p>The configuration that specifies how the deployment traffic is routed.</p>
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
}

/// <p>Represents the output of a CreateDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentConfigOutput {
    /// <p>A unique deployment configuration ID.</p>
    #[serde(rename = "deploymentConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_id: Option<String>,
}

/// <p>Represents the input of a CreateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentGroupInput {
    /// <p>Information to add about Amazon CloudWatch alarms when the deployment group is created.</p>
    #[serde(rename = "alarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Configuration information for an automatic rollback that is added when a deployment group is created.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>A list of associated Amazon EC2 Auto Scaling groups.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>Information about blue/green deployment options for a deployment group.</p>
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// <p>If specified, the deployment configuration name can be either one of the predefined configurations provided with AWS CodeDeploy or a custom deployment configuration that you create by calling the create deployment configuration operation.</p> <p>CodeDeployDefault.OneAtATime is the default deployment configuration. It is used if a configuration isn't specified for the deployment or deployment group.</p> <p>For more information about the predefined deployment configurations in AWS CodeDeploy, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-configurations.html">Working with Deployment Groups in AWS CodeDeploy</a> in the AWS CodeDeploy User Guide.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>The name of a new deployment group for the specified application.</p>
    #[serde(rename = "deploymentGroupName")]
    pub deployment_group_name: String,
    /// <p>Information about the type of deployment, in-place or blue/green, that you want to run and whether to route deployment traffic behind a load balancer.</p>
    #[serde(rename = "deploymentStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    /// <p>The Amazon EC2 tags on which to filter. The deployment group includes EC2 instances with any of the specified tags. Cannot be used in the same call as ec2TagSet.</p>
    #[serde(rename = "ec2TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_filters: Option<Vec<EC2TagFilter>>,
    /// <p>Information about groups of tags applied to EC2 instances. The deployment group includes only EC2 instances identified by all the tag groups. Cannot be used in the same call as ec2TagFilters.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
    /// <p> The target Amazon ECS services in the deployment group. This applies only to deployment groups that use the Amazon ECS compute platform. A target Amazon ECS service is specified as an Amazon ECS cluster and service name pair using the format <code>&lt;clustername&gt;:&lt;servicename&gt;</code>. </p>
    #[serde(rename = "ecsServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    /// <p>Information about the load balancer used in a deployment.</p>
    #[serde(rename = "loadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// <p>The on-premises instance tags on which to filter. The deployment group includes on-premises instances with any of the specified tags. Cannot be used in the same call as OnPremisesTagSet.</p>
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    /// <p>Information about groups of tags applied to on-premises instances. The deployment group includes only on-premises instances identified by all of the tag groups. Cannot be used in the same call as onPremisesInstanceTagFilters.</p>
    #[serde(rename = "onPremisesTagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// <p>A service role ARN that allows AWS CodeDeploy to act on the user's behalf when interacting with AWS services.</p>
    #[serde(rename = "serviceRoleArn")]
    pub service_role_arn: String,
    /// <p> The metadata that you apply to CodeDeploy deployment groups to help you organize and categorize them. Each tag consists of a key and an optional value, both of which you define. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Information about triggers to create when the deployment group is created. For examples, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-sns.html">Create a Trigger for an AWS CodeDeploy Event</a> in the AWS CodeDeploy User Guide.</p>
    #[serde(rename = "triggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

/// <p>Represents the output of a CreateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentGroupOutput {
    /// <p>A unique deployment group ID.</p>
    #[serde(rename = "deploymentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_id: Option<String>,
}

/// <p>Represents the input of a CreateDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Configuration information for an automatic rollback that is added when a deployment is created.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>The name of a deployment configuration associated with the IAM user or AWS account.</p> <p>If not specified, the value configured in the deployment group is used as the default. If the deployment group does not have a deployment configuration associated with it, CodeDeployDefault.OneAtATime is used by default.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>The name of the deployment group.</p>
    #[serde(rename = "deploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// <p>A comment about the deployment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Information about how AWS CodeDeploy handles files that already exist in a deployment target location but weren&#39;t part of the previous successful deployment.</p> <p>The fileExistsBehavior parameter takes any of the following values:</p> <ul> <li> <p>DISALLOW: The deployment fails. This is also the default behavior if no option is specified.</p> </li> <li> <p>OVERWRITE: The version of the file from the application revision currently being deployed replaces the version already on the instance.</p> </li> <li> <p>RETAIN: The version of the file already on the instance is kept and used as part of the new deployment.</p> </li> </ul></p>
    #[serde(rename = "fileExistsBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_exists_behavior: Option<String>,
    /// <p> If true, then if an ApplicationStop, BeforeBlockTraffic, or AfterBlockTraffic deployment lifecycle event to an instance fails, then the deployment continues to the next deployment lifecycle event. For example, if ApplicationStop fails, the deployment continues with DownloadBundle. If BeforeBlockTraffic fails, the deployment continues with BlockTraffic. If AfterBlockTraffic fails, the deployment continues with ApplicationStop. </p> <p> If false or not specified, then if a lifecycle event fails during a deployment to an instance, that deployment fails. If deployment to that instance is part of an overall deployment and the number of healthy hosts is not less than the minimum number of healthy hosts, then a deployment to the next instance is attempted. </p> <p> During a deployment, the AWS CodeDeploy agent runs the scripts specified for ApplicationStop, BeforeBlockTraffic, and AfterBlockTraffic in the AppSpec file from the previous successful deployment. (All other scripts are run from the AppSpec file in the current deployment.) If one of these scripts contains an error and does not run successfully, the deployment can fail. </p> <p> If the cause of the failure is a script from the last successful deployment that will never run successfully, create a new deployment and use <code>ignoreApplicationStopFailures</code> to specify that the ApplicationStop, BeforeBlockTraffic, and AfterBlockTraffic failures should be ignored. </p>
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    /// <p> The type and location of the revision to deploy. </p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    /// <p> Information about the instances that belong to the replacement environment in a blue/green deployment. </p>
    #[serde(rename = "targetInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<TargetInstances>,
    /// <p> Indicates whether to deploy to all instances or only to instances that are not running the latest application revision. </p>
    #[serde(rename = "updateOutdatedInstancesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_outdated_instances_only: Option<bool>,
}

/// <p> Represents the output of a CreateDeployment operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentOutput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Represents the input of a DeleteApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
}

/// <p>Represents the input of a DeleteDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeploymentConfigInput {
    /// <p>The name of a deployment configuration associated with the IAM user or AWS account.</p>
    #[serde(rename = "deploymentConfigName")]
    pub deployment_config_name: String,
}

/// <p>Represents the input of a DeleteDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeploymentGroupInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The name of a deployment group for the specified application.</p>
    #[serde(rename = "deploymentGroupName")]
    pub deployment_group_name: String,
}

/// <p>Represents the output of a DeleteDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeploymentGroupOutput {
    /// <p>If the output contains no data, and the corresponding deployment group contained at least one Auto Scaling group, AWS CodeDeploy successfully removed all corresponding Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group. If the output contains data, AWS CodeDeploy could not remove some Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group.</p>
    #[serde(rename = "hooksNotCleanedUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_not_cleaned_up: Option<Vec<AutoScalingGroup>>,
}

/// <p>Represents the input of a DeleteGitHubAccount operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGitHubAccountTokenInput {
    /// <p>The name of the GitHub account connection to delete.</p>
    #[serde(rename = "tokenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

/// <p>Represents the output of a DeleteGitHubAccountToken operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGitHubAccountTokenOutput {
    /// <p>The name of the GitHub account connection that was deleted.</p>
    #[serde(rename = "tokenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

/// <p>Information about a deployment configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentConfigInfo {
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The time at which the deployment configuration was created.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The deployment configuration ID.</p>
    #[serde(rename = "deploymentConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_id: Option<String>,
    /// <p>The deployment configuration name.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>Information about the number or percentage of minimum healthy instance.</p>
    #[serde(rename = "minimumHealthyHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
    /// <p>The configuration that specifies how the deployment traffic is routed. Only deployments with a Lambda compute platform can specify this.</p>
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
}

/// <p>Information about a deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentGroupInfo {
    /// <p>A list of alarms associated with the deployment group.</p>
    #[serde(rename = "alarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>Information about the automatic rollback configuration associated with the deployment group.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>A list of associated Auto Scaling groups.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    /// <p>Information about blue/green deployment options for a deployment group.</p>
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>The deployment configuration name.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>The deployment group ID.</p>
    #[serde(rename = "deploymentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_id: Option<String>,
    /// <p>The deployment group name.</p>
    #[serde(rename = "deploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// <p>Information about the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer.</p>
    #[serde(rename = "deploymentStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    /// <p>The Amazon EC2 tags on which to filter. The deployment group includes EC2 instances with any of the specified tags.</p>
    #[serde(rename = "ec2TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_filters: Option<Vec<EC2TagFilter>>,
    /// <p>Information about groups of tags applied to an EC2 instance. The deployment group includes only EC2 instances identified by all of the tag groups. Cannot be used in the same call as ec2TagFilters.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
    /// <p> The target Amazon ECS services in the deployment group. This applies only to deployment groups that use the Amazon ECS compute platform. A target Amazon ECS service is specified as an Amazon ECS cluster and service name pair using the format <code>&lt;clustername&gt;:&lt;servicename&gt;</code>. </p>
    #[serde(rename = "ecsServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    /// <p>Information about the most recent attempted deployment to the deployment group.</p>
    #[serde(rename = "lastAttemptedDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempted_deployment: Option<LastDeploymentInfo>,
    /// <p>Information about the most recent successful deployment to the deployment group.</p>
    #[serde(rename = "lastSuccessfulDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_deployment: Option<LastDeploymentInfo>,
    /// <p>Information about the load balancer to use in a deployment.</p>
    #[serde(rename = "loadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// <p>The on-premises instance tags on which to filter. The deployment group includes on-premises instances with any of the specified tags.</p>
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    /// <p>Information about groups of tags applied to an on-premises instance. The deployment group includes only on-premises instances identified by all the tag groups. Cannot be used in the same call as onPremisesInstanceTagFilters.</p>
    #[serde(rename = "onPremisesTagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// <p>A service role Amazon Resource Name (ARN) that grants CodeDeploy permission to make calls to AWS services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/getting-started-create-service-role.html">Create a Service Role for AWS CodeDeploy</a> in the <i>AWS CodeDeploy User Guide</i>.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>Information about the deployment group's target revision, including type and location.</p>
    #[serde(rename = "targetRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<RevisionLocation>,
    /// <p>Information about triggers associated with the deployment group.</p>
    #[serde(rename = "triggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

/// <p>Information about a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentInfo {
    /// <p>Provides information about the results of a deployment, such as whether instances in the original environment in a blue/green deployment were not terminated.</p>
    #[serde(rename = "additionalDeploymentStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_deployment_status_info: Option<String>,
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>Information about the automatic rollback configuration associated with the deployment.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>Information about blue/green deployment options for this deployment.</p>
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// <p>A timestamp that indicates when the deployment was complete.</p>
    #[serde(rename = "completeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<f64>,
    /// <p>The destination platform type for the deployment (<code>Lambda</code>, <code>Server</code>, or <code>ECS</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>A timestamp that indicates when the deployment was created.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p><p>The means by which the deployment was created:</p> <ul> <li> <p>user: A user created the deployment.</p> </li> <li> <p>autoscaling: Amazon EC2 Auto Scaling created the deployment.</p> </li> <li> <p>codeDeployRollback: A rollback process created the deployment.</p> </li> </ul></p>
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// <p> The deployment configuration name. </p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p> The deployment group name. </p>
    #[serde(rename = "deploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>A summary of the deployment status of the instances in the deployment.</p>
    #[serde(rename = "deploymentOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_overview: Option<DeploymentOverview>,
    /// <p>Messages that contain information about the status of a deployment.</p>
    #[serde(rename = "deploymentStatusMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_messages: Option<Vec<String>>,
    /// <p>Information about the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer.</p>
    #[serde(rename = "deploymentStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    /// <p>A comment about the deployment.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information about any error associated with this deployment.</p>
    #[serde(rename = "errorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_information: Option<ErrorInformation>,
    /// <p><p>Information about how AWS CodeDeploy handles files that already exist in a deployment target location but weren&#39;t part of the previous successful deployment.</p> <ul> <li> <p>DISALLOW: The deployment fails. This is also the default behavior if no option is specified.</p> </li> <li> <p>OVERWRITE: The version of the file from the application revision currently being deployed replaces the version already on the instance.</p> </li> <li> <p>RETAIN: The version of the file already on the instance is kept and used as part of the new deployment.</p> </li> </ul></p>
    #[serde(rename = "fileExistsBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_exists_behavior: Option<String>,
    /// <p> If true, then if an <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, or <code>AfterBlockTraffic</code> deployment lifecycle event to an instance fails, then the deployment continues to the next deployment lifecycle event. For example, if <code>ApplicationStop</code> fails, the deployment continues with DownloadBundle. If <code>BeforeBlockTraffic</code> fails, the deployment continues with <code>BlockTraffic</code>. If <code>AfterBlockTraffic</code> fails, the deployment continues with <code>ApplicationStop</code>. </p> <p> If false or not specified, then if a lifecycle event fails during a deployment to an instance, that deployment fails. If deployment to that instance is part of an overall deployment and the number of healthy hosts is not less than the minimum number of healthy hosts, then a deployment to the next instance is attempted. </p> <p> During a deployment, the AWS CodeDeploy agent runs the scripts specified for <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, and <code>AfterBlockTraffic</code> in the AppSpec file from the previous successful deployment. (All other scripts are run from the AppSpec file in the current deployment.) If one of these scripts contains an error and does not run successfully, the deployment can fail. </p> <p> If the cause of the failure is a script from the last successful deployment that will never run successfully, create a new deployment and use <code>ignoreApplicationStopFailures</code> to specify that the <code>ApplicationStop</code>, <code>BeforeBlockTraffic</code>, and <code>AfterBlockTraffic</code> failures should be ignored. </p>
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    /// <p>Indicates whether the wait period set for the termination of instances in the original environment has started. Status is 'false' if the KEEP_ALIVE option is specified. Otherwise, 'true' as soon as the termination wait period starts.</p>
    #[serde(rename = "instanceTerminationWaitTimeStarted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_termination_wait_time_started: Option<bool>,
    /// <p>Information about the load balancer used in the deployment.</p>
    #[serde(rename = "loadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// <p>Information about the application revision that was deployed to the deployment group before the most recent successful deployment.</p>
    #[serde(rename = "previousRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_revision: Option<RevisionLocation>,
    /// <p>Information about the location of stored application artifacts and the service from which to retrieve them.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    /// <p>Information about a deployment rollback.</p>
    #[serde(rename = "rollbackInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_info: Option<RollbackInfo>,
    /// <p>A timestamp that indicates when the deployment was deployed to the deployment group.</p> <p>In some cases, the reported value of the start time might be later than the complete time. This is due to differences in the clock settings of backend servers that participate in the deployment process.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The current state of the deployment as a whole.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Information about the instances that belong to the replacement environment in a blue/green deployment.</p>
    #[serde(rename = "targetInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<TargetInstances>,
    /// <p>Indicates whether only instances that are not running the latest application revision are to be deployed to.</p>
    #[serde(rename = "updateOutdatedInstancesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_outdated_instances_only: Option<bool>,
}

/// <p>Information about the deployment status of the instances in the deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentOverview {
    /// <p>The number of instances in the deployment in a failed state.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    /// <p>The number of instances in which the deployment is in progress.</p>
    #[serde(rename = "InProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// <p>The number of instances in the deployment in a pending state.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// <p>The number of instances in a replacement environment ready to receive traffic in a blue/green deployment.</p>
    #[serde(rename = "Ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i64>,
    /// <p>The number of instances in the deployment in a skipped state.</p>
    #[serde(rename = "Skipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    /// <p>The number of instances in the deployment to which revisions have been successfully deployed.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i64>,
}

/// <p>Information about how traffic is rerouted to instances in a replacement environment in a blue/green deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentReadyOption {
    /// <p><p>Information about when to reroute traffic from an original environment to a replacement environment in a blue/green deployment.</p> <ul> <li> <p>CONTINUE<em>DEPLOYMENT: Register new instances with the load balancer immediately after the new application revision is installed on the instances in the replacement environment.</p> </li> <li> <p>STOP</em>DEPLOYMENT: Do not register new instances with a load balancer unless traffic rerouting is started using <a>ContinueDeployment</a>. If traffic rerouting is not started before the end of the specified wait period, the deployment status is changed to Stopped.</p> </li> </ul></p>
    #[serde(rename = "actionOnTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_timeout: Option<String>,
    /// <p>The number of minutes to wait before the status of a blue/green deployment is changed to Stopped if rerouting is not started manually. Applies only to the STOP_DEPLOYMENT option for actionOnTimeout</p>
    #[serde(rename = "waitTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time_in_minutes: Option<i64>,
}

/// <p>Information about the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStyle {
    /// <p>Indicates whether to route deployment traffic behind a load balancer.</p>
    #[serde(rename = "deploymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    /// <p>Indicates whether to run an in-place deployment or a blue/green deployment.</p>
    #[serde(rename = "deploymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
}

/// <p> Information about the deployment target. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeploymentTarget {
    /// <p> The deployment type that is specific to the deployment's compute platform. </p>
    #[serde(rename = "deploymentTargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_target_type: Option<String>,
    /// <p> Information about the target for a deployment that uses the Amazon ECS compute platform. </p>
    #[serde(rename = "ecsTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_target: Option<ECSTarget>,
    /// <p> Information about the target for a deployment that uses the EC2/On-premises compute platform. </p>
    #[serde(rename = "instanceTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_target: Option<InstanceTarget>,
    /// <p> Information about the target for a deployment that uses the AWS Lambda compute platform. </p>
    #[serde(rename = "lambdaTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_target: Option<LambdaTarget>,
}

/// <p>Represents the input of a DeregisterOnPremisesInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterOnPremisesInstanceInput {
    /// <p>The name of the on-premises instance to deregister.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

/// <p>Diagnostic information about executable scripts that are part of a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Diagnostics {
    /// <p><p>The associated error code:</p> <ul> <li> <p>Success: The specified script ran.</p> </li> <li> <p>ScriptMissing: The specified script was not found in the specified location.</p> </li> <li> <p>ScriptNotExecutable: The specified script is not a recognized executable file type.</p> </li> <li> <p>ScriptTimedOut: The specified script did not finish running in the specified time period.</p> </li> <li> <p>ScriptFailed: The specified script failed to run as expected.</p> </li> <li> <p>UnknownError: The specified script did not run for an unknown reason.</p> </li> </ul></p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The last portion of the diagnostic log.</p> <p>If available, AWS CodeDeploy returns up to the last 4 KB of the diagnostic log.</p>
    #[serde(rename = "logTail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_tail: Option<String>,
    /// <p>The message associated with the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the script.</p>
    #[serde(rename = "scriptName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
}

/// <p>Information about an EC2 tag filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EC2TagFilter {
    /// <p>The tag filter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p><p>The tag filter type:</p> <ul> <li> <p>KEY<em>ONLY: Key only.</p> </li> <li> <p>VALUE</em>ONLY: Value only.</p> </li> <li> <p>KEY<em>AND</em>VALUE: Key and value.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The tag filter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about groups of EC2 instance tags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EC2TagSet {
    /// <p>A list that contains other lists of EC2 instance tag groups. For an instance to be included in the deployment group, it must be identified by all of the tag groups in the list.</p>
    #[serde(rename = "ec2TagSetList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set_list: Option<Vec<Vec<EC2TagFilter>>>,
}

/// <p> Contains the service and cluster names used to identify an Amazon ECS deployment's target. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECSService {
    /// <p> The name of the cluster that the Amazon ECS service is associated with. </p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p> The name of the target Amazon ECS service. </p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p> Information about the target of an Amazon ECS deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ECSTarget {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> The date and time when the target Amazon ECS application was updated by a deployment. </p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p> The lifecycle events of the deployment to this target Amazon ECS application. </p>
    #[serde(rename = "lifecycleEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    /// <p> The status an Amazon ECS deployment's target ECS application. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The ARN of the target. </p>
    #[serde(rename = "targetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p> The unique ID of a deployment target that has a type of <code>ecsTarget</code>. </p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p> The <code>ECSTaskSet</code> objects associated with the ECS target. </p>
    #[serde(rename = "taskSetsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets_info: Option<Vec<ECSTaskSet>>,
}

/// <p> Information about a set of Amazon ECS tasks in an AWS CodeDeploy deployment. An Amazon ECS task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic. An AWS CodeDeploy application that uses the Amazon ECS compute platform deploys a containerized application in an Amazon ECS service as a task set. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ECSTaskSet {
    /// <p> The number of tasks in a task set. During a deployment that uses the Amazon ECS compute type, CodeDeploy instructs Amazon ECS to create a new task set and uses this value to determine how many tasks to create. After the updated task set is created, CodeDeploy shifts traffic to the new task set. </p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p> A unique ID of an <code>ECSTaskSet</code>. </p>
    #[serde(rename = "identifer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifer: Option<String>,
    /// <p> The number of tasks in the task set that are in the <code>PENDING</code> status during an Amazon ECS deployment. A task in the <code>PENDING</code> state is preparing to enter the <code>RUNNING</code> state. A task set enters the <code>PENDING</code> status when it launches for the first time, or when it is restarted after being in the <code>STOPPED</code> state. </p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p> The number of tasks in the task set that are in the <code>RUNNING</code> status during an Amazon ECS deployment. A task in the <code>RUNNING</code> state is running and ready for use. </p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p><p> The status of the task set. There are three valid task set statuses: </p> <ul> <li> <p> <code>PRIMARY</code>: Indicates the task set is serving production traffic. </p> </li> <li> <p> <code>ACTIVE</code>: Indicates the task set is not serving production traffic. </p> </li> <li> <p> <code>DRAINING</code>: Indicates the tasks in the task set are being stopped and their corresponding targets are being deregistered from their target group. </p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The target group associated with the task set. The target group is used by AWS CodeDeploy to manage traffic to a task set. </p>
    #[serde(rename = "targetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group: Option<TargetGroupInfo>,
    /// <p> A label that identifies whether the ECS task set is an original target (<code>BLUE</code>) or a replacement target (<code>GREEN</code>). </p>
    #[serde(rename = "taskSetLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set_label: Option<String>,
    /// <p> The percentage of traffic served by this task set. </p>
    #[serde(rename = "trafficWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_weight: Option<f64>,
}

/// <p>Information about a load balancer in Elastic Load Balancing to use in a deployment. Instances are registered directly with a load balancer, and traffic is routed to the load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ELBInfo {
    /// <p>For blue/green deployments, the name of the load balancer that is used to route traffic from original instances to replacement instances in a blue/green deployment. For in-place deployments, the name of the load balancer that instances are deregistered from so they are not serving traffic during a deployment, and then re-registered with after the deployment is complete.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a deployment error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorInformation {
    /// <p><p>For more information, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/error-codes.html">Error Codes for AWS CodeDeploy</a> in the <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide">AWS CodeDeploy User Guide</a>.</p> <p>The error code:</p> <ul> <li> <p>APPLICATION<em>MISSING: The application was missing. This error code is most likely raised if the application is deleted after the deployment is created, but before it is started.</p> </li> <li> <p>DEPLOYMENT</em>GROUP<em>MISSING: The deployment group was missing. This error code is most likely raised if the deployment group is deleted after the deployment is created, but before it is started.</p> </li> <li> <p>HEALTH</em>CONSTRAINTS: The deployment failed on too many instances to be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>HEALTH<em>CONSTRAINTS</em>INVALID: The revision cannot be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>IAM<em>ROLE</em>MISSING: The service role cannot be accessed.</p> </li> <li> <p>IAM<em>ROLE</em>PERMISSIONS: The service role does not have the correct permissions.</p> </li> <li> <p>INTERNAL<em>ERROR: There was an internal error.</p> </li> <li> <p>NO</em>EC2<em>SUBSCRIPTION: The calling account is not subscribed to Amazon EC2.</p> </li> <li> <p>NO</em>INSTANCES: No instances were specified, or no instances can be found.</p> </li> <li> <p>OVER<em>MAX</em>INSTANCES: The maximum number of instances was exceeded.</p> </li> <li> <p>THROTTLED: The operation was throttled because the calling account exceeded the throttling limits of one or more AWS services.</p> </li> <li> <p>TIMEOUT: The deployment has timed out.</p> </li> <li> <p>REVISION_MISSING: The revision ID was missing. This error code is most likely raised if the revision is deleted after the deployment is created, but before it is started.</p> </li> </ul></p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>An accompanying error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Information about an application revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenericRevisionInfo {
    /// <p>The deployment groups for which this is the current target revision.</p>
    #[serde(rename = "deploymentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups: Option<Vec<String>>,
    /// <p>A comment about the revision.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When the revision was first used by AWS CodeDeploy.</p>
    #[serde(rename = "firstUsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_used_time: Option<f64>,
    /// <p>When the revision was last used by AWS CodeDeploy.</p>
    #[serde(rename = "lastUsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_time: Option<f64>,
    /// <p>When the revision was registered with AWS CodeDeploy.</p>
    #[serde(rename = "registerTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<f64>,
}

/// <p>Represents the input of a GetApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
}

/// <p>Represents the output of a GetApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApplicationOutput {
    /// <p>Information about the application.</p>
    #[serde(rename = "application")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationInfo>,
}

/// <p>Represents the input of a GetApplicationRevision operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationRevisionInput {
    /// <p>The name of the application that corresponds to the revision.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Information about the application revision to get, including type and location.</p>
    #[serde(rename = "revision")]
    pub revision: RevisionLocation,
}

/// <p>Represents the output of a GetApplicationRevision operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApplicationRevisionOutput {
    /// <p>The name of the application that corresponds to the revision.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>Additional information about the revision, including type and location.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    /// <p>General information about the revision.</p>
    #[serde(rename = "revisionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_info: Option<GenericRevisionInfo>,
}

/// <p>Represents the input of a GetDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentConfigInput {
    /// <p>The name of a deployment configuration associated with the IAM user or AWS account.</p>
    #[serde(rename = "deploymentConfigName")]
    pub deployment_config_name: String,
}

/// <p>Represents the output of a GetDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentConfigOutput {
    /// <p>Information about the deployment configuration.</p>
    #[serde(rename = "deploymentConfigInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_info: Option<DeploymentConfigInfo>,
}

/// <p>Represents the input of a GetDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentGroupInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The name of a deployment group for the specified application.</p>
    #[serde(rename = "deploymentGroupName")]
    pub deployment_group_name: String,
}

/// <p>Represents the output of a GetDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentGroupOutput {
    /// <p>Information about the deployment group.</p>
    #[serde(rename = "deploymentGroupInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_info: Option<DeploymentGroupInfo>,
}

/// <p>Represents the input of a GetDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentInput {
    /// <p> The unique ID of a deployment associated with the IAM user or AWS account. </p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
}

/// <p> Represents the input of a GetDeploymentInstance operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentInstanceInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p> The unique ID of an instance in the deployment group. </p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
}

/// <p> Represents the output of a GetDeploymentInstance operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentInstanceOutput {
    /// <p> Information about the instance. </p>
    #[serde(rename = "instanceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_summary: Option<InstanceSummary>,
}

/// <p>Represents the output of a GetDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentOutput {
    /// <p>Information about the deployment.</p>
    #[serde(rename = "deploymentInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_info: Option<DeploymentInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeploymentTargetInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> The unique ID of a deployment target. </p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeploymentTargetOutput {
    /// <p> A deployment target that contains information about a deployment such as its status, lifecyle events, and when it was last updated. It also contains metadata about the deployment target. The deployment target metadata depends on the deployment target's type (<code>instanceTarget</code>, <code>lambdaTarget</code>, or <code>ecsTarget</code>). </p>
    #[serde(rename = "deploymentTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_target: Option<DeploymentTarget>,
}

/// <p> Represents the input of a GetOnPremisesInstance operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOnPremisesInstanceInput {
    /// <p> The name of the on-premises instance about which to get information. </p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

/// <p> Represents the output of a GetOnPremisesInstance operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOnPremisesInstanceOutput {
    /// <p> Information about the on-premises instance. </p>
    #[serde(rename = "instanceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_info: Option<InstanceInfo>,
}

/// <p>Information about the location of application artifacts stored in GitHub.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubLocation {
    /// <p>The SHA1 commit ID of the GitHub commit that represents the bundled artifacts for the application revision.</p>
    #[serde(rename = "commitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// <p>The GitHub account and repository pair that stores a reference to the commit that represents the bundled artifacts for the application revision. </p> <p>Specified as account/repository.</p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// <p>Information about the instances that belong to the replacement environment in a blue/green deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GreenFleetProvisioningOption {
    /// <p><p>The method used to add instances to a replacement environment.</p> <ul> <li> <p>DISCOVER<em>EXISTING: Use instances that already exist or will be created manually.</p> </li> <li> <p>COPY</em>AUTO<em>SCALING</em>GROUP: Use settings from a specified Auto Scaling group to define and create instances in a new Auto Scaling group.</p> </li> </ul></p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// <p>Information about an on-premises instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceInfo {
    /// <p>If the on-premises instance was deregistered, the time at which the on-premises instance was deregistered.</p>
    #[serde(rename = "deregisterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregister_time: Option<f64>,
    /// <p>The ARN of the IAM session associated with the on-premises instance.</p>
    #[serde(rename = "iamSessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_session_arn: Option<String>,
    /// <p>The IAM user ARN associated with the on-premises instance.</p>
    #[serde(rename = "iamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The ARN of the on-premises instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p>The name of the on-premises instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The time at which the on-premises instance was registered.</p>
    #[serde(rename = "registerTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<f64>,
    /// <p>The tags currently associated with the on-premises instance.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about an instance in a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceSummary {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p><p>Information about which environment an instance belongs to in a blue/green deployment.</p> <ul> <li> <p>BLUE: The instance is part of the original environment.</p> </li> <li> <p>GREEN: The instance is part of the replacement environment.</p> </li> </ul></p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>A timestamp that indicaties when the instance information was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A list of lifecycle events for this instance.</p>
    #[serde(rename = "lifecycleEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    /// <p><p>The deployment status for this instance:</p> <ul> <li> <p>Pending: The deployment is pending for this instance.</p> </li> <li> <p>In Progress: The deployment is in progress for this instance.</p> </li> <li> <p>Succeeded: The deployment has succeeded for this instance.</p> </li> <li> <p>Failed: The deployment has failed for this instance.</p> </li> <li> <p>Skipped: The deployment has been skipped for this instance.</p> </li> <li> <p>Unknown: The deployment status is unknown for this instance.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p> A target Amazon EC2 or on-premises instance during a deployment that uses the EC2/On-premises compute platform. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceTarget {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> A label that identifies whether the instance is an original target (<code>BLUE</code>) or a replacement target (<code>GREEN</code>). </p>
    #[serde(rename = "instanceLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_label: Option<String>,
    /// <p> The date and time when the target instance was updated by a deployment. </p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p> The lifecycle events of the deployment to this target instance. </p>
    #[serde(rename = "lifecycleEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    /// <p> The status an EC2/On-premises deployment's target instance. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The ARN of the target. </p>
    #[serde(rename = "targetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p> The unique ID of a deployment target that has a type of <code>instanceTarget</code>. </p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// <p> Information about a Lambda function specified in a deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionInfo {
    /// <p> The version of a Lambda function that production traffic points to. </p>
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <p> The alias of a Lambda function. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>. </p>
    #[serde(rename = "functionAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
    /// <p> The name of a Lambda function. </p>
    #[serde(rename = "functionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p> The version of a Lambda function that production traffic points to after the Lambda function is deployed. </p>
    #[serde(rename = "targetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
    /// <p> The percentage of production traffic that the target version of a Lambda function receives. </p>
    #[serde(rename = "targetVersionWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version_weight: Option<f64>,
}

/// <p> Information about the target AWS Lambda function during an AWS Lambda deployment. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaTarget {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> A <code>LambdaFunctionInfo</code> object that describes a target Lambda function. </p>
    #[serde(rename = "lambdaFunctionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_info: Option<LambdaFunctionInfo>,
    /// <p> The date and time when the target Lambda function was updated by a deployment. </p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p> The lifecycle events of the deployment to this target Lambda function. </p>
    #[serde(rename = "lifecycleEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    /// <p> The status an AWS Lambda deployment's target Lambda function. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> The ARN of the target. </p>
    #[serde(rename = "targetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p> The unique ID of a deployment target that has a type of <code>lambdaTarget</code>. </p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// <p>Information about the most recent attempted or successful deployment to a deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LastDeploymentInfo {
    /// <p>A timestamp that indicates when the most recent deployment to the deployment group started.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>A timestamp that indicates when the most recent deployment to the deployment group was complete.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The status of the most recent deployment.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a deployment lifecycle event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecycleEvent {
    /// <p>Diagnostic information about the deployment lifecycle event.</p>
    #[serde(rename = "diagnostics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    /// <p>A timestamp that indicates when the deployment lifecycle event ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The deployment lifecycle event name, such as ApplicationStop, BeforeInstall, AfterInstall, ApplicationStart, or ValidateService.</p>
    #[serde(rename = "lifecycleEventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_name: Option<String>,
    /// <p>A timestamp that indicates when the deployment lifecycle event started.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The deployment lifecycle event status:</p> <ul> <li> <p>Pending: The deployment lifecycle event is pending.</p> </li> <li> <p>InProgress: The deployment lifecycle event is in progress.</p> </li> <li> <p>Succeeded: The deployment lifecycle event ran successfully.</p> </li> <li> <p>Failed: The deployment lifecycle event has failed.</p> </li> <li> <p>Skipped: The deployment lifecycle event has been skipped.</p> </li> <li> <p>Unknown: The deployment lifecycle event is unknown.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p> Represents the input of a ListApplicationRevisions operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationRevisionsInput {
    /// <p> The name of an AWS CodeDeploy application associated with the IAM user or AWS account. </p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p><p> Whether to list revisions based on whether the revision is the target revision of an deployment group: </p> <ul> <li> <p>include: List revisions that are target revisions of a deployment group.</p> </li> <li> <p>exclude: Do not list revisions that are target revisions of a deployment group.</p> </li> <li> <p>ignore: List all revisions.</p> </li> </ul></p>
    #[serde(rename = "deployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed: Option<String>,
    /// <p>An identifier returned from the previous <code>ListApplicationRevisions</code> call. It can be used to return the next set of applications in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> An Amazon S3 bucket name to limit the search for revisions. </p> <p> If set to null, all of the user's buckets are searched. </p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p> A key prefix for the set of Amazon S3 objects to limit the search for revisions. </p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
    /// <p>The column name to use to sort the list results:</p> <ul> <li> <p>registerTime: Sort by the time the revisions were registered with AWS CodeDeploy.</p> </li> <li> <p>firstUsedTime: Sort by the time the revisions were first used in a deployment.</p> </li> <li> <p>lastUsedTime: Sort by the time the revisions were last used in a deployment.</p> </li> </ul> <p> If not specified or set to null, the results are returned in an arbitrary order. </p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p> The order in which to sort the list results: </p> <ul> <li> <p>ascending: ascending order.</p> </li> <li> <p>descending: descending order.</p> </li> </ul> <p>If not specified, the results are sorted in ascending order.</p> <p>If set to null, the results are sorted in an arbitrary order.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the output of a ListApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationRevisionsOutput {
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list application revisions call to return the next set of application revisions in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of locations that contain the matching revisions.</p>
    #[serde(rename = "revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionLocation>>,
}

/// <p>Represents the input of a ListApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationsInput {
    /// <p>An identifier returned from the previous list applications call. It can be used to return the next set of applications in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationsOutput {
    /// <p>A list of application names.</p>
    #[serde(rename = "applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list applications call to return the next set of applications in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListDeploymentConfigs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentConfigsInput {
    /// <p>An identifier returned from the previous <code>ListDeploymentConfigs</code> call. It can be used to return the next set of deployment configurations in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeploymentConfigs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentConfigsOutput {
    /// <p>A list of deployment configurations, including built-in configurations such as CodeDeployDefault.OneAtATime.</p>
    #[serde(rename = "deploymentConfigsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configs_list: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment configurations call to return the next set of deployment configurations in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentGroupsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentGroupsOutput {
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>A list of deployment group names.</p>
    #[serde(rename = "deploymentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment groups call to return the next set of deployment groups in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Represents the input of a ListDeploymentInstances operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentInstancesInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p><p>A subset of instances to list by status:</p> <ul> <li> <p>Pending: Include those instances with pending deployments.</p> </li> <li> <p>InProgress: Include those instances where deployments are still in progress.</p> </li> <li> <p>Succeeded: Include those instances with successful deployments.</p> </li> <li> <p>Failed: Include those instances with failed deployments.</p> </li> <li> <p>Skipped: Include those instances with skipped deployments.</p> </li> <li> <p>Unknown: Include those instances with deployments in an unknown state.</p> </li> </ul></p>
    #[serde(rename = "instanceStatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status_filter: Option<Vec<String>>,
    /// <p>The set of instances in a blue/green deployment, either those in the original environment ("BLUE") or those in the replacement environment ("GREEN"), for which you want to view instance information.</p>
    #[serde(rename = "instanceTypeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_filter: Option<Vec<String>>,
    /// <p>An identifier returned from the previous list deployment instances call. It can be used to return the next set of deployment instances in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeploymentInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentInstancesOutput {
    /// <p>A list of instance IDs.</p>
    #[serde(rename = "instancesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_list: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment instances call to return the next set of deployment instances in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentTargetsInput {
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> A token identifier returned from the previous <code>ListDeploymentTargets</code> call. It can be used to return the next set of deployment targets in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p> A key used to filter the returned targets. The two valid values are:</p> <ul> <li> <p> <code>TargetStatus</code> - A <code>TargetStatus</code> filter string can be <code>Failed</code>, <code>InProgress</code>, <code>Pending</code>, <code>Ready</code>, <code>Skipped</code>, <code>Succeeded</code>, or <code>Unknown</code>. </p> </li> <li> <p> <code>ServerInstanceLabel</code> - A <code>ServerInstanceLabel</code> filter string can be <code>Blue</code> or <code>Green</code>. </p> </li> </ul></p>
    #[serde(rename = "targetFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_filters: Option<::std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentTargetsOutput {
    /// <p> If a large amount of information is returned, a token identifier is also returned. It can be used in a subsequent <code>ListDeploymentTargets</code> call to return the next set of deployment targets in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The unique IDs of deployment targets. </p>
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<String>>,
}

/// <p>Represents the input of a ListDeployments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeploymentsInput {
    /// <p><p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p> <note> <p>If <code>applicationName</code> is specified, then <code>deploymentGroupName</code> must be specified. If it is not specified, then <code>deploymentGroupName</code> must not be specified. </p> </note></p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>A time range (start and end) for returning a subset of the list of deployments.</p>
    #[serde(rename = "createTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<TimeRange>,
    /// <p><p>The name of a deployment group for the specified application.</p> <note> <p>If <code>deploymentGroupName</code> is specified, then <code>applicationName</code> must be specified. If it is not specified, then <code>applicationName</code> must not be specified. </p> </note></p>
    #[serde(rename = "deploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// <p><p>A subset of deployments to list by status:</p> <ul> <li> <p>Created: Include created deployments in the resulting list.</p> </li> <li> <p>Queued: Include queued deployments in the resulting list.</p> </li> <li> <p>In Progress: Include in-progress deployments in the resulting list.</p> </li> <li> <p>Succeeded: Include successful deployments in the resulting list.</p> </li> <li> <p>Failed: Include failed deployments in the resulting list.</p> </li> <li> <p>Stopped: Include stopped deployments in the resulting list.</p> </li> </ul></p>
    #[serde(rename = "includeOnlyStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_only_statuses: Option<Vec<String>>,
    /// <p>An identifier returned from the previous list deployments call. It can be used to return the next set of deployments in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeployments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeploymentsOutput {
    /// <p>A list of deployment IDs.</p>
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployments call to return the next set of deployments in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListGitHubAccountTokenNames operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGitHubAccountTokenNamesInput {
    /// <p>An identifier returned from the previous ListGitHubAccountTokenNames call. It can be used to return the next set of names in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListGitHubAccountTokenNames operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGitHubAccountTokenNamesOutput {
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent ListGitHubAccountTokenNames call to return the next set of names in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of names of connections to GitHub accounts.</p>
    #[serde(rename = "tokenNameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name_list: Option<Vec<String>>,
}

/// <p>Represents the input of a ListOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOnPremisesInstancesInput {
    /// <p>An identifier returned from the previous list on-premises instances call. It can be used to return the next set of on-premises instances in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The registration status of the on-premises instances:</p> <ul> <li> <p>Deregistered: Include deregistered on-premises instances in the resulting list.</p> </li> <li> <p>Registered: Include registered on-premises instances in the resulting list.</p> </li> </ul></p>
    #[serde(rename = "registrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
    /// <p>The on-premises instance tags that are used to restrict the on-premises instance names returned.</p>
    #[serde(rename = "tagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

/// <p>Represents the output of the list on-premises instances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOnPremisesInstancesOutput {
    /// <p>The list of matching on-premises instance names.</p>
    #[serde(rename = "instanceNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_names: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list on-premises instances call to return the next set of on-premises instances in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>An identifier returned from the previous <code>ListTagsForResource</code> call. It can be used to return the next set of applications in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The ARN of a CodeDeploy resource. <code>ListTagsForResource</code> returns all the tags associated with the resource that is identified by the <code>ResourceArn</code>. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list application revisions call to return the next set of application revisions in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A list of tags returned by <code>ListTagsForResource</code>. The tags are associated with the resource identified by the input <code>ResourceArn</code> parameter. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about the Elastic Load Balancing load balancer or target group used in a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerInfo {
    /// <p><p>An array that contains information about the load balancer to use for load balancing in a deployment. In Elastic Load Balancing, load balancers are used with Classic Load Balancers.</p> <note> <p> Adding more than one load balancer to the array is not supported. </p> </note></p>
    #[serde(rename = "elbInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elb_info_list: Option<Vec<ELBInfo>>,
    /// <p><p>An array that contains information about the target group to use for load balancing in a deployment. In Elastic Load Balancing, target groups are used with Application Load Balancers.</p> <note> <p> Adding more than one target group to the array is not supported. </p> </note></p>
    #[serde(rename = "targetGroupInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_info_list: Option<Vec<TargetGroupInfo>>,
    /// <p> The target group pair information. This is an array of <code>TargeGroupPairInfo</code> objects with a maximum size of one. </p>
    #[serde(rename = "targetGroupPairInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_pair_info_list: Option<Vec<TargetGroupPairInfo>>,
}

/// <p>Information about minimum healthy instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumHealthyHosts {
    /// <p>The minimum healthy instance type:</p> <ul> <li> <p>HOST_COUNT: The minimum number of healthy instance as an absolute value.</p> </li> <li> <p>FLEET_PERCENT: The minimum number of healthy instance as a percentage of the total number of instance in the deployment.</p> </li> </ul> <p>In an example of nine instance, if a HOST_COUNT of six is specified, deploy to up to three instances at a time. The deployment is successful if six or more instances are deployed to successfully. Otherwise, the deployment fails. If a FLEET_PERCENT of 40 is specified, deploy to up to five instance at a time. The deployment is successful if four or more instance are deployed to successfully. Otherwise, the deployment fails.</p> <note> <p>In a call to the <code>GetDeploymentConfig</code>, CodeDeployDefault.OneAtATime returns a minimum healthy instance type of MOST_CONCURRENCY and a value of 1. This means a deployment to only one instance at a time. (You cannot set the type to MOST_CONCURRENCY, only to HOST_COUNT or FLEET_PERCENT.) In addition, with CodeDeployDefault.OneAtATime, AWS CodeDeploy attempts to ensure that all instances but one are kept in a healthy state during the deployment. Although this allows one instance at a time to be taken offline for a new deployment, it also means that if the deployment to the last instance fails, the overall deployment is still successful.</p> </note> <p>For more information, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/instances-health.html">AWS CodeDeploy Instance Health</a> in the <i>AWS CodeDeploy User Guide</i>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The minimum healthy instance value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// <p>Information about groups of on-premises instance tags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesTagSet {
    /// <p>A list that contains other lists of on-premises instance tag groups. For an instance to be included in the deployment group, it must be identified by all of the tag groups in the list.</p>
    #[serde(rename = "onPremisesTagSetList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set_list: Option<Vec<Vec<TagFilter>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecycleEventHookExecutionStatusInput {
    /// <p> The unique ID of a deployment. Pass this ID to a Lambda function that validates a deployment lifecycle event. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p> The execution ID of a deployment's lifecycle hook. A deployment lifecycle hook is specified in the <code>hooks</code> section of the AppSpec file. </p>
    #[serde(rename = "lifecycleEventHookExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_hook_execution_id: Option<String>,
    /// <p>The result of a Lambda function that validates a deployment lifecycle event (<code>Succeeded</code> or <code>Failed</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLifecycleEventHookExecutionStatusOutput {
    /// <p>The execution ID of the lifecycle event hook. A hook is specified in the <code>hooks</code> section of the deployment's AppSpec file.</p>
    #[serde(rename = "lifecycleEventHookExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_hook_execution_id: Option<String>,
}

/// <p>A revision for an AWS Lambda deployment that is a YAML-formatted or JSON-formatted string. For AWS Lambda deployments, the revision is the same as the AppSpec file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawString {
    /// <p>The YAML-formatted or JSON-formatted revision string. It includes information about which Lambda function to update and optional Lambda functions that validate deployment lifecycle events.</p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The SHA256 hash value of the revision content.</p>
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_256: Option<String>,
}

/// <p>Represents the input of a RegisterApplicationRevision operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterApplicationRevisionInput {
    /// <p>The name of an AWS CodeDeploy application associated with the IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>A comment about the revision.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information about the application revision to register, including type and location.</p>
    #[serde(rename = "revision")]
    pub revision: RevisionLocation,
}

/// <p>Represents the input of the register on-premises instance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterOnPremisesInstanceInput {
    /// <p>The ARN of the IAM session to associate with the on-premises instance.</p>
    #[serde(rename = "iamSessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_session_arn: Option<String>,
    /// <p>The ARN of the IAM user to associate with the on-premises instance.</p>
    #[serde(rename = "iamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The name of the on-premises instance to register.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

/// <p>Represents the input of a RemoveTagsFromOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromOnPremisesInstancesInput {
    /// <p>The names of the on-premises instances from which to remove tags.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The tag key-value pairs to remove from the on-premises instances.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>Information about an application revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevisionInfo {
    /// <p>Information about an application revision, including usage details and associated deployment groups.</p>
    #[serde(rename = "genericRevisionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_revision_info: Option<GenericRevisionInfo>,
    /// <p>Information about the location and type of an application revision.</p>
    #[serde(rename = "revisionLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_location: Option<RevisionLocation>,
}

/// <p>Information about the location of an application revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevisionLocation {
    /// <p> The content of an AppSpec file for an AWS Lambda or Amazon ECS deployment. The content is formatted as JSON or YAML and stored as a RawString. </p>
    #[serde(rename = "appSpecContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_spec_content: Option<AppSpecContent>,
    /// <p>Information about the location of application artifacts stored in GitHub.</p>
    #[serde(rename = "gitHubLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_location: Option<GitHubLocation>,
    /// <p><p>The type of application revision:</p> <ul> <li> <p>S3: An application revision stored in Amazon S3.</p> </li> <li> <p>GitHub: An application revision stored in GitHub (EC2/On-premises deployments only).</p> </li> <li> <p>String: A YAML-formatted or JSON-formatted string (AWS Lambda deployments only).</p> </li> </ul></p>
    #[serde(rename = "revisionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_type: Option<String>,
    /// <p>Information about the location of a revision stored in Amazon S3. </p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
    /// <p>Information about the location of an AWS Lambda deployment revision stored as a RawString.</p>
    #[serde(rename = "string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<RawString>,
}

/// <p>Information about a deployment rollback.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RollbackInfo {
    /// <p>The ID of the deployment rollback.</p>
    #[serde(rename = "rollbackDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_deployment_id: Option<String>,
    /// <p>Information that describes the status of a deployment rollback (for example, whether the deployment can't be rolled back, is in progress, failed, or succeeded). </p>
    #[serde(rename = "rollbackMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_message: Option<String>,
    /// <p>The deployment ID of the deployment that was underway and triggered a rollback deployment because it failed or was stopped.</p>
    #[serde(rename = "rollbackTriggeringDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_triggering_deployment_id: Option<String>,
}

/// <p>Information about the location of application artifacts stored in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Location {
    /// <p>The name of the Amazon S3 bucket where the application revision is stored.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p><p>The file type of the application revision. Must be one of the following:</p> <ul> <li> <p>tar: A tar archive file.</p> </li> <li> <p>tgz: A compressed tar archive file.</p> </li> <li> <p>zip: A zip archive file.</p> </li> </ul></p>
    #[serde(rename = "bundleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    /// <p>The ETag of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the ETag is not specified as an input parameter, ETag validation of the object is skipped.</p>
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The name of the Amazon S3 object that represents the bundled artifacts for the application revision.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A specific version of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the version is not specified, the system uses the most recent version by default.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SkipWaitTimeForInstanceTerminationInput {
    /// <p> The unique ID of a blue/green deployment for which you want to skip the instance termination wait time. </p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p> Represents the input of a StopDeployment operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDeploymentInput {
    /// <p> Indicates, when a deployment is stopped, whether instances that have been updated should be rolled back to the previous version of the application revision. </p>
    #[serde(rename = "autoRollbackEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_enabled: Option<bool>,
    /// <p> The unique ID of a deployment. </p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
}

/// <p> Represents the output of a StopDeployment operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopDeploymentOutput {
    /// <p><p>The status of the stop deployment operation:</p> <ul> <li> <p>Pending: The stop operation is pending.</p> </li> <li> <p>Succeeded: The stop operation was successful.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>An accompanying status message.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag's value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about an on-premises instance tag filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagFilter {
    /// <p>The on-premises instance tag filter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p><p>The on-premises instance tag filter type:</p> <ul> <li> <p>KEY<em>ONLY: Key only.</p> </li> <li> <p>VALUE</em>ONLY: Value only.</p> </li> <li> <p>KEY<em>AND</em>VALUE: Key and value.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The on-premises instance tag filter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p> The ARN of a resource, such as a CodeDeploy application or deployment group. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p> A list of tags that <code>TagResource</code> associates with a resource. The resource is identified by the <code>ResourceArn</code> input parameter. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>Information about a target group in Elastic Load Balancing to use in a deployment. Instances are registered as targets in a target group, and traffic is routed to the target group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetGroupInfo {
    /// <p>For blue/green deployments, the name of the target group that instances in the original environment are deregistered from, and instances in the replacement environment are registered with. For in-place deployments, the name of the target group that instances are deregistered from, so they are not serving traffic during a deployment, and then re-registered with after the deployment is complete. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p> Information about two target groups and how traffic is routed during an Amazon ECS deployment. An optional test traffic route can be specified. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetGroupPairInfo {
    /// <p> The path used by a load balancer to route production traffic when an Amazon ECS deployment is complete. </p>
    #[serde(rename = "prodTrafficRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_traffic_route: Option<TrafficRoute>,
    /// <p> One pair of target groups. One is associated with the original task set. The second is associated with the task set that serves traffic after the deployment is complete. </p>
    #[serde(rename = "targetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<Vec<TargetGroupInfo>>,
    /// <p> An optional path used by a load balancer to route test traffic after an Amazon ECS deployment. Validation can occur while test traffic is served during a deployment. </p>
    #[serde(rename = "testTrafficRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_traffic_route: Option<TrafficRoute>,
}

/// <p>Information about the instances to be used in the replacement environment in a blue/green deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetInstances {
    /// <p>The names of one or more Auto Scaling groups to identify a replacement environment for a blue/green deployment.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>Information about the groups of EC2 instance tags that an instance must be identified by in order for it to be included in the replacement environment for a blue/green deployment. Cannot be used in the same call as tagFilters.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
    /// <p>The tag filter key, type, and value used to identify Amazon EC2 instances in a replacement environment for a blue/green deployment. Cannot be used in the same call as ec2TagSet.</p>
    #[serde(rename = "tagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<EC2TagFilter>>,
}

/// <p>A configuration that shifts traffic from one version of a Lambda function to another in two increments. The original and target Lambda function versions are specified in the deployment's AppSpec file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeBasedCanary {
    /// <p>The number of minutes between the first and second traffic shifts of a <code>TimeBasedCanary</code> deployment.</p>
    #[serde(rename = "canaryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_interval: Option<i64>,
    /// <p>The percentage of traffic to shift in the first increment of a <code>TimeBasedCanary</code> deployment.</p>
    #[serde(rename = "canaryPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_percentage: Option<i64>,
}

/// <p>A configuration that shifts traffic from one version of a Lambda function to another in equal increments, with an equal number of minutes between each increment. The original and target Lambda function versions are specified in the deployment's AppSpec file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeBasedLinear {
    /// <p>The number of minutes between each incremental traffic shift of a <code>TimeBasedLinear</code> deployment.</p>
    #[serde(rename = "linearInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_interval: Option<i64>,
    /// <p>The percentage of traffic that is shifted at the start of each increment of a <code>TimeBasedLinear</code> deployment.</p>
    #[serde(rename = "linearPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_percentage: Option<i64>,
}

/// <p>Information about a time range.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimeRange {
    /// <p><p>The end time of the time range.</p> <note> <p>Specify null to leave the end time open-ended.</p> </note></p>
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// <p><p>The start time of the time range.</p> <note> <p>Specify null to leave the start time open-ended.</p> </note></p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p> Information about a listener. The listener contains the path used to route traffic that is received from the load balancer to a target group. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficRoute {
    /// <p> The ARN of one listener. The listener identifies the route between a target group and a load balancer. This is an array of strings with a maximum size of one. </p>
    #[serde(rename = "listenerArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arns: Option<Vec<String>>,
}

/// <p>The configuration that specifies how traffic is shifted from one version of a Lambda function to another version during an AWS Lambda deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficRoutingConfig {
    /// <p>A configuration that shifts traffic from one version of a Lambda function to another in two increments. The original and target Lambda function versions are specified in the deployment's AppSpec file.</p>
    #[serde(rename = "timeBasedCanary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_canary: Option<TimeBasedCanary>,
    /// <p>A configuration that shifts traffic from one version of a Lambda function to another in equal increments, with an equal number of minutes between each increment. The original and target Lambda function versions are specified in the deployment's AppSpec file.</p>
    #[serde(rename = "timeBasedLinear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_linear: Option<TimeBasedLinear>,
    /// <p>The type of traffic shifting (<code>TimeBasedCanary</code> or <code>TimeBasedLinear</code>) used by a deployment configuration .</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about notification triggers for the deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerConfig {
    /// <p>The event type or types for which notifications are triggered.</p>
    #[serde(rename = "triggerEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_events: Option<Vec<String>>,
    /// <p>The name of the notification trigger.</p>
    #[serde(rename = "triggerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    /// <p>The ARN of the Amazon Simple Notification Service topic through which notifications about deployment or instance events are sent.</p>
    #[serde(rename = "triggerTargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p> The ARN that specifies from which resource to disassociate the tags with the keys in the <code>TagKeys</code> input paramter. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p> A list of keys of <code>Tag</code> objects. The <code>Tag</code> objects identified by the keys are disassociated from the resource specified by the <code>ResourceArn</code> input parameter. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

/// <p>Represents the input of an UpdateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationInput {
    /// <p>The current name of the application you want to change.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>The new name to give the application.</p>
    #[serde(rename = "newApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_application_name: Option<String>,
}

/// <p>Represents the input of an UpdateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDeploymentGroupInput {
    /// <p>Information to add or change about Amazon CloudWatch alarms when the deployment group is updated.</p>
    #[serde(rename = "alarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// <p>The application name that corresponds to the deployment group to update.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Information for an automatic rollback configuration that is added or changed when a deployment group is updated.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>The replacement list of Auto Scaling groups to be included in the deployment group, if you want to change them. To keep the Auto Scaling groups, enter their names. To remove Auto Scaling groups, do not enter any Auto Scaling group names.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>Information about blue/green deployment options for a deployment group.</p>
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// <p>The current name of the deployment group.</p>
    #[serde(rename = "currentDeploymentGroupName")]
    pub current_deployment_group_name: String,
    /// <p>The replacement deployment configuration name to use, if you want to change it.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>Information about the type of deployment, either in-place or blue/green, you want to run and whether to route deployment traffic behind a load balancer.</p>
    #[serde(rename = "deploymentStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    /// <p>The replacement set of Amazon EC2 tags on which to filter, if you want to change them. To keep the existing tags, enter their names. To remove tags, do not enter any tag names.</p>
    #[serde(rename = "ec2TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_filters: Option<Vec<EC2TagFilter>>,
    /// <p>Information about groups of tags applied to on-premises instances. The deployment group includes only EC2 instances identified by all the tag groups.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
    /// <p> The target Amazon ECS services in the deployment group. This applies only to deployment groups that use the Amazon ECS compute platform. A target Amazon ECS service is specified as an Amazon ECS cluster and service name pair using the format <code>&lt;clustername&gt;:&lt;servicename&gt;</code>. </p>
    #[serde(rename = "ecsServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    /// <p>Information about the load balancer used in a deployment.</p>
    #[serde(rename = "loadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// <p>The new name of the deployment group, if you want to change it.</p>
    #[serde(rename = "newDeploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_deployment_group_name: Option<String>,
    /// <p>The replacement set of on-premises instance tags on which to filter, if you want to change them. To keep the existing tags, enter their names. To remove tags, do not enter any tag names.</p>
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    /// <p>Information about an on-premises instance tag set. The deployment group includes only on-premises instances identified by all the tag groups.</p>
    #[serde(rename = "onPremisesTagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// <p>A replacement ARN for the service role, if you want to change it.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>Information about triggers to change when the deployment group is updated. For examples, see <a href="https://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-edit.html">Modify Triggers in an AWS CodeDeploy Deployment Group</a> in the AWS CodeDeploy User Guide.</p>
    #[serde(rename = "triggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

/// <p>Represents the output of an UpdateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeploymentGroupOutput {
    /// <p>If the output contains no data, and the corresponding deployment group contained at least one Auto Scaling group, AWS CodeDeploy successfully removed all corresponding Auto Scaling lifecycle event hooks from the AWS account. If the output contains data, AWS CodeDeploy could not remove some Auto Scaling lifecycle event hooks from the AWS account.</p>
    #[serde(rename = "hooksNotCleanedUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_not_cleaned_up: Option<Vec<AutoScalingGroup>>,
}

/// Errors returned by AddTagsToOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum AddTagsToOnPremisesInstancesError {
    /// <p>The maximum number of allowed on-premises instances in a single call was exceeded.</p>
    InstanceLimitExceeded(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance is not registered.</p>
    InstanceNotRegistered(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
}

impl AddTagsToOnPremisesInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddTagsToOnPremisesInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceLimitExceededException" => {
                    return RusotoError::Service(
                        AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(err.msg),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(
                        AddTagsToOnPremisesInstancesError::InstanceNameRequired(err.msg),
                    )
                }
                "InstanceNotRegisteredException" => {
                    return RusotoError::Service(
                        AddTagsToOnPremisesInstancesError::InstanceNotRegistered(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        AddTagsToOnPremisesInstancesError::InvalidInstanceName(err.msg),
                    )
                }
                "InvalidTagException" => {
                    return RusotoError::Service(AddTagsToOnPremisesInstancesError::InvalidTag(
                        err.msg,
                    ))
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(
                        AddTagsToOnPremisesInstancesError::TagLimitExceeded(err.msg),
                    )
                }
                "TagRequiredException" => {
                    return RusotoError::Service(AddTagsToOnPremisesInstancesError::TagRequired(
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
impl fmt::Display for AddTagsToOnPremisesInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AddTagsToOnPremisesInstancesError::InstanceNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            AddTagsToOnPremisesInstancesError::InstanceNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            AddTagsToOnPremisesInstancesError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
            AddTagsToOnPremisesInstancesError::InvalidTag(ref cause) => write!(f, "{}", cause),
            AddTagsToOnPremisesInstancesError::TagLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AddTagsToOnPremisesInstancesError::TagRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToOnPremisesInstancesError {}
/// Errors returned by BatchGetApplicationRevisions
#[derive(Debug, PartialEq)]
pub enum BatchGetApplicationRevisionsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
}

impl BatchGetApplicationRevisionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchGetApplicationRevisionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::ApplicationNameRequired(err.msg),
                    )
                }
                "BatchLimitExceededException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::BatchLimitExceeded(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidRevisionException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::InvalidRevision(err.msg),
                    )
                }
                "RevisionRequiredException" => {
                    return RusotoError::Service(
                        BatchGetApplicationRevisionsError::RevisionRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetApplicationRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetApplicationRevisionsError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetApplicationRevisionsError::BatchLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetApplicationRevisionsError::InvalidApplicationName(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetApplicationRevisionsError::InvalidRevision(ref cause) => write!(f, "{}", cause),
            BatchGetApplicationRevisionsError::RevisionRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetApplicationRevisionsError {}
/// Errors returned by BatchGetApplications
#[derive(Debug, PartialEq)]
pub enum BatchGetApplicationsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
}

impl BatchGetApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetApplicationsError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        BatchGetApplicationsError::ApplicationNameRequired(err.msg),
                    )
                }
                "BatchLimitExceededException" => {
                    return RusotoError::Service(BatchGetApplicationsError::BatchLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(BatchGetApplicationsError::InvalidApplicationName(
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
impl fmt::Display for BatchGetApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetApplicationsError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            BatchGetApplicationsError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            BatchGetApplicationsError::BatchLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchGetApplicationsError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetApplicationsError {}
/// Errors returned by BatchGetDeploymentGroups
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentGroupsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
}

impl BatchGetDeploymentGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetDeploymentGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::ApplicationNameRequired(err.msg),
                    )
                }
                "BatchLimitExceededException" => {
                    return RusotoError::Service(BatchGetDeploymentGroupsError::BatchLimitExceeded(
                        err.msg,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetDeploymentGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentGroupsError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentGroupsError::BatchLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchGetDeploymentGroupsError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentGroupsError::InvalidApplicationName(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetDeploymentGroupsError {}
/// Errors returned by BatchGetDeploymentInstances
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentInstancesError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The instance ID was not specified.</p>
    InstanceIdRequired(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl BatchGetDeploymentInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchGetDeploymentInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchLimitExceededException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::BatchLimitExceeded(err.msg),
                    )
                }
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::DeploymentIdRequired(err.msg),
                    )
                }
                "InstanceIdRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::InstanceIdRequired(err.msg),
                    )
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::InvalidComputePlatform(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::InvalidDeploymentId(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentInstancesError::InvalidInstanceName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetDeploymentInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetDeploymentInstancesError::BatchLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::DeploymentIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::InstanceIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::InvalidComputePlatform(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::InvalidDeploymentId(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentInstancesError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetDeploymentInstancesError {}
/// Errors returned by BatchGetDeploymentTargets
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentTargetsError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p> The provided target ID does not belong to the attempted deployment. </p>
    DeploymentTargetDoesNotExist(String),
    /// <p> A deployment target ID was not provided. </p>
    DeploymentTargetIdRequired(String),
    /// <p> The maximum number of targets that can be associated with an Amazon ECS or AWS Lambda deployment was exceeded. The target list of both types of deployments must have exactly one item. This exception does not apply to EC2/On-premises deployments. </p>
    DeploymentTargetListSizeExceeded(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p> The target ID provided was not valid. </p>
    InvalidDeploymentTargetId(String),
}

impl BatchGetDeploymentTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetDeploymentTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentIdRequired(err.msg),
                    )
                }
                "DeploymentNotStartedException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentNotStarted(err.msg),
                    )
                }
                "DeploymentTargetDoesNotExistException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentTargetDoesNotExist(err.msg),
                    )
                }
                "DeploymentTargetIdRequiredException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentTargetIdRequired(err.msg),
                    )
                }
                "DeploymentTargetListSizeExceededException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::DeploymentTargetListSizeExceeded(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::InvalidDeploymentId(err.msg),
                    )
                }
                "InvalidDeploymentTargetIdException" => {
                    return RusotoError::Service(
                        BatchGetDeploymentTargetsError::InvalidDeploymentTargetId(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetDeploymentTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetDeploymentTargetsError::DeploymentDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::DeploymentIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::DeploymentNotStarted(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::DeploymentTargetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::DeploymentTargetIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::DeploymentTargetListSizeExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::InvalidDeploymentId(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetDeploymentTargetsError::InvalidDeploymentTargetId(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetDeploymentTargetsError {}
/// Errors returned by BatchGetDeployments
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentsError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
}

impl BatchGetDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetDeploymentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchLimitExceededException" => {
                    return RusotoError::Service(BatchGetDeploymentsError::BatchLimitExceeded(
                        err.msg,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(BatchGetDeploymentsError::DeploymentIdRequired(
                        err.msg,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(BatchGetDeploymentsError::InvalidDeploymentId(
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
impl fmt::Display for BatchGetDeploymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetDeploymentsError::BatchLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchGetDeploymentsError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            BatchGetDeploymentsError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetDeploymentsError {}
/// Errors returned by BatchGetOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum BatchGetOnPremisesInstancesError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl BatchGetOnPremisesInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchGetOnPremisesInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchLimitExceededException" => {
                    return RusotoError::Service(
                        BatchGetOnPremisesInstancesError::BatchLimitExceeded(err.msg),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(
                        BatchGetOnPremisesInstancesError::InstanceNameRequired(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        BatchGetOnPremisesInstancesError::InvalidInstanceName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetOnPremisesInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetOnPremisesInstancesError::BatchLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetOnPremisesInstancesError::InstanceNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchGetOnPremisesInstancesError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetOnPremisesInstancesError {}
/// Errors returned by ContinueDeployment
#[derive(Debug, PartialEq)]
pub enum ContinueDeploymentError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The deployment does not have a status of Ready and can't continue yet.</p>
    DeploymentIsNotInReadyState(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>The specified deployment status doesn't exist or cannot be determined.</p>
    InvalidDeploymentStatus(String),
    /// <p> The wait type is invalid. </p>
    InvalidDeploymentWaitType(String),
    /// <p>A call was submitted that is not supported for the specified deployment type.</p>
    UnsupportedActionForDeploymentType(String),
}

impl ContinueDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ContinueDeploymentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentAlreadyCompletedException" => {
                    return RusotoError::Service(
                        ContinueDeploymentError::DeploymentAlreadyCompleted(err.msg),
                    )
                }
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(ContinueDeploymentError::DeploymentDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(ContinueDeploymentError::DeploymentIdRequired(
                        err.msg,
                    ))
                }
                "DeploymentIsNotInReadyStateException" => {
                    return RusotoError::Service(
                        ContinueDeploymentError::DeploymentIsNotInReadyState(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(ContinueDeploymentError::InvalidDeploymentId(
                        err.msg,
                    ))
                }
                "InvalidDeploymentStatusException" => {
                    return RusotoError::Service(ContinueDeploymentError::InvalidDeploymentStatus(
                        err.msg,
                    ))
                }
                "InvalidDeploymentWaitTypeException" => {
                    return RusotoError::Service(
                        ContinueDeploymentError::InvalidDeploymentWaitType(err.msg),
                    )
                }
                "UnsupportedActionForDeploymentTypeException" => {
                    return RusotoError::Service(
                        ContinueDeploymentError::UnsupportedActionForDeploymentType(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ContinueDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContinueDeploymentError::DeploymentAlreadyCompleted(ref cause) => {
                write!(f, "{}", cause)
            }
            ContinueDeploymentError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            ContinueDeploymentError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            ContinueDeploymentError::DeploymentIsNotInReadyState(ref cause) => {
                write!(f, "{}", cause)
            }
            ContinueDeploymentError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
            ContinueDeploymentError::InvalidDeploymentStatus(ref cause) => write!(f, "{}", cause),
            ContinueDeploymentError::InvalidDeploymentWaitType(ref cause) => write!(f, "{}", cause),
            ContinueDeploymentError::UnsupportedActionForDeploymentType(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ContinueDeploymentError {}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>An application with the specified name with the IAM user or AWS account already exists.</p>
    ApplicationAlreadyExists(String),
    /// <p>More applications were attempted to be created than are allowed.</p>
    ApplicationLimitExceeded(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p> The specified tags are not valid. </p>
    InvalidTagsToAdd(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationAlreadyExistsException" => {
                    return RusotoError::Service(CreateApplicationError::ApplicationAlreadyExists(
                        err.msg,
                    ))
                }
                "ApplicationLimitExceededException" => {
                    return RusotoError::Service(CreateApplicationError::ApplicationLimitExceeded(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(CreateApplicationError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidComputePlatform(
                        err.msg,
                    ))
                }
                "InvalidTagsToAddException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidTagsToAdd(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::ApplicationAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ApplicationLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidComputePlatform(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidTagsToAdd(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The number of allowed deployments was exceeded.</p>
    DeploymentLimitExceeded(String),
    /// <p>The description is too long.</p>
    DescriptionTooLong(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled, but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid fileExistsBehavior option was specified to determine how AWS CodeDeploy handles files or directories that already exist in a deployment target location, but weren't part of the previous successful deployment. Valid values include "DISALLOW," "OVERWRITE," and "RETAIN."</p>
    InvalidFileExistsBehavior(String),
    /// <p>The GitHub token is not valid.</p>
    InvalidGitHubAccountToken(String),
    /// <p>The IgnoreApplicationStopFailures value is invalid. For AWS Lambda deployments, <code>false</code> is expected. For EC2/On-premises deployments, <code>true</code> or <code>false</code> is expected.</p>
    InvalidIgnoreApplicationStopFailuresValue(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Amazon EC2 Auto Scaling.</p>
    InvalidRole(String),
    /// <p><p>The target instance configuration is invalid. Possible causes include:</p> <ul> <li> <p>Configuration data for target instances was entered for an in-place deployment.</p> </li> <li> <p>The limit of 10 tags for a tag type was exceeded.</p> </li> <li> <p>The combined length of the tag names exceeded the limit. </p> </li> <li> <p>A specified tag is not currently applied to any instances.</p> </li> </ul></p>
    InvalidTargetInstances(String),
    /// <p>The UpdateOutdatedInstancesOnly value is invalid. For AWS Lambda deployments, <code>false</code> is expected. For EC2/On-premises deployments, <code>true</code> or <code>false</code> is expected.</p>
    InvalidUpdateOutdatedInstancesOnlyValue(String),
    /// <p>The named revision does not exist with the IAM user or AWS account.</p>
    RevisionDoesNotExist(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
    /// <p>An API function was called too frequently.</p>
    Throttling(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(CreateDeploymentError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(CreateDeploymentError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::DeploymentGroupDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "DeploymentLimitExceededException" => {
                    return RusotoError::Service(CreateDeploymentError::DeploymentLimitExceeded(
                        err.msg,
                    ))
                }
                "DescriptionTooLongException" => {
                    return RusotoError::Service(CreateDeploymentError::DescriptionTooLong(err.msg))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidAutoRollbackConfigException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidAutoRollbackConfig(
                        err.msg,
                    ))
                }
                "InvalidAutoScalingGroupException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidAutoScalingGroup(
                        err.msg,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidDeploymentGroupName(
                        err.msg,
                    ))
                }
                "InvalidFileExistsBehaviorException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidFileExistsBehavior(
                        err.msg,
                    ))
                }
                "InvalidGitHubAccountTokenException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidGitHubAccountToken(
                        err.msg,
                    ))
                }
                "InvalidIgnoreApplicationStopFailuresValueException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::InvalidIgnoreApplicationStopFailuresValue(err.msg),
                    )
                }
                "InvalidLoadBalancerInfoException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidLoadBalancerInfo(
                        err.msg,
                    ))
                }
                "InvalidRevisionException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidRevision(err.msg))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidRole(err.msg))
                }
                "InvalidTargetInstancesException" => {
                    return RusotoError::Service(CreateDeploymentError::InvalidTargetInstances(
                        err.msg,
                    ))
                }
                "InvalidUpdateOutdatedInstancesOnlyValueException" => {
                    return RusotoError::Service(
                        CreateDeploymentError::InvalidUpdateOutdatedInstancesOnlyValue(err.msg),
                    )
                }
                "RevisionDoesNotExistException" => {
                    return RusotoError::Service(CreateDeploymentError::RevisionDoesNotExist(
                        err.msg,
                    ))
                }
                "RevisionRequiredException" => {
                    return RusotoError::Service(CreateDeploymentError::RevisionRequired(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDeploymentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentError::DeploymentGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::DeploymentGroupNameRequired(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::DeploymentLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::DescriptionTooLong(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidAutoRollbackConfig(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidAutoScalingGroup(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidDeploymentConfigName(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidDeploymentGroupName(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidFileExistsBehavior(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidGitHubAccountToken(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidIgnoreApplicationStopFailuresValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentError::InvalidLoadBalancerInfo(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidRevision(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidTargetInstances(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::InvalidUpdateOutdatedInstancesOnlyValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentError::RevisionDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::RevisionRequired(ref cause) => write!(f, "{}", cause),
            CreateDeploymentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentError {}
/// Errors returned by CreateDeploymentConfig
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentConfigError {
    /// <p>A deployment configuration with the specified name with the IAM user or AWS account already exists .</p>
    DeploymentConfigAlreadyExists(String),
    /// <p>The deployment configurations limit was exceeded.</p>
    DeploymentConfigLimitExceeded(String),
    /// <p>The deployment configuration name was not specified.</p>
    DeploymentConfigNameRequired(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The minimum healthy instance value was specified in an invalid format.</p>
    InvalidMinimumHealthyHostValue(String),
    /// <p> The configuration that specifies how traffic is routed during a deployment is invalid.</p>
    InvalidTrafficRoutingConfiguration(String),
}

impl CreateDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentConfigAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::DeploymentConfigAlreadyExists(err.msg),
                    )
                }
                "DeploymentConfigLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::DeploymentConfigLimitExceeded(err.msg),
                    )
                }
                "DeploymentConfigNameRequiredException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::DeploymentConfigNameRequired(err.msg),
                    )
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::InvalidComputePlatform(err.msg),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "InvalidMinimumHealthyHostValueException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(err.msg),
                    )
                }
                "InvalidTrafficRoutingConfigurationException" => {
                    return RusotoError::Service(
                        CreateDeploymentConfigError::InvalidTrafficRoutingConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentConfigError::DeploymentConfigAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::DeploymentConfigLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::InvalidComputePlatform(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentConfigError::InvalidTrafficRoutingConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDeploymentConfigError {}
/// Errors returned by CreateDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentGroupError {
    /// <p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
    AlarmsLimitExceeded(String),
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>A deployment group with the specified name with the IAM user or AWS account already exists.</p>
    DeploymentGroupAlreadyExists(String),
    /// <p> The deployment groups limit was exceeded.</p>
    DeploymentGroupLimitExceeded(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p> The Amazon ECS service is associated with more than one deployment groups. An Amazon ECS service can be associated with only one deployment group. </p>
    ECSServiceMappingLimitExceeded(String),
    /// <p><p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the limit of 255 characters.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled, but the alarm list is empty.</p> </li> </ul></p>
    InvalidAlarmConfig(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled, but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The configuration for the blue/green deployment group was provided in an invalid format. For information about deployment configuration format, see <a>CreateDeploymentConfig</a>.</p>
    InvalidBlueGreenDeploymentConfiguration(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid deployment style was specified. Valid deployment types include "IN_PLACE" and "BLUE_GREEN." Valid deployment options include "WITH_TRAFFIC_CONTROL" and "WITHOUT_TRAFFIC_CONTROL."</p>
    InvalidDeploymentStyle(String),
    /// <p>A call was submitted that specified both Ec2TagFilters and Ec2TagSet, but only one of these data types can be used in a single call.</p>
    InvalidEC2TagCombination(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidEC2Tag(String),
    /// <p> The Amazon ECS service identifier is not valid. </p>
    InvalidECSService(String),
    /// <p>The input was specified in an invalid format.</p>
    InvalidInput(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>A call was submitted that specified both OnPremisesTagFilters and OnPremisesTagSet, but only one of these data types can be used in a single call.</p>
    InvalidOnPremisesTagCombination(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Amazon EC2 Auto Scaling.</p>
    InvalidRole(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p> The specified tags are not valid. </p>
    InvalidTagsToAdd(String),
    /// <p> A target group pair associated with this deployment is not valid. </p>
    InvalidTargetGroupPair(String),
    /// <p>The trigger was specified in an invalid format.</p>
    InvalidTriggerConfig(String),
    /// <p>The limit for lifecycle hooks was exceeded.</p>
    LifecycleHookLimitExceeded(String),
    /// <p>The role ID was not specified.</p>
    RoleRequired(String),
    /// <p>The number of tag groups included in the tag set list exceeded the maximum allowed limit of 3.</p>
    TagSetListLimitExceeded(String),
    /// <p>An API function was called too frequently.</p>
    Throttling(String),
    /// <p>The maximum allowed number of triggers was exceeded.</p>
    TriggerTargetsLimitExceeded(String),
}

impl CreateDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlarmsLimitExceededException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::AlarmsLimitExceeded(
                        err.msg,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::ApplicationNameRequired(err.msg),
                    )
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::DeploymentGroupAlreadyExists(err.msg),
                    )
                }
                "DeploymentGroupLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::DeploymentGroupLimitExceeded(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "ECSServiceMappingLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::ECSServiceMappingLimitExceeded(err.msg),
                    )
                }
                "InvalidAlarmConfigException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidAlarmConfig(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidAutoRollbackConfigException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidAutoRollbackConfig(err.msg),
                    )
                }
                "InvalidAutoScalingGroupException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidAutoScalingGroup(err.msg),
                    )
                }
                "InvalidBlueGreenDeploymentConfigurationException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(
                            err.msg,
                        ),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidDeploymentGroupName(err.msg),
                    )
                }
                "InvalidDeploymentStyleException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidDeploymentStyle(err.msg),
                    )
                }
                "InvalidEC2TagCombinationException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidEC2TagCombination(err.msg),
                    )
                }
                "InvalidEC2TagException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidEC2Tag(err.msg))
                }
                "InvalidECSServiceException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidECSService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidInput(err.msg))
                }
                "InvalidLoadBalancerInfoException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidLoadBalancerInfo(err.msg),
                    )
                }
                "InvalidOnPremisesTagCombinationException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidOnPremisesTagCombination(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidRole(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidTag(err.msg))
                }
                "InvalidTagsToAddException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidTagsToAdd(
                        err.msg,
                    ))
                }
                "InvalidTargetGroupPairException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::InvalidTargetGroupPair(err.msg),
                    )
                }
                "InvalidTriggerConfigException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::InvalidTriggerConfig(
                        err.msg,
                    ))
                }
                "LifecycleHookLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::LifecycleHookLimitExceeded(err.msg),
                    )
                }
                "RoleRequiredException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::RoleRequired(err.msg))
                }
                "TagSetListLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::TagSetListLimitExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDeploymentGroupError::Throttling(err.msg))
                }
                "TriggerTargetsLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeploymentGroupError::TriggerTargetsLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeploymentGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::DeploymentGroupLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::ECSServiceMappingLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidAlarmConfig(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidDeploymentStyle(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidEC2TagCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidEC2Tag(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidECSService(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidLoadBalancerInfo(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidOnPremisesTagCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::InvalidRole(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidTag(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidTagsToAdd(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidTargetGroupPair(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::InvalidTriggerConfig(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::RoleRequired(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::TagSetListLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeploymentGroupError::Throttling(ref cause) => write!(f, "{}", cause),
            CreateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDeploymentGroupError {}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Amazon EC2 Auto Scaling.</p>
    InvalidRole(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(DeleteApplicationError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(DeleteApplicationError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(DeleteApplicationError::InvalidRole(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InvalidRole(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
/// Errors returned by DeleteDeploymentConfig
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentConfigError {
    /// <p>The deployment configuration is still in use.</p>
    DeploymentConfigInUse(String),
    /// <p>The deployment configuration name was not specified.</p>
    DeploymentConfigNameRequired(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>An invalid operation was detected.</p>
    InvalidOperation(String),
}

impl DeleteDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeploymentConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentConfigInUseException" => {
                    return RusotoError::Service(
                        DeleteDeploymentConfigError::DeploymentConfigInUse(err.msg),
                    )
                }
                "DeploymentConfigNameRequiredException" => {
                    return RusotoError::Service(
                        DeleteDeploymentConfigError::DeploymentConfigNameRequired(err.msg),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        DeleteDeploymentConfigError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "InvalidOperationException" => {
                    return RusotoError::Service(DeleteDeploymentConfigError::InvalidOperation(
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
impl fmt::Display for DeleteDeploymentConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeploymentConfigError::DeploymentConfigInUse(ref cause) => write!(f, "{}", cause),
            DeleteDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeploymentConfigError::InvalidOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeploymentConfigError {}
/// Errors returned by DeleteDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentGroupError {
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Amazon EC2 Auto Scaling.</p>
    InvalidRole(String),
}

impl DeleteDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeploymentGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        DeleteDeploymentGroupError::ApplicationNameRequired(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        DeleteDeploymentGroupError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        DeleteDeploymentGroupError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(
                        DeleteDeploymentGroupError::InvalidDeploymentGroupName(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(DeleteDeploymentGroupError::InvalidRole(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDeploymentGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeploymentGroupError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeploymentGroupError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            DeleteDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeploymentGroupError::InvalidRole(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeploymentGroupError {}
/// Errors returned by DeleteGitHubAccountToken
#[derive(Debug, PartialEq)]
pub enum DeleteGitHubAccountTokenError {
    /// <p>No GitHub account connection exists with the named specified in the call.</p>
    GitHubAccountTokenDoesNotExist(String),
    /// <p>The call is missing a required GitHub account connection name.</p>
    GitHubAccountTokenNameRequired(String),
    /// <p>The format of the specified GitHub account connection name is invalid.</p>
    InvalidGitHubAccountTokenName(String),
    /// <p>The API used does not support the deployment.</p>
    OperationNotSupported(String),
    /// <p>The specified resource could not be validated.</p>
    ResourceValidation(String),
}

impl DeleteGitHubAccountTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGitHubAccountTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GitHubAccountTokenDoesNotExistException" => {
                    return RusotoError::Service(
                        DeleteGitHubAccountTokenError::GitHubAccountTokenDoesNotExist(err.msg),
                    )
                }
                "GitHubAccountTokenNameRequiredException" => {
                    return RusotoError::Service(
                        DeleteGitHubAccountTokenError::GitHubAccountTokenNameRequired(err.msg),
                    )
                }
                "InvalidGitHubAccountTokenNameException" => {
                    return RusotoError::Service(
                        DeleteGitHubAccountTokenError::InvalidGitHubAccountTokenName(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        DeleteGitHubAccountTokenError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceValidationException" => {
                    return RusotoError::Service(DeleteGitHubAccountTokenError::ResourceValidation(
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
impl fmt::Display for DeleteGitHubAccountTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGitHubAccountTokenError::GitHubAccountTokenDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGitHubAccountTokenError::GitHubAccountTokenNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGitHubAccountTokenError::InvalidGitHubAccountTokenName(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGitHubAccountTokenError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGitHubAccountTokenError::ResourceValidation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGitHubAccountTokenError {}
/// Errors returned by DeregisterOnPremisesInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterOnPremisesInstanceError {
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl DeregisterOnPremisesInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterOnPremisesInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(
                        DeregisterOnPremisesInstanceError::InstanceNameRequired(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        DeregisterOnPremisesInstanceError::InvalidInstanceName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterOnPremisesInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeregisterOnPremisesInstanceError {}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
}

impl GetApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(GetApplicationError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(GetApplicationError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(GetApplicationError::InvalidApplicationName(
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
impl fmt::Display for GetApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetApplicationError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            GetApplicationError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationError {}
/// Errors returned by GetApplicationRevision
#[derive(Debug, PartialEq)]
pub enum GetApplicationRevisionError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The named revision does not exist with the IAM user or AWS account.</p>
    RevisionDoesNotExist(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
}

impl GetApplicationRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationRevisionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        GetApplicationRevisionError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        GetApplicationRevisionError::ApplicationNameRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        GetApplicationRevisionError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidRevisionException" => {
                    return RusotoError::Service(GetApplicationRevisionError::InvalidRevision(
                        err.msg,
                    ))
                }
                "RevisionDoesNotExistException" => {
                    return RusotoError::Service(GetApplicationRevisionError::RevisionDoesNotExist(
                        err.msg,
                    ))
                }
                "RevisionRequiredException" => {
                    return RusotoError::Service(GetApplicationRevisionError::RevisionRequired(
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
impl fmt::Display for GetApplicationRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationRevisionError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApplicationRevisionError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApplicationRevisionError::InvalidApplicationName(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApplicationRevisionError::InvalidRevision(ref cause) => write!(f, "{}", cause),
            GetApplicationRevisionError::RevisionDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetApplicationRevisionError::RevisionRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationRevisionError {}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
}

impl GetDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(GetDeploymentError::DeploymentDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(GetDeploymentError::DeploymentIdRequired(err.msg))
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(GetDeploymentError::InvalidDeploymentId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDeploymentError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            GetDeploymentError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentError {}
/// Errors returned by GetDeploymentConfig
#[derive(Debug, PartialEq)]
pub enum GetDeploymentConfigError {
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The deployment configuration name was not specified.</p>
    DeploymentConfigNameRequired(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
}

impl GetDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        GetDeploymentConfigError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentConfigNameRequiredException" => {
                    return RusotoError::Service(
                        GetDeploymentConfigError::DeploymentConfigNameRequired(err.msg),
                    )
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(GetDeploymentConfigError::InvalidComputePlatform(
                        err.msg,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        GetDeploymentConfigError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentConfigError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentConfigError::InvalidComputePlatform(ref cause) => write!(f, "{}", cause),
            GetDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDeploymentConfigError {}
/// Errors returned by GetDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum GetDeploymentGroupError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
}

impl GetDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(GetDeploymentGroupError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(GetDeploymentGroupError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        GetDeploymentGroupError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(
                        GetDeploymentGroupError::DeploymentGroupDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        GetDeploymentGroupError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(GetDeploymentGroupError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(
                        GetDeploymentGroupError::InvalidDeploymentGroupName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeploymentGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentGroupError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDeploymentGroupError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            GetDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentGroupError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            GetDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDeploymentGroupError {}
/// Errors returned by GetDeploymentInstance
#[derive(Debug, PartialEq)]
pub enum GetDeploymentInstanceError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified instance does not exist in the deployment group.</p>
    InstanceDoesNotExist(String),
    /// <p>The instance ID was not specified.</p>
    InstanceIdRequired(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl GetDeploymentInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        GetDeploymentInstanceError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(GetDeploymentInstanceError::DeploymentIdRequired(
                        err.msg,
                    ))
                }
                "InstanceDoesNotExistException" => {
                    return RusotoError::Service(GetDeploymentInstanceError::InstanceDoesNotExist(
                        err.msg,
                    ))
                }
                "InstanceIdRequiredException" => {
                    return RusotoError::Service(GetDeploymentInstanceError::InstanceIdRequired(
                        err.msg,
                    ))
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(
                        GetDeploymentInstanceError::InvalidComputePlatform(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(GetDeploymentInstanceError::InvalidDeploymentId(
                        err.msg,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(GetDeploymentInstanceError::InvalidInstanceName(
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
impl fmt::Display for GetDeploymentInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentInstanceError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::InstanceDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::InstanceIdRequired(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::InvalidComputePlatform(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
            GetDeploymentInstanceError::InvalidInstanceName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentInstanceError {}
/// Errors returned by GetDeploymentTarget
#[derive(Debug, PartialEq)]
pub enum GetDeploymentTargetError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p> The provided target ID does not belong to the attempted deployment. </p>
    DeploymentTargetDoesNotExist(String),
    /// <p> A deployment target ID was not provided. </p>
    DeploymentTargetIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p> The target ID provided was not valid. </p>
    InvalidDeploymentTargetId(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl GetDeploymentTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(GetDeploymentTargetError::DeploymentDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(GetDeploymentTargetError::DeploymentIdRequired(
                        err.msg,
                    ))
                }
                "DeploymentNotStartedException" => {
                    return RusotoError::Service(GetDeploymentTargetError::DeploymentNotStarted(
                        err.msg,
                    ))
                }
                "DeploymentTargetDoesNotExistException" => {
                    return RusotoError::Service(
                        GetDeploymentTargetError::DeploymentTargetDoesNotExist(err.msg),
                    )
                }
                "DeploymentTargetIdRequiredException" => {
                    return RusotoError::Service(
                        GetDeploymentTargetError::DeploymentTargetIdRequired(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(GetDeploymentTargetError::InvalidDeploymentId(
                        err.msg,
                    ))
                }
                "InvalidDeploymentTargetIdException" => {
                    return RusotoError::Service(
                        GetDeploymentTargetError::InvalidDeploymentTargetId(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(GetDeploymentTargetError::InvalidInstanceName(
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
impl fmt::Display for GetDeploymentTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeploymentTargetError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetDeploymentTargetError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            GetDeploymentTargetError::DeploymentNotStarted(ref cause) => write!(f, "{}", cause),
            GetDeploymentTargetError::DeploymentTargetDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentTargetError::DeploymentTargetIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentTargetError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
            GetDeploymentTargetError::InvalidDeploymentTargetId(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeploymentTargetError::InvalidInstanceName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeploymentTargetError {}
/// Errors returned by GetOnPremisesInstance
#[derive(Debug, PartialEq)]
pub enum GetOnPremisesInstanceError {
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance is not registered.</p>
    InstanceNotRegistered(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
}

impl GetOnPremisesInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOnPremisesInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(GetOnPremisesInstanceError::InstanceNameRequired(
                        err.msg,
                    ))
                }
                "InstanceNotRegisteredException" => {
                    return RusotoError::Service(GetOnPremisesInstanceError::InstanceNotRegistered(
                        err.msg,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(GetOnPremisesInstanceError::InvalidInstanceName(
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
impl fmt::Display for GetOnPremisesInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOnPremisesInstanceError::InstanceNameRequired(ref cause) => write!(f, "{}", cause),
            GetOnPremisesInstanceError::InstanceNotRegistered(ref cause) => write!(f, "{}", cause),
            GetOnPremisesInstanceError::InvalidInstanceName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOnPremisesInstanceError {}
/// Errors returned by ListApplicationRevisions
#[derive(Debug, PartialEq)]
pub enum ListApplicationRevisionsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>A bucket name is required, but was not provided.</p>
    BucketNameFilterRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The bucket name either doesn't exist or was specified in an invalid format.</p>
    InvalidBucketNameFilter(String),
    /// <p>The deployed state filter was specified in an invalid format.</p>
    InvalidDeployedStateFilter(String),
    /// <p>The specified key prefix filter was specified in an invalid format.</p>
    InvalidKeyPrefixFilter(String),
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The column name to sort by is either not present or was specified in an invalid format.</p>
    InvalidSortBy(String),
    /// <p>The sort order was specified in an invalid format.</p>
    InvalidSortOrder(String),
}

impl ListApplicationRevisionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationRevisionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::ApplicationNameRequired(err.msg),
                    )
                }
                "BucketNameFilterRequiredException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::BucketNameFilterRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidBucketNameFilterException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::InvalidBucketNameFilter(err.msg),
                    )
                }
                "InvalidDeployedStateFilterException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::InvalidDeployedStateFilter(err.msg),
                    )
                }
                "InvalidKeyPrefixFilterException" => {
                    return RusotoError::Service(
                        ListApplicationRevisionsError::InvalidKeyPrefixFilter(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListApplicationRevisionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidSortByException" => {
                    return RusotoError::Service(ListApplicationRevisionsError::InvalidSortBy(
                        err.msg,
                    ))
                }
                "InvalidSortOrderException" => {
                    return RusotoError::Service(ListApplicationRevisionsError::InvalidSortOrder(
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
impl fmt::Display for ListApplicationRevisionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::BucketNameFilterRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::InvalidApplicationName(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::InvalidBucketNameFilter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::InvalidDeployedStateFilter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::InvalidKeyPrefixFilter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListApplicationRevisionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListApplicationRevisionsError::InvalidSortBy(ref cause) => write!(f, "{}", cause),
            ListApplicationRevisionsError::InvalidSortOrder(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationRevisionsError {}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListApplicationsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
/// Errors returned by ListDeploymentConfigs
#[derive(Debug, PartialEq)]
pub enum ListDeploymentConfigsError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
}

impl ListDeploymentConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDeploymentConfigsError::InvalidNextToken(
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
impl fmt::Display for ListDeploymentConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentConfigsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentConfigsError {}
/// Errors returned by ListDeploymentGroups
#[derive(Debug, PartialEq)]
pub enum ListDeploymentGroupsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
}

impl ListDeploymentGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        ListDeploymentGroupsError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        ListDeploymentGroupsError::ApplicationNameRequired(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(ListDeploymentGroupsError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDeploymentGroupsError::InvalidNextToken(
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
impl fmt::Display for ListDeploymentGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListDeploymentGroupsError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            ListDeploymentGroupsError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            ListDeploymentGroupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentGroupsError {}
/// Errors returned by ListDeploymentInstances
#[derive(Debug, PartialEq)]
pub enum ListDeploymentInstancesError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>An instance type was specified for an in-place deployment. Instance types are supported for blue/green deployments only.</p>
    InvalidDeploymentInstanceType(String),
    /// <p>The specified instance status does not exist.</p>
    InvalidInstanceStatus(String),
    /// <p>An invalid instance type was specified for instances in a blue/green deployment. Valid values include "Blue" for an original environment and "Green" for a replacement environment.</p>
    InvalidInstanceType(String),
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p> The target filter name is invalid. </p>
    InvalidTargetFilterName(String),
}

impl ListDeploymentInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::DeploymentIdRequired(err.msg),
                    )
                }
                "DeploymentNotStartedException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::DeploymentNotStarted(err.msg),
                    )
                }
                "InvalidComputePlatformException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::InvalidComputePlatform(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(ListDeploymentInstancesError::InvalidDeploymentId(
                        err.msg,
                    ))
                }
                "InvalidDeploymentInstanceTypeException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::InvalidDeploymentInstanceType(err.msg),
                    )
                }
                "InvalidInstanceStatusException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::InvalidInstanceStatus(err.msg),
                    )
                }
                "InvalidInstanceTypeException" => {
                    return RusotoError::Service(ListDeploymentInstancesError::InvalidInstanceType(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDeploymentInstancesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidTargetFilterNameException" => {
                    return RusotoError::Service(
                        ListDeploymentInstancesError::InvalidTargetFilterName(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeploymentInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDeploymentInstancesError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            ListDeploymentInstancesError::DeploymentNotStarted(ref cause) => write!(f, "{}", cause),
            ListDeploymentInstancesError::InvalidComputePlatform(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDeploymentInstancesError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
            ListDeploymentInstancesError::InvalidDeploymentInstanceType(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDeploymentInstancesError::InvalidInstanceStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDeploymentInstancesError::InvalidInstanceType(ref cause) => write!(f, "{}", cause),
            ListDeploymentInstancesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDeploymentInstancesError::InvalidTargetFilterName(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDeploymentInstancesError {}
/// Errors returned by ListDeploymentTargets
#[derive(Debug, PartialEq)]
pub enum ListDeploymentTargetsError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>An instance type was specified for an in-place deployment. Instance types are supported for blue/green deployments only.</p>
    InvalidDeploymentInstanceType(String),
    /// <p>The specified instance status does not exist.</p>
    InvalidInstanceStatus(String),
    /// <p>An invalid instance type was specified for instances in a blue/green deployment. Valid values include "Blue" for an original environment and "Green" for a replacement environment.</p>
    InvalidInstanceType(String),
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
}

impl ListDeploymentTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        ListDeploymentTargetsError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::DeploymentIdRequired(
                        err.msg,
                    ))
                }
                "DeploymentNotStartedException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::DeploymentNotStarted(
                        err.msg,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::InvalidDeploymentId(
                        err.msg,
                    ))
                }
                "InvalidDeploymentInstanceTypeException" => {
                    return RusotoError::Service(
                        ListDeploymentTargetsError::InvalidDeploymentInstanceType(err.msg),
                    )
                }
                "InvalidInstanceStatusException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::InvalidInstanceStatus(
                        err.msg,
                    ))
                }
                "InvalidInstanceTypeException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::InvalidInstanceType(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDeploymentTargetsError::InvalidNextToken(
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
impl fmt::Display for ListDeploymentTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentTargetsError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::DeploymentNotStarted(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::InvalidDeploymentInstanceType(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDeploymentTargetsError::InvalidInstanceStatus(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::InvalidInstanceType(ref cause) => write!(f, "{}", cause),
            ListDeploymentTargetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentTargetsError {}
/// Errors returned by ListDeployments
#[derive(Debug, PartialEq)]
pub enum ListDeploymentsError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>The specified deployment status doesn't exist or cannot be determined.</p>
    InvalidDeploymentStatus(String),
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The specified time range was specified in an invalid format.</p>
    InvalidTimeRange(String),
}

impl ListDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeploymentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(ListDeploymentsError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(ListDeploymentsError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(ListDeploymentsError::DeploymentGroupDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(ListDeploymentsError::DeploymentGroupNameRequired(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(ListDeploymentsError::InvalidApplicationName(
                        err.msg,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(ListDeploymentsError::InvalidDeploymentGroupName(
                        err.msg,
                    ))
                }
                "InvalidDeploymentStatusException" => {
                    return RusotoError::Service(ListDeploymentsError::InvalidDeploymentStatus(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDeploymentsError::InvalidNextToken(err.msg))
                }
                "InvalidTimeRangeException" => {
                    return RusotoError::Service(ListDeploymentsError::InvalidTimeRange(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeploymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeploymentsError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::DeploymentGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::DeploymentGroupNameRequired(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InvalidDeploymentGroupName(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InvalidDeploymentStatus(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDeploymentsError::InvalidTimeRange(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDeploymentsError {}
/// Errors returned by ListGitHubAccountTokenNames
#[derive(Debug, PartialEq)]
pub enum ListGitHubAccountTokenNamesError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The API used does not support the deployment.</p>
    OperationNotSupported(String),
    /// <p>The specified resource could not be validated.</p>
    ResourceValidation(String),
}

impl ListGitHubAccountTokenNamesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListGitHubAccountTokenNamesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListGitHubAccountTokenNamesError::InvalidNextToken(err.msg),
                    )
                }
                "OperationNotSupportedException" => {
                    return RusotoError::Service(
                        ListGitHubAccountTokenNamesError::OperationNotSupported(err.msg),
                    )
                }
                "ResourceValidationException" => {
                    return RusotoError::Service(
                        ListGitHubAccountTokenNamesError::ResourceValidation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGitHubAccountTokenNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGitHubAccountTokenNamesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListGitHubAccountTokenNamesError::OperationNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            ListGitHubAccountTokenNamesError::ResourceValidation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListGitHubAccountTokenNamesError {}
/// Errors returned by ListOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum ListOnPremisesInstancesError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The registration status was specified in an invalid format.</p>
    InvalidRegistrationStatus(String),
    /// <p>The tag filter was specified in an invalid format.</p>
    InvalidTagFilter(String),
}

impl ListOnPremisesInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOnPremisesInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListOnPremisesInstancesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidRegistrationStatusException" => {
                    return RusotoError::Service(
                        ListOnPremisesInstancesError::InvalidRegistrationStatus(err.msg),
                    )
                }
                "InvalidTagFilterException" => {
                    return RusotoError::Service(ListOnPremisesInstancesError::InvalidTagFilter(
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
impl fmt::Display for ListOnPremisesInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOnPremisesInstancesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListOnPremisesInstancesError::InvalidRegistrationStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOnPremisesInstancesError::InvalidTagFilter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOnPremisesInstancesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> The specified ARN is not supported. For example, it might be an ARN for a resource that is not expected. </p>
    ArnNotSupported(String),
    /// <p> The specified ARN is not in a valid format. </p>
    InvalidArn(String),
    /// <p> The ARN of a resource is required, but was not found. </p>
    ResourceArnRequired(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ArnNotSupportedException" => {
                    return RusotoError::Service(ListTagsForResourceError::ArnNotSupported(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArn(err.msg))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceArnRequired(
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
            ListTagsForResourceError::ArnNotSupported(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutLifecycleEventHookExecutionStatus
#[derive(Debug, PartialEq)]
pub enum PutLifecycleEventHookExecutionStatusError {
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>A lifecycle event hook is invalid. Review the <code>hooks</code> section in your AppSpec file to ensure the lifecycle events and <code>hooks</code> functions are valid.</p>
    InvalidLifecycleEventHookExecutionId(String),
    /// <p>The result of a Lambda validation function that verifies a lifecycle event is invalid. It should return <code>Succeeded</code> or <code>Failed</code>.</p>
    InvalidLifecycleEventHookExecutionStatus(String),
    /// <p>An attempt to return the status of an already completed lifecycle event occurred.</p>
    LifecycleEventAlreadyCompleted(String),
    /// <p>A call was submitted that is not supported for the specified deployment type.</p>
    UnsupportedActionForDeploymentType(String),
}

impl PutLifecycleEventHookExecutionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutLifecycleEventHookExecutionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "DeploymentDoesNotExistException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::DeploymentDoesNotExist(err.msg)),
"DeploymentIdRequiredException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::DeploymentIdRequired(err.msg)),
"InvalidDeploymentIdException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::InvalidDeploymentId(err.msg)),
"InvalidLifecycleEventHookExecutionIdException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionId(err.msg)),
"InvalidLifecycleEventHookExecutionStatusException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionStatus(err.msg)),
"LifecycleEventAlreadyCompletedException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::LifecycleEventAlreadyCompleted(err.msg)),
"UnsupportedActionForDeploymentTypeException" => return RusotoError::Service(PutLifecycleEventHookExecutionStatusError::UnsupportedActionForDeploymentType(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutLifecycleEventHookExecutionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLifecycleEventHookExecutionStatusError::DeploymentDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            PutLifecycleEventHookExecutionStatusError::DeploymentIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            PutLifecycleEventHookExecutionStatusError::InvalidDeploymentId(ref cause) => {
                write!(f, "{}", cause)
            }
            PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionId(
                ref cause,
            ) => write!(f, "{}", cause),
            PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionStatus(
                ref cause,
            ) => write!(f, "{}", cause),
            PutLifecycleEventHookExecutionStatusError::LifecycleEventAlreadyCompleted(
                ref cause,
            ) => write!(f, "{}", cause),
            PutLifecycleEventHookExecutionStatusError::UnsupportedActionForDeploymentType(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLifecycleEventHookExecutionStatusError {}
/// Errors returned by RegisterApplicationRevision
#[derive(Debug, PartialEq)]
pub enum RegisterApplicationRevisionError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The description is too long.</p>
    DescriptionTooLong(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
}

impl RegisterApplicationRevisionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterApplicationRevisionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        RegisterApplicationRevisionError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        RegisterApplicationRevisionError::ApplicationNameRequired(err.msg),
                    )
                }
                "DescriptionTooLongException" => {
                    return RusotoError::Service(
                        RegisterApplicationRevisionError::DescriptionTooLong(err.msg),
                    )
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        RegisterApplicationRevisionError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidRevisionException" => {
                    return RusotoError::Service(RegisterApplicationRevisionError::InvalidRevision(
                        err.msg,
                    ))
                }
                "RevisionRequiredException" => {
                    return RusotoError::Service(
                        RegisterApplicationRevisionError::RevisionRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterApplicationRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterApplicationRevisionError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterApplicationRevisionError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterApplicationRevisionError::DescriptionTooLong(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterApplicationRevisionError::InvalidApplicationName(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterApplicationRevisionError::InvalidRevision(ref cause) => write!(f, "{}", cause),
            RegisterApplicationRevisionError::RevisionRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterApplicationRevisionError {}
/// Errors returned by RegisterOnPremisesInstance
#[derive(Debug, PartialEq)]
pub enum RegisterOnPremisesInstanceError {
    /// <p>No IAM ARN was included in the request. You must use an IAM session ARN or IAM user ARN in the request.</p>
    IamArnRequired(String),
    /// <p>The request included an IAM session ARN that has already been used to register a different instance.</p>
    IamSessionArnAlreadyRegistered(String),
    /// <p>The specified IAM user ARN is already registered with an on-premises instance.</p>
    IamUserArnAlreadyRegistered(String),
    /// <p>An IAM user ARN was not specified.</p>
    IamUserArnRequired(String),
    /// <p>The specified on-premises instance name is already registered.</p>
    InstanceNameAlreadyRegistered(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The IAM session ARN was specified in an invalid format.</p>
    InvalidIamSessionArn(String),
    /// <p>The IAM user ARN was specified in an invalid format.</p>
    InvalidIamUserArn(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>Both an IAM user ARN and an IAM session ARN were included in the request. Use only one ARN type.</p>
    MultipleIamArnsProvided(String),
}

impl RegisterOnPremisesInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterOnPremisesInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IamArnRequiredException" => {
                    return RusotoError::Service(RegisterOnPremisesInstanceError::IamArnRequired(
                        err.msg,
                    ))
                }
                "IamSessionArnAlreadyRegisteredException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(err.msg),
                    )
                }
                "IamUserArnAlreadyRegisteredException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(err.msg),
                    )
                }
                "IamUserArnRequiredException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::IamUserArnRequired(err.msg),
                    )
                }
                "InstanceNameAlreadyRegisteredException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(err.msg),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::InstanceNameRequired(err.msg),
                    )
                }
                "InvalidIamSessionArnException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::InvalidIamSessionArn(err.msg),
                    )
                }
                "InvalidIamUserArnException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::InvalidIamUserArn(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::InvalidInstanceName(err.msg),
                    )
                }
                "MultipleIamArnsProvidedException" => {
                    return RusotoError::Service(
                        RegisterOnPremisesInstanceError::MultipleIamArnsProvided(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterOnPremisesInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterOnPremisesInstanceError::IamArnRequired(ref cause) => write!(f, "{}", cause),
            RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::IamUserArnRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::InvalidIamSessionArn(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::InvalidIamUserArn(ref cause) => write!(f, "{}", cause),
            RegisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterOnPremisesInstanceError::MultipleIamArnsProvided(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RegisterOnPremisesInstanceError {}
/// Errors returned by RemoveTagsFromOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromOnPremisesInstancesError {
    /// <p>The maximum number of allowed on-premises instances in a single call was exceeded.</p>
    InstanceLimitExceeded(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance is not registered.</p>
    InstanceNotRegistered(String),
    /// <p>The on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
}

impl RemoveTagsFromOnPremisesInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveTagsFromOnPremisesInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceLimitExceededException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(err.msg),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(err.msg),
                    )
                }
                "InstanceNotRegisteredException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(err.msg),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::InvalidInstanceName(err.msg),
                    )
                }
                "InvalidTagException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::InvalidTag(err.msg),
                    )
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(err.msg),
                    )
                }
                "TagRequiredException" => {
                    return RusotoError::Service(
                        RemoveTagsFromOnPremisesInstancesError::TagRequired(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsFromOnPremisesInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromOnPremisesInstancesError::InvalidInstanceName(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromOnPremisesInstancesError::InvalidTag(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromOnPremisesInstancesError::TagRequired(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveTagsFromOnPremisesInstancesError {}
/// Errors returned by SkipWaitTimeForInstanceTermination
#[derive(Debug, PartialEq)]
pub enum SkipWaitTimeForInstanceTerminationError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>A call was submitted that is not supported for the specified deployment type.</p>
    UnsupportedActionForDeploymentType(String),
}

impl SkipWaitTimeForInstanceTerminationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SkipWaitTimeForInstanceTerminationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentAlreadyCompletedException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::DeploymentAlreadyCompleted(
                            err.msg,
                        ),
                    )
                }
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::DeploymentDoesNotExist(err.msg),
                    )
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::DeploymentIdRequired(err.msg),
                    )
                }
                "DeploymentNotStartedException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::DeploymentNotStarted(err.msg),
                    )
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::InvalidDeploymentId(err.msg),
                    )
                }
                "UnsupportedActionForDeploymentTypeException" => {
                    return RusotoError::Service(
                        SkipWaitTimeForInstanceTerminationError::UnsupportedActionForDeploymentType(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SkipWaitTimeForInstanceTerminationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SkipWaitTimeForInstanceTerminationError::DeploymentAlreadyCompleted(ref cause) => {
                write!(f, "{}", cause)
            }
            SkipWaitTimeForInstanceTerminationError::DeploymentDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            SkipWaitTimeForInstanceTerminationError::DeploymentIdRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            SkipWaitTimeForInstanceTerminationError::DeploymentNotStarted(ref cause) => {
                write!(f, "{}", cause)
            }
            SkipWaitTimeForInstanceTerminationError::InvalidDeploymentId(ref cause) => {
                write!(f, "{}", cause)
            }
            SkipWaitTimeForInstanceTerminationError::UnsupportedActionForDeploymentType(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for SkipWaitTimeForInstanceTerminationError {}
/// Errors returned by StopDeployment
#[derive(Debug, PartialEq)]
pub enum StopDeploymentError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment with the IAM user or AWS account does not exist.</p>
    DeploymentDoesNotExist(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
}

impl StopDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopDeploymentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DeploymentAlreadyCompletedException" => {
                    return RusotoError::Service(StopDeploymentError::DeploymentAlreadyCompleted(
                        err.msg,
                    ))
                }
                "DeploymentDoesNotExistException" => {
                    return RusotoError::Service(StopDeploymentError::DeploymentDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(StopDeploymentError::DeploymentGroupDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return RusotoError::Service(StopDeploymentError::DeploymentIdRequired(err.msg))
                }
                "InvalidDeploymentIdException" => {
                    return RusotoError::Service(StopDeploymentError::InvalidDeploymentId(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDeploymentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDeploymentError::DeploymentAlreadyCompleted(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::DeploymentDoesNotExist(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::DeploymentGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::DeploymentIdRequired(ref cause) => write!(f, "{}", cause),
            StopDeploymentError::InvalidDeploymentId(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDeploymentError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p> The specified ARN is not supported. For example, it might be an ARN for a resource that is not expected. </p>
    ArnNotSupported(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p> The specified ARN is not in a valid format. </p>
    InvalidArn(String),
    /// <p> The specified tags are not valid. </p>
    InvalidTagsToAdd(String),
    /// <p> The ARN of a resource is required, but was not found. </p>
    ResourceArnRequired(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(TagResourceError::ApplicationDoesNotExist(err.msg))
                }
                "ArnNotSupportedException" => {
                    return RusotoError::Service(TagResourceError::ArnNotSupported(err.msg))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(TagResourceError::DeploymentConfigDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(TagResourceError::DeploymentGroupDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "InvalidTagsToAddException" => {
                    return RusotoError::Service(TagResourceError::InvalidTagsToAdd(err.msg))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(TagResourceError::ResourceArnRequired(err.msg))
                }
                "TagRequiredException" => {
                    return RusotoError::Service(TagResourceError::TagRequired(err.msg))
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
            TagResourceError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            TagResourceError::ArnNotSupported(ref cause) => write!(f, "{}", cause),
            TagResourceError::DeploymentConfigDoesNotExist(ref cause) => write!(f, "{}", cause),
            TagResourceError::DeploymentGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTagsToAdd(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p> The specified ARN is not supported. For example, it might be an ARN for a resource that is not expected. </p>
    ArnNotSupported(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p> The specified ARN is not in a valid format. </p>
    InvalidArn(String),
    /// <p> The specified tags are not valid. </p>
    InvalidTagsToAdd(String),
    /// <p> The ARN of a resource is required, but was not found. </p>
    ResourceArnRequired(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(UntagResourceError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ArnNotSupportedException" => {
                    return RusotoError::Service(UntagResourceError::ArnNotSupported(err.msg))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(UntagResourceError::DeploymentConfigDoesNotExist(
                        err.msg,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(UntagResourceError::DeploymentGroupDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "InvalidTagsToAddException" => {
                    return RusotoError::Service(UntagResourceError::InvalidTagsToAdd(err.msg))
                }
                "ResourceArnRequiredException" => {
                    return RusotoError::Service(UntagResourceError::ResourceArnRequired(err.msg))
                }
                "TagRequiredException" => {
                    return RusotoError::Service(UntagResourceError::TagRequired(err.msg))
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
            UntagResourceError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ArnNotSupported(ref cause) => write!(f, "{}", cause),
            UntagResourceError::DeploymentConfigDoesNotExist(ref cause) => write!(f, "{}", cause),
            UntagResourceError::DeploymentGroupDoesNotExist(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidTagsToAdd(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceArnRequired(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TagRequired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>An application with the specified name with the IAM user or AWS account already exists.</p>
    ApplicationAlreadyExists(String),
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ApplicationAlreadyExistsException" => {
                    return RusotoError::Service(UpdateApplicationError::ApplicationAlreadyExists(
                        err.msg,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(UpdateApplicationError::ApplicationDoesNotExist(
                        err.msg,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(UpdateApplicationError::ApplicationNameRequired(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidApplicationName(
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
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::ApplicationAlreadyExists(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ApplicationDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ApplicationNameRequired(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Errors returned by UpdateDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentGroupError {
    /// <p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
    AlarmsLimitExceeded(String),
    /// <p>The application does not exist with the IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>A deployment group with the specified name with the IAM user or AWS account already exists.</p>
    DeploymentGroupAlreadyExists(String),
    /// <p>The named deployment group with the IAM user or AWS account does not exist.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p> The Amazon ECS service is associated with more than one deployment groups. An Amazon ECS service can be associated with only one deployment group. </p>
    ECSServiceMappingLimitExceeded(String),
    /// <p><p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the limit of 255 characters.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled, but the alarm list is empty.</p> </li> </ul></p>
    InvalidAlarmConfig(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled, but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The configuration for the blue/green deployment group was provided in an invalid format. For information about deployment configuration format, see <a>CreateDeploymentConfig</a>.</p>
    InvalidBlueGreenDeploymentConfiguration(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid deployment style was specified. Valid deployment types include "IN_PLACE" and "BLUE_GREEN." Valid deployment options include "WITH_TRAFFIC_CONTROL" and "WITHOUT_TRAFFIC_CONTROL."</p>
    InvalidDeploymentStyle(String),
    /// <p>A call was submitted that specified both Ec2TagFilters and Ec2TagSet, but only one of these data types can be used in a single call.</p>
    InvalidEC2TagCombination(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidEC2Tag(String),
    /// <p> The Amazon ECS service identifier is not valid. </p>
    InvalidECSService(String),
    /// <p>The input was specified in an invalid format.</p>
    InvalidInput(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>A call was submitted that specified both OnPremisesTagFilters and OnPremisesTagSet, but only one of these data types can be used in a single call.</p>
    InvalidOnPremisesTagCombination(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Amazon EC2 Auto Scaling.</p>
    InvalidRole(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p> A target group pair associated with this deployment is not valid. </p>
    InvalidTargetGroupPair(String),
    /// <p>The trigger was specified in an invalid format.</p>
    InvalidTriggerConfig(String),
    /// <p>The limit for lifecycle hooks was exceeded.</p>
    LifecycleHookLimitExceeded(String),
    /// <p>The number of tag groups included in the tag set list exceeded the maximum allowed limit of 3.</p>
    TagSetListLimitExceeded(String),
    /// <p>An API function was called too frequently.</p>
    Throttling(String),
    /// <p>The maximum allowed number of triggers was exceeded.</p>
    TriggerTargetsLimitExceeded(String),
}

impl UpdateDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeploymentGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlarmsLimitExceededException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::AlarmsLimitExceeded(
                        err.msg,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::ApplicationDoesNotExist(err.msg),
                    )
                }
                "ApplicationNameRequiredException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::ApplicationNameRequired(err.msg),
                    )
                }
                "DeploymentConfigDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupAlreadyExistsException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(err.msg),
                    )
                }
                "DeploymentGroupDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(err.msg),
                    )
                }
                "DeploymentGroupNameRequiredException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::DeploymentGroupNameRequired(err.msg),
                    )
                }
                "ECSServiceMappingLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::ECSServiceMappingLimitExceeded(err.msg),
                    )
                }
                "InvalidAlarmConfigException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidAlarmConfig(
                        err.msg,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidApplicationName(err.msg),
                    )
                }
                "InvalidAutoRollbackConfigException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidAutoRollbackConfig(err.msg),
                    )
                }
                "InvalidAutoScalingGroupException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidAutoScalingGroup(err.msg),
                    )
                }
                "InvalidBlueGreenDeploymentConfigurationException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(
                            err.msg,
                        ),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidDeploymentConfigName(err.msg),
                    )
                }
                "InvalidDeploymentGroupNameException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidDeploymentGroupName(err.msg),
                    )
                }
                "InvalidDeploymentStyleException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidDeploymentStyle(err.msg),
                    )
                }
                "InvalidEC2TagCombinationException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidEC2TagCombination(err.msg),
                    )
                }
                "InvalidEC2TagException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidEC2Tag(err.msg))
                }
                "InvalidECSServiceException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidECSService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidInput(err.msg))
                }
                "InvalidLoadBalancerInfoException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidLoadBalancerInfo(err.msg),
                    )
                }
                "InvalidOnPremisesTagCombinationException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidOnPremisesTagCombination(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidRole(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidTag(err.msg))
                }
                "InvalidTargetGroupPairException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::InvalidTargetGroupPair(err.msg),
                    )
                }
                "InvalidTriggerConfigException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::InvalidTriggerConfig(
                        err.msg,
                    ))
                }
                "LifecycleHookLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::LifecycleHookLimitExceeded(err.msg),
                    )
                }
                "TagSetListLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::TagSetListLimitExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDeploymentGroupError::Throttling(err.msg))
                }
                "TriggerTargetsLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDeploymentGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::ApplicationNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::ECSServiceMappingLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidAlarmConfig(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidApplicationName(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidDeploymentStyle(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidEC2TagCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidEC2Tag(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidECSService(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidLoadBalancerInfo(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidOnPremisesTagCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::InvalidRole(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidTag(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidTargetGroupPair(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::InvalidTriggerConfig(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::TagSetListLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeploymentGroupError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateDeploymentGroupError {}
/// Trait representing the capabilities of the CodeDeploy API. CodeDeploy clients implement this trait.
#[async_trait]
pub trait CodeDeploy {
    /// <p>Adds tags to on-premises instances.</p>
    async fn add_tags_to_on_premises_instances(
        &self,
        input: AddTagsToOnPremisesInstancesInput,
    ) -> Result<(), RusotoError<AddTagsToOnPremisesInstancesError>>;

    /// <p>Gets information about one or more application revisions. The maximum number of application revisions that can be returned is 25.</p>
    async fn batch_get_application_revisions(
        &self,
        input: BatchGetApplicationRevisionsInput,
    ) -> Result<BatchGetApplicationRevisionsOutput, RusotoError<BatchGetApplicationRevisionsError>>;

    /// <p>Gets information about one or more applications. The maximum number of applications that can be returned is 25.</p>
    async fn batch_get_applications(
        &self,
        input: BatchGetApplicationsInput,
    ) -> Result<BatchGetApplicationsOutput, RusotoError<BatchGetApplicationsError>>;

    /// <p>Gets information about one or more deployment groups.</p>
    async fn batch_get_deployment_groups(
        &self,
        input: BatchGetDeploymentGroupsInput,
    ) -> Result<BatchGetDeploymentGroupsOutput, RusotoError<BatchGetDeploymentGroupsError>>;

    /// <p><note> <p> This method works, but is deprecated. Use <code>BatchGetDeploymentTargets</code> instead. </p> </note> <p> Returns an array of one or more instances associated with a deployment. This method works with EC2/On-premises and AWS Lambda compute platforms. The newer <code>BatchGetDeploymentTargets</code> works with all compute platforms. The maximum number of instances that can be returned is 25.</p></p>
    async fn batch_get_deployment_instances(
        &self,
        input: BatchGetDeploymentInstancesInput,
    ) -> Result<BatchGetDeploymentInstancesOutput, RusotoError<BatchGetDeploymentInstancesError>>;

    /// <p><p> Returns an array of one or more targets associated with a deployment. This method works with all compute types and should be used instead of the deprecated <code>BatchGetDeploymentInstances</code>. The maximum number of targets that can be returned is 25.</p> <p> The type of targets returned depends on the deployment&#39;s compute platform: </p> <ul> <li> <p> <b>EC2/On-premises</b>: Information about EC2 instance targets. </p> </li> <li> <p> <b>AWS Lambda</b>: Information about Lambda functions targets. </p> </li> <li> <p> <b>Amazon ECS</b>: Information about Amazon ECS service targets. </p> </li> </ul></p>
    async fn batch_get_deployment_targets(
        &self,
        input: BatchGetDeploymentTargetsInput,
    ) -> Result<BatchGetDeploymentTargetsOutput, RusotoError<BatchGetDeploymentTargetsError>>;

    /// <p>Gets information about one or more deployments. The maximum number of deployments that can be returned is 25.</p>
    async fn batch_get_deployments(
        &self,
        input: BatchGetDeploymentsInput,
    ) -> Result<BatchGetDeploymentsOutput, RusotoError<BatchGetDeploymentsError>>;

    /// <p>Gets information about one or more on-premises instances. The maximum number of on-premises instances that can be returned is 25.</p>
    async fn batch_get_on_premises_instances(
        &self,
        input: BatchGetOnPremisesInstancesInput,
    ) -> Result<BatchGetOnPremisesInstancesOutput, RusotoError<BatchGetOnPremisesInstancesError>>;

    /// <p>For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by registering instances in the replacement environment with the load balancer, can start as soon as all instances have a status of Ready.) </p>
    async fn continue_deployment(
        &self,
        input: ContinueDeploymentInput,
    ) -> Result<(), RusotoError<ContinueDeploymentError>>;

    /// <p>Creates an application.</p>
    async fn create_application(
        &self,
        input: CreateApplicationInput,
    ) -> Result<CreateApplicationOutput, RusotoError<CreateApplicationError>>;

    /// <p>Deploys an application revision through the specified deployment group.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentInput,
    ) -> Result<CreateDeploymentOutput, RusotoError<CreateDeploymentError>>;

    /// <p> Creates a deployment configuration. </p>
    async fn create_deployment_config(
        &self,
        input: CreateDeploymentConfigInput,
    ) -> Result<CreateDeploymentConfigOutput, RusotoError<CreateDeploymentConfigError>>;

    /// <p>Creates a deployment group to which application revisions are deployed.</p>
    async fn create_deployment_group(
        &self,
        input: CreateDeploymentGroupInput,
    ) -> Result<CreateDeploymentGroupOutput, RusotoError<CreateDeploymentGroupError>>;

    /// <p>Deletes an application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationInput,
    ) -> Result<(), RusotoError<DeleteApplicationError>>;

    /// <p><p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note></p>
    async fn delete_deployment_config(
        &self,
        input: DeleteDeploymentConfigInput,
    ) -> Result<(), RusotoError<DeleteDeploymentConfigError>>;

    /// <p>Deletes a deployment group.</p>
    async fn delete_deployment_group(
        &self,
        input: DeleteDeploymentGroupInput,
    ) -> Result<DeleteDeploymentGroupOutput, RusotoError<DeleteDeploymentGroupError>>;

    /// <p>Deletes a GitHub account connection.</p>
    async fn delete_git_hub_account_token(
        &self,
        input: DeleteGitHubAccountTokenInput,
    ) -> Result<DeleteGitHubAccountTokenOutput, RusotoError<DeleteGitHubAccountTokenError>>;

    /// <p>Deregisters an on-premises instance.</p>
    async fn deregister_on_premises_instance(
        &self,
        input: DeregisterOnPremisesInstanceInput,
    ) -> Result<(), RusotoError<DeregisterOnPremisesInstanceError>>;

    /// <p>Gets information about an application.</p>
    async fn get_application(
        &self,
        input: GetApplicationInput,
    ) -> Result<GetApplicationOutput, RusotoError<GetApplicationError>>;

    /// <p>Gets information about an application revision.</p>
    async fn get_application_revision(
        &self,
        input: GetApplicationRevisionInput,
    ) -> Result<GetApplicationRevisionOutput, RusotoError<GetApplicationRevisionError>>;

    /// <p><p>Gets information about a deployment.</p> <note> <p> The <code>content</code> property of the <code>appSpecContent</code> object in the returned revision is always null. Use <code>GetApplicationRevision</code> and the <code>sha256</code> property of the returned <code>appSpecContent</code> object to get the content of the deployment’s AppSpec file. </p> </note></p>
    async fn get_deployment(
        &self,
        input: GetDeploymentInput,
    ) -> Result<GetDeploymentOutput, RusotoError<GetDeploymentError>>;

    /// <p>Gets information about a deployment configuration.</p>
    async fn get_deployment_config(
        &self,
        input: GetDeploymentConfigInput,
    ) -> Result<GetDeploymentConfigOutput, RusotoError<GetDeploymentConfigError>>;

    /// <p>Gets information about a deployment group.</p>
    async fn get_deployment_group(
        &self,
        input: GetDeploymentGroupInput,
    ) -> Result<GetDeploymentGroupOutput, RusotoError<GetDeploymentGroupError>>;

    /// <p>Gets information about an instance as part of a deployment.</p>
    async fn get_deployment_instance(
        &self,
        input: GetDeploymentInstanceInput,
    ) -> Result<GetDeploymentInstanceOutput, RusotoError<GetDeploymentInstanceError>>;

    /// <p> Returns information about a deployment target. </p>
    async fn get_deployment_target(
        &self,
        input: GetDeploymentTargetInput,
    ) -> Result<GetDeploymentTargetOutput, RusotoError<GetDeploymentTargetError>>;

    /// <p> Gets information about an on-premises instance. </p>
    async fn get_on_premises_instance(
        &self,
        input: GetOnPremisesInstanceInput,
    ) -> Result<GetOnPremisesInstanceOutput, RusotoError<GetOnPremisesInstanceError>>;

    /// <p>Lists information about revisions for an application.</p>
    async fn list_application_revisions(
        &self,
        input: ListApplicationRevisionsInput,
    ) -> Result<ListApplicationRevisionsOutput, RusotoError<ListApplicationRevisionsError>>;

    /// <p>Lists the applications registered with the IAM user or AWS account.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsInput,
    ) -> Result<ListApplicationsOutput, RusotoError<ListApplicationsError>>;

    /// <p>Lists the deployment configurations with the IAM user or AWS account.</p>
    async fn list_deployment_configs(
        &self,
        input: ListDeploymentConfigsInput,
    ) -> Result<ListDeploymentConfigsOutput, RusotoError<ListDeploymentConfigsError>>;

    /// <p>Lists the deployment groups for an application registered with the IAM user or AWS account.</p>
    async fn list_deployment_groups(
        &self,
        input: ListDeploymentGroupsInput,
    ) -> Result<ListDeploymentGroupsOutput, RusotoError<ListDeploymentGroupsError>>;

    /// <p><note> <p> The newer BatchGetDeploymentTargets should be used instead because it works with all compute types. <code>ListDeploymentInstances</code> throws an exception if it is used with a compute platform other than EC2/On-premises or AWS Lambda. </p> </note> <p> Lists the instance for a deployment associated with the IAM user or AWS account. </p></p>
    async fn list_deployment_instances(
        &self,
        input: ListDeploymentInstancesInput,
    ) -> Result<ListDeploymentInstancesOutput, RusotoError<ListDeploymentInstancesError>>;

    /// <p> Returns an array of target IDs that are associated a deployment. </p>
    async fn list_deployment_targets(
        &self,
        input: ListDeploymentTargetsInput,
    ) -> Result<ListDeploymentTargetsOutput, RusotoError<ListDeploymentTargetsError>>;

    /// <p>Lists the deployments in a deployment group for an application registered with the IAM user or AWS account.</p>
    async fn list_deployments(
        &self,
        input: ListDeploymentsInput,
    ) -> Result<ListDeploymentsOutput, RusotoError<ListDeploymentsError>>;

    /// <p>Lists the names of stored connections to GitHub accounts.</p>
    async fn list_git_hub_account_token_names(
        &self,
        input: ListGitHubAccountTokenNamesInput,
    ) -> Result<ListGitHubAccountTokenNamesOutput, RusotoError<ListGitHubAccountTokenNamesError>>;

    /// <p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names are listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>
    async fn list_on_premises_instances(
        &self,
        input: ListOnPremisesInstancesInput,
    ) -> Result<ListOnPremisesInstancesOutput, RusotoError<ListOnPremisesInstancesError>>;

    /// <p> Returns a list of tags for the resource identified by a specified ARN. Tags are used to organize and categorize your CodeDeploy resources. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p> Sets the result of a Lambda validation function. The function validates one or both lifecycle events (<code>BeforeAllowTraffic</code> and <code>AfterAllowTraffic</code>) and returns <code>Succeeded</code> or <code>Failed</code>. </p>
    async fn put_lifecycle_event_hook_execution_status(
        &self,
        input: PutLifecycleEventHookExecutionStatusInput,
    ) -> Result<
        PutLifecycleEventHookExecutionStatusOutput,
        RusotoError<PutLifecycleEventHookExecutionStatusError>,
    >;

    /// <p>Registers with AWS CodeDeploy a revision for the specified application.</p>
    async fn register_application_revision(
        &self,
        input: RegisterApplicationRevisionInput,
    ) -> Result<(), RusotoError<RegisterApplicationRevisionError>>;

    /// <p><p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note></p>
    async fn register_on_premises_instance(
        &self,
        input: RegisterOnPremisesInstanceInput,
    ) -> Result<(), RusotoError<RegisterOnPremisesInstanceError>>;

    /// <p>Removes one or more tags from one or more on-premises instances.</p>
    async fn remove_tags_from_on_premises_instances(
        &self,
        input: RemoveTagsFromOnPremisesInstancesInput,
    ) -> Result<(), RusotoError<RemoveTagsFromOnPremisesInstancesError>>;

    /// <p>In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is complete.</p>
    async fn skip_wait_time_for_instance_termination(
        &self,
        input: SkipWaitTimeForInstanceTerminationInput,
    ) -> Result<(), RusotoError<SkipWaitTimeForInstanceTerminationError>>;

    /// <p>Attempts to stop an ongoing deployment.</p>
    async fn stop_deployment(
        &self,
        input: StopDeploymentInput,
    ) -> Result<StopDeploymentOutput, RusotoError<StopDeploymentError>>;

    /// <p> Associates the list of tags in the input <code>Tags</code> parameter with the resource identified by the <code>ResourceArn</code> input parameter. </p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p> Disassociates a resource from a list of tags. The resource is identified by the <code>ResourceArn</code> input parameter. The tags are identfied by the list of keys in the <code>TagKeys</code> input parameter. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p>Changes the name of an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationInput,
    ) -> Result<(), RusotoError<UpdateApplicationError>>;

    /// <p>Changes information about a deployment group.</p>
    async fn update_deployment_group(
        &self,
        input: UpdateDeploymentGroupInput,
    ) -> Result<UpdateDeploymentGroupOutput, RusotoError<UpdateDeploymentGroupError>>;
}
/// A client for the CodeDeploy API.
#[derive(Clone)]
pub struct CodeDeployClient {
    client: Client,
    region: region::Region,
}

impl CodeDeployClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeDeployClient {
        CodeDeployClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeDeployClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeDeployClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeDeployClient {
        CodeDeployClient { client, region }
    }
}

#[async_trait]
impl CodeDeploy for CodeDeployClient {
    /// <p>Adds tags to on-premises instances.</p>
    async fn add_tags_to_on_premises_instances(
        &self,
        input: AddTagsToOnPremisesInstancesInput,
    ) -> Result<(), RusotoError<AddTagsToOnPremisesInstancesError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.AddTagsToOnPremisesInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsToOnPremisesInstancesError::from_response(response))
        }
    }

    /// <p>Gets information about one or more application revisions. The maximum number of application revisions that can be returned is 25.</p>
    async fn batch_get_application_revisions(
        &self,
        input: BatchGetApplicationRevisionsInput,
    ) -> Result<BatchGetApplicationRevisionsOutput, RusotoError<BatchGetApplicationRevisionsError>>
    {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetApplicationRevisions",
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
                .deserialize::<BatchGetApplicationRevisionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetApplicationRevisionsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more applications. The maximum number of applications that can be returned is 25.</p>
    async fn batch_get_applications(
        &self,
        input: BatchGetApplicationsInput,
    ) -> Result<BatchGetApplicationsOutput, RusotoError<BatchGetApplicationsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetApplications");
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
                .deserialize::<BatchGetApplicationsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetApplicationsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more deployment groups.</p>
    async fn batch_get_deployment_groups(
        &self,
        input: BatchGetDeploymentGroupsInput,
    ) -> Result<BatchGetDeploymentGroupsOutput, RusotoError<BatchGetDeploymentGroupsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetDeploymentGroups",
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
                .deserialize::<BatchGetDeploymentGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetDeploymentGroupsError::from_response(response))
        }
    }

    /// <p><note> <p> This method works, but is deprecated. Use <code>BatchGetDeploymentTargets</code> instead. </p> </note> <p> Returns an array of one or more instances associated with a deployment. This method works with EC2/On-premises and AWS Lambda compute platforms. The newer <code>BatchGetDeploymentTargets</code> works with all compute platforms. The maximum number of instances that can be returned is 25.</p></p>
    async fn batch_get_deployment_instances(
        &self,
        input: BatchGetDeploymentInstancesInput,
    ) -> Result<BatchGetDeploymentInstancesOutput, RusotoError<BatchGetDeploymentInstancesError>>
    {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetDeploymentInstances",
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
                .deserialize::<BatchGetDeploymentInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetDeploymentInstancesError::from_response(response))
        }
    }

    /// <p><p> Returns an array of one or more targets associated with a deployment. This method works with all compute types and should be used instead of the deprecated <code>BatchGetDeploymentInstances</code>. The maximum number of targets that can be returned is 25.</p> <p> The type of targets returned depends on the deployment&#39;s compute platform: </p> <ul> <li> <p> <b>EC2/On-premises</b>: Information about EC2 instance targets. </p> </li> <li> <p> <b>AWS Lambda</b>: Information about Lambda functions targets. </p> </li> <li> <p> <b>Amazon ECS</b>: Information about Amazon ECS service targets. </p> </li> </ul></p>
    async fn batch_get_deployment_targets(
        &self,
        input: BatchGetDeploymentTargetsInput,
    ) -> Result<BatchGetDeploymentTargetsOutput, RusotoError<BatchGetDeploymentTargetsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetDeploymentTargets",
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
                .deserialize::<BatchGetDeploymentTargetsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetDeploymentTargetsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more deployments. The maximum number of deployments that can be returned is 25.</p>
    async fn batch_get_deployments(
        &self,
        input: BatchGetDeploymentsInput,
    ) -> Result<BatchGetDeploymentsOutput, RusotoError<BatchGetDeploymentsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetDeployments");
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
                .deserialize::<BatchGetDeploymentsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetDeploymentsError::from_response(response))
        }
    }

    /// <p>Gets information about one or more on-premises instances. The maximum number of on-premises instances that can be returned is 25.</p>
    async fn batch_get_on_premises_instances(
        &self,
        input: BatchGetOnPremisesInstancesInput,
    ) -> Result<BatchGetOnPremisesInstancesOutput, RusotoError<BatchGetOnPremisesInstancesError>>
    {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetOnPremisesInstances",
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
                .deserialize::<BatchGetOnPremisesInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetOnPremisesInstancesError::from_response(response))
        }
    }

    /// <p>For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by registering instances in the replacement environment with the load balancer, can start as soon as all instances have a status of Ready.) </p>
    async fn continue_deployment(
        &self,
        input: ContinueDeploymentInput,
    ) -> Result<(), RusotoError<ContinueDeploymentError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ContinueDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ContinueDeploymentError::from_response(response))
        }
    }

    /// <p>Creates an application.</p>
    async fn create_application(
        &self,
        input: CreateApplicationInput,
    ) -> Result<CreateApplicationOutput, RusotoError<CreateApplicationError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateApplicationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApplicationError::from_response(response))
        }
    }

    /// <p>Deploys an application revision through the specified deployment group.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentInput,
    ) -> Result<CreateDeploymentOutput, RusotoError<CreateDeploymentError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDeploymentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentError::from_response(response))
        }
    }

    /// <p> Creates a deployment configuration. </p>
    async fn create_deployment_config(
        &self,
        input: CreateDeploymentConfigInput,
    ) -> Result<CreateDeploymentConfigOutput, RusotoError<CreateDeploymentConfigError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentConfig");
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
                .deserialize::<CreateDeploymentConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentConfigError::from_response(response))
        }
    }

    /// <p>Creates a deployment group to which application revisions are deployed.</p>
    async fn create_deployment_group(
        &self,
        input: CreateDeploymentGroupInput,
    ) -> Result<CreateDeploymentGroupOutput, RusotoError<CreateDeploymentGroupError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentGroup");
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
                .deserialize::<CreateDeploymentGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentGroupError::from_response(response))
        }
    }

    /// <p>Deletes an application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationInput,
    ) -> Result<(), RusotoError<DeleteApplicationError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApplicationError::from_response(response))
        }
    }

    /// <p><p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note></p>
    async fn delete_deployment_config(
        &self,
        input: DeleteDeploymentConfigInput,
    ) -> Result<(), RusotoError<DeleteDeploymentConfigError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeploymentConfigError::from_response(response))
        }
    }

    /// <p>Deletes a deployment group.</p>
    async fn delete_deployment_group(
        &self,
        input: DeleteDeploymentGroupInput,
    ) -> Result<DeleteDeploymentGroupOutput, RusotoError<DeleteDeploymentGroupError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentGroup");
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
                .deserialize::<DeleteDeploymentGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDeploymentGroupError::from_response(response))
        }
    }

    /// <p>Deletes a GitHub account connection.</p>
    async fn delete_git_hub_account_token(
        &self,
        input: DeleteGitHubAccountTokenInput,
    ) -> Result<DeleteGitHubAccountTokenOutput, RusotoError<DeleteGitHubAccountTokenError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.DeleteGitHubAccountToken",
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
                .deserialize::<DeleteGitHubAccountTokenOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGitHubAccountTokenError::from_response(response))
        }
    }

    /// <p>Deregisters an on-premises instance.</p>
    async fn deregister_on_premises_instance(
        &self,
        input: DeregisterOnPremisesInstanceInput,
    ) -> Result<(), RusotoError<DeregisterOnPremisesInstanceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.DeregisterOnPremisesInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterOnPremisesInstanceError::from_response(response))
        }
    }

    /// <p>Gets information about an application.</p>
    async fn get_application(
        &self,
        input: GetApplicationInput,
    ) -> Result<GetApplicationOutput, RusotoError<GetApplicationError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetApplicationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationError::from_response(response))
        }
    }

    /// <p>Gets information about an application revision.</p>
    async fn get_application_revision(
        &self,
        input: GetApplicationRevisionInput,
    ) -> Result<GetApplicationRevisionOutput, RusotoError<GetApplicationRevisionError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplicationRevision");
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
                .deserialize::<GetApplicationRevisionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationRevisionError::from_response(response))
        }
    }

    /// <p><p>Gets information about a deployment.</p> <note> <p> The <code>content</code> property of the <code>appSpecContent</code> object in the returned revision is always null. Use <code>GetApplicationRevision</code> and the <code>sha256</code> property of the returned <code>appSpecContent</code> object to get the content of the deployment’s AppSpec file. </p> </note></p>
    async fn get_deployment(
        &self,
        input: GetDeploymentInput,
    ) -> Result<GetDeploymentOutput, RusotoError<GetDeploymentError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDeploymentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentError::from_response(response))
        }
    }

    /// <p>Gets information about a deployment configuration.</p>
    async fn get_deployment_config(
        &self,
        input: GetDeploymentConfigInput,
    ) -> Result<GetDeploymentConfigOutput, RusotoError<GetDeploymentConfigError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentConfig");
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
                .deserialize::<GetDeploymentConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentConfigError::from_response(response))
        }
    }

    /// <p>Gets information about a deployment group.</p>
    async fn get_deployment_group(
        &self,
        input: GetDeploymentGroupInput,
    ) -> Result<GetDeploymentGroupOutput, RusotoError<GetDeploymentGroupError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentGroup");
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
                .deserialize::<GetDeploymentGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentGroupError::from_response(response))
        }
    }

    /// <p>Gets information about an instance as part of a deployment.</p>
    async fn get_deployment_instance(
        &self,
        input: GetDeploymentInstanceInput,
    ) -> Result<GetDeploymentInstanceOutput, RusotoError<GetDeploymentInstanceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentInstance");
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
                .deserialize::<GetDeploymentInstanceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentInstanceError::from_response(response))
        }
    }

    /// <p> Returns information about a deployment target. </p>
    async fn get_deployment_target(
        &self,
        input: GetDeploymentTargetInput,
    ) -> Result<GetDeploymentTargetOutput, RusotoError<GetDeploymentTargetError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentTarget");
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
                .deserialize::<GetDeploymentTargetOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeploymentTargetError::from_response(response))
        }
    }

    /// <p> Gets information about an on-premises instance. </p>
    async fn get_on_premises_instance(
        &self,
        input: GetOnPremisesInstanceInput,
    ) -> Result<GetOnPremisesInstanceOutput, RusotoError<GetOnPremisesInstanceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetOnPremisesInstance");
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
                .deserialize::<GetOnPremisesInstanceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOnPremisesInstanceError::from_response(response))
        }
    }

    /// <p>Lists information about revisions for an application.</p>
    async fn list_application_revisions(
        &self,
        input: ListApplicationRevisionsInput,
    ) -> Result<ListApplicationRevisionsOutput, RusotoError<ListApplicationRevisionsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListApplicationRevisions",
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
                .deserialize::<ListApplicationRevisionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationRevisionsError::from_response(response))
        }
    }

    /// <p>Lists the applications registered with the IAM user or AWS account.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsInput,
    ) -> Result<ListApplicationsOutput, RusotoError<ListApplicationsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListApplicationsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListApplicationsError::from_response(response))
        }
    }

    /// <p>Lists the deployment configurations with the IAM user or AWS account.</p>
    async fn list_deployment_configs(
        &self,
        input: ListDeploymentConfigsInput,
    ) -> Result<ListDeploymentConfigsOutput, RusotoError<ListDeploymentConfigsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentConfigs");
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
                .deserialize::<ListDeploymentConfigsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentConfigsError::from_response(response))
        }
    }

    /// <p>Lists the deployment groups for an application registered with the IAM user or AWS account.</p>
    async fn list_deployment_groups(
        &self,
        input: ListDeploymentGroupsInput,
    ) -> Result<ListDeploymentGroupsOutput, RusotoError<ListDeploymentGroupsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentGroups");
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
                .deserialize::<ListDeploymentGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentGroupsError::from_response(response))
        }
    }

    /// <p><note> <p> The newer BatchGetDeploymentTargets should be used instead because it works with all compute types. <code>ListDeploymentInstances</code> throws an exception if it is used with a compute platform other than EC2/On-premises or AWS Lambda. </p> </note> <p> Lists the instance for a deployment associated with the IAM user or AWS account. </p></p>
    async fn list_deployment_instances(
        &self,
        input: ListDeploymentInstancesInput,
    ) -> Result<ListDeploymentInstancesOutput, RusotoError<ListDeploymentInstancesError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListDeploymentInstances",
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
                .deserialize::<ListDeploymentInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentInstancesError::from_response(response))
        }
    }

    /// <p> Returns an array of target IDs that are associated a deployment. </p>
    async fn list_deployment_targets(
        &self,
        input: ListDeploymentTargetsInput,
    ) -> Result<ListDeploymentTargetsOutput, RusotoError<ListDeploymentTargetsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentTargets");
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
                .deserialize::<ListDeploymentTargetsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentTargetsError::from_response(response))
        }
    }

    /// <p>Lists the deployments in a deployment group for an application registered with the IAM user or AWS account.</p>
    async fn list_deployments(
        &self,
        input: ListDeploymentsInput,
    ) -> Result<ListDeploymentsOutput, RusotoError<ListDeploymentsError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeployments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDeploymentsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeploymentsError::from_response(response))
        }
    }

    /// <p>Lists the names of stored connections to GitHub accounts.</p>
    async fn list_git_hub_account_token_names(
        &self,
        input: ListGitHubAccountTokenNamesInput,
    ) -> Result<ListGitHubAccountTokenNamesOutput, RusotoError<ListGitHubAccountTokenNamesError>>
    {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListGitHubAccountTokenNames",
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
                .deserialize::<ListGitHubAccountTokenNamesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListGitHubAccountTokenNamesError::from_response(response))
        }
    }

    /// <p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names are listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>
    async fn list_on_premises_instances(
        &self,
        input: ListOnPremisesInstancesInput,
    ) -> Result<ListOnPremisesInstancesOutput, RusotoError<ListOnPremisesInstancesError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListOnPremisesInstances",
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
                .deserialize::<ListOnPremisesInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListOnPremisesInstancesError::from_response(response))
        }
    }

    /// <p> Returns a list of tags for the resource identified by a specified ARN. Tags are used to organize and categorize your CodeDeploy resources. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p> Sets the result of a Lambda validation function. The function validates one or both lifecycle events (<code>BeforeAllowTraffic</code> and <code>AfterAllowTraffic</code>) and returns <code>Succeeded</code> or <code>Failed</code>. </p>
    async fn put_lifecycle_event_hook_execution_status(
        &self,
        input: PutLifecycleEventHookExecutionStatusInput,
    ) -> Result<
        PutLifecycleEventHookExecutionStatusOutput,
        RusotoError<PutLifecycleEventHookExecutionStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.PutLifecycleEventHookExecutionStatus",
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
                .deserialize::<PutLifecycleEventHookExecutionStatusOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutLifecycleEventHookExecutionStatusError::from_response(
                response,
            ))
        }
    }

    /// <p>Registers with AWS CodeDeploy a revision for the specified application.</p>
    async fn register_application_revision(
        &self,
        input: RegisterApplicationRevisionInput,
    ) -> Result<(), RusotoError<RegisterApplicationRevisionError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RegisterApplicationRevision",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterApplicationRevisionError::from_response(response))
        }
    }

    /// <p><p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note></p>
    async fn register_on_premises_instance(
        &self,
        input: RegisterOnPremisesInstanceInput,
    ) -> Result<(), RusotoError<RegisterOnPremisesInstanceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RegisterOnPremisesInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterOnPremisesInstanceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from one or more on-premises instances.</p>
    async fn remove_tags_from_on_premises_instances(
        &self,
        input: RemoveTagsFromOnPremisesInstancesInput,
    ) -> Result<(), RusotoError<RemoveTagsFromOnPremisesInstancesError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RemoveTagsFromOnPremisesInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsFromOnPremisesInstancesError::from_response(
                response,
            ))
        }
    }

    /// <p>In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is complete.</p>
    async fn skip_wait_time_for_instance_termination(
        &self,
        input: SkipWaitTimeForInstanceTerminationInput,
    ) -> Result<(), RusotoError<SkipWaitTimeForInstanceTerminationError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.SkipWaitTimeForInstanceTermination",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SkipWaitTimeForInstanceTerminationError::from_response(
                response,
            ))
        }
    }

    /// <p>Attempts to stop an ongoing deployment.</p>
    async fn stop_deployment(
        &self,
        input: StopDeploymentInput,
    ) -> Result<StopDeploymentOutput, RusotoError<StopDeploymentError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.StopDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopDeploymentOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopDeploymentError::from_response(response))
        }
    }

    /// <p> Associates the list of tags in the input <code>Tags</code> parameter with the resource identified by the <code>ResourceArn</code> input parameter. </p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p> Disassociates a resource from a list of tags. The resource is identified by the <code>ResourceArn</code> input parameter. The tags are identfied by the list of keys in the <code>TagKeys</code> input parameter. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Changes the name of an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationInput,
    ) -> Result<(), RusotoError<UpdateApplicationError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApplicationError::from_response(response))
        }
    }

    /// <p>Changes information about a deployment group.</p>
    async fn update_deployment_group(
        &self,
        input: UpdateDeploymentGroupInput,
    ) -> Result<UpdateDeploymentGroupOutput, RusotoError<UpdateDeploymentGroupError>> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateDeploymentGroup");
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
                .deserialize::<UpdateDeploymentGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeploymentGroupError::from_response(response))
        }
    }
}
