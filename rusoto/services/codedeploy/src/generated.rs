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
/// <p>Represents the input of, and adds tags to, an on-premises instance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p><p>Indicates whether a deployment should continue if information about the current state of alarms cannot be retrieved from Amazon CloudWatch. The default value is false.</p> <ul> <li> <p>true: The deployment will proceed even if alarm status information can&#39;t be retrieved from Amazon CloudWatch.</p> </li> <li> <p>false: The deployment will stop if alarm status information can&#39;t be retrieved from Amazon CloudWatch.</p> </li> </ul></p>
    #[serde(rename = "ignorePollAlarmFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_poll_alarm_failure: Option<bool>,
}

/// <p>Information about an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>True if the user has authenticated with GitHub for the specified application; otherwise, false.</p>
    #[serde(rename = "linkedToGitHub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_to_git_hub: Option<bool>,
}

/// <p>Information about a configuration for automatically rolling back to a previous version of an application revision when a deployment doesn't complete successfully.</p>
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
pub struct BatchGetApplicationRevisionsInput {
    /// <p>The name of an AWS CodeDeploy application about which to get revision information.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Information to get about the application revisions, including type and location.</p>
    #[serde(rename = "revisions")]
    pub revisions: Vec<RevisionLocation>,
}

/// <p>Represents the output of a BatchGetApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetApplicationRevisionsOutput {
    /// <p>The name of the application that corresponds to the revisions.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>Information about errors that may have occurred during the API call.</p>
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
pub struct BatchGetApplicationsInput {
    /// <p>A list of application names separated by spaces.</p>
    #[serde(rename = "applicationNames")]
    pub application_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetApplicationsOutput {
    /// <p>Information about the applications.</p>
    #[serde(rename = "applicationsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_info: Option<Vec<ApplicationInfo>>,
}

/// <p>Represents the input of a BatchGetDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetDeploymentGroupsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The deployment groups' names.</p>
    #[serde(rename = "deploymentGroupNames")]
    pub deployment_group_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetDeploymentGroupsOutput {
    /// <p>Information about the deployment groups.</p>
    #[serde(rename = "deploymentGroupsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups_info: Option<Vec<DeploymentGroupInfo>>,
    /// <p>Information about errors that may have occurred during the API call.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p>Represents the input of a BatchGetDeploymentInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetDeploymentInstancesInput {
    /// <p>The unique ID of a deployment.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>The unique IDs of instances in the deployment group.</p>
    #[serde(rename = "instanceIds")]
    pub instance_ids: Vec<String>,
}

/// <p>Represents the output of a BatchGetDeploymentInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetDeploymentInstancesOutput {
    /// <p>Information about errors that may have occurred during the API call.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Information about the instance.</p>
    #[serde(rename = "instancesSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_summary: Option<Vec<InstanceSummary>>,
}

/// <p>Represents the input of a BatchGetDeployments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetDeploymentsInput {
    /// <p>A list of deployment IDs, separated by spaces.</p>
    #[serde(rename = "deploymentIds")]
    pub deployment_ids: Vec<String>,
}

/// <p>Represents the output of a BatchGetDeployments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetDeploymentsOutput {
    /// <p>Information about the deployments.</p>
    #[serde(rename = "deploymentsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_info: Option<Vec<DeploymentInfo>>,
}

/// <p>Represents the input of a BatchGetOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetOnPremisesInstancesInput {
    /// <p>The names of the on-premises instances about which to get information.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
}

/// <p>Represents the output of a BatchGetOnPremisesInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about whether instances in the original environment are terminated when a blue/green deployment is successful.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlueInstanceTerminationOption {
    /// <p><p>The action to take on instances in the original environment after a successful blue/green deployment.</p> <ul> <li> <p>TERMINATE: Instances are terminated after a specified wait time.</p> </li> <li> <p>KEEP_ALIVE: Instances are left running after they are deregistered from the load balancer and removed from the deployment group.</p> </li> </ul></p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The number of minutes to wait after a successful blue/green deployment before terminating instances from the original environment. The maximum setting is 2880 minutes (2 days).</p>
    #[serde(rename = "terminationWaitTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_wait_time_in_minutes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ContinueDeploymentInput {
    /// <p>The deployment ID of the blue/green deployment for which you want to start rerouting traffic to the replacement environment.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Represents the input of a CreateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationInput {
    /// <p>The name of the application. This name must be unique with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p> The destination platform type for the deployment (<code>Lambda</code> or <code>Server</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
}

/// <p>Represents the output of a CreateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateApplicationOutput {
    /// <p>A unique application ID.</p>
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

/// <p>Represents the input of a CreateDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentConfigInput {
    /// <p>The destination platform type for the deployment (<code>Lambda</code> or <code>Server</code>&gt;).</p>
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
    /// <p>The configuration that specifies how the deployment traffic will be routed.</p>
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
}

/// <p>Represents the output of a CreateDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeploymentConfigOutput {
    /// <p>A unique deployment configuration ID.</p>
    #[serde(rename = "deploymentConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_id: Option<String>,
}

/// <p>Represents the input of a CreateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentGroupInput {
    /// <p>Information to add about Amazon CloudWatch alarms when the deployment group is created.</p>
    #[serde(rename = "alarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Configuration information for an automatic rollback that is added when a deployment group is created.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>A list of associated Auto Scaling groups.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>Information about blue/green deployment options for a deployment group.</p>
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    /// <p>If specified, the deployment configuration name can be either one of the predefined configurations provided with AWS CodeDeploy or a custom deployment configuration that you create by calling the create deployment configuration operation.</p> <p>CodeDeployDefault.OneAtATime is the default deployment configuration. It is used if a configuration isn't specified for the deployment or the deployment group.</p> <p>For more information about the predefined deployment configurations in AWS CodeDeploy, see <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-configurations.html">Working with Deployment Groups in AWS CodeDeploy</a> in the AWS CodeDeploy User Guide.</p>
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
    /// <p>The Amazon EC2 tags on which to filter. The deployment group will include EC2 instances with any of the specified tags. Cannot be used in the same call as ec2TagSet.</p>
    #[serde(rename = "ec2TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_filters: Option<Vec<EC2TagFilter>>,
    /// <p>Information about groups of tags applied to EC2 instances. The deployment group will include only EC2 instances identified by all the tag groups. Cannot be used in the same call as ec2TagFilters.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
    /// <p>Information about the load balancer used in a deployment.</p>
    #[serde(rename = "loadBalancerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    /// <p>The on-premises instance tags on which to filter. The deployment group will include on-premises instances with any of the specified tags. Cannot be used in the same call as OnPremisesTagSet.</p>
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    /// <p>Information about groups of tags applied to on-premises instances. The deployment group will include only on-premises instances identified by all the tag groups. Cannot be used in the same call as onPremisesInstanceTagFilters.</p>
    #[serde(rename = "onPremisesTagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// <p>A service role ARN that allows AWS CodeDeploy to act on the user's behalf when interacting with AWS services.</p>
    #[serde(rename = "serviceRoleArn")]
    pub service_role_arn: String,
    /// <p>Information about triggers to create when the deployment group is created. For examples, see <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-sns.html">Create a Trigger for an AWS CodeDeploy Event</a> in the AWS CodeDeploy User Guide.</p>
    #[serde(rename = "triggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

/// <p>Represents the output of a CreateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeploymentGroupOutput {
    /// <p>A unique deployment group ID.</p>
    #[serde(rename = "deploymentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_id: Option<String>,
}

/// <p>Represents the input of a CreateDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Configuration information for an automatic rollback that is added when a deployment is created.</p>
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    /// <p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p> <p>If not specified, the value configured in the deployment group will be used as the default. If the deployment group does not have a deployment configuration associated with it, then CodeDeployDefault.OneAtATime will be used by default.</p>
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
    /// <p>If set to true, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will not be considered to have failed at that point and will continue on to the BeforeInstall deployment lifecycle event.</p> <p>If set to false or not specified, then if the deployment causes the ApplicationStop deployment lifecycle event to fail to an instance, the deployment to that instance will stop, and the deployment to that instance will be considered to have failed.</p>
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    /// <p>The type and location of the revision to deploy.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    /// <p>Information about the instances that will belong to the replacement environment in a blue/green deployment.</p>
    #[serde(rename = "targetInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<TargetInstances>,
    /// <p>Indicates whether to deploy to all instances or only to instances that are not running the latest application revision.</p>
    #[serde(rename = "updateOutdatedInstancesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_outdated_instances_only: Option<bool>,
}

/// <p>Represents the output of a CreateDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeploymentOutput {
    /// <p>A unique deployment ID.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Represents the input of a DeleteApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
}

/// <p>Represents the input of a DeleteDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeploymentConfigInput {
    /// <p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "deploymentConfigName")]
    pub deployment_config_name: String,
}

/// <p>Represents the input of a DeleteDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeploymentGroupInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The name of an existing deployment group for the specified application.</p>
    #[serde(rename = "deploymentGroupName")]
    pub deployment_group_name: String,
}

/// <p>Represents the output of a DeleteDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDeploymentGroupOutput {
    /// <p>If the output contains no data, and the corresponding deployment group contained at least one Auto Scaling group, AWS CodeDeploy successfully removed all corresponding Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group. If the output contains data, AWS CodeDeploy could not remove some Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group.</p>
    #[serde(rename = "hooksNotCleanedUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_not_cleaned_up: Option<Vec<AutoScalingGroup>>,
}

/// <p>Represents the input of a DeleteGitHubAccount operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGitHubAccountTokenInput {
    /// <p>The name of the GitHub account connection to delete.</p>
    #[serde(rename = "tokenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

/// <p>Represents the output of a DeleteGitHubAccountToken operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteGitHubAccountTokenOutput {
    /// <p>The name of the GitHub account connection that was deleted.</p>
    #[serde(rename = "tokenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

/// <p>Information about a deployment configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeploymentConfigInfo {
    /// <p>The destination platform type for the deployment (<code>Lambda</code> or <code>Server</code>).</p>
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
    /// <p>The configuration specifying how the deployment traffic will be routed. Only deployments with a Lambda compute platform can specify this.</p>
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
}

/// <p>Information about a deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The destination platform type for the deployment group (<code>Lambda</code> or <code>Server</code>).</p>
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
    /// <p>Information about groups of tags applied to an EC2 instance. The deployment group includes only EC2 instances identified by all the tag groups. Cannot be used in the same call as ec2TagFilters.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
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
    /// <p>A service role ARN.</p>
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
    /// <p>A timestamp indicating when the deployment was complete.</p>
    #[serde(rename = "completeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<f64>,
    /// <p>The destination platform type for the deployment (<code>Lambda</code> or <code>Server</code>).</p>
    #[serde(rename = "computePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    /// <p>A timestamp indicating when the deployment was created.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p><p>The means by which the deployment was created:</p> <ul> <li> <p>user: A user created the deployment.</p> </li> <li> <p>autoscaling: Auto Scaling created the deployment.</p> </li> <li> <p>codeDeployRollback: A rollback process created the deployment.</p> </li> </ul></p>
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// <p>The deployment configuration name.</p>
    #[serde(rename = "deploymentConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    /// <p>The deployment group name.</p>
    #[serde(rename = "deploymentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    /// <p>The deployment ID.</p>
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
    /// <p>If true, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will not be considered to have failed at that point and will continue on to the BeforeInstall deployment lifecycle event.</p> <p>If false or not specified, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will stop, and the deployment to that instance will be considered to have failed.</p>
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    /// <p>Indicates whether the wait period set for the termination of instances in the original environment has started. Status is 'false' if the KEEP_ALIVE option is specified; otherwise, 'true' as soon as the termination wait period starts.</p>
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
    /// <p>A timestamp indicating when the deployment was deployed to the deployment group.</p> <p>In some cases, the reported value of the start time may be later than the complete time. This is due to differences in the clock settings of back-end servers that participate in the deployment process.</p>
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
    /// <p>The number of minutes to wait before the status of a blue/green deployment changed to Stopped if rerouting is not started manually. Applies only to the STOP_DEPLOYMENT option for actionOnTimeout</p>
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

/// <p>Represents the input of a DeregisterOnPremisesInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterOnPremisesInstanceInput {
    /// <p>The name of the on-premises instance to deregister.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

/// <p>Diagnostic information about executable scripts that are part of a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>A list containing other lists of EC2 instance tag groups. In order for an instance to be included in the deployment group, it must be identified by all the tag groups in the list.</p>
    #[serde(rename = "ec2TagSetList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set_list: Option<Vec<Vec<EC2TagFilter>>>,
}

/// <p>Information about a load balancer in Elastic Load Balancing to use in a deployment. Instances are registered directly with a load balancer, and traffic is routed to the load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ELBInfo {
    /// <p>For blue/green deployments, the name of the load balancer that will be used to route traffic from original instances to replacement instances in a blue/green deployment. For in-place deployments, the name of the load balancer that instances are deregistered from so they are not serving traffic during a deployment, and then re-registered with after the deployment completes.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information about a deployment error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ErrorInformation {
    /// <p><p>For information about additional error codes, see <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide/error-codes.html">Error Codes for AWS CodeDeploy</a> in the <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide">AWS CodeDeploy User Guide</a>.</p> <p>The error code:</p> <ul> <li> <p>APPLICATION<em>MISSING: The application was missing. This error code will most likely be raised if the application is deleted after the deployment is created but before it is started.</p> </li> <li> <p>DEPLOYMENT</em>GROUP<em>MISSING: The deployment group was missing. This error code will most likely be raised if the deployment group is deleted after the deployment is created but before it is started.</p> </li> <li> <p>HEALTH</em>CONSTRAINTS: The deployment failed on too many instances to be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>HEALTH<em>CONSTRAINTS</em>INVALID: The revision cannot be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>IAM<em>ROLE</em>MISSING: The service role cannot be accessed.</p> </li> <li> <p>IAM<em>ROLE</em>PERMISSIONS: The service role does not have the correct permissions.</p> </li> <li> <p>INTERNAL<em>ERROR: There was an internal error.</p> </li> <li> <p>NO</em>EC2<em>SUBSCRIPTION: The calling account is not subscribed to the Amazon EC2 service.</p> </li> <li> <p>NO</em>INSTANCES: No instance were specified, or no instance can be found.</p> </li> <li> <p>OVER<em>MAX</em>INSTANCES: The maximum number of instance was exceeded.</p> </li> <li> <p>THROTTLED: The operation was throttled because the calling account exceeded the throttling limits of one or more AWS services.</p> </li> <li> <p>TIMEOUT: The deployment has timed out.</p> </li> <li> <p>REVISION_MISSING: The revision ID was missing. This error code will most likely be raised if the revision is deleted after the deployment is created but before it is started.</p> </li> </ul></p>
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
pub struct GetApplicationInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
}

/// <p>Represents the output of a GetApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetApplicationOutput {
    /// <p>Information about the application.</p>
    #[serde(rename = "application")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationInfo>,
}

/// <p>Represents the input of a GetApplicationRevision operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetDeploymentConfigInput {
    /// <p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "deploymentConfigName")]
    pub deployment_config_name: String,
}

/// <p>Represents the output of a GetDeploymentConfig operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeploymentConfigOutput {
    /// <p>Information about the deployment configuration.</p>
    #[serde(rename = "deploymentConfigInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_info: Option<DeploymentConfigInfo>,
}

/// <p>Represents the input of a GetDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentGroupInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The name of an existing deployment group for the specified application.</p>
    #[serde(rename = "deploymentGroupName")]
    pub deployment_group_name: String,
}

/// <p>Represents the output of a GetDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeploymentGroupOutput {
    /// <p>Information about the deployment group.</p>
    #[serde(rename = "deploymentGroupInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_info: Option<DeploymentGroupInfo>,
}

/// <p>Represents the input of a GetDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentInput {
    /// <p>A deployment ID associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
}

/// <p>Represents the input of a GetDeploymentInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentInstanceInput {
    /// <p>The unique ID of a deployment.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>The unique ID of an instance in the deployment group.</p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
}

/// <p>Represents the output of a GetDeploymentInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeploymentInstanceOutput {
    /// <p>Information about the instance.</p>
    #[serde(rename = "instanceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_summary: Option<InstanceSummary>,
}

/// <p>Represents the output of a GetDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeploymentOutput {
    /// <p>Information about the deployment.</p>
    #[serde(rename = "deploymentInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_info: Option<DeploymentInfo>,
}

/// <p>Represents the input of a GetOnPremisesInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOnPremisesInstanceInput {
    /// <p>The name of the on-premises instance about which to get information.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

/// <p>Represents the output of a GetOnPremisesInstance operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetOnPremisesInstanceOutput {
    /// <p>Information about the on-premises instance.</p>
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
pub struct InstanceSummary {
    /// <p>The deployment ID.</p>
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
    /// <p>A timestamp indicating when the instance information was last updated.</p>
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

/// <p>Information about the most recent attempted or successful deployment to a deployment group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LastDeploymentInfo {
    /// <p>A timestamp indicating when the most recent deployment to the deployment group started.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The deployment ID.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>A timestamp indicating when the most recent deployment to the deployment group completed.</p>
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
pub struct LifecycleEvent {
    /// <p>Diagnostic information about the deployment lifecycle event.</p>
    #[serde(rename = "diagnostics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    /// <p>A timestamp indicating when the deployment lifecycle event ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The deployment lifecycle event name, such as ApplicationStop, BeforeInstall, AfterInstall, ApplicationStart, or ValidateService.</p>
    #[serde(rename = "lifecycleEventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_name: Option<String>,
    /// <p>A timestamp indicating when the deployment lifecycle event started.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The deployment lifecycle event status:</p> <ul> <li> <p>Pending: The deployment lifecycle event is pending.</p> </li> <li> <p>InProgress: The deployment lifecycle event is in progress.</p> </li> <li> <p>Succeeded: The deployment lifecycle event ran successfully.</p> </li> <li> <p>Failed: The deployment lifecycle event has failed.</p> </li> <li> <p>Skipped: The deployment lifecycle event has been skipped.</p> </li> <li> <p>Unknown: The deployment lifecycle event is unknown.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the input of a ListApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationRevisionsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p><p>Whether to list revisions based on whether the revision is the target revision of an deployment group:</p> <ul> <li> <p>include: List revisions that are target revisions of a deployment group.</p> </li> <li> <p>exclude: Do not list revisions that are target revisions of a deployment group.</p> </li> <li> <p>ignore: List all revisions.</p> </li> </ul></p>
    #[serde(rename = "deployed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed: Option<String>,
    /// <p>An identifier returned from the previous list application revisions call. It can be used to return the next set of applications in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An Amazon S3 bucket name to limit the search for revisions.</p> <p>If set to null, all of the user's buckets will be searched.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p>A key prefix for the set of Amazon S3 objects to limit the search for revisions.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
    /// <p>The column name to use to sort the list results:</p> <ul> <li> <p>registerTime: Sort by the time the revisions were registered with AWS CodeDeploy.</p> </li> <li> <p>firstUsedTime: Sort by the time the revisions were first used in a deployment.</p> </li> <li> <p>lastUsedTime: Sort by the time the revisions were last used in a deployment.</p> </li> </ul> <p>If not specified or set to null, the results will be returned in an arbitrary order.</p>
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The order in which to sort the list results:</p> <ul> <li> <p>ascending: ascending order.</p> </li> <li> <p>descending: descending order.</p> </li> </ul> <p>If not specified, the results will be sorted in ascending order.</p> <p>If set to null, the results will be sorted in an arbitrary order.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the output of a ListApplicationRevisions operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListApplicationRevisionsOutput {
    /// <p>If a large amount of information is returned, an identifier will also be returned. It can be used in a subsequent list application revisions call to return the next set of application revisions in the list.</p>
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
pub struct ListApplicationsInput {
    /// <p>An identifier returned from the previous list applications call. It can be used to return the next set of applications in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListApplications operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListApplicationsOutput {
    /// <p>A list of application names.</p>
    #[serde(rename = "applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list applications call to return the next set of applications, will also be returned. in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListDeploymentConfigs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeploymentConfigsInput {
    /// <p>An identifier returned from the previous list deployment configurations call. It can be used to return the next set of deployment configurations in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeploymentConfigs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ListDeploymentGroupsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListDeploymentGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeploymentGroupsOutput {
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>A list of corresponding deployment group names.</p>
    #[serde(rename = "deploymentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups: Option<Vec<String>>,
    /// <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment groups call to return the next set of deployment groups in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input of a ListDeploymentInstances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeploymentInstancesInput {
    /// <p>The unique ID of a deployment.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p><p>A subset of instances to list by status:</p> <ul> <li> <p>Pending: Include those instance with pending deployments.</p> </li> <li> <p>InProgress: Include those instance where deployments are still in progress.</p> </li> <li> <p>Succeeded: Include those instances with successful deployments.</p> </li> <li> <p>Failed: Include those instance with failed deployments.</p> </li> <li> <p>Skipped: Include those instance with skipped deployments.</p> </li> <li> <p>Unknown: Include those instance with deployments in an unknown state.</p> </li> </ul></p>
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

/// <p>Represents the input of a ListDeployments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeploymentsInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// <p>A time range (start and end) for returning a subset of the list of deployments.</p>
    #[serde(rename = "createTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<TimeRange>,
    /// <p>The name of an existing deployment group for the specified application.</p>
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
pub struct ListGitHubAccountTokenNamesInput {
    /// <p>An identifier returned from the previous ListGitHubAccountTokenNames call. It can be used to return the next set of names in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the output of a ListGitHubAccountTokenNames operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct ListOnPremisesInstancesInput {
    /// <p>An identifier returned from the previous list on-premises instances call. It can be used to return the next set of on-premises instances in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The registration status of the on-premises instances:</p> <ul> <li> <p>Deregistered: Include deregistered on-premises instances in the resulting list.</p> </li> <li> <p>Registered: Include registered on-premises instances in the resulting list.</p> </li> </ul></p>
    #[serde(rename = "registrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
    /// <p>The on-premises instance tags that will be used to restrict the corresponding on-premises instance names returned.</p>
    #[serde(rename = "tagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

/// <p>Represents the output of list on-premises instances operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about the Elastic Load Balancing load balancer or target group used in a deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerInfo {
    /// <p><p>An array containing information about the load balancer to use for load balancing in a deployment. In Elastic Load Balancing, load balancers are used with Classic Load Balancers.</p> <note> <p> Adding more than one load balancer to the array is not supported. </p> </note></p>
    #[serde(rename = "elbInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elb_info_list: Option<Vec<ELBInfo>>,
    /// <p><p>An array containing information about the target group to use for load balancing in a deployment. In Elastic Load Balancing, target groups are used with Application Load Balancers.</p> <note> <p> Adding more than one target group to the array is not supported. </p> </note></p>
    #[serde(rename = "targetGroupInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_info_list: Option<Vec<TargetGroupInfo>>,
}

/// <p>Information about minimum healthy instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimumHealthyHosts {
    /// <p>The minimum healthy instance type:</p> <ul> <li> <p>HOST_COUNT: The minimum number of healthy instance as an absolute value.</p> </li> <li> <p>FLEET_PERCENT: The minimum number of healthy instance as a percentage of the total number of instance in the deployment.</p> </li> </ul> <p>In an example of nine instance, if a HOST_COUNT of six is specified, deploy to up to three instances at a time. The deployment will be successful if six or more instances are deployed to successfully; otherwise, the deployment fails. If a FLEET_PERCENT of 40 is specified, deploy to up to five instance at a time. The deployment will be successful if four or more instance are deployed to successfully; otherwise, the deployment fails.</p> <note> <p>In a call to the get deployment configuration operation, CodeDeployDefault.OneAtATime will return a minimum healthy instance type of MOST_CONCURRENCY and a value of 1. This means a deployment to only one instance at a time. (You cannot set the type to MOST_CONCURRENCY, only to HOST_COUNT or FLEET_PERCENT.) In addition, with CodeDeployDefault.OneAtATime, AWS CodeDeploy will try to ensure that all instances but one are kept in a healthy state during the deployment. Although this allows one instance at a time to be taken offline for a new deployment, it also means that if the deployment to the last instance fails, the overall deployment still succeeds.</p> </note> <p>For more information, see <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide/instances-health.html">AWS CodeDeploy Instance Health</a> in the <i>AWS CodeDeploy User Guide</i>.</p>
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
    /// <p>A list containing other lists of on-premises instance tag groups. In order for an instance to be included in the deployment group, it must be identified by all the tag groups in the list.</p>
    #[serde(rename = "onPremisesTagSetList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set_list: Option<Vec<Vec<TagFilter>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLifecycleEventHookExecutionStatusInput {
    /// <p>The ID of the deployment. Pass this ID to a Lambda function that validates a deployment lifecycle event.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The execution ID of a deployment's lifecycle hook. A deployment lifecycle hook is specified in the <code>hooks</code> section of the AppSpec file.</p>
    #[serde(rename = "lifecycleEventHookExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_hook_execution_id: Option<String>,
    /// <p>The result of a Lambda function that validates a deployment lifecycle event (<code>Succeeded</code> or <code>Failed</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The SHA256 hash value of the revision that is specified as a RawString.</p>
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_256: Option<String>,
}

/// <p>Represents the input of a RegisterApplicationRevision operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterApplicationRevisionInput {
    /// <p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>
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
    /// <p>Information about the location of application artifacts stored in GitHub.</p>
    #[serde(rename = "gitHubLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_location: Option<GitHubLocation>,
    /// <p><p>The type of application revision:</p> <ul> <li> <p>S3: An application revision stored in Amazon S3.</p> </li> <li> <p>GitHub: An application revision stored in GitHub (EC2/On-premises deployments only)</p> </li> <li> <p>String: A YAML-formatted or JSON-formatted string (AWS Lambda deployments only)</p> </li> </ul></p>
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
pub struct RollbackInfo {
    /// <p>The ID of the deployment rollback.</p>
    #[serde(rename = "rollbackDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_deployment_id: Option<String>,
    /// <p>Information describing the status of a deployment rollback; for example, whether the deployment can't be rolled back, is in progress, failed, or succeeded. </p>
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
    /// <p>The ETag of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the ETag is not specified as an input parameter, ETag validation of the object will be skipped.</p>
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The name of the Amazon S3 object that represents the bundled artifacts for the application revision.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A specific version of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the version is not specified, the system will use the most recent version by default.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SkipWaitTimeForInstanceTerminationInput {
    /// <p>The ID of the blue/green deployment for which you want to skip the instance termination wait time.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

/// <p>Represents the input of a StopDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopDeploymentInput {
    /// <p>Indicates, when a deployment is stopped, whether instances that have been updated should be rolled back to the previous version of the application revision.</p>
    #[serde(rename = "autoRollbackEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_enabled: Option<bool>,
    /// <p>The unique ID of a deployment.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
}

/// <p>Represents the output of a StopDeployment operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a target group in Elastic Load Balancing to use in a deployment. Instances are registered as targets in a target group, and traffic is routed to the target group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetGroupInfo {
    /// <p>For blue/green deployments, the name of the target group that instances in the original environment are deregistered from, and instances in the replacement environment registered with. For in-place deployments, the name of the target group that instances are deregistered from, so they are not serving traffic during a deployment, and then re-registered with after the deployment completes. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

/// <p>Represents the input of an UpdateApplication operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateDeploymentGroupInput {
    /// <p>Information to add or change about Amazon CloudWatch alarms when the deployment group is updated.</p>
    #[serde(rename = "alarmConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    /// <p>The application name corresponding to the deployment group to update.</p>
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
    /// <p>Information about groups of tags applied to on-premises instances. The deployment group will include only EC2 instances identified by all the tag groups.</p>
    #[serde(rename = "ec2TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_tag_set: Option<EC2TagSet>,
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
    /// <p>Information about an on-premises instance tag set. The deployment group will include only on-premises instances identified by all the tag groups.</p>
    #[serde(rename = "onPremisesTagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    /// <p>A replacement ARN for the service role, if you want to change it.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>Information about triggers to change when the deployment group is updated. For examples, see <a href="http://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-edit.html">Modify Triggers in an AWS CodeDeploy Deployment Group</a> in the AWS CodeDeploy User Guide.</p>
    #[serde(rename = "triggerConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

/// <p>Represents the output of an UpdateDeploymentGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>The specified tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
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

impl AddTagsToOnPremisesInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> AddTagsToOnPremisesInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceLimitExceededException" => {
                    return AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InstanceNameRequiredException" => {
                    return AddTagsToOnPremisesInstancesError::InstanceNameRequired(String::from(
                        error_message,
                    ))
                }
                "InstanceNotRegisteredException" => {
                    return AddTagsToOnPremisesInstancesError::InstanceNotRegistered(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return AddTagsToOnPremisesInstancesError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "InvalidTagException" => {
                    return AddTagsToOnPremisesInstancesError::InvalidTag(String::from(
                        error_message,
                    ))
                }
                "TagLimitExceededException" => {
                    return AddTagsToOnPremisesInstancesError::TagLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "TagRequiredException" => {
                    return AddTagsToOnPremisesInstancesError::TagRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AddTagsToOnPremisesInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AddTagsToOnPremisesInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AddTagsToOnPremisesInstancesError {
    fn from(err: serde_json::error::Error) -> AddTagsToOnPremisesInstancesError {
        AddTagsToOnPremisesInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToOnPremisesInstancesError {
    fn from(err: CredentialsError) -> AddTagsToOnPremisesInstancesError {
        AddTagsToOnPremisesInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToOnPremisesInstancesError {
    fn from(err: HttpDispatchError) -> AddTagsToOnPremisesInstancesError {
        AddTagsToOnPremisesInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToOnPremisesInstancesError {
    fn from(err: io::Error) -> AddTagsToOnPremisesInstancesError {
        AddTagsToOnPremisesInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToOnPremisesInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToOnPremisesInstancesError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::InstanceNotRegistered(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::InvalidInstanceName(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::InvalidTag(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::TagLimitExceeded(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::TagRequired(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::Validation(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::Credentials(ref err) => err.description(),
            AddTagsToOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToOnPremisesInstancesError::ParseError(ref cause) => cause,
            AddTagsToOnPremisesInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetApplicationRevisions
#[derive(Debug, PartialEq)]
pub enum BatchGetApplicationRevisionsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
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

impl BatchGetApplicationRevisionsError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetApplicationRevisionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return BatchGetApplicationRevisionsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return BatchGetApplicationRevisionsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "BatchLimitExceededException" => {
                    return BatchGetApplicationRevisionsError::BatchLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return BatchGetApplicationRevisionsError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidRevisionException" => {
                    return BatchGetApplicationRevisionsError::InvalidRevision(String::from(
                        error_message,
                    ))
                }
                "RevisionRequiredException" => {
                    return BatchGetApplicationRevisionsError::RevisionRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetApplicationRevisionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetApplicationRevisionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetApplicationRevisionsError {
    fn from(err: serde_json::error::Error) -> BatchGetApplicationRevisionsError {
        BatchGetApplicationRevisionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetApplicationRevisionsError {
    fn from(err: CredentialsError) -> BatchGetApplicationRevisionsError {
        BatchGetApplicationRevisionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetApplicationRevisionsError {
    fn from(err: HttpDispatchError) -> BatchGetApplicationRevisionsError {
        BatchGetApplicationRevisionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetApplicationRevisionsError {
    fn from(err: io::Error) -> BatchGetApplicationRevisionsError {
        BatchGetApplicationRevisionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetApplicationRevisionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetApplicationRevisionsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => cause,
            BatchGetApplicationRevisionsError::ApplicationNameRequired(ref cause) => cause,
            BatchGetApplicationRevisionsError::BatchLimitExceeded(ref cause) => cause,
            BatchGetApplicationRevisionsError::InvalidApplicationName(ref cause) => cause,
            BatchGetApplicationRevisionsError::InvalidRevision(ref cause) => cause,
            BatchGetApplicationRevisionsError::RevisionRequired(ref cause) => cause,
            BatchGetApplicationRevisionsError::Validation(ref cause) => cause,
            BatchGetApplicationRevisionsError::Credentials(ref err) => err.description(),
            BatchGetApplicationRevisionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetApplicationRevisionsError::ParseError(ref cause) => cause,
            BatchGetApplicationRevisionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetApplications
#[derive(Debug, PartialEq)]
pub enum BatchGetApplicationsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
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

impl BatchGetApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetApplicationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return BatchGetApplicationsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return BatchGetApplicationsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "BatchLimitExceededException" => {
                    return BatchGetApplicationsError::BatchLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return BatchGetApplicationsError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetApplicationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetApplicationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetApplicationsError {
    fn from(err: serde_json::error::Error) -> BatchGetApplicationsError {
        BatchGetApplicationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetApplicationsError {
    fn from(err: CredentialsError) -> BatchGetApplicationsError {
        BatchGetApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetApplicationsError {
    fn from(err: HttpDispatchError) -> BatchGetApplicationsError {
        BatchGetApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetApplicationsError {
    fn from(err: io::Error) -> BatchGetApplicationsError {
        BatchGetApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetApplicationsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetApplicationsError::ApplicationDoesNotExist(ref cause) => cause,
            BatchGetApplicationsError::ApplicationNameRequired(ref cause) => cause,
            BatchGetApplicationsError::BatchLimitExceeded(ref cause) => cause,
            BatchGetApplicationsError::InvalidApplicationName(ref cause) => cause,
            BatchGetApplicationsError::Validation(ref cause) => cause,
            BatchGetApplicationsError::Credentials(ref err) => err.description(),
            BatchGetApplicationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetApplicationsError::ParseError(ref cause) => cause,
            BatchGetApplicationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetDeploymentGroups
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentGroupsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
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

impl BatchGetDeploymentGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetDeploymentGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return BatchGetDeploymentGroupsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return BatchGetDeploymentGroupsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "BatchLimitExceededException" => {
                    return BatchGetDeploymentGroupsError::BatchLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return BatchGetDeploymentGroupsError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetDeploymentGroupsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetDeploymentGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetDeploymentGroupsError {
    fn from(err: serde_json::error::Error) -> BatchGetDeploymentGroupsError {
        BatchGetDeploymentGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetDeploymentGroupsError {
    fn from(err: CredentialsError) -> BatchGetDeploymentGroupsError {
        BatchGetDeploymentGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetDeploymentGroupsError {
    fn from(err: HttpDispatchError) -> BatchGetDeploymentGroupsError {
        BatchGetDeploymentGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetDeploymentGroupsError {
    fn from(err: io::Error) -> BatchGetDeploymentGroupsError {
        BatchGetDeploymentGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetDeploymentGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetDeploymentGroupsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => cause,
            BatchGetDeploymentGroupsError::ApplicationNameRequired(ref cause) => cause,
            BatchGetDeploymentGroupsError::BatchLimitExceeded(ref cause) => cause,
            BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(ref cause) => cause,
            BatchGetDeploymentGroupsError::InvalidApplicationName(ref cause) => cause,
            BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(ref cause) => cause,
            BatchGetDeploymentGroupsError::Validation(ref cause) => cause,
            BatchGetDeploymentGroupsError::Credentials(ref err) => err.description(),
            BatchGetDeploymentGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetDeploymentGroupsError::ParseError(ref cause) => cause,
            BatchGetDeploymentGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetDeploymentInstances
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentInstancesError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The instance ID was not specified.</p>
    InstanceIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
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

impl BatchGetDeploymentInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetDeploymentInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchLimitExceededException" => {
                    return BatchGetDeploymentInstancesError::BatchLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "DeploymentDoesNotExistException" => {
                    return BatchGetDeploymentInstancesError::DeploymentDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return BatchGetDeploymentInstancesError::DeploymentIdRequired(String::from(
                        error_message,
                    ))
                }
                "InstanceIdRequiredException" => {
                    return BatchGetDeploymentInstancesError::InstanceIdRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return BatchGetDeploymentInstancesError::InvalidDeploymentId(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return BatchGetDeploymentInstancesError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetDeploymentInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetDeploymentInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetDeploymentInstancesError {
    fn from(err: serde_json::error::Error) -> BatchGetDeploymentInstancesError {
        BatchGetDeploymentInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetDeploymentInstancesError {
    fn from(err: CredentialsError) -> BatchGetDeploymentInstancesError {
        BatchGetDeploymentInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetDeploymentInstancesError {
    fn from(err: HttpDispatchError) -> BatchGetDeploymentInstancesError {
        BatchGetDeploymentInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetDeploymentInstancesError {
    fn from(err: io::Error) -> BatchGetDeploymentInstancesError {
        BatchGetDeploymentInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetDeploymentInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetDeploymentInstancesError {
    fn description(&self) -> &str {
        match *self {
            BatchGetDeploymentInstancesError::BatchLimitExceeded(ref cause) => cause,
            BatchGetDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => cause,
            BatchGetDeploymentInstancesError::DeploymentIdRequired(ref cause) => cause,
            BatchGetDeploymentInstancesError::InstanceIdRequired(ref cause) => cause,
            BatchGetDeploymentInstancesError::InvalidDeploymentId(ref cause) => cause,
            BatchGetDeploymentInstancesError::InvalidInstanceName(ref cause) => cause,
            BatchGetDeploymentInstancesError::Validation(ref cause) => cause,
            BatchGetDeploymentInstancesError::Credentials(ref err) => err.description(),
            BatchGetDeploymentInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetDeploymentInstancesError::ParseError(ref cause) => cause,
            BatchGetDeploymentInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetDeployments
#[derive(Debug, PartialEq)]
pub enum BatchGetDeploymentsError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
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

impl BatchGetDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetDeploymentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchLimitExceededException" => {
                    return BatchGetDeploymentsError::BatchLimitExceeded(String::from(error_message))
                }
                "DeploymentIdRequiredException" => {
                    return BatchGetDeploymentsError::DeploymentIdRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return BatchGetDeploymentsError::InvalidDeploymentId(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetDeploymentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetDeploymentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetDeploymentsError {
    fn from(err: serde_json::error::Error) -> BatchGetDeploymentsError {
        BatchGetDeploymentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetDeploymentsError {
    fn from(err: CredentialsError) -> BatchGetDeploymentsError {
        BatchGetDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetDeploymentsError {
    fn from(err: HttpDispatchError) -> BatchGetDeploymentsError {
        BatchGetDeploymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetDeploymentsError {
    fn from(err: io::Error) -> BatchGetDeploymentsError {
        BatchGetDeploymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetDeploymentsError::BatchLimitExceeded(ref cause) => cause,
            BatchGetDeploymentsError::DeploymentIdRequired(ref cause) => cause,
            BatchGetDeploymentsError::InvalidDeploymentId(ref cause) => cause,
            BatchGetDeploymentsError::Validation(ref cause) => cause,
            BatchGetDeploymentsError::Credentials(ref err) => err.description(),
            BatchGetDeploymentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetDeploymentsError::ParseError(ref cause) => cause,
            BatchGetDeploymentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchGetOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum BatchGetOnPremisesInstancesError {
    /// <p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
    BatchLimitExceeded(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
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

impl BatchGetOnPremisesInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetOnPremisesInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchLimitExceededException" => {
                    return BatchGetOnPremisesInstancesError::BatchLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InstanceNameRequiredException" => {
                    return BatchGetOnPremisesInstancesError::InstanceNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return BatchGetOnPremisesInstancesError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchGetOnPremisesInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchGetOnPremisesInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetOnPremisesInstancesError {
    fn from(err: serde_json::error::Error) -> BatchGetOnPremisesInstancesError {
        BatchGetOnPremisesInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetOnPremisesInstancesError {
    fn from(err: CredentialsError) -> BatchGetOnPremisesInstancesError {
        BatchGetOnPremisesInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetOnPremisesInstancesError {
    fn from(err: HttpDispatchError) -> BatchGetOnPremisesInstancesError {
        BatchGetOnPremisesInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetOnPremisesInstancesError {
    fn from(err: io::Error) -> BatchGetOnPremisesInstancesError {
        BatchGetOnPremisesInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetOnPremisesInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetOnPremisesInstancesError {
    fn description(&self) -> &str {
        match *self {
            BatchGetOnPremisesInstancesError::BatchLimitExceeded(ref cause) => cause,
            BatchGetOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,
            BatchGetOnPremisesInstancesError::InvalidInstanceName(ref cause) => cause,
            BatchGetOnPremisesInstancesError::Validation(ref cause) => cause,
            BatchGetOnPremisesInstancesError::Credentials(ref err) => err.description(),
            BatchGetOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetOnPremisesInstancesError::ParseError(ref cause) => cause,
            BatchGetOnPremisesInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ContinueDeployment
#[derive(Debug, PartialEq)]
pub enum ContinueDeploymentError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The deployment does not have a status of Ready and can't continue yet.</p>
    DeploymentIsNotInReadyState(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>A call was submitted that is not supported for the specified deployment type.</p>
    UnsupportedActionForDeploymentType(String),
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

impl ContinueDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> ContinueDeploymentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentAlreadyCompletedException" => {
                    return ContinueDeploymentError::DeploymentAlreadyCompleted(String::from(
                        error_message,
                    ))
                }
                "DeploymentDoesNotExistException" => {
                    return ContinueDeploymentError::DeploymentDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return ContinueDeploymentError::DeploymentIdRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentIsNotInReadyStateException" => {
                    return ContinueDeploymentError::DeploymentIsNotInReadyState(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return ContinueDeploymentError::InvalidDeploymentId(String::from(error_message))
                }
                "UnsupportedActionForDeploymentTypeException" => {
                    return ContinueDeploymentError::UnsupportedActionForDeploymentType(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return ContinueDeploymentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ContinueDeploymentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ContinueDeploymentError {
    fn from(err: serde_json::error::Error) -> ContinueDeploymentError {
        ContinueDeploymentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ContinueDeploymentError {
    fn from(err: CredentialsError) -> ContinueDeploymentError {
        ContinueDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ContinueDeploymentError {
    fn from(err: HttpDispatchError) -> ContinueDeploymentError {
        ContinueDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for ContinueDeploymentError {
    fn from(err: io::Error) -> ContinueDeploymentError {
        ContinueDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ContinueDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ContinueDeploymentError {
    fn description(&self) -> &str {
        match *self {
            ContinueDeploymentError::DeploymentAlreadyCompleted(ref cause) => cause,
            ContinueDeploymentError::DeploymentDoesNotExist(ref cause) => cause,
            ContinueDeploymentError::DeploymentIdRequired(ref cause) => cause,
            ContinueDeploymentError::DeploymentIsNotInReadyState(ref cause) => cause,
            ContinueDeploymentError::InvalidDeploymentId(ref cause) => cause,
            ContinueDeploymentError::UnsupportedActionForDeploymentType(ref cause) => cause,
            ContinueDeploymentError::Validation(ref cause) => cause,
            ContinueDeploymentError::Credentials(ref err) => err.description(),
            ContinueDeploymentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ContinueDeploymentError::ParseError(ref cause) => cause,
            ContinueDeploymentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>An application with the specified name already exists with the applicable IAM user or AWS account.</p>
    ApplicationAlreadyExists(String),
    /// <p>More applications were attempted to be created than are allowed.</p>
    ApplicationLimitExceeded(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The computePlatform is invalid. The computePlatform should be <code>Lambda</code> or <code>Server</code>.</p>
    InvalidComputePlatform(String),
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

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateApplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationAlreadyExistsException" => {
                    return CreateApplicationError::ApplicationAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ApplicationLimitExceededException" => {
                    return CreateApplicationError::ApplicationLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return CreateApplicationError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return CreateApplicationError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidComputePlatformException" => {
                    return CreateApplicationError::InvalidComputePlatform(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateApplicationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateApplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateApplicationError {
    fn from(err: serde_json::error::Error) -> CreateApplicationError {
        CreateApplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::ApplicationAlreadyExists(ref cause) => cause,
            CreateApplicationError::ApplicationLimitExceeded(ref cause) => cause,
            CreateApplicationError::ApplicationNameRequired(ref cause) => cause,
            CreateApplicationError::InvalidApplicationName(ref cause) => cause,
            CreateApplicationError::InvalidComputePlatform(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::ParseError(ref cause) => cause,
            CreateApplicationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The number of allowed deployments was exceeded.</p>
    DeploymentLimitExceeded(String),
    /// <p>The description is too long.</p>
    DescriptionTooLong(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid fileExistsBehavior option was specified to determine how AWS CodeDeploy handles files or directories that already exist in a deployment target location but weren't part of the previous successful deployment. Valid values include "DISALLOW", "OVERWRITE", and "RETAIN".</p>
    InvalidFileExistsBehavior(String),
    /// <p>The GitHub token is not valid.</p>
    InvalidGitHubAccountToken(String),
    /// <p>The IgnoreApplicationStopFailures value is invalid. For AWS Lambda deployments, <code>false</code> is expected. For EC2/On-premises deployments, <code>true</code> or <code>false</code> is expected.</p>
    InvalidIgnoreApplicationStopFailuresValue(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
    InvalidRole(String),
    /// <p><p>The target instance configuration is invalid. Possible causes include:</p> <ul> <li> <p>Configuration data for target instances was entered for an in-place deployment.</p> </li> <li> <p>The limit of 10 tags for a tag type was exceeded.</p> </li> <li> <p>The combined length of the tag names exceeded the limit. </p> </li> <li> <p>A specified tag is not currently applied to any instances.</p> </li> </ul></p>
    InvalidTargetInstances(String),
    /// <p>The UpdateOutdatedInstancesOnly value is invalid. For AWS Lambda deployments, <code>false</code> is expected. For EC2/On-premises deployments, <code>true</code> or <code>false</code> is expected.</p>
    InvalidUpdateOutdatedInstancesOnlyValue(String),
    /// <p>The named revision does not exist with the applicable IAM user or AWS account.</p>
    RevisionDoesNotExist(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
    /// <p>An API function was called too frequently.</p>
    Throttling(String),
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

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDeploymentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return CreateDeploymentError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return CreateDeploymentError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return CreateDeploymentError::DeploymentConfigDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return CreateDeploymentError::DeploymentGroupDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return CreateDeploymentError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentLimitExceededException" => {
                    return CreateDeploymentError::DeploymentLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "DescriptionTooLongException" => {
                    return CreateDeploymentError::DescriptionTooLong(String::from(error_message))
                }
                "InvalidApplicationNameException" => {
                    return CreateDeploymentError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoRollbackConfigException" => {
                    return CreateDeploymentError::InvalidAutoRollbackConfig(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoScalingGroupException" => {
                    return CreateDeploymentError::InvalidAutoScalingGroup(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return CreateDeploymentError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return CreateDeploymentError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "InvalidFileExistsBehaviorException" => {
                    return CreateDeploymentError::InvalidFileExistsBehavior(String::from(
                        error_message,
                    ))
                }
                "InvalidGitHubAccountTokenException" => {
                    return CreateDeploymentError::InvalidGitHubAccountToken(String::from(
                        error_message,
                    ))
                }
                "InvalidIgnoreApplicationStopFailuresValueException" => {
                    return CreateDeploymentError::InvalidIgnoreApplicationStopFailuresValue(
                        String::from(error_message),
                    )
                }
                "InvalidLoadBalancerInfoException" => {
                    return CreateDeploymentError::InvalidLoadBalancerInfo(String::from(
                        error_message,
                    ))
                }
                "InvalidRevisionException" => {
                    return CreateDeploymentError::InvalidRevision(String::from(error_message))
                }
                "InvalidRoleException" => {
                    return CreateDeploymentError::InvalidRole(String::from(error_message))
                }
                "InvalidTargetInstancesException" => {
                    return CreateDeploymentError::InvalidTargetInstances(String::from(
                        error_message,
                    ))
                }
                "InvalidUpdateOutdatedInstancesOnlyValueException" => {
                    return CreateDeploymentError::InvalidUpdateOutdatedInstancesOnlyValue(
                        String::from(error_message),
                    )
                }
                "RevisionDoesNotExistException" => {
                    return CreateDeploymentError::RevisionDoesNotExist(String::from(error_message))
                }
                "RevisionRequiredException" => {
                    return CreateDeploymentError::RevisionRequired(String::from(error_message))
                }
                "ThrottlingException" => {
                    return CreateDeploymentError::Throttling(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateDeploymentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateDeploymentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDeploymentError {
    fn from(err: serde_json::error::Error) -> CreateDeploymentError {
        CreateDeploymentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeploymentError {
    fn from(err: CredentialsError) -> CreateDeploymentError {
        CreateDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeploymentError {
    fn from(err: HttpDispatchError) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeploymentError {
    fn from(err: io::Error) -> CreateDeploymentError {
        CreateDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentError::ApplicationDoesNotExist(ref cause) => cause,
            CreateDeploymentError::ApplicationNameRequired(ref cause) => cause,
            CreateDeploymentError::DeploymentConfigDoesNotExist(ref cause) => cause,
            CreateDeploymentError::DeploymentGroupDoesNotExist(ref cause) => cause,
            CreateDeploymentError::DeploymentGroupNameRequired(ref cause) => cause,
            CreateDeploymentError::DeploymentLimitExceeded(ref cause) => cause,
            CreateDeploymentError::DescriptionTooLong(ref cause) => cause,
            CreateDeploymentError::InvalidApplicationName(ref cause) => cause,
            CreateDeploymentError::InvalidAutoRollbackConfig(ref cause) => cause,
            CreateDeploymentError::InvalidAutoScalingGroup(ref cause) => cause,
            CreateDeploymentError::InvalidDeploymentConfigName(ref cause) => cause,
            CreateDeploymentError::InvalidDeploymentGroupName(ref cause) => cause,
            CreateDeploymentError::InvalidFileExistsBehavior(ref cause) => cause,
            CreateDeploymentError::InvalidGitHubAccountToken(ref cause) => cause,
            CreateDeploymentError::InvalidIgnoreApplicationStopFailuresValue(ref cause) => cause,
            CreateDeploymentError::InvalidLoadBalancerInfo(ref cause) => cause,
            CreateDeploymentError::InvalidRevision(ref cause) => cause,
            CreateDeploymentError::InvalidRole(ref cause) => cause,
            CreateDeploymentError::InvalidTargetInstances(ref cause) => cause,
            CreateDeploymentError::InvalidUpdateOutdatedInstancesOnlyValue(ref cause) => cause,
            CreateDeploymentError::RevisionDoesNotExist(ref cause) => cause,
            CreateDeploymentError::RevisionRequired(ref cause) => cause,
            CreateDeploymentError::Throttling(ref cause) => cause,
            CreateDeploymentError::Validation(ref cause) => cause,
            CreateDeploymentError::Credentials(ref err) => err.description(),
            CreateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDeploymentError::ParseError(ref cause) => cause,
            CreateDeploymentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDeploymentConfig
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentConfigError {
    /// <p>A deployment configuration with the specified name already exists with the applicable IAM user or AWS account.</p>
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

impl CreateDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDeploymentConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentConfigAlreadyExistsException" => {
                    return CreateDeploymentConfigError::DeploymentConfigAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigLimitExceededException" => {
                    return CreateDeploymentConfigError::DeploymentConfigLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigNameRequiredException" => {
                    return CreateDeploymentConfigError::DeploymentConfigNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidComputePlatformException" => {
                    return CreateDeploymentConfigError::InvalidComputePlatform(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return CreateDeploymentConfigError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "InvalidMinimumHealthyHostValueException" => {
                    return CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(
                        String::from(error_message),
                    )
                }
                "InvalidTrafficRoutingConfigurationException" => {
                    return CreateDeploymentConfigError::InvalidTrafficRoutingConfiguration(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return CreateDeploymentConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateDeploymentConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDeploymentConfigError {
    fn from(err: serde_json::error::Error) -> CreateDeploymentConfigError {
        CreateDeploymentConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeploymentConfigError {
    fn from(err: CredentialsError) -> CreateDeploymentConfigError {
        CreateDeploymentConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeploymentConfigError {
    fn from(err: HttpDispatchError) -> CreateDeploymentConfigError {
        CreateDeploymentConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeploymentConfigError {
    fn from(err: io::Error) -> CreateDeploymentConfigError {
        CreateDeploymentConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeploymentConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentConfigError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentConfigError::DeploymentConfigAlreadyExists(ref cause) => cause,
            CreateDeploymentConfigError::DeploymentConfigLimitExceeded(ref cause) => cause,
            CreateDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,
            CreateDeploymentConfigError::InvalidComputePlatform(ref cause) => cause,
            CreateDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,
            CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(ref cause) => cause,
            CreateDeploymentConfigError::InvalidTrafficRoutingConfiguration(ref cause) => cause,
            CreateDeploymentConfigError::Validation(ref cause) => cause,
            CreateDeploymentConfigError::Credentials(ref err) => err.description(),
            CreateDeploymentConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDeploymentConfigError::ParseError(ref cause) => cause,
            CreateDeploymentConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentGroupError {
    /// <p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
    AlarmsLimitExceeded(String),
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>A deployment group with the specified name already exists with the applicable IAM user or AWS account.</p>
    DeploymentGroupAlreadyExists(String),
    /// <p> The deployment groups limit was exceeded.</p>
    DeploymentGroupLimitExceeded(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p><p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the 255 character limit.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled but the alarm list is empty.</p> </li> </ul></p>
    InvalidAlarmConfig(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The configuration for the blue/green deployment group was provided in an invalid format. For information about deployment configuration format, see <a>CreateDeploymentConfig</a>.</p>
    InvalidBlueGreenDeploymentConfiguration(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid deployment style was specified. Valid deployment types include "IN_PLACE" and "BLUE_GREEN". Valid deployment options include "WITH_TRAFFIC_CONTROL" and "WITHOUT_TRAFFIC_CONTROL".</p>
    InvalidDeploymentStyle(String),
    /// <p>A call was submitted that specified both Ec2TagFilters and Ec2TagSet, but only one of these data types can be used in a single call.</p>
    InvalidEC2TagCombination(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidEC2Tag(String),
    /// <p>The specified input was specified in an invalid format.</p>
    InvalidInput(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>A call was submitted that specified both OnPremisesTagFilters and OnPremisesTagSet, but only one of these data types can be used in a single call.</p>
    InvalidOnPremisesTagCombination(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
    InvalidRole(String),
    /// <p>The specified tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The trigger was specified in an invalid format.</p>
    InvalidTriggerConfig(String),
    /// <p>The limit for lifecycle hooks was exceeded.</p>
    LifecycleHookLimitExceeded(String),
    /// <p>The role ID was not specified.</p>
    RoleRequired(String),
    /// <p>The number of tag groups included in the tag set list exceeded the maximum allowed limit of 3.</p>
    TagSetListLimitExceeded(String),
    /// <p>The maximum allowed number of triggers was exceeded.</p>
    TriggerTargetsLimitExceeded(String),
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

impl CreateDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDeploymentGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlarmsLimitExceededException" => {
                    return CreateDeploymentGroupError::AlarmsLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return CreateDeploymentGroupError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return CreateDeploymentGroupError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return CreateDeploymentGroupError::DeploymentConfigDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupAlreadyExistsException" => {
                    return CreateDeploymentGroupError::DeploymentGroupAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupLimitExceededException" => {
                    return CreateDeploymentGroupError::DeploymentGroupLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return CreateDeploymentGroupError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidAlarmConfigException" => {
                    return CreateDeploymentGroupError::InvalidAlarmConfig(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return CreateDeploymentGroupError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoRollbackConfigException" => {
                    return CreateDeploymentGroupError::InvalidAutoRollbackConfig(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoScalingGroupException" => {
                    return CreateDeploymentGroupError::InvalidAutoScalingGroup(String::from(
                        error_message,
                    ))
                }
                "InvalidBlueGreenDeploymentConfigurationException" => {
                    return CreateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(
                        String::from(error_message),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return CreateDeploymentGroupError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return CreateDeploymentGroupError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentStyleException" => {
                    return CreateDeploymentGroupError::InvalidDeploymentStyle(String::from(
                        error_message,
                    ))
                }
                "InvalidEC2TagCombinationException" => {
                    return CreateDeploymentGroupError::InvalidEC2TagCombination(String::from(
                        error_message,
                    ))
                }
                "InvalidEC2TagException" => {
                    return CreateDeploymentGroupError::InvalidEC2Tag(String::from(error_message))
                }
                "InvalidInputException" => {
                    return CreateDeploymentGroupError::InvalidInput(String::from(error_message))
                }
                "InvalidLoadBalancerInfoException" => {
                    return CreateDeploymentGroupError::InvalidLoadBalancerInfo(String::from(
                        error_message,
                    ))
                }
                "InvalidOnPremisesTagCombinationException" => {
                    return CreateDeploymentGroupError::InvalidOnPremisesTagCombination(
                        String::from(error_message),
                    )
                }
                "InvalidRoleException" => {
                    return CreateDeploymentGroupError::InvalidRole(String::from(error_message))
                }
                "InvalidTagException" => {
                    return CreateDeploymentGroupError::InvalidTag(String::from(error_message))
                }
                "InvalidTriggerConfigException" => {
                    return CreateDeploymentGroupError::InvalidTriggerConfig(String::from(
                        error_message,
                    ))
                }
                "LifecycleHookLimitExceededException" => {
                    return CreateDeploymentGroupError::LifecycleHookLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "RoleRequiredException" => {
                    return CreateDeploymentGroupError::RoleRequired(String::from(error_message))
                }
                "TagSetListLimitExceededException" => {
                    return CreateDeploymentGroupError::TagSetListLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "TriggerTargetsLimitExceededException" => {
                    return CreateDeploymentGroupError::TriggerTargetsLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateDeploymentGroupError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateDeploymentGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDeploymentGroupError {
    fn from(err: serde_json::error::Error) -> CreateDeploymentGroupError {
        CreateDeploymentGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeploymentGroupError {
    fn from(err: CredentialsError) -> CreateDeploymentGroupError {
        CreateDeploymentGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeploymentGroupError {
    fn from(err: HttpDispatchError) -> CreateDeploymentGroupError {
        CreateDeploymentGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeploymentGroupError {
    fn from(err: io::Error) -> CreateDeploymentGroupError {
        CreateDeploymentGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeploymentGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => cause,
            CreateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,
            CreateDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,
            CreateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => cause,
            CreateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => cause,
            CreateDeploymentGroupError::DeploymentGroupLimitExceeded(ref cause) => cause,
            CreateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,
            CreateDeploymentGroupError::InvalidAlarmConfig(ref cause) => cause,
            CreateDeploymentGroupError::InvalidApplicationName(ref cause) => cause,
            CreateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => cause,
            CreateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => cause,
            CreateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(ref cause) => cause,
            CreateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => cause,
            CreateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,
            CreateDeploymentGroupError::InvalidDeploymentStyle(ref cause) => cause,
            CreateDeploymentGroupError::InvalidEC2TagCombination(ref cause) => cause,
            CreateDeploymentGroupError::InvalidEC2Tag(ref cause) => cause,
            CreateDeploymentGroupError::InvalidInput(ref cause) => cause,
            CreateDeploymentGroupError::InvalidLoadBalancerInfo(ref cause) => cause,
            CreateDeploymentGroupError::InvalidOnPremisesTagCombination(ref cause) => cause,
            CreateDeploymentGroupError::InvalidRole(ref cause) => cause,
            CreateDeploymentGroupError::InvalidTag(ref cause) => cause,
            CreateDeploymentGroupError::InvalidTriggerConfig(ref cause) => cause,
            CreateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => cause,
            CreateDeploymentGroupError::RoleRequired(ref cause) => cause,
            CreateDeploymentGroupError::TagSetListLimitExceeded(ref cause) => cause,
            CreateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => cause,
            CreateDeploymentGroupError::Validation(ref cause) => cause,
            CreateDeploymentGroupError::Credentials(ref err) => err.description(),
            CreateDeploymentGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDeploymentGroupError::ParseError(ref cause) => cause,
            CreateDeploymentGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
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

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteApplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationNameRequiredException" => {
                    return DeleteApplicationError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return DeleteApplicationError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteApplicationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteApplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteApplicationError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationError {
        DeleteApplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationError {
    fn from(err: CredentialsError) -> DeleteApplicationError {
        DeleteApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationError {
    fn from(err: io::Error) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::ApplicationNameRequired(ref cause) => cause,
            DeleteApplicationError::InvalidApplicationName(ref cause) => cause,
            DeleteApplicationError::Validation(ref cause) => cause,
            DeleteApplicationError::Credentials(ref err) => err.description(),
            DeleteApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationError::ParseError(ref cause) => cause,
            DeleteApplicationError::Unknown(_) => "unknown error",
        }
    }
}
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

impl DeleteDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDeploymentConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentConfigInUseException" => {
                    return DeleteDeploymentConfigError::DeploymentConfigInUse(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigNameRequiredException" => {
                    return DeleteDeploymentConfigError::DeploymentConfigNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return DeleteDeploymentConfigError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "InvalidOperationException" => {
                    return DeleteDeploymentConfigError::InvalidOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteDeploymentConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteDeploymentConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDeploymentConfigError {
    fn from(err: serde_json::error::Error) -> DeleteDeploymentConfigError {
        DeleteDeploymentConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeploymentConfigError {
    fn from(err: CredentialsError) -> DeleteDeploymentConfigError {
        DeleteDeploymentConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeploymentConfigError {
    fn from(err: HttpDispatchError) -> DeleteDeploymentConfigError {
        DeleteDeploymentConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeploymentConfigError {
    fn from(err: io::Error) -> DeleteDeploymentConfigError {
        DeleteDeploymentConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeploymentConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeploymentConfigError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeploymentConfigError::DeploymentConfigInUse(ref cause) => cause,
            DeleteDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,
            DeleteDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,
            DeleteDeploymentConfigError::InvalidOperation(ref cause) => cause,
            DeleteDeploymentConfigError::Validation(ref cause) => cause,
            DeleteDeploymentConfigError::Credentials(ref err) => err.description(),
            DeleteDeploymentConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDeploymentConfigError::ParseError(ref cause) => cause,
            DeleteDeploymentConfigError::Unknown(_) => "unknown error",
        }
    }
}
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
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
    InvalidRole(String),
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

impl DeleteDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDeploymentGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationNameRequiredException" => {
                    return DeleteDeploymentGroupError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return DeleteDeploymentGroupError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return DeleteDeploymentGroupError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return DeleteDeploymentGroupError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "InvalidRoleException" => {
                    return DeleteDeploymentGroupError::InvalidRole(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteDeploymentGroupError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteDeploymentGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDeploymentGroupError {
    fn from(err: serde_json::error::Error) -> DeleteDeploymentGroupError {
        DeleteDeploymentGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeploymentGroupError {
    fn from(err: CredentialsError) -> DeleteDeploymentGroupError {
        DeleteDeploymentGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeploymentGroupError {
    fn from(err: HttpDispatchError) -> DeleteDeploymentGroupError {
        DeleteDeploymentGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeploymentGroupError {
    fn from(err: io::Error) -> DeleteDeploymentGroupError {
        DeleteDeploymentGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeploymentGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeploymentGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,
            DeleteDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,
            DeleteDeploymentGroupError::InvalidApplicationName(ref cause) => cause,
            DeleteDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,
            DeleteDeploymentGroupError::InvalidRole(ref cause) => cause,
            DeleteDeploymentGroupError::Validation(ref cause) => cause,
            DeleteDeploymentGroupError::Credentials(ref err) => err.description(),
            DeleteDeploymentGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDeploymentGroupError::ParseError(ref cause) => cause,
            DeleteDeploymentGroupError::Unknown(_) => "unknown error",
        }
    }
}
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

impl DeleteGitHubAccountTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteGitHubAccountTokenError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "GitHubAccountTokenDoesNotExistException" => {
                    return DeleteGitHubAccountTokenError::GitHubAccountTokenDoesNotExist(
                        String::from(error_message),
                    )
                }
                "GitHubAccountTokenNameRequiredException" => {
                    return DeleteGitHubAccountTokenError::GitHubAccountTokenNameRequired(
                        String::from(error_message),
                    )
                }
                "InvalidGitHubAccountTokenNameException" => {
                    return DeleteGitHubAccountTokenError::InvalidGitHubAccountTokenName(
                        String::from(error_message),
                    )
                }
                "OperationNotSupportedException" => {
                    return DeleteGitHubAccountTokenError::OperationNotSupported(String::from(
                        error_message,
                    ))
                }
                "ResourceValidationException" => {
                    return DeleteGitHubAccountTokenError::ResourceValidation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteGitHubAccountTokenError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteGitHubAccountTokenError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteGitHubAccountTokenError {
    fn from(err: serde_json::error::Error) -> DeleteGitHubAccountTokenError {
        DeleteGitHubAccountTokenError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGitHubAccountTokenError {
    fn from(err: CredentialsError) -> DeleteGitHubAccountTokenError {
        DeleteGitHubAccountTokenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGitHubAccountTokenError {
    fn from(err: HttpDispatchError) -> DeleteGitHubAccountTokenError {
        DeleteGitHubAccountTokenError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGitHubAccountTokenError {
    fn from(err: io::Error) -> DeleteGitHubAccountTokenError {
        DeleteGitHubAccountTokenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGitHubAccountTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGitHubAccountTokenError {
    fn description(&self) -> &str {
        match *self {
            DeleteGitHubAccountTokenError::GitHubAccountTokenDoesNotExist(ref cause) => cause,
            DeleteGitHubAccountTokenError::GitHubAccountTokenNameRequired(ref cause) => cause,
            DeleteGitHubAccountTokenError::InvalidGitHubAccountTokenName(ref cause) => cause,
            DeleteGitHubAccountTokenError::OperationNotSupported(ref cause) => cause,
            DeleteGitHubAccountTokenError::ResourceValidation(ref cause) => cause,
            DeleteGitHubAccountTokenError::Validation(ref cause) => cause,
            DeleteGitHubAccountTokenError::Credentials(ref err) => err.description(),
            DeleteGitHubAccountTokenError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteGitHubAccountTokenError::ParseError(ref cause) => cause,
            DeleteGitHubAccountTokenError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeregisterOnPremisesInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterOnPremisesInstanceError {
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
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

impl DeregisterOnPremisesInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeregisterOnPremisesInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceNameRequiredException" => {
                    return DeregisterOnPremisesInstanceError::InstanceNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return DeregisterOnPremisesInstanceError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeregisterOnPremisesInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeregisterOnPremisesInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeregisterOnPremisesInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterOnPremisesInstanceError {
        DeregisterOnPremisesInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterOnPremisesInstanceError {
    fn from(err: CredentialsError) -> DeregisterOnPremisesInstanceError {
        DeregisterOnPremisesInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterOnPremisesInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterOnPremisesInstanceError {
        DeregisterOnPremisesInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterOnPremisesInstanceError {
    fn from(err: io::Error) -> DeregisterOnPremisesInstanceError {
        DeregisterOnPremisesInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterOnPremisesInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterOnPremisesInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,
            DeregisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,
            DeregisterOnPremisesInstanceError::Validation(ref cause) => cause,
            DeregisterOnPremisesInstanceError::Credentials(ref err) => err.description(),
            DeregisterOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterOnPremisesInstanceError::ParseError(ref cause) => cause,
            DeregisterOnPremisesInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
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

impl GetApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetApplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return GetApplicationError::ApplicationDoesNotExist(String::from(error_message))
                }
                "ApplicationNameRequiredException" => {
                    return GetApplicationError::ApplicationNameRequired(String::from(error_message))
                }
                "InvalidApplicationNameException" => {
                    return GetApplicationError::InvalidApplicationName(String::from(error_message))
                }
                "ValidationException" => {
                    return GetApplicationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApplicationError {
    fn from(err: serde_json::error::Error) -> GetApplicationError {
        GetApplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationError {
    fn from(err: CredentialsError) -> GetApplicationError {
        GetApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationError {
    fn from(err: HttpDispatchError) -> GetApplicationError {
        GetApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationError {
    fn from(err: io::Error) -> GetApplicationError {
        GetApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationError::ApplicationDoesNotExist(ref cause) => cause,
            GetApplicationError::ApplicationNameRequired(ref cause) => cause,
            GetApplicationError::InvalidApplicationName(ref cause) => cause,
            GetApplicationError::Validation(ref cause) => cause,
            GetApplicationError::Credentials(ref err) => err.description(),
            GetApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApplicationError::ParseError(ref cause) => cause,
            GetApplicationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApplicationRevision
#[derive(Debug, PartialEq)]
pub enum GetApplicationRevisionError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The revision was specified in an invalid format.</p>
    InvalidRevision(String),
    /// <p>The named revision does not exist with the applicable IAM user or AWS account.</p>
    RevisionDoesNotExist(String),
    /// <p>The revision ID was not specified.</p>
    RevisionRequired(String),
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

impl GetApplicationRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> GetApplicationRevisionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return GetApplicationRevisionError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return GetApplicationRevisionError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return GetApplicationRevisionError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidRevisionException" => {
                    return GetApplicationRevisionError::InvalidRevision(String::from(error_message))
                }
                "RevisionDoesNotExistException" => {
                    return GetApplicationRevisionError::RevisionDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "RevisionRequiredException" => {
                    return GetApplicationRevisionError::RevisionRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetApplicationRevisionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApplicationRevisionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApplicationRevisionError {
    fn from(err: serde_json::error::Error) -> GetApplicationRevisionError {
        GetApplicationRevisionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationRevisionError {
    fn from(err: CredentialsError) -> GetApplicationRevisionError {
        GetApplicationRevisionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationRevisionError {
    fn from(err: HttpDispatchError) -> GetApplicationRevisionError {
        GetApplicationRevisionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationRevisionError {
    fn from(err: io::Error) -> GetApplicationRevisionError {
        GetApplicationRevisionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationRevisionError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationRevisionError::ApplicationDoesNotExist(ref cause) => cause,
            GetApplicationRevisionError::ApplicationNameRequired(ref cause) => cause,
            GetApplicationRevisionError::InvalidApplicationName(ref cause) => cause,
            GetApplicationRevisionError::InvalidRevision(ref cause) => cause,
            GetApplicationRevisionError::RevisionDoesNotExist(ref cause) => cause,
            GetApplicationRevisionError::RevisionRequired(ref cause) => cause,
            GetApplicationRevisionError::Validation(ref cause) => cause,
            GetApplicationRevisionError::Credentials(ref err) => err.description(),
            GetApplicationRevisionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApplicationRevisionError::ParseError(ref cause) => cause,
            GetApplicationRevisionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
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

impl GetDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDeploymentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentDoesNotExistException" => {
                    return GetDeploymentError::DeploymentDoesNotExist(String::from(error_message))
                }
                "DeploymentIdRequiredException" => {
                    return GetDeploymentError::DeploymentIdRequired(String::from(error_message))
                }
                "InvalidDeploymentIdException" => {
                    return GetDeploymentError::InvalidDeploymentId(String::from(error_message))
                }
                "ValidationException" => {
                    return GetDeploymentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDeploymentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeploymentError {
    fn from(err: serde_json::error::Error) -> GetDeploymentError {
        GetDeploymentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentError {
    fn from(err: CredentialsError) -> GetDeploymentError {
        GetDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentError {
    fn from(err: HttpDispatchError) -> GetDeploymentError {
        GetDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentError {
    fn from(err: io::Error) -> GetDeploymentError {
        GetDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentError::DeploymentDoesNotExist(ref cause) => cause,
            GetDeploymentError::DeploymentIdRequired(ref cause) => cause,
            GetDeploymentError::InvalidDeploymentId(ref cause) => cause,
            GetDeploymentError::Validation(ref cause) => cause,
            GetDeploymentError::Credentials(ref err) => err.description(),
            GetDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeploymentError::ParseError(ref cause) => cause,
            GetDeploymentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDeploymentConfig
#[derive(Debug, PartialEq)]
pub enum GetDeploymentConfigError {
    /// <p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>The deployment configuration name was not specified.</p>
    DeploymentConfigNameRequired(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
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

impl GetDeploymentConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDeploymentConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentConfigDoesNotExistException" => {
                    return GetDeploymentConfigError::DeploymentConfigDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigNameRequiredException" => {
                    return GetDeploymentConfigError::DeploymentConfigNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentConfigNameException" => {
                    return GetDeploymentConfigError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetDeploymentConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDeploymentConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeploymentConfigError {
    fn from(err: serde_json::error::Error) -> GetDeploymentConfigError {
        GetDeploymentConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentConfigError {
    fn from(err: CredentialsError) -> GetDeploymentConfigError {
        GetDeploymentConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentConfigError {
    fn from(err: HttpDispatchError) -> GetDeploymentConfigError {
        GetDeploymentConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentConfigError {
    fn from(err: io::Error) -> GetDeploymentConfigError {
        GetDeploymentConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentConfigError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentConfigError::DeploymentConfigDoesNotExist(ref cause) => cause,
            GetDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,
            GetDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,
            GetDeploymentConfigError::Validation(ref cause) => cause,
            GetDeploymentConfigError::Credentials(ref err) => err.description(),
            GetDeploymentConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeploymentConfigError::ParseError(ref cause) => cause,
            GetDeploymentConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum GetDeploymentGroupError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
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

impl GetDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDeploymentGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return GetDeploymentGroupError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return GetDeploymentGroupError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return GetDeploymentGroupError::DeploymentGroupDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return GetDeploymentGroupError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return GetDeploymentGroupError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return GetDeploymentGroupError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetDeploymentGroupError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDeploymentGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeploymentGroupError {
    fn from(err: serde_json::error::Error) -> GetDeploymentGroupError {
        GetDeploymentGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentGroupError {
    fn from(err: CredentialsError) -> GetDeploymentGroupError {
        GetDeploymentGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentGroupError {
    fn from(err: HttpDispatchError) -> GetDeploymentGroupError {
        GetDeploymentGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentGroupError {
    fn from(err: io::Error) -> GetDeploymentGroupError {
        GetDeploymentGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentGroupError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,
            GetDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,
            GetDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => cause,
            GetDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,
            GetDeploymentGroupError::InvalidApplicationName(ref cause) => cause,
            GetDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,
            GetDeploymentGroupError::Validation(ref cause) => cause,
            GetDeploymentGroupError::Credentials(ref err) => err.description(),
            GetDeploymentGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeploymentGroupError::ParseError(ref cause) => cause,
            GetDeploymentGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDeploymentInstance
#[derive(Debug, PartialEq)]
pub enum GetDeploymentInstanceError {
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified instance does not exist in the deployment group.</p>
    InstanceDoesNotExist(String),
    /// <p>The instance ID was not specified.</p>
    InstanceIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
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

impl GetDeploymentInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetDeploymentInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentDoesNotExistException" => {
                    return GetDeploymentInstanceError::DeploymentDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return GetDeploymentInstanceError::DeploymentIdRequired(String::from(
                        error_message,
                    ))
                }
                "InstanceDoesNotExistException" => {
                    return GetDeploymentInstanceError::InstanceDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "InstanceIdRequiredException" => {
                    return GetDeploymentInstanceError::InstanceIdRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return GetDeploymentInstanceError::InvalidDeploymentId(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return GetDeploymentInstanceError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetDeploymentInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetDeploymentInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeploymentInstanceError {
    fn from(err: serde_json::error::Error) -> GetDeploymentInstanceError {
        GetDeploymentInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentInstanceError {
    fn from(err: CredentialsError) -> GetDeploymentInstanceError {
        GetDeploymentInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentInstanceError {
    fn from(err: HttpDispatchError) -> GetDeploymentInstanceError {
        GetDeploymentInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentInstanceError {
    fn from(err: io::Error) -> GetDeploymentInstanceError {
        GetDeploymentInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentInstanceError::DeploymentDoesNotExist(ref cause) => cause,
            GetDeploymentInstanceError::DeploymentIdRequired(ref cause) => cause,
            GetDeploymentInstanceError::InstanceDoesNotExist(ref cause) => cause,
            GetDeploymentInstanceError::InstanceIdRequired(ref cause) => cause,
            GetDeploymentInstanceError::InvalidDeploymentId(ref cause) => cause,
            GetDeploymentInstanceError::InvalidInstanceName(ref cause) => cause,
            GetDeploymentInstanceError::Validation(ref cause) => cause,
            GetDeploymentInstanceError::Credentials(ref err) => err.description(),
            GetDeploymentInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeploymentInstanceError::ParseError(ref cause) => cause,
            GetDeploymentInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetOnPremisesInstance
#[derive(Debug, PartialEq)]
pub enum GetOnPremisesInstanceError {
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance is not registered.</p>
    InstanceNotRegistered(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
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

impl GetOnPremisesInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> GetOnPremisesInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceNameRequiredException" => {
                    return GetOnPremisesInstanceError::InstanceNameRequired(String::from(
                        error_message,
                    ))
                }
                "InstanceNotRegisteredException" => {
                    return GetOnPremisesInstanceError::InstanceNotRegistered(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return GetOnPremisesInstanceError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetOnPremisesInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetOnPremisesInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetOnPremisesInstanceError {
    fn from(err: serde_json::error::Error) -> GetOnPremisesInstanceError {
        GetOnPremisesInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOnPremisesInstanceError {
    fn from(err: CredentialsError) -> GetOnPremisesInstanceError {
        GetOnPremisesInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOnPremisesInstanceError {
    fn from(err: HttpDispatchError) -> GetOnPremisesInstanceError {
        GetOnPremisesInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOnPremisesInstanceError {
    fn from(err: io::Error) -> GetOnPremisesInstanceError {
        GetOnPremisesInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOnPremisesInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOnPremisesInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,
            GetOnPremisesInstanceError::InstanceNotRegistered(ref cause) => cause,
            GetOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,
            GetOnPremisesInstanceError::Validation(ref cause) => cause,
            GetOnPremisesInstanceError::Credentials(ref err) => err.description(),
            GetOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetOnPremisesInstanceError::ParseError(ref cause) => cause,
            GetOnPremisesInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListApplicationRevisions
#[derive(Debug, PartialEq)]
pub enum ListApplicationRevisionsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
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

impl ListApplicationRevisionsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListApplicationRevisionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return ListApplicationRevisionsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return ListApplicationRevisionsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "BucketNameFilterRequiredException" => {
                    return ListApplicationRevisionsError::BucketNameFilterRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return ListApplicationRevisionsError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidBucketNameFilterException" => {
                    return ListApplicationRevisionsError::InvalidBucketNameFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidDeployedStateFilterException" => {
                    return ListApplicationRevisionsError::InvalidDeployedStateFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidKeyPrefixFilterException" => {
                    return ListApplicationRevisionsError::InvalidKeyPrefixFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return ListApplicationRevisionsError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "InvalidSortByException" => {
                    return ListApplicationRevisionsError::InvalidSortBy(String::from(error_message))
                }
                "InvalidSortOrderException" => {
                    return ListApplicationRevisionsError::InvalidSortOrder(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListApplicationRevisionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListApplicationRevisionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListApplicationRevisionsError {
    fn from(err: serde_json::error::Error) -> ListApplicationRevisionsError {
        ListApplicationRevisionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationRevisionsError {
    fn from(err: CredentialsError) -> ListApplicationRevisionsError {
        ListApplicationRevisionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationRevisionsError {
    fn from(err: HttpDispatchError) -> ListApplicationRevisionsError {
        ListApplicationRevisionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationRevisionsError {
    fn from(err: io::Error) -> ListApplicationRevisionsError {
        ListApplicationRevisionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationRevisionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationRevisionsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => cause,
            ListApplicationRevisionsError::ApplicationNameRequired(ref cause) => cause,
            ListApplicationRevisionsError::BucketNameFilterRequired(ref cause) => cause,
            ListApplicationRevisionsError::InvalidApplicationName(ref cause) => cause,
            ListApplicationRevisionsError::InvalidBucketNameFilter(ref cause) => cause,
            ListApplicationRevisionsError::InvalidDeployedStateFilter(ref cause) => cause,
            ListApplicationRevisionsError::InvalidKeyPrefixFilter(ref cause) => cause,
            ListApplicationRevisionsError::InvalidNextToken(ref cause) => cause,
            ListApplicationRevisionsError::InvalidSortBy(ref cause) => cause,
            ListApplicationRevisionsError::InvalidSortOrder(ref cause) => cause,
            ListApplicationRevisionsError::Validation(ref cause) => cause,
            ListApplicationRevisionsError::Credentials(ref err) => err.description(),
            ListApplicationRevisionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListApplicationRevisionsError::ParseError(ref cause) => cause,
            ListApplicationRevisionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The next token was specified in an invalid format.</p>
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

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListApplicationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListApplicationsError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return ListApplicationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListApplicationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListApplicationsError {
    fn from(err: serde_json::error::Error) -> ListApplicationsError {
        ListApplicationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationsError {
    fn from(err: CredentialsError) -> ListApplicationsError {
        ListApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationsError {
    fn from(err: HttpDispatchError) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationsError {
    fn from(err: io::Error) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::InvalidNextToken(ref cause) => cause,
            ListApplicationsError::Validation(ref cause) => cause,
            ListApplicationsError::Credentials(ref err) => err.description(),
            ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListApplicationsError::ParseError(ref cause) => cause,
            ListApplicationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeploymentConfigs
#[derive(Debug, PartialEq)]
pub enum ListDeploymentConfigsError {
    /// <p>The next token was specified in an invalid format.</p>
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

impl ListDeploymentConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDeploymentConfigsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListDeploymentConfigsError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return ListDeploymentConfigsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDeploymentConfigsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeploymentConfigsError {
    fn from(err: serde_json::error::Error) -> ListDeploymentConfigsError {
        ListDeploymentConfigsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeploymentConfigsError {
    fn from(err: CredentialsError) -> ListDeploymentConfigsError {
        ListDeploymentConfigsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeploymentConfigsError {
    fn from(err: HttpDispatchError) -> ListDeploymentConfigsError {
        ListDeploymentConfigsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeploymentConfigsError {
    fn from(err: io::Error) -> ListDeploymentConfigsError {
        ListDeploymentConfigsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeploymentConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentConfigsError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentConfigsError::InvalidNextToken(ref cause) => cause,
            ListDeploymentConfigsError::Validation(ref cause) => cause,
            ListDeploymentConfigsError::Credentials(ref err) => err.description(),
            ListDeploymentConfigsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeploymentConfigsError::ParseError(ref cause) => cause,
            ListDeploymentConfigsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeploymentGroups
#[derive(Debug, PartialEq)]
pub enum ListDeploymentGroupsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The next token was specified in an invalid format.</p>
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

impl ListDeploymentGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDeploymentGroupsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return ListDeploymentGroupsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return ListDeploymentGroupsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return ListDeploymentGroupsError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return ListDeploymentGroupsError::InvalidNextToken(String::from(error_message))
                }
                "ValidationException" => {
                    return ListDeploymentGroupsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDeploymentGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeploymentGroupsError {
    fn from(err: serde_json::error::Error) -> ListDeploymentGroupsError {
        ListDeploymentGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeploymentGroupsError {
    fn from(err: CredentialsError) -> ListDeploymentGroupsError {
        ListDeploymentGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeploymentGroupsError {
    fn from(err: HttpDispatchError) -> ListDeploymentGroupsError {
        ListDeploymentGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeploymentGroupsError {
    fn from(err: io::Error) -> ListDeploymentGroupsError {
        ListDeploymentGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeploymentGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => cause,
            ListDeploymentGroupsError::ApplicationNameRequired(ref cause) => cause,
            ListDeploymentGroupsError::InvalidApplicationName(ref cause) => cause,
            ListDeploymentGroupsError::InvalidNextToken(ref cause) => cause,
            ListDeploymentGroupsError::Validation(ref cause) => cause,
            ListDeploymentGroupsError::Credentials(ref err) => err.description(),
            ListDeploymentGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeploymentGroupsError::ParseError(ref cause) => cause,
            ListDeploymentGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeploymentInstances
#[derive(Debug, PartialEq)]
pub enum ListDeploymentInstancesError {
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
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

impl ListDeploymentInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDeploymentInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentDoesNotExistException" => {
                    return ListDeploymentInstancesError::DeploymentDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentIdRequiredException" => {
                    return ListDeploymentInstancesError::DeploymentIdRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentNotStartedException" => {
                    return ListDeploymentInstancesError::DeploymentNotStarted(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentIdException" => {
                    return ListDeploymentInstancesError::InvalidDeploymentId(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentInstanceTypeException" => {
                    return ListDeploymentInstancesError::InvalidDeploymentInstanceType(
                        String::from(error_message),
                    )
                }
                "InvalidInstanceStatusException" => {
                    return ListDeploymentInstancesError::InvalidInstanceStatus(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceTypeException" => {
                    return ListDeploymentInstancesError::InvalidInstanceType(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return ListDeploymentInstancesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListDeploymentInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDeploymentInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeploymentInstancesError {
    fn from(err: serde_json::error::Error) -> ListDeploymentInstancesError {
        ListDeploymentInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeploymentInstancesError {
    fn from(err: CredentialsError) -> ListDeploymentInstancesError {
        ListDeploymentInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeploymentInstancesError {
    fn from(err: HttpDispatchError) -> ListDeploymentInstancesError {
        ListDeploymentInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeploymentInstancesError {
    fn from(err: io::Error) -> ListDeploymentInstancesError {
        ListDeploymentInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeploymentInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => cause,
            ListDeploymentInstancesError::DeploymentIdRequired(ref cause) => cause,
            ListDeploymentInstancesError::DeploymentNotStarted(ref cause) => cause,
            ListDeploymentInstancesError::InvalidDeploymentId(ref cause) => cause,
            ListDeploymentInstancesError::InvalidDeploymentInstanceType(ref cause) => cause,
            ListDeploymentInstancesError::InvalidInstanceStatus(ref cause) => cause,
            ListDeploymentInstancesError::InvalidInstanceType(ref cause) => cause,
            ListDeploymentInstancesError::InvalidNextToken(ref cause) => cause,
            ListDeploymentInstancesError::Validation(ref cause) => cause,
            ListDeploymentInstancesError::Credentials(ref err) => err.description(),
            ListDeploymentInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeploymentInstancesError::ParseError(ref cause) => cause,
            ListDeploymentInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeployments
#[derive(Debug, PartialEq)]
pub enum ListDeploymentsError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
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

impl ListDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDeploymentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return ListDeploymentsError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return ListDeploymentsError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return ListDeploymentsError::DeploymentGroupDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return ListDeploymentsError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return ListDeploymentsError::InvalidApplicationName(String::from(error_message))
                }
                "InvalidDeploymentGroupNameException" => {
                    return ListDeploymentsError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentStatusException" => {
                    return ListDeploymentsError::InvalidDeploymentStatus(String::from(
                        error_message,
                    ))
                }
                "InvalidNextTokenException" => {
                    return ListDeploymentsError::InvalidNextToken(String::from(error_message))
                }
                "InvalidTimeRangeException" => {
                    return ListDeploymentsError::InvalidTimeRange(String::from(error_message))
                }
                "ValidationException" => {
                    return ListDeploymentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListDeploymentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeploymentsError {
    fn from(err: serde_json::error::Error) -> ListDeploymentsError {
        ListDeploymentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeploymentsError {
    fn from(err: CredentialsError) -> ListDeploymentsError {
        ListDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeploymentsError {
    fn from(err: HttpDispatchError) -> ListDeploymentsError {
        ListDeploymentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeploymentsError {
    fn from(err: io::Error) -> ListDeploymentsError {
        ListDeploymentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            ListDeploymentsError::ApplicationDoesNotExist(ref cause) => cause,
            ListDeploymentsError::ApplicationNameRequired(ref cause) => cause,
            ListDeploymentsError::DeploymentGroupDoesNotExist(ref cause) => cause,
            ListDeploymentsError::DeploymentGroupNameRequired(ref cause) => cause,
            ListDeploymentsError::InvalidApplicationName(ref cause) => cause,
            ListDeploymentsError::InvalidDeploymentGroupName(ref cause) => cause,
            ListDeploymentsError::InvalidDeploymentStatus(ref cause) => cause,
            ListDeploymentsError::InvalidNextToken(ref cause) => cause,
            ListDeploymentsError::InvalidTimeRange(ref cause) => cause,
            ListDeploymentsError::Validation(ref cause) => cause,
            ListDeploymentsError::Credentials(ref err) => err.description(),
            ListDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDeploymentsError::ParseError(ref cause) => cause,
            ListDeploymentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListGitHubAccountTokenNames
#[derive(Debug, PartialEq)]
pub enum ListGitHubAccountTokenNamesError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The API used does not support the deployment.</p>
    OperationNotSupported(String),
    /// <p>The specified resource could not be validated.</p>
    ResourceValidation(String),
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

impl ListGitHubAccountTokenNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListGitHubAccountTokenNamesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListGitHubAccountTokenNamesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "OperationNotSupportedException" => {
                    return ListGitHubAccountTokenNamesError::OperationNotSupported(String::from(
                        error_message,
                    ))
                }
                "ResourceValidationException" => {
                    return ListGitHubAccountTokenNamesError::ResourceValidation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListGitHubAccountTokenNamesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListGitHubAccountTokenNamesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListGitHubAccountTokenNamesError {
    fn from(err: serde_json::error::Error) -> ListGitHubAccountTokenNamesError {
        ListGitHubAccountTokenNamesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGitHubAccountTokenNamesError {
    fn from(err: CredentialsError) -> ListGitHubAccountTokenNamesError {
        ListGitHubAccountTokenNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGitHubAccountTokenNamesError {
    fn from(err: HttpDispatchError) -> ListGitHubAccountTokenNamesError {
        ListGitHubAccountTokenNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGitHubAccountTokenNamesError {
    fn from(err: io::Error) -> ListGitHubAccountTokenNamesError {
        ListGitHubAccountTokenNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGitHubAccountTokenNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGitHubAccountTokenNamesError {
    fn description(&self) -> &str {
        match *self {
            ListGitHubAccountTokenNamesError::InvalidNextToken(ref cause) => cause,
            ListGitHubAccountTokenNamesError::OperationNotSupported(ref cause) => cause,
            ListGitHubAccountTokenNamesError::ResourceValidation(ref cause) => cause,
            ListGitHubAccountTokenNamesError::Validation(ref cause) => cause,
            ListGitHubAccountTokenNamesError::Credentials(ref err) => err.description(),
            ListGitHubAccountTokenNamesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListGitHubAccountTokenNamesError::ParseError(ref cause) => cause,
            ListGitHubAccountTokenNamesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum ListOnPremisesInstancesError {
    /// <p>The next token was specified in an invalid format.</p>
    InvalidNextToken(String),
    /// <p>The registration status was specified in an invalid format.</p>
    InvalidRegistrationStatus(String),
    /// <p>The specified tag filter was specified in an invalid format.</p>
    InvalidTagFilter(String),
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

impl ListOnPremisesInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListOnPremisesInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidNextTokenException" => {
                    return ListOnPremisesInstancesError::InvalidNextToken(String::from(
                        error_message,
                    ))
                }
                "InvalidRegistrationStatusException" => {
                    return ListOnPremisesInstancesError::InvalidRegistrationStatus(String::from(
                        error_message,
                    ))
                }
                "InvalidTagFilterException" => {
                    return ListOnPremisesInstancesError::InvalidTagFilter(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListOnPremisesInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListOnPremisesInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListOnPremisesInstancesError {
    fn from(err: serde_json::error::Error) -> ListOnPremisesInstancesError {
        ListOnPremisesInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOnPremisesInstancesError {
    fn from(err: CredentialsError) -> ListOnPremisesInstancesError {
        ListOnPremisesInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOnPremisesInstancesError {
    fn from(err: HttpDispatchError) -> ListOnPremisesInstancesError {
        ListOnPremisesInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOnPremisesInstancesError {
    fn from(err: io::Error) -> ListOnPremisesInstancesError {
        ListOnPremisesInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOnPremisesInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOnPremisesInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListOnPremisesInstancesError::InvalidNextToken(ref cause) => cause,
            ListOnPremisesInstancesError::InvalidRegistrationStatus(ref cause) => cause,
            ListOnPremisesInstancesError::InvalidTagFilter(ref cause) => cause,
            ListOnPremisesInstancesError::Validation(ref cause) => cause,
            ListOnPremisesInstancesError::Credentials(ref err) => err.description(),
            ListOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOnPremisesInstancesError::ParseError(ref cause) => cause,
            ListOnPremisesInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutLifecycleEventHookExecutionStatus
#[derive(Debug, PartialEq)]
pub enum PutLifecycleEventHookExecutionStatusError {
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
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

impl PutLifecycleEventHookExecutionStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> PutLifecycleEventHookExecutionStatusError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                                "DeploymentDoesNotExistException" => return PutLifecycleEventHookExecutionStatusError::DeploymentDoesNotExist(String::from(error_message)),
"DeploymentIdRequiredException" => return PutLifecycleEventHookExecutionStatusError::DeploymentIdRequired(String::from(error_message)),
"InvalidDeploymentIdException" => return PutLifecycleEventHookExecutionStatusError::InvalidDeploymentId(String::from(error_message)),
"InvalidLifecycleEventHookExecutionIdException" => return PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionId(String::from(error_message)),
"InvalidLifecycleEventHookExecutionStatusException" => return PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionStatus(String::from(error_message)),
"LifecycleEventAlreadyCompletedException" => return PutLifecycleEventHookExecutionStatusError::LifecycleEventAlreadyCompleted(String::from(error_message)),
"UnsupportedActionForDeploymentTypeException" => return PutLifecycleEventHookExecutionStatusError::UnsupportedActionForDeploymentType(String::from(error_message)),
"ValidationException" => return PutLifecycleEventHookExecutionStatusError::Validation(error_message.to_string()),
_ => {}
                            }
        }
        return PutLifecycleEventHookExecutionStatusError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutLifecycleEventHookExecutionStatusError {
    fn from(err: serde_json::error::Error) -> PutLifecycleEventHookExecutionStatusError {
        PutLifecycleEventHookExecutionStatusError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutLifecycleEventHookExecutionStatusError {
    fn from(err: CredentialsError) -> PutLifecycleEventHookExecutionStatusError {
        PutLifecycleEventHookExecutionStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutLifecycleEventHookExecutionStatusError {
    fn from(err: HttpDispatchError) -> PutLifecycleEventHookExecutionStatusError {
        PutLifecycleEventHookExecutionStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutLifecycleEventHookExecutionStatusError {
    fn from(err: io::Error) -> PutLifecycleEventHookExecutionStatusError {
        PutLifecycleEventHookExecutionStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutLifecycleEventHookExecutionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLifecycleEventHookExecutionStatusError {
    fn description(&self) -> &str {
        match *self {
            PutLifecycleEventHookExecutionStatusError::DeploymentDoesNotExist(ref cause) => cause,
            PutLifecycleEventHookExecutionStatusError::DeploymentIdRequired(ref cause) => cause,
            PutLifecycleEventHookExecutionStatusError::InvalidDeploymentId(ref cause) => cause,
            PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionId(
                ref cause,
            ) => cause,
            PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionStatus(
                ref cause,
            ) => cause,
            PutLifecycleEventHookExecutionStatusError::LifecycleEventAlreadyCompleted(
                ref cause,
            ) => cause,
            PutLifecycleEventHookExecutionStatusError::UnsupportedActionForDeploymentType(
                ref cause,
            ) => cause,
            PutLifecycleEventHookExecutionStatusError::Validation(ref cause) => cause,
            PutLifecycleEventHookExecutionStatusError::Credentials(ref err) => err.description(),
            PutLifecycleEventHookExecutionStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutLifecycleEventHookExecutionStatusError::ParseError(ref cause) => cause,
            PutLifecycleEventHookExecutionStatusError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterApplicationRevision
#[derive(Debug, PartialEq)]
pub enum RegisterApplicationRevisionError {
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
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

impl RegisterApplicationRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterApplicationRevisionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationDoesNotExistException" => {
                    return RegisterApplicationRevisionError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return RegisterApplicationRevisionError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DescriptionTooLongException" => {
                    return RegisterApplicationRevisionError::DescriptionTooLong(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return RegisterApplicationRevisionError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidRevisionException" => {
                    return RegisterApplicationRevisionError::InvalidRevision(String::from(
                        error_message,
                    ))
                }
                "RevisionRequiredException" => {
                    return RegisterApplicationRevisionError::RevisionRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RegisterApplicationRevisionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RegisterApplicationRevisionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterApplicationRevisionError {
    fn from(err: serde_json::error::Error) -> RegisterApplicationRevisionError {
        RegisterApplicationRevisionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterApplicationRevisionError {
    fn from(err: CredentialsError) -> RegisterApplicationRevisionError {
        RegisterApplicationRevisionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterApplicationRevisionError {
    fn from(err: HttpDispatchError) -> RegisterApplicationRevisionError {
        RegisterApplicationRevisionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterApplicationRevisionError {
    fn from(err: io::Error) -> RegisterApplicationRevisionError {
        RegisterApplicationRevisionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterApplicationRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterApplicationRevisionError {
    fn description(&self) -> &str {
        match *self {
            RegisterApplicationRevisionError::ApplicationDoesNotExist(ref cause) => cause,
            RegisterApplicationRevisionError::ApplicationNameRequired(ref cause) => cause,
            RegisterApplicationRevisionError::DescriptionTooLong(ref cause) => cause,
            RegisterApplicationRevisionError::InvalidApplicationName(ref cause) => cause,
            RegisterApplicationRevisionError::InvalidRevision(ref cause) => cause,
            RegisterApplicationRevisionError::RevisionRequired(ref cause) => cause,
            RegisterApplicationRevisionError::Validation(ref cause) => cause,
            RegisterApplicationRevisionError::Credentials(ref err) => err.description(),
            RegisterApplicationRevisionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterApplicationRevisionError::ParseError(ref cause) => cause,
            RegisterApplicationRevisionError::Unknown(_) => "unknown error",
        }
    }
}
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
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>Both an IAM user ARN and an IAM session ARN were included in the request. Use only one ARN type.</p>
    MultipleIamArnsProvided(String),
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

impl RegisterOnPremisesInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RegisterOnPremisesInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "IamArnRequiredException" => {
                    return RegisterOnPremisesInstanceError::IamArnRequired(String::from(
                        error_message,
                    ))
                }
                "IamSessionArnAlreadyRegisteredException" => {
                    return RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(
                        String::from(error_message),
                    )
                }
                "IamUserArnAlreadyRegisteredException" => {
                    return RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(
                        String::from(error_message),
                    )
                }
                "IamUserArnRequiredException" => {
                    return RegisterOnPremisesInstanceError::IamUserArnRequired(String::from(
                        error_message,
                    ))
                }
                "InstanceNameAlreadyRegisteredException" => {
                    return RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(
                        String::from(error_message),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RegisterOnPremisesInstanceError::InstanceNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidIamSessionArnException" => {
                    return RegisterOnPremisesInstanceError::InvalidIamSessionArn(String::from(
                        error_message,
                    ))
                }
                "InvalidIamUserArnException" => {
                    return RegisterOnPremisesInstanceError::InvalidIamUserArn(String::from(
                        error_message,
                    ))
                }
                "InvalidInstanceNameException" => {
                    return RegisterOnPremisesInstanceError::InvalidInstanceName(String::from(
                        error_message,
                    ))
                }
                "MultipleIamArnsProvidedException" => {
                    return RegisterOnPremisesInstanceError::MultipleIamArnsProvided(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RegisterOnPremisesInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RegisterOnPremisesInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterOnPremisesInstanceError {
    fn from(err: serde_json::error::Error) -> RegisterOnPremisesInstanceError {
        RegisterOnPremisesInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterOnPremisesInstanceError {
    fn from(err: CredentialsError) -> RegisterOnPremisesInstanceError {
        RegisterOnPremisesInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterOnPremisesInstanceError {
    fn from(err: HttpDispatchError) -> RegisterOnPremisesInstanceError {
        RegisterOnPremisesInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterOnPremisesInstanceError {
    fn from(err: io::Error) -> RegisterOnPremisesInstanceError {
        RegisterOnPremisesInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterOnPremisesInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterOnPremisesInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterOnPremisesInstanceError::IamArnRequired(ref cause) => cause,
            RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(ref cause) => cause,
            RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(ref cause) => cause,
            RegisterOnPremisesInstanceError::IamUserArnRequired(ref cause) => cause,
            RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(ref cause) => cause,
            RegisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,
            RegisterOnPremisesInstanceError::InvalidIamSessionArn(ref cause) => cause,
            RegisterOnPremisesInstanceError::InvalidIamUserArn(ref cause) => cause,
            RegisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,
            RegisterOnPremisesInstanceError::MultipleIamArnsProvided(ref cause) => cause,
            RegisterOnPremisesInstanceError::Validation(ref cause) => cause,
            RegisterOnPremisesInstanceError::Credentials(ref err) => err.description(),
            RegisterOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterOnPremisesInstanceError::ParseError(ref cause) => cause,
            RegisterOnPremisesInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveTagsFromOnPremisesInstances
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromOnPremisesInstancesError {
    /// <p>The maximum number of allowed on-premises instances in a single call was exceeded.</p>
    InstanceLimitExceeded(String),
    /// <p>An on-premises instance name was not specified.</p>
    InstanceNameRequired(String),
    /// <p>The specified on-premises instance is not registered.</p>
    InstanceNotRegistered(String),
    /// <p>The specified on-premises instance name was specified in an invalid format.</p>
    InvalidInstanceName(String),
    /// <p>The specified tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
    /// <p>A tag was not specified.</p>
    TagRequired(String),
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

impl RemoveTagsFromOnPremisesInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RemoveTagsFromOnPremisesInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InstanceLimitExceededException" => {
                    return RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(
                        String::from(error_message),
                    )
                }
                "InstanceNameRequiredException" => {
                    return RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(
                        String::from(error_message),
                    )
                }
                "InstanceNotRegisteredException" => {
                    return RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(
                        String::from(error_message),
                    )
                }
                "InvalidInstanceNameException" => {
                    return RemoveTagsFromOnPremisesInstancesError::InvalidInstanceName(
                        String::from(error_message),
                    )
                }
                "InvalidTagException" => {
                    return RemoveTagsFromOnPremisesInstancesError::InvalidTag(String::from(
                        error_message,
                    ))
                }
                "TagLimitExceededException" => {
                    return RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "TagRequiredException" => {
                    return RemoveTagsFromOnPremisesInstancesError::TagRequired(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RemoveTagsFromOnPremisesInstancesError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return RemoveTagsFromOnPremisesInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromOnPremisesInstancesError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromOnPremisesInstancesError {
        RemoveTagsFromOnPremisesInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromOnPremisesInstancesError {
    fn from(err: CredentialsError) -> RemoveTagsFromOnPremisesInstancesError {
        RemoveTagsFromOnPremisesInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromOnPremisesInstancesError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromOnPremisesInstancesError {
        RemoveTagsFromOnPremisesInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromOnPremisesInstancesError {
    fn from(err: io::Error) -> RemoveTagsFromOnPremisesInstancesError {
        RemoveTagsFromOnPremisesInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromOnPremisesInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromOnPremisesInstancesError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::InvalidInstanceName(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::InvalidTag(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::TagRequired(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::Validation(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::Credentials(ref err) => err.description(),
            RemoveTagsFromOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromOnPremisesInstancesError::ParseError(ref cause) => cause,
            RemoveTagsFromOnPremisesInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SkipWaitTimeForInstanceTermination
#[derive(Debug, PartialEq)]
pub enum SkipWaitTimeForInstanceTerminationError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>The specified deployment has not started.</p>
    DeploymentNotStarted(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
    /// <p>A call was submitted that is not supported for the specified deployment type.</p>
    UnsupportedActionForDeploymentType(String),
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

impl SkipWaitTimeForInstanceTerminationError {
    pub fn from_response(res: BufferedHttpResponse) -> SkipWaitTimeForInstanceTerminationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                                "DeploymentAlreadyCompletedException" => return SkipWaitTimeForInstanceTerminationError::DeploymentAlreadyCompleted(String::from(error_message)),
"DeploymentDoesNotExistException" => return SkipWaitTimeForInstanceTerminationError::DeploymentDoesNotExist(String::from(error_message)),
"DeploymentIdRequiredException" => return SkipWaitTimeForInstanceTerminationError::DeploymentIdRequired(String::from(error_message)),
"DeploymentNotStartedException" => return SkipWaitTimeForInstanceTerminationError::DeploymentNotStarted(String::from(error_message)),
"InvalidDeploymentIdException" => return SkipWaitTimeForInstanceTerminationError::InvalidDeploymentId(String::from(error_message)),
"UnsupportedActionForDeploymentTypeException" => return SkipWaitTimeForInstanceTerminationError::UnsupportedActionForDeploymentType(String::from(error_message)),
"ValidationException" => return SkipWaitTimeForInstanceTerminationError::Validation(error_message.to_string()),
_ => {}
                            }
        }
        return SkipWaitTimeForInstanceTerminationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SkipWaitTimeForInstanceTerminationError {
    fn from(err: serde_json::error::Error) -> SkipWaitTimeForInstanceTerminationError {
        SkipWaitTimeForInstanceTerminationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SkipWaitTimeForInstanceTerminationError {
    fn from(err: CredentialsError) -> SkipWaitTimeForInstanceTerminationError {
        SkipWaitTimeForInstanceTerminationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SkipWaitTimeForInstanceTerminationError {
    fn from(err: HttpDispatchError) -> SkipWaitTimeForInstanceTerminationError {
        SkipWaitTimeForInstanceTerminationError::HttpDispatch(err)
    }
}
impl From<io::Error> for SkipWaitTimeForInstanceTerminationError {
    fn from(err: io::Error) -> SkipWaitTimeForInstanceTerminationError {
        SkipWaitTimeForInstanceTerminationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SkipWaitTimeForInstanceTerminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SkipWaitTimeForInstanceTerminationError {
    fn description(&self) -> &str {
        match *self {
            SkipWaitTimeForInstanceTerminationError::DeploymentAlreadyCompleted(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::DeploymentDoesNotExist(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::DeploymentIdRequired(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::DeploymentNotStarted(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::InvalidDeploymentId(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::UnsupportedActionForDeploymentType(
                ref cause,
            ) => cause,
            SkipWaitTimeForInstanceTerminationError::Validation(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::Credentials(ref err) => err.description(),
            SkipWaitTimeForInstanceTerminationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SkipWaitTimeForInstanceTerminationError::ParseError(ref cause) => cause,
            SkipWaitTimeForInstanceTerminationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopDeployment
#[derive(Debug, PartialEq)]
pub enum StopDeploymentError {
    /// <p>The deployment is already complete.</p>
    DeploymentAlreadyCompleted(String),
    /// <p>The deployment does not exist with the applicable IAM user or AWS account.</p>
    DeploymentDoesNotExist(String),
    /// <p>At least one deployment ID must be specified.</p>
    DeploymentIdRequired(String),
    /// <p>At least one of the deployment IDs was specified in an invalid format.</p>
    InvalidDeploymentId(String),
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

impl StopDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> StopDeploymentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeploymentAlreadyCompletedException" => {
                    return StopDeploymentError::DeploymentAlreadyCompleted(String::from(
                        error_message,
                    ))
                }
                "DeploymentDoesNotExistException" => {
                    return StopDeploymentError::DeploymentDoesNotExist(String::from(error_message))
                }
                "DeploymentIdRequiredException" => {
                    return StopDeploymentError::DeploymentIdRequired(String::from(error_message))
                }
                "InvalidDeploymentIdException" => {
                    return StopDeploymentError::InvalidDeploymentId(String::from(error_message))
                }
                "ValidationException" => {
                    return StopDeploymentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopDeploymentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopDeploymentError {
    fn from(err: serde_json::error::Error) -> StopDeploymentError {
        StopDeploymentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopDeploymentError {
    fn from(err: CredentialsError) -> StopDeploymentError {
        StopDeploymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopDeploymentError {
    fn from(err: HttpDispatchError) -> StopDeploymentError {
        StopDeploymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopDeploymentError {
    fn from(err: io::Error) -> StopDeploymentError {
        StopDeploymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopDeploymentError {
    fn description(&self) -> &str {
        match *self {
            StopDeploymentError::DeploymentAlreadyCompleted(ref cause) => cause,
            StopDeploymentError::DeploymentDoesNotExist(ref cause) => cause,
            StopDeploymentError::DeploymentIdRequired(ref cause) => cause,
            StopDeploymentError::InvalidDeploymentId(ref cause) => cause,
            StopDeploymentError::Validation(ref cause) => cause,
            StopDeploymentError::Credentials(ref err) => err.description(),
            StopDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopDeploymentError::ParseError(ref cause) => cause,
            StopDeploymentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>An application with the specified name already exists with the applicable IAM user or AWS account.</p>
    ApplicationAlreadyExists(String),
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
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

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ApplicationAlreadyExistsException" => {
                    return UpdateApplicationError::ApplicationAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return UpdateApplicationError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return UpdateApplicationError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return UpdateApplicationError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateApplicationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApplicationError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
        UpdateApplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::ApplicationAlreadyExists(ref cause) => cause,
            UpdateApplicationError::ApplicationDoesNotExist(ref cause) => cause,
            UpdateApplicationError::ApplicationNameRequired(ref cause) => cause,
            UpdateApplicationError::InvalidApplicationName(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::ParseError(ref cause) => cause,
            UpdateApplicationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDeploymentGroup
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentGroupError {
    /// <p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
    AlarmsLimitExceeded(String),
    /// <p>The application does not exist with the applicable IAM user or AWS account.</p>
    ApplicationDoesNotExist(String),
    /// <p>The minimum number of required application names was not specified.</p>
    ApplicationNameRequired(String),
    /// <p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
    DeploymentConfigDoesNotExist(String),
    /// <p>A deployment group with the specified name already exists with the applicable IAM user or AWS account.</p>
    DeploymentGroupAlreadyExists(String),
    /// <p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
    DeploymentGroupDoesNotExist(String),
    /// <p>The deployment group name was not specified.</p>
    DeploymentGroupNameRequired(String),
    /// <p><p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the 255 character limit.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled but the alarm list is empty.</p> </li> </ul></p>
    InvalidAlarmConfig(String),
    /// <p>The application name was specified in an invalid format.</p>
    InvalidApplicationName(String),
    /// <p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
    InvalidAutoRollbackConfig(String),
    /// <p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
    InvalidAutoScalingGroup(String),
    /// <p>The configuration for the blue/green deployment group was provided in an invalid format. For information about deployment configuration format, see <a>CreateDeploymentConfig</a>.</p>
    InvalidBlueGreenDeploymentConfiguration(String),
    /// <p>The deployment configuration name was specified in an invalid format.</p>
    InvalidDeploymentConfigName(String),
    /// <p>The deployment group name was specified in an invalid format.</p>
    InvalidDeploymentGroupName(String),
    /// <p>An invalid deployment style was specified. Valid deployment types include "IN_PLACE" and "BLUE_GREEN". Valid deployment options include "WITH_TRAFFIC_CONTROL" and "WITHOUT_TRAFFIC_CONTROL".</p>
    InvalidDeploymentStyle(String),
    /// <p>A call was submitted that specified both Ec2TagFilters and Ec2TagSet, but only one of these data types can be used in a single call.</p>
    InvalidEC2TagCombination(String),
    /// <p>The tag was specified in an invalid format.</p>
    InvalidEC2Tag(String),
    /// <p>The specified input was specified in an invalid format.</p>
    InvalidInput(String),
    /// <p>An invalid load balancer name, or no load balancer name, was specified.</p>
    InvalidLoadBalancerInfo(String),
    /// <p>A call was submitted that specified both OnPremisesTagFilters and OnPremisesTagSet, but only one of these data types can be used in a single call.</p>
    InvalidOnPremisesTagCombination(String),
    /// <p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
    InvalidRole(String),
    /// <p>The specified tag was specified in an invalid format.</p>
    InvalidTag(String),
    /// <p>The trigger was specified in an invalid format.</p>
    InvalidTriggerConfig(String),
    /// <p>The limit for lifecycle hooks was exceeded.</p>
    LifecycleHookLimitExceeded(String),
    /// <p>The number of tag groups included in the tag set list exceeded the maximum allowed limit of 3.</p>
    TagSetListLimitExceeded(String),
    /// <p>The maximum allowed number of triggers was exceeded.</p>
    TriggerTargetsLimitExceeded(String),
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

impl UpdateDeploymentGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDeploymentGroupError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AlarmsLimitExceededException" => {
                    return UpdateDeploymentGroupError::AlarmsLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ApplicationDoesNotExistException" => {
                    return UpdateDeploymentGroupError::ApplicationDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "ApplicationNameRequiredException" => {
                    return UpdateDeploymentGroupError::ApplicationNameRequired(String::from(
                        error_message,
                    ))
                }
                "DeploymentConfigDoesNotExistException" => {
                    return UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupAlreadyExistsException" => {
                    return UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupDoesNotExistException" => {
                    return UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(String::from(
                        error_message,
                    ))
                }
                "DeploymentGroupNameRequiredException" => {
                    return UpdateDeploymentGroupError::DeploymentGroupNameRequired(String::from(
                        error_message,
                    ))
                }
                "InvalidAlarmConfigException" => {
                    return UpdateDeploymentGroupError::InvalidAlarmConfig(String::from(
                        error_message,
                    ))
                }
                "InvalidApplicationNameException" => {
                    return UpdateDeploymentGroupError::InvalidApplicationName(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoRollbackConfigException" => {
                    return UpdateDeploymentGroupError::InvalidAutoRollbackConfig(String::from(
                        error_message,
                    ))
                }
                "InvalidAutoScalingGroupException" => {
                    return UpdateDeploymentGroupError::InvalidAutoScalingGroup(String::from(
                        error_message,
                    ))
                }
                "InvalidBlueGreenDeploymentConfigurationException" => {
                    return UpdateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(
                        String::from(error_message),
                    )
                }
                "InvalidDeploymentConfigNameException" => {
                    return UpdateDeploymentGroupError::InvalidDeploymentConfigName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentGroupNameException" => {
                    return UpdateDeploymentGroupError::InvalidDeploymentGroupName(String::from(
                        error_message,
                    ))
                }
                "InvalidDeploymentStyleException" => {
                    return UpdateDeploymentGroupError::InvalidDeploymentStyle(String::from(
                        error_message,
                    ))
                }
                "InvalidEC2TagCombinationException" => {
                    return UpdateDeploymentGroupError::InvalidEC2TagCombination(String::from(
                        error_message,
                    ))
                }
                "InvalidEC2TagException" => {
                    return UpdateDeploymentGroupError::InvalidEC2Tag(String::from(error_message))
                }
                "InvalidInputException" => {
                    return UpdateDeploymentGroupError::InvalidInput(String::from(error_message))
                }
                "InvalidLoadBalancerInfoException" => {
                    return UpdateDeploymentGroupError::InvalidLoadBalancerInfo(String::from(
                        error_message,
                    ))
                }
                "InvalidOnPremisesTagCombinationException" => {
                    return UpdateDeploymentGroupError::InvalidOnPremisesTagCombination(
                        String::from(error_message),
                    )
                }
                "InvalidRoleException" => {
                    return UpdateDeploymentGroupError::InvalidRole(String::from(error_message))
                }
                "InvalidTagException" => {
                    return UpdateDeploymentGroupError::InvalidTag(String::from(error_message))
                }
                "InvalidTriggerConfigException" => {
                    return UpdateDeploymentGroupError::InvalidTriggerConfig(String::from(
                        error_message,
                    ))
                }
                "LifecycleHookLimitExceededException" => {
                    return UpdateDeploymentGroupError::LifecycleHookLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "TagSetListLimitExceededException" => {
                    return UpdateDeploymentGroupError::TagSetListLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "TriggerTargetsLimitExceededException" => {
                    return UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateDeploymentGroupError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateDeploymentGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDeploymentGroupError {
    fn from(err: serde_json::error::Error) -> UpdateDeploymentGroupError {
        UpdateDeploymentGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeploymentGroupError {
    fn from(err: CredentialsError) -> UpdateDeploymentGroupError {
        UpdateDeploymentGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeploymentGroupError {
    fn from(err: HttpDispatchError) -> UpdateDeploymentGroupError {
        UpdateDeploymentGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeploymentGroupError {
    fn from(err: io::Error) -> UpdateDeploymentGroupError {
        UpdateDeploymentGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeploymentGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeploymentGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => cause,
            UpdateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,
            UpdateDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,
            UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => cause,
            UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => cause,
            UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => cause,
            UpdateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidAlarmConfig(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidApplicationName(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidBlueGreenDeploymentConfiguration(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidDeploymentStyle(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidEC2TagCombination(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidEC2Tag(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidInput(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidLoadBalancerInfo(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidOnPremisesTagCombination(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidRole(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidTag(ref cause) => cause,
            UpdateDeploymentGroupError::InvalidTriggerConfig(ref cause) => cause,
            UpdateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => cause,
            UpdateDeploymentGroupError::TagSetListLimitExceeded(ref cause) => cause,
            UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => cause,
            UpdateDeploymentGroupError::Validation(ref cause) => cause,
            UpdateDeploymentGroupError::Credentials(ref err) => err.description(),
            UpdateDeploymentGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDeploymentGroupError::ParseError(ref cause) => cause,
            UpdateDeploymentGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the CodeDeploy API. CodeDeploy clients implement this trait.
pub trait CodeDeploy {
    /// <p>Adds tags to on-premises instances.</p>
    fn add_tags_to_on_premises_instances(
        &self,
        input: AddTagsToOnPremisesInstancesInput,
    ) -> RusotoFuture<(), AddTagsToOnPremisesInstancesError>;

    /// <p>Gets information about one or more application revisions.</p>
    fn batch_get_application_revisions(
        &self,
        input: BatchGetApplicationRevisionsInput,
    ) -> RusotoFuture<BatchGetApplicationRevisionsOutput, BatchGetApplicationRevisionsError>;

    /// <p>Gets information about one or more applications.</p>
    fn batch_get_applications(
        &self,
        input: BatchGetApplicationsInput,
    ) -> RusotoFuture<BatchGetApplicationsOutput, BatchGetApplicationsError>;

    /// <p>Gets information about one or more deployment groups.</p>
    fn batch_get_deployment_groups(
        &self,
        input: BatchGetDeploymentGroupsInput,
    ) -> RusotoFuture<BatchGetDeploymentGroupsOutput, BatchGetDeploymentGroupsError>;

    /// <p>Gets information about one or more instance that are part of a deployment group.</p>
    fn batch_get_deployment_instances(
        &self,
        input: BatchGetDeploymentInstancesInput,
    ) -> RusotoFuture<BatchGetDeploymentInstancesOutput, BatchGetDeploymentInstancesError>;

    /// <p>Gets information about one or more deployments.</p>
    fn batch_get_deployments(
        &self,
        input: BatchGetDeploymentsInput,
    ) -> RusotoFuture<BatchGetDeploymentsOutput, BatchGetDeploymentsError>;

    /// <p>Gets information about one or more on-premises instances.</p>
    fn batch_get_on_premises_instances(
        &self,
        input: BatchGetOnPremisesInstancesInput,
    ) -> RusotoFuture<BatchGetOnPremisesInstancesOutput, BatchGetOnPremisesInstancesError>;

    /// <p>For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by registering instances in the replacement environment with the load balancer, can start as soon as all instances have a status of Ready.) </p>
    fn continue_deployment(
        &self,
        input: ContinueDeploymentInput,
    ) -> RusotoFuture<(), ContinueDeploymentError>;

    /// <p>Creates an application.</p>
    fn create_application(
        &self,
        input: CreateApplicationInput,
    ) -> RusotoFuture<CreateApplicationOutput, CreateApplicationError>;

    /// <p>Deploys an application revision through the specified deployment group.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentInput,
    ) -> RusotoFuture<CreateDeploymentOutput, CreateDeploymentError>;

    /// <p>Creates a deployment configuration.</p>
    fn create_deployment_config(
        &self,
        input: CreateDeploymentConfigInput,
    ) -> RusotoFuture<CreateDeploymentConfigOutput, CreateDeploymentConfigError>;

    /// <p>Creates a deployment group to which application revisions will be deployed.</p>
    fn create_deployment_group(
        &self,
        input: CreateDeploymentGroupInput,
    ) -> RusotoFuture<CreateDeploymentGroupOutput, CreateDeploymentGroupError>;

    /// <p>Deletes an application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationInput,
    ) -> RusotoFuture<(), DeleteApplicationError>;

    /// <p><p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note></p>
    fn delete_deployment_config(
        &self,
        input: DeleteDeploymentConfigInput,
    ) -> RusotoFuture<(), DeleteDeploymentConfigError>;

    /// <p>Deletes a deployment group.</p>
    fn delete_deployment_group(
        &self,
        input: DeleteDeploymentGroupInput,
    ) -> RusotoFuture<DeleteDeploymentGroupOutput, DeleteDeploymentGroupError>;

    /// <p>Deletes a GitHub account connection.</p>
    fn delete_git_hub_account_token(
        &self,
        input: DeleteGitHubAccountTokenInput,
    ) -> RusotoFuture<DeleteGitHubAccountTokenOutput, DeleteGitHubAccountTokenError>;

    /// <p>Deregisters an on-premises instance.</p>
    fn deregister_on_premises_instance(
        &self,
        input: DeregisterOnPremisesInstanceInput,
    ) -> RusotoFuture<(), DeregisterOnPremisesInstanceError>;

    /// <p>Gets information about an application.</p>
    fn get_application(
        &self,
        input: GetApplicationInput,
    ) -> RusotoFuture<GetApplicationOutput, GetApplicationError>;

    /// <p>Gets information about an application revision.</p>
    fn get_application_revision(
        &self,
        input: GetApplicationRevisionInput,
    ) -> RusotoFuture<GetApplicationRevisionOutput, GetApplicationRevisionError>;

    /// <p>Gets information about a deployment.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentInput,
    ) -> RusotoFuture<GetDeploymentOutput, GetDeploymentError>;

    /// <p>Gets information about a deployment configuration.</p>
    fn get_deployment_config(
        &self,
        input: GetDeploymentConfigInput,
    ) -> RusotoFuture<GetDeploymentConfigOutput, GetDeploymentConfigError>;

    /// <p>Gets information about a deployment group.</p>
    fn get_deployment_group(
        &self,
        input: GetDeploymentGroupInput,
    ) -> RusotoFuture<GetDeploymentGroupOutput, GetDeploymentGroupError>;

    /// <p>Gets information about an instance as part of a deployment.</p>
    fn get_deployment_instance(
        &self,
        input: GetDeploymentInstanceInput,
    ) -> RusotoFuture<GetDeploymentInstanceOutput, GetDeploymentInstanceError>;

    /// <p>Gets information about an on-premises instance.</p>
    fn get_on_premises_instance(
        &self,
        input: GetOnPremisesInstanceInput,
    ) -> RusotoFuture<GetOnPremisesInstanceOutput, GetOnPremisesInstanceError>;

    /// <p>Lists information about revisions for an application.</p>
    fn list_application_revisions(
        &self,
        input: ListApplicationRevisionsInput,
    ) -> RusotoFuture<ListApplicationRevisionsOutput, ListApplicationRevisionsError>;

    /// <p>Lists the applications registered with the applicable IAM user or AWS account.</p>
    fn list_applications(
        &self,
        input: ListApplicationsInput,
    ) -> RusotoFuture<ListApplicationsOutput, ListApplicationsError>;

    /// <p>Lists the deployment configurations with the applicable IAM user or AWS account.</p>
    fn list_deployment_configs(
        &self,
        input: ListDeploymentConfigsInput,
    ) -> RusotoFuture<ListDeploymentConfigsOutput, ListDeploymentConfigsError>;

    /// <p>Lists the deployment groups for an application registered with the applicable IAM user or AWS account.</p>
    fn list_deployment_groups(
        &self,
        input: ListDeploymentGroupsInput,
    ) -> RusotoFuture<ListDeploymentGroupsOutput, ListDeploymentGroupsError>;

    /// <p>Lists the instance for a deployment associated with the applicable IAM user or AWS account.</p>
    fn list_deployment_instances(
        &self,
        input: ListDeploymentInstancesInput,
    ) -> RusotoFuture<ListDeploymentInstancesOutput, ListDeploymentInstancesError>;

    /// <p>Lists the deployments in a deployment group for an application registered with the applicable IAM user or AWS account.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsInput,
    ) -> RusotoFuture<ListDeploymentsOutput, ListDeploymentsError>;

    /// <p>Lists the names of stored connections to GitHub accounts.</p>
    fn list_git_hub_account_token_names(
        &self,
        input: ListGitHubAccountTokenNamesInput,
    ) -> RusotoFuture<ListGitHubAccountTokenNamesOutput, ListGitHubAccountTokenNamesError>;

    /// <p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names will be listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>
    fn list_on_premises_instances(
        &self,
        input: ListOnPremisesInstancesInput,
    ) -> RusotoFuture<ListOnPremisesInstancesOutput, ListOnPremisesInstancesError>;

    /// <p>Sets the result of a Lambda validation function. The function validates one or both lifecycle events (<code>BeforeAllowTraffic</code> and <code>AfterAllowTraffic</code>) and returns <code>Succeeded</code> or <code>Failed</code>.</p>
    fn put_lifecycle_event_hook_execution_status(
        &self,
        input: PutLifecycleEventHookExecutionStatusInput,
    ) -> RusotoFuture<
        PutLifecycleEventHookExecutionStatusOutput,
        PutLifecycleEventHookExecutionStatusError,
    >;

    /// <p>Registers with AWS CodeDeploy a revision for the specified application.</p>
    fn register_application_revision(
        &self,
        input: RegisterApplicationRevisionInput,
    ) -> RusotoFuture<(), RegisterApplicationRevisionError>;

    /// <p><p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note></p>
    fn register_on_premises_instance(
        &self,
        input: RegisterOnPremisesInstanceInput,
    ) -> RusotoFuture<(), RegisterOnPremisesInstanceError>;

    /// <p>Removes one or more tags from one or more on-premises instances.</p>
    fn remove_tags_from_on_premises_instances(
        &self,
        input: RemoveTagsFromOnPremisesInstancesInput,
    ) -> RusotoFuture<(), RemoveTagsFromOnPremisesInstancesError>;

    /// <p>In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is completed.</p>
    fn skip_wait_time_for_instance_termination(
        &self,
        input: SkipWaitTimeForInstanceTerminationInput,
    ) -> RusotoFuture<(), SkipWaitTimeForInstanceTerminationError>;

    /// <p>Attempts to stop an ongoing deployment.</p>
    fn stop_deployment(
        &self,
        input: StopDeploymentInput,
    ) -> RusotoFuture<StopDeploymentOutput, StopDeploymentError>;

    /// <p>Changes the name of an application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationInput,
    ) -> RusotoFuture<(), UpdateApplicationError>;

    /// <p>Changes information about a deployment group.</p>
    fn update_deployment_group(
        &self,
        input: UpdateDeploymentGroupInput,
    ) -> RusotoFuture<UpdateDeploymentGroupOutput, UpdateDeploymentGroupError>;
}
/// A client for the CodeDeploy API.
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeDeployClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CodeDeployClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CodeDeploy for CodeDeployClient {
    /// <p>Adds tags to on-premises instances.</p>
    fn add_tags_to_on_premises_instances(
        &self,
        input: AddTagsToOnPremisesInstancesInput,
    ) -> RusotoFuture<(), AddTagsToOnPremisesInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.AddTagsToOnPremisesInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToOnPremisesInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about one or more application revisions.</p>
    fn batch_get_application_revisions(
        &self,
        input: BatchGetApplicationRevisionsInput,
    ) -> RusotoFuture<BatchGetApplicationRevisionsOutput, BatchGetApplicationRevisionsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetApplicationRevisions",
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

                    serde_json::from_str::<BatchGetApplicationRevisionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetApplicationRevisionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about one or more applications.</p>
    fn batch_get_applications(
        &self,
        input: BatchGetApplicationsInput,
    ) -> RusotoFuture<BatchGetApplicationsOutput, BatchGetApplicationsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetApplicationsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetApplicationsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about one or more deployment groups.</p>
    fn batch_get_deployment_groups(
        &self,
        input: BatchGetDeploymentGroupsInput,
    ) -> RusotoFuture<BatchGetDeploymentGroupsOutput, BatchGetDeploymentGroupsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetDeploymentGroups",
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

                    serde_json::from_str::<BatchGetDeploymentGroupsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetDeploymentGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about one or more instance that are part of a deployment group.</p>
    fn batch_get_deployment_instances(
        &self,
        input: BatchGetDeploymentInstancesInput,
    ) -> RusotoFuture<BatchGetDeploymentInstancesOutput, BatchGetDeploymentInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetDeploymentInstances",
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

                    serde_json::from_str::<BatchGetDeploymentInstancesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetDeploymentInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about one or more deployments.</p>
    fn batch_get_deployments(
        &self,
        input: BatchGetDeploymentsInput,
    ) -> RusotoFuture<BatchGetDeploymentsOutput, BatchGetDeploymentsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetDeployments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetDeploymentsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetDeploymentsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about one or more on-premises instances.</p>
    fn batch_get_on_premises_instances(
        &self,
        input: BatchGetOnPremisesInstancesInput,
    ) -> RusotoFuture<BatchGetOnPremisesInstancesOutput, BatchGetOnPremisesInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.BatchGetOnPremisesInstances",
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

                    serde_json::from_str::<BatchGetOnPremisesInstancesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetOnPremisesInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by registering instances in the replacement environment with the load balancer, can start as soon as all instances have a status of Ready.) </p>
    fn continue_deployment(
        &self,
        input: ContinueDeploymentInput,
    ) -> RusotoFuture<(), ContinueDeploymentError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ContinueDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ContinueDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an application.</p>
    fn create_application(
        &self,
        input: CreateApplicationInput,
    ) -> RusotoFuture<CreateApplicationOutput, CreateApplicationError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateApplicationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deploys an application revision through the specified deployment group.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentInput,
    ) -> RusotoFuture<CreateDeploymentOutput, CreateDeploymentError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDeploymentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a deployment configuration.</p>
    fn create_deployment_config(
        &self,
        input: CreateDeploymentConfigInput,
    ) -> RusotoFuture<CreateDeploymentConfigOutput, CreateDeploymentConfigError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDeploymentConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDeploymentConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a deployment group to which application revisions will be deployed.</p>
    fn create_deployment_group(
        &self,
        input: CreateDeploymentGroupInput,
    ) -> RusotoFuture<CreateDeploymentGroupOutput, CreateDeploymentGroupError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDeploymentGroupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDeploymentGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationInput,
    ) -> RusotoFuture<(), DeleteApplicationError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note></p>
    fn delete_deployment_config(
        &self,
        input: DeleteDeploymentConfigInput,
    ) -> RusotoFuture<(), DeleteDeploymentConfigError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDeploymentConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a deployment group.</p>
    fn delete_deployment_group(
        &self,
        input: DeleteDeploymentGroupInput,
    ) -> RusotoFuture<DeleteDeploymentGroupOutput, DeleteDeploymentGroupError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDeploymentGroupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDeploymentGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a GitHub account connection.</p>
    fn delete_git_hub_account_token(
        &self,
        input: DeleteGitHubAccountTokenInput,
    ) -> RusotoFuture<DeleteGitHubAccountTokenOutput, DeleteGitHubAccountTokenError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.DeleteGitHubAccountToken",
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

                    serde_json::from_str::<DeleteGitHubAccountTokenOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteGitHubAccountTokenError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deregisters an on-premises instance.</p>
    fn deregister_on_premises_instance(
        &self,
        input: DeregisterOnPremisesInstanceInput,
    ) -> RusotoFuture<(), DeregisterOnPremisesInstanceError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.DeregisterOnPremisesInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterOnPremisesInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about an application.</p>
    fn get_application(
        &self,
        input: GetApplicationInput,
    ) -> RusotoFuture<GetApplicationOutput, GetApplicationError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetApplicationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about an application revision.</p>
    fn get_application_revision(
        &self,
        input: GetApplicationRevisionInput,
    ) -> RusotoFuture<GetApplicationRevisionOutput, GetApplicationRevisionError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplicationRevision");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetApplicationRevisionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApplicationRevisionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a deployment.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentInput,
    ) -> RusotoFuture<GetDeploymentOutput, GetDeploymentError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeploymentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about a deployment configuration.</p>
    fn get_deployment_config(
        &self,
        input: GetDeploymentConfigInput,
    ) -> RusotoFuture<GetDeploymentConfigOutput, GetDeploymentConfigError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeploymentConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDeploymentConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a deployment group.</p>
    fn get_deployment_group(
        &self,
        input: GetDeploymentGroupInput,
    ) -> RusotoFuture<GetDeploymentGroupOutput, GetDeploymentGroupError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeploymentGroupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about an instance as part of a deployment.</p>
    fn get_deployment_instance(
        &self,
        input: GetDeploymentInstanceInput,
    ) -> RusotoFuture<GetDeploymentInstanceOutput, GetDeploymentInstanceError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeploymentInstanceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDeploymentInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about an on-premises instance.</p>
    fn get_on_premises_instance(
        &self,
        input: GetOnPremisesInstanceInput,
    ) -> RusotoFuture<GetOnPremisesInstanceOutput, GetOnPremisesInstanceError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.GetOnPremisesInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOnPremisesInstanceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetOnPremisesInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists information about revisions for an application.</p>
    fn list_application_revisions(
        &self,
        input: ListApplicationRevisionsInput,
    ) -> RusotoFuture<ListApplicationRevisionsOutput, ListApplicationRevisionsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListApplicationRevisions",
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

                    serde_json::from_str::<ListApplicationRevisionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationRevisionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the applications registered with the applicable IAM user or AWS account.</p>
    fn list_applications(
        &self,
        input: ListApplicationsInput,
    ) -> RusotoFuture<ListApplicationsOutput, ListApplicationsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListApplicationsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListApplicationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the deployment configurations with the applicable IAM user or AWS account.</p>
    fn list_deployment_configs(
        &self,
        input: ListDeploymentConfigsInput,
    ) -> RusotoFuture<ListDeploymentConfigsOutput, ListDeploymentConfigsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentConfigs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeploymentConfigsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDeploymentConfigsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the deployment groups for an application registered with the applicable IAM user or AWS account.</p>
    fn list_deployment_groups(
        &self,
        input: ListDeploymentGroupsInput,
    ) -> RusotoFuture<ListDeploymentGroupsOutput, ListDeploymentGroupsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeploymentGroupsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDeploymentGroupsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the instance for a deployment associated with the applicable IAM user or AWS account.</p>
    fn list_deployment_instances(
        &self,
        input: ListDeploymentInstancesInput,
    ) -> RusotoFuture<ListDeploymentInstancesOutput, ListDeploymentInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListDeploymentInstances",
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

                    serde_json::from_str::<ListDeploymentInstancesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeploymentInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the deployments in a deployment group for an application registered with the applicable IAM user or AWS account.</p>
    fn list_deployments(
        &self,
        input: ListDeploymentsInput,
    ) -> RusotoFuture<ListDeploymentsOutput, ListDeploymentsError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeployments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeploymentsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDeploymentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the names of stored connections to GitHub accounts.</p>
    fn list_git_hub_account_token_names(
        &self,
        input: ListGitHubAccountTokenNamesInput,
    ) -> RusotoFuture<ListGitHubAccountTokenNamesOutput, ListGitHubAccountTokenNamesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListGitHubAccountTokenNames",
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

                    serde_json::from_str::<ListGitHubAccountTokenNamesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListGitHubAccountTokenNamesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names will be listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>
    fn list_on_premises_instances(
        &self,
        input: ListOnPremisesInstancesInput,
    ) -> RusotoFuture<ListOnPremisesInstancesOutput, ListOnPremisesInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.ListOnPremisesInstances",
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

                    serde_json::from_str::<ListOnPremisesInstancesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOnPremisesInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the result of a Lambda validation function. The function validates one or both lifecycle events (<code>BeforeAllowTraffic</code> and <code>AfterAllowTraffic</code>) and returns <code>Succeeded</code> or <code>Failed</code>.</p>
    fn put_lifecycle_event_hook_execution_status(
        &self,
        input: PutLifecycleEventHookExecutionStatusInput,
    ) -> RusotoFuture<
        PutLifecycleEventHookExecutionStatusOutput,
        PutLifecycleEventHookExecutionStatusError,
    > {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.PutLifecycleEventHookExecutionStatus",
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

                    serde_json::from_str::<PutLifecycleEventHookExecutionStatusOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutLifecycleEventHookExecutionStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Registers with AWS CodeDeploy a revision for the specified application.</p>
    fn register_application_revision(
        &self,
        input: RegisterApplicationRevisionInput,
    ) -> RusotoFuture<(), RegisterApplicationRevisionError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RegisterApplicationRevision",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterApplicationRevisionError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note></p>
    fn register_on_premises_instance(
        &self,
        input: RegisterOnPremisesInstanceInput,
    ) -> RusotoFuture<(), RegisterOnPremisesInstanceError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RegisterOnPremisesInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterOnPremisesInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes one or more tags from one or more on-premises instances.</p>
    fn remove_tags_from_on_premises_instances(
        &self,
        input: RemoveTagsFromOnPremisesInstancesInput,
    ) -> RusotoFuture<(), RemoveTagsFromOnPremisesInstancesError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.RemoveTagsFromOnPremisesInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromOnPremisesInstancesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is completed.</p>
    fn skip_wait_time_for_instance_termination(
        &self,
        input: SkipWaitTimeForInstanceTerminationInput,
    ) -> RusotoFuture<(), SkipWaitTimeForInstanceTerminationError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeploy_20141006.SkipWaitTimeForInstanceTermination",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SkipWaitTimeForInstanceTerminationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Attempts to stop an ongoing deployment.</p>
    fn stop_deployment(
        &self,
        input: StopDeploymentInput,
    ) -> RusotoFuture<StopDeploymentOutput, StopDeploymentError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.StopDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopDeploymentOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the name of an application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationInput,
    ) -> RusotoFuture<(), UpdateApplicationError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes information about a deployment group.</p>
    fn update_deployment_group(
        &self,
        input: UpdateDeploymentGroupInput,
    ) -> RusotoFuture<UpdateDeploymentGroupOutput, UpdateDeploymentGroupError> {
        let mut request = SignedRequest::new("POST", "codedeploy", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateDeploymentGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDeploymentGroupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateDeploymentGroupError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
