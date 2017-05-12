#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;

        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
    
use serde_json;
        use rusoto_core::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;
#[doc="<p>Represents the input of, and adds tags to, an on-premises instance operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddTagsToOnPremisesInstancesInput {
                #[doc="<p>The names of the on-premises instances to which to add tags.</p>"]
#[serde(rename="instanceNames")]
pub instance_names: InstanceNameList,
#[doc="<p>The tag key-value pairs to add to the on-premises instances.</p> <p>Keys and values are both required. Keys cannot be null or empty strings. Value-only tags are not allowed.</p>"]
#[serde(rename="tags")]
pub tags: TagList,
            }
            
#[doc="<p>Information about an alarm.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Alarm {
                #[doc="<p>The name of the alarm. Maximum length is 255 characters. Each alarm name can be used only once in a list of alarms.</p>"]
#[serde(rename="name")]
pub name: Option<AlarmName>,
            }
            
#[doc="<p>Information about alarms associated with the deployment group.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct AlarmConfiguration {
                #[doc="<p>A list of alarms configured for the deployment group. A maximum of 10 alarms can be added to a deployment group.</p>"]
#[serde(rename="alarms")]
pub alarms: Option<AlarmList>,
#[doc="<p>Indicates whether the alarm configuration is enabled.</p>"]
#[serde(rename="enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<Boolean>,
#[doc="<p>Indicates whether a deployment should continue if information about the current state of alarms cannot be retrieved from Amazon CloudWatch. The default value is false.</p> <ul> <li> <p>true: The deployment will proceed even if alarm status information can't be retrieved from Amazon CloudWatch.</p> </li> <li> <p>false: The deployment will stop if alarm status information can't be retrieved from Amazon CloudWatch.</p> </li> </ul>"]
#[serde(rename="ignorePollAlarmFailure")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub ignore_poll_alarm_failure: Option<Boolean>,
            }
            
pub type AlarmList = Vec<Alarm>;
pub type AlarmName = String;
pub type ApplicationId = String;
#[doc="<p>Information about an application.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ApplicationInfo {
                #[doc="<p>The application ID.</p>"]
#[serde(rename="applicationId")]
pub application_id: Option<ApplicationId>,
#[doc="<p>The application name.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>The time at which the application was created.</p>"]
#[serde(rename="createTime")]
pub create_time: Option<Timestamp>,
#[doc="<p>True if the user has authenticated with GitHub for the specified application; otherwise, false.</p>"]
#[serde(rename="linkedToGitHub")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub linked_to_git_hub: Option<Boolean>,
            }
            
pub type ApplicationName = String;
pub type ApplicationRevisionSortBy = String;
pub type ApplicationsInfoList = Vec<ApplicationInfo>;
pub type ApplicationsList = Vec<ApplicationName>;
#[doc="<p>Information about a configuration for automatically rolling back to a previous version of an application revision when a deployment doesn't complete successfully.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct AutoRollbackConfiguration {
                #[doc="<p>Indicates whether a defined automatic rollback configuration is currently enabled.</p>"]
#[serde(rename="enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<Boolean>,
#[doc="<p>The event type or types that trigger a rollback.</p>"]
#[serde(rename="events")]
pub events: Option<AutoRollbackEventsList>,
            }
            
pub type AutoRollbackEvent = String;
pub type AutoRollbackEventsList = Vec<AutoRollbackEvent>;
#[doc="<p>Information about an Auto Scaling group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutoScalingGroup {
                #[doc="<p>An Auto Scaling lifecycle event hook name.</p>"]
#[serde(rename="hook")]
pub hook: Option<AutoScalingGroupHook>,
#[doc="<p>The Auto Scaling group name.</p>"]
#[serde(rename="name")]
pub name: Option<AutoScalingGroupName>,
            }
            
pub type AutoScalingGroupHook = String;
pub type AutoScalingGroupList = Vec<AutoScalingGroup>;
pub type AutoScalingGroupName = String;
pub type AutoScalingGroupNameList = Vec<AutoScalingGroupName>;
#[doc="<p>Represents the input of a batch get application revisions operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetApplicationRevisionsInput {
                #[doc="<p>The name of an AWS CodeDeploy application about which to get revision information.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Information to get about the application revisions, including type and location.</p>"]
#[serde(rename="revisions")]
pub revisions: RevisionLocationList,
            }
            
#[doc="<p>Represents the output of a batch get application revisions operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetApplicationRevisionsOutput {
                #[doc="<p>The name of the application that corresponds to the revisions.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>Information about errors that may have occurred during the API call.</p>"]
#[serde(rename="errorMessage")]
pub error_message: Option<ErrorMessage>,
#[doc="<p>Additional information about the revisions, including the type and location.</p>"]
#[serde(rename="revisions")]
pub revisions: Option<RevisionInfoList>,
            }
            
#[doc="<p>Represents the input of a batch get applications operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetApplicationsInput {
                #[doc="<p>A list of application names separated by spaces.</p>"]
#[serde(rename="applicationNames")]
pub application_names: Option<ApplicationsList>,
            }
            
#[doc="<p>Represents the output of a batch get applications operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetApplicationsOutput {
                #[doc="<p>Information about the applications.</p>"]
#[serde(rename="applicationsInfo")]
pub applications_info: Option<ApplicationsInfoList>,
            }
            
#[doc="<p>Represents the input of a batch get deployment groups operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetDeploymentGroupsInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>The deployment groups' names.</p>"]
#[serde(rename="deploymentGroupNames")]
pub deployment_group_names: DeploymentGroupsList,
            }
            
#[doc="<p>Represents the output of a batch get deployment groups operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetDeploymentGroupsOutput {
                #[doc="<p>Information about the deployment groups.</p>"]
#[serde(rename="deploymentGroupsInfo")]
pub deployment_groups_info: Option<DeploymentGroupInfoList>,
#[doc="<p>Information about errors that may have occurred during the API call.</p>"]
#[serde(rename="errorMessage")]
pub error_message: Option<ErrorMessage>,
            }
            
#[doc="<p>Represents the input of a batch get deployment instances operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetDeploymentInstancesInput {
                #[doc="<p>The unique ID of a deployment.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: DeploymentId,
#[doc="<p>The unique IDs of instances in the deployment group.</p>"]
#[serde(rename="instanceIds")]
pub instance_ids: InstancesList,
            }
            
#[doc="<p>Represents the output of a batch get deployment instance operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetDeploymentInstancesOutput {
                #[doc="<p>Information about errors that may have occurred during the API call.</p>"]
#[serde(rename="errorMessage")]
pub error_message: Option<ErrorMessage>,
#[doc="<p>Information about the instance.</p>"]
#[serde(rename="instancesSummary")]
pub instances_summary: Option<InstanceSummaryList>,
            }
            
#[doc="<p>Represents the input of a batch get deployments operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetDeploymentsInput {
                #[doc="<p>A list of deployment IDs, separated by spaces.</p>"]
#[serde(rename="deploymentIds")]
pub deployment_ids: Option<DeploymentsList>,
            }
            
#[doc="<p>Represents the output of a batch get deployments operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetDeploymentsOutput {
                #[doc="<p>Information about the deployments.</p>"]
#[serde(rename="deploymentsInfo")]
pub deployments_info: Option<DeploymentsInfoList>,
            }
            
#[doc="<p>Represents the input of a batch get on-premises instances operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetOnPremisesInstancesInput {
                #[doc="<p>The names of the on-premises instances about which to get information.</p>"]
#[serde(rename="instanceNames")]
pub instance_names: Option<InstanceNameList>,
            }
            
#[doc="<p>Represents the output of a batch get on-premises instances operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetOnPremisesInstancesOutput {
                #[doc="<p>Information about the on-premises instances.</p>"]
#[serde(rename="instanceInfos")]
pub instance_infos: Option<InstanceInfoList>,
            }
            
pub type Boolean = bool;
pub type BundleType = String;
pub type CommitId = String;
#[doc="<p>Represents the input of a create application operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateApplicationInput {
                #[doc="<p>The name of the application. This name must be unique with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
            }
            
#[doc="<p>Represents the output of a create application operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateApplicationOutput {
                #[doc="<p>A unique application ID.</p>"]
#[serde(rename="applicationId")]
pub application_id: Option<ApplicationId>,
            }
            
#[doc="<p>Represents the input of a create deployment configuration operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateDeploymentConfigInput {
                #[doc="<p>The name of the deployment configuration to create.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: DeploymentConfigName,
#[doc="<p>The minimum number of healthy instances that should be available at any time during the deployment. There are two parameters expected in the input: type and value.</p> <p>The type parameter takes either of the following values:</p> <ul> <li> <p>HOST_COUNT: The value parameter represents the minimum number of healthy instances as an absolute value.</p> </li> <li> <p>FLEET_PERCENT: The value parameter represents the minimum number of healthy instances as a percentage of the total number of instances in the deployment. If you specify FLEET_PERCENT, at the start of the deployment, AWS CodeDeploy converts the percentage to the equivalent number of instance and rounds up fractional instances.</p> </li> </ul> <p>The value parameter takes an integer.</p> <p>For example, to set a minimum of 95% healthy instance, specify a type of FLEET_PERCENT and a value of 95.</p>"]
#[serde(rename="minimumHealthyHosts")]
pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
            }
            
#[doc="<p>Represents the output of a create deployment configuration operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateDeploymentConfigOutput {
                #[doc="<p>A unique deployment configuration ID.</p>"]
#[serde(rename="deploymentConfigId")]
pub deployment_config_id: Option<DeploymentConfigId>,
            }
            
#[doc="<p>Represents the input of a create deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateDeploymentGroupInput {
                #[doc="<p>Information to add about Amazon CloudWatch alarms when the deployment group is created. </p>"]
#[serde(rename="alarmConfiguration")]
pub alarm_configuration: Option<AlarmConfiguration>,
#[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Configuration information for an automatic rollback that is added when a deployment group is created.</p>"]
#[serde(rename="autoRollbackConfiguration")]
pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
#[doc="<p>A list of associated Auto Scaling groups.</p>"]
#[serde(rename="autoScalingGroups")]
pub auto_scaling_groups: Option<AutoScalingGroupNameList>,
#[doc="<p>If specified, the deployment configuration name can be either one of the predefined configurations provided with AWS CodeDeploy or a custom deployment configuration that you create by calling the create deployment configuration operation.</p> <p>CodeDeployDefault.OneAtATime is the default deployment configuration. It is used if a configuration isn't specified for the deployment or the deployment group.</p> <p>For more information about the predefined deployment configurations in AWS CodeDeploy, see see <a href=\"http://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-configurations.html\">Working with Deployment Groups in AWS CodeDeploy</a> in the AWS CodeDeploy User Guide.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>The name of a new deployment group for the specified application.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: DeploymentGroupName,
#[doc="<p>The Amazon EC2 tags on which to filter.</p>"]
#[serde(rename="ec2TagFilters")]
pub ec_2_tag_filters: Option<EC2TagFilterList>,
#[doc="<p>The on-premises instance tags on which to filter.</p>"]
#[serde(rename="onPremisesInstanceTagFilters")]
pub on_premises_instance_tag_filters: Option<TagFilterList>,
#[doc="<p>A service role ARN that allows AWS CodeDeploy to act on the user's behalf when interacting with AWS services.</p>"]
#[serde(rename="serviceRoleArn")]
pub service_role_arn: Role,
#[doc="<p>Information about triggers to create when the deployment group is created. For examples, see <a href=\"http://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-sns.html\">Create a Trigger for an AWS CodeDeploy Event</a> in the AWS CodeDeploy User Guide.</p>"]
#[serde(rename="triggerConfigurations")]
pub trigger_configurations: Option<TriggerConfigList>,
            }
            
#[doc="<p>Represents the output of a create deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateDeploymentGroupOutput {
                #[doc="<p>A unique deployment group ID.</p>"]
#[serde(rename="deploymentGroupId")]
pub deployment_group_id: Option<DeploymentGroupId>,
            }
            
#[doc="<p>Represents the input of a create deployment operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateDeploymentInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Configuration information for an automatic rollback that is added when a deployment is created.</p>"]
#[serde(rename="autoRollbackConfiguration")]
pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
#[doc="<p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p> <p>If not specified, the value configured in the deployment group will be used as the default. If the deployment group does not have a deployment configuration associated with it, then CodeDeployDefault.OneAtATime will be used by default.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>The name of the deployment group.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: Option<DeploymentGroupName>,
#[doc="<p>A comment about the deployment.</p>"]
#[serde(rename="description")]
pub description: Option<Description>,
#[doc="<p>If set to true, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will not be considered to have failed at that point and will continue on to the BeforeInstall deployment lifecycle event.</p> <p>If set to false or not specified, then if the deployment causes the ApplicationStop deployment lifecycle event to fail to an instance, the deployment to that instance will stop, and the deployment to that instance will be considered to have failed.</p>"]
#[serde(rename="ignoreApplicationStopFailures")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub ignore_application_stop_failures: Option<Boolean>,
#[doc="<p>The type and location of the revision to deploy.</p>"]
#[serde(rename="revision")]
pub revision: Option<RevisionLocation>,
#[doc="<p>Indicates whether to deploy to all instances or only to instances that are not running the latest application revision.</p>"]
#[serde(rename="updateOutdatedInstancesOnly")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub update_outdated_instances_only: Option<Boolean>,
            }
            
#[doc="<p>Represents the output of a create deployment operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateDeploymentOutput {
                #[doc="<p>A unique deployment ID.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: Option<DeploymentId>,
            }
            
#[doc="<p>Represents the input of a delete application operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteApplicationInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
            }
            
#[doc="<p>Represents the input of a delete deployment configuration operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteDeploymentConfigInput {
                #[doc="<p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: DeploymentConfigName,
            }
            
#[doc="<p>Represents the input of a delete deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteDeploymentGroupInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>The name of an existing deployment group for the specified application.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: DeploymentGroupName,
            }
            
#[doc="<p>Represents the output of a delete deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteDeploymentGroupOutput {
                #[doc="<p>If the output contains no data, and the corresponding deployment group contained at least one Auto Scaling group, AWS CodeDeploy successfully removed all corresponding Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group. If the output contains data, AWS CodeDeploy could not remove some Auto Scaling lifecycle event hooks from the Amazon EC2 instances in the Auto Scaling group.</p>"]
#[serde(rename="hooksNotCleanedUp")]
pub hooks_not_cleaned_up: Option<AutoScalingGroupList>,
            }
            
pub type DeploymentConfigId = String;
#[doc="<p>Information about a deployment configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeploymentConfigInfo {
                #[doc="<p>The time at which the deployment configuration was created.</p>"]
#[serde(rename="createTime")]
pub create_time: Option<Timestamp>,
#[doc="<p>The deployment configuration ID.</p>"]
#[serde(rename="deploymentConfigId")]
pub deployment_config_id: Option<DeploymentConfigId>,
#[doc="<p>The deployment configuration name.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>Information about the number or percentage of minimum healthy instance.</p>"]
#[serde(rename="minimumHealthyHosts")]
pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
            }
            
pub type DeploymentConfigName = String;
pub type DeploymentConfigsList = Vec<DeploymentConfigName>;
pub type DeploymentCreator = String;
pub type DeploymentGroupId = String;
#[doc="<p>Information about a deployment group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeploymentGroupInfo {
                #[doc="<p>A list of alarms associated with the deployment group.</p>"]
#[serde(rename="alarmConfiguration")]
pub alarm_configuration: Option<AlarmConfiguration>,
#[doc="<p>The application name.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>Information about the automatic rollback configuration associated with the deployment group.</p>"]
#[serde(rename="autoRollbackConfiguration")]
pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
#[doc="<p>A list of associated Auto Scaling groups.</p>"]
#[serde(rename="autoScalingGroups")]
pub auto_scaling_groups: Option<AutoScalingGroupList>,
#[doc="<p>The deployment configuration name.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>The deployment group ID.</p>"]
#[serde(rename="deploymentGroupId")]
pub deployment_group_id: Option<DeploymentGroupId>,
#[doc="<p>The deployment group name.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: Option<DeploymentGroupName>,
#[doc="<p>The Amazon EC2 tags on which to filter.</p>"]
#[serde(rename="ec2TagFilters")]
pub ec_2_tag_filters: Option<EC2TagFilterList>,
#[doc="<p>The on-premises instance tags on which to filter.</p>"]
#[serde(rename="onPremisesInstanceTagFilters")]
pub on_premises_instance_tag_filters: Option<TagFilterList>,
#[doc="<p>A service role ARN.</p>"]
#[serde(rename="serviceRoleArn")]
pub service_role_arn: Option<Role>,
#[doc="<p>Information about the deployment group's target revision, including type and location.</p>"]
#[serde(rename="targetRevision")]
pub target_revision: Option<RevisionLocation>,
#[doc="<p>Information about triggers associated with the deployment group.</p>"]
#[serde(rename="triggerConfigurations")]
pub trigger_configurations: Option<TriggerConfigList>,
            }
            
pub type DeploymentGroupInfoList = Vec<DeploymentGroupInfo>;
pub type DeploymentGroupName = String;
pub type DeploymentGroupsList = Vec<DeploymentGroupName>;
pub type DeploymentId = String;
#[doc="<p>Information about a deployment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeploymentInfo {
                #[doc="<p>The application name.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>Information about the automatic rollback configuration associated with the deployment.</p>"]
#[serde(rename="autoRollbackConfiguration")]
pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
#[doc="<p>A timestamp indicating when the deployment was complete.</p>"]
#[serde(rename="completeTime")]
pub complete_time: Option<Timestamp>,
#[doc="<p>A timestamp indicating when the deployment was created.</p>"]
#[serde(rename="createTime")]
pub create_time: Option<Timestamp>,
#[doc="<p>The means by which the deployment was created:</p> <ul> <li> <p>user: A user created the deployment.</p> </li> <li> <p>autoscaling: Auto Scaling created the deployment.</p> </li> <li> <p>codeDeployRollback: A rollback process created the deployment.</p> </li> </ul>"]
#[serde(rename="creator")]
pub creator: Option<DeploymentCreator>,
#[doc="<p>The deployment configuration name.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>The deployment group name.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: Option<DeploymentGroupName>,
#[doc="<p>The deployment ID.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: Option<DeploymentId>,
#[doc="<p>A summary of the deployment status of the instances in the deployment.</p>"]
#[serde(rename="deploymentOverview")]
pub deployment_overview: Option<DeploymentOverview>,
#[doc="<p>A comment about the deployment.</p>"]
#[serde(rename="description")]
pub description: Option<Description>,
#[doc="<p>Information about any error associated with this deployment.</p>"]
#[serde(rename="errorInformation")]
pub error_information: Option<ErrorInformation>,
#[doc="<p>If true, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will not be considered to have failed at that point and will continue on to the BeforeInstall deployment lifecycle event.</p> <p>If false or not specified, then if the deployment causes the ApplicationStop deployment lifecycle event to an instance to fail, the deployment to that instance will stop, and the deployment to that instance will be considered to have failed.</p>"]
#[serde(rename="ignoreApplicationStopFailures")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub ignore_application_stop_failures: Option<Boolean>,
#[doc="<p>Information about the location of stored application artifacts and the service from which to retrieve them.</p>"]
#[serde(rename="revision")]
pub revision: Option<RevisionLocation>,
#[doc="<p>Information about a deployment rollback.</p>"]
#[serde(rename="rollbackInfo")]
pub rollback_info: Option<RollbackInfo>,
#[doc="<p>A timestamp indicating when the deployment was deployed to the deployment group.</p> <p>In some cases, the reported value of the start time may be later than the complete time. This is due to differences in the clock settings of back-end servers that participate in the deployment process.</p>"]
#[serde(rename="startTime")]
pub start_time: Option<Timestamp>,
#[doc="<p>The current state of the deployment as a whole.</p>"]
#[serde(rename="status")]
pub status: Option<DeploymentStatus>,
#[doc="<p>Indicates whether only instances that are not running the latest application revision are to be deployed to.</p>"]
#[serde(rename="updateOutdatedInstancesOnly")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub update_outdated_instances_only: Option<Boolean>,
            }
            
#[doc="<p>Information about the deployment status of the instances in the deployment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeploymentOverview {
                #[doc="<p>The number of instances in the deployment in a failed state.</p>"]
#[serde(rename="Failed")]
pub failed: Option<InstanceCount>,
#[doc="<p>The number of instances in which the deployment is in progress.</p>"]
#[serde(rename="InProgress")]
pub in_progress: Option<InstanceCount>,
#[doc="<p>The number of instances in the deployment in a pending state.</p>"]
#[serde(rename="Pending")]
pub pending: Option<InstanceCount>,
#[doc="<p>The number of instances in the deployment in a skipped state.</p>"]
#[serde(rename="Skipped")]
pub skipped: Option<InstanceCount>,
#[doc="<p>The number of instances in the deployment to which revisions have been successfully deployed.</p>"]
#[serde(rename="Succeeded")]
pub succeeded: Option<InstanceCount>,
            }
            
pub type DeploymentStatus = String;
pub type DeploymentStatusList = Vec<DeploymentStatus>;
pub type DeploymentsInfoList = Vec<DeploymentInfo>;
pub type DeploymentsList = Vec<DeploymentId>;
#[doc="<p>Represents the input of a deregister on-premises instance operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeregisterOnPremisesInstanceInput {
                #[doc="<p>The name of the on-premises instance to deregister.</p>"]
#[serde(rename="instanceName")]
pub instance_name: InstanceName,
            }
            
pub type Description = String;
#[doc="<p>Diagnostic information about executable scripts that are part of a deployment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Diagnostics {
                #[doc="<p>The associated error code:</p> <ul> <li> <p>Success: The specified script ran.</p> </li> <li> <p>ScriptMissing: The specified script was not found in the specified location.</p> </li> <li> <p>ScriptNotExecutable: The specified script is not a recognized executable file type.</p> </li> <li> <p>ScriptTimedOut: The specified script did not finish running in the specified time period.</p> </li> <li> <p>ScriptFailed: The specified script failed to run as expected.</p> </li> <li> <p>UnknownError: The specified script did not run for an unknown reason.</p> </li> </ul>"]
#[serde(rename="errorCode")]
pub error_code: Option<LifecycleErrorCode>,
#[doc="<p>The last portion of the diagnostic log.</p> <p>If available, AWS CodeDeploy returns up to the last 4 KB of the diagnostic log.</p>"]
#[serde(rename="logTail")]
pub log_tail: Option<LogTail>,
#[doc="<p>The message associated with the error.</p>"]
#[serde(rename="message")]
pub message: Option<LifecycleMessage>,
#[doc="<p>The name of the script.</p>"]
#[serde(rename="scriptName")]
pub script_name: Option<ScriptName>,
            }
            
#[doc="<p>Information about a tag filter.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct EC2TagFilter {
                #[doc="<p>The tag filter key.</p>"]
#[serde(rename="Key")]
pub key: Option<Key>,
#[doc="<p>The tag filter type:</p> <ul> <li> <p>KEY_ONLY: Key only.</p> </li> <li> <p>VALUE_ONLY: Value only.</p> </li> <li> <p>KEY_AND_VALUE: Key and value.</p> </li> </ul>"]
#[serde(rename="Type")]
pub type_: Option<EC2TagFilterType>,
#[doc="<p>The tag filter value.</p>"]
#[serde(rename="Value")]
pub value: Option<Value>,
            }
            
pub type EC2TagFilterList = Vec<EC2TagFilter>;
pub type EC2TagFilterType = String;
pub type ETag = String;
pub type ErrorCode = String;
#[doc="<p>Information about a deployment error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ErrorInformation {
                #[doc="<p>The error code:</p> <ul> <li> <p>APPLICATION_MISSING: The application was missing. This error code will most likely be raised if the application is deleted after the deployment is created but before it is started.</p> </li> <li> <p>DEPLOYMENT_GROUP_MISSING: The deployment group was missing. This error code will most likely be raised if the deployment group is deleted after the deployment is created but before it is started.</p> </li> <li> <p>HEALTH_CONSTRAINTS: The deployment failed on too many instances to be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>HEALTH_CONSTRAINTS_INVALID: The revision cannot be successfully deployed within the instance health constraints specified.</p> </li> <li> <p>IAM_ROLE_MISSING: The service role cannot be accessed.</p> </li> <li> <p>IAM_ROLE_PERMISSIONS: The service role does not have the correct permissions.</p> </li> <li> <p>INTERNAL_ERROR: There was an internal error.</p> </li> <li> <p>NO_EC2_SUBSCRIPTION: The calling account is not subscribed to the Amazon EC2 service.</p> </li> <li> <p>NO_INSTANCES: No instance were specified, or no instance can be found.</p> </li> <li> <p>OVER_MAX_INSTANCES: The maximum number of instance was exceeded.</p> </li> <li> <p>THROTTLED: The operation was throttled because the calling account exceeded the throttling limits of one or more AWS services.</p> </li> <li> <p>TIMEOUT: The deployment has timed out.</p> </li> <li> <p>REVISION_MISSING: The revision ID was missing. This error code will most likely be raised if the revision is deleted after the deployment is created but before it is started.</p> </li> </ul>"]
#[serde(rename="code")]
pub code: Option<ErrorCode>,
#[doc="<p>An accompanying error message.</p>"]
#[serde(rename="message")]
pub message: Option<ErrorMessage>,
            }
            
pub type ErrorMessage = String;
#[doc="<p>Information about an application revision.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GenericRevisionInfo {
                #[doc="<p>The deployment groups for which this is the current target revision.</p>"]
#[serde(rename="deploymentGroups")]
pub deployment_groups: Option<DeploymentGroupsList>,
#[doc="<p>A comment about the revision.</p>"]
#[serde(rename="description")]
pub description: Option<Description>,
#[doc="<p>When the revision was first used by AWS CodeDeploy.</p>"]
#[serde(rename="firstUsedTime")]
pub first_used_time: Option<Timestamp>,
#[doc="<p>When the revision was last used by AWS CodeDeploy.</p>"]
#[serde(rename="lastUsedTime")]
pub last_used_time: Option<Timestamp>,
#[doc="<p>When the revision was registered with AWS CodeDeploy.</p>"]
#[serde(rename="registerTime")]
pub register_time: Option<Timestamp>,
            }
            
#[doc="<p>Represents the input of a get application operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetApplicationInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
            }
            
#[doc="<p>Represents the output of a get application operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetApplicationOutput {
                #[doc="<p>Information about the application.</p>"]
#[serde(rename="application")]
pub application: Option<ApplicationInfo>,
            }
            
#[doc="<p>Represents the input of a get application revision operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetApplicationRevisionInput {
                #[doc="<p>The name of the application that corresponds to the revision.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Information about the application revision to get, including type and location.</p>"]
#[serde(rename="revision")]
pub revision: RevisionLocation,
            }
            
#[doc="<p>Represents the output of a get application revision operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetApplicationRevisionOutput {
                #[doc="<p>The name of the application that corresponds to the revision.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>Additional information about the revision, including type and location.</p>"]
#[serde(rename="revision")]
pub revision: Option<RevisionLocation>,
#[doc="<p>General information about the revision.</p>"]
#[serde(rename="revisionInfo")]
pub revision_info: Option<GenericRevisionInfo>,
            }
            
#[doc="<p>Represents the input of a get deployment configuration operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDeploymentConfigInput {
                #[doc="<p>The name of a deployment configuration associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: DeploymentConfigName,
            }
            
#[doc="<p>Represents the output of a get deployment configuration operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDeploymentConfigOutput {
                #[doc="<p>Information about the deployment configuration.</p>"]
#[serde(rename="deploymentConfigInfo")]
pub deployment_config_info: Option<DeploymentConfigInfo>,
            }
            
#[doc="<p>Represents the input of a get deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDeploymentGroupInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>The name of an existing deployment group for the specified application.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: DeploymentGroupName,
            }
            
#[doc="<p>Represents the output of a get deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDeploymentGroupOutput {
                #[doc="<p>Information about the deployment group.</p>"]
#[serde(rename="deploymentGroupInfo")]
pub deployment_group_info: Option<DeploymentGroupInfo>,
            }
            
#[doc="<p>Represents the input of a get deployment operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDeploymentInput {
                #[doc="<p>A deployment ID associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: DeploymentId,
            }
            
#[doc="<p>Represents the input of a get deployment instance operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDeploymentInstanceInput {
                #[doc="<p>The unique ID of a deployment.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: DeploymentId,
#[doc="<p>The unique ID of an instance in the deployment group.</p>"]
#[serde(rename="instanceId")]
pub instance_id: InstanceId,
            }
            
#[doc="<p>Represents the output of a get deployment instance operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDeploymentInstanceOutput {
                #[doc="<p>Information about the instance.</p>"]
#[serde(rename="instanceSummary")]
pub instance_summary: Option<InstanceSummary>,
            }
            
#[doc="<p>Represents the output of a get deployment operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDeploymentOutput {
                #[doc="<p>Information about the deployment.</p>"]
#[serde(rename="deploymentInfo")]
pub deployment_info: Option<DeploymentInfo>,
            }
            
#[doc="<p>Represents the input of a get on-premises instance operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetOnPremisesInstanceInput {
                #[doc="<p>The name of the on-premises instance about which to get information.</p>"]
#[serde(rename="instanceName")]
pub instance_name: InstanceName,
            }
            
#[doc="<p>Represents the output of a get on-premises instance operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetOnPremisesInstanceOutput {
                #[doc="<p>Information about the on-premises instance.</p>"]
#[serde(rename="instanceInfo")]
pub instance_info: Option<InstanceInfo>,
            }
            
#[doc="<p>Information about the location of application artifacts stored in GitHub.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct GitHubLocation {
                #[doc="<p>The SHA1 commit ID of the GitHub commit that represents the bundled artifacts for the application revision.</p>"]
#[serde(rename="commitId")]
pub commit_id: Option<CommitId>,
#[doc="<p>The GitHub account and repository pair that stores a reference to the commit that represents the bundled artifacts for the application revision. </p> <p>Specified as account/repository.</p>"]
#[serde(rename="repository")]
pub repository: Option<Repository>,
            }
            
pub type IamSessionArn = String;
pub type IamUserArn = String;
pub type InstanceArn = String;
pub type InstanceCount = i64;
pub type InstanceId = String;
#[doc="<p>Information about an on-premises instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceInfo {
                #[doc="<p>If the on-premises instance was deregistered, the time at which the on-premises instance was deregistered.</p>"]
#[serde(rename="deregisterTime")]
pub deregister_time: Option<Timestamp>,
#[doc="<p>The ARN of the IAM session associated with the on-premises instance.</p>"]
#[serde(rename="iamSessionArn")]
pub iam_session_arn: Option<IamSessionArn>,
#[doc="<p>The IAM user ARN associated with the on-premises instance.</p>"]
#[serde(rename="iamUserArn")]
pub iam_user_arn: Option<IamUserArn>,
#[doc="<p>The ARN of the on-premises instance.</p>"]
#[serde(rename="instanceArn")]
pub instance_arn: Option<InstanceArn>,
#[doc="<p>The name of the on-premises instance.</p>"]
#[serde(rename="instanceName")]
pub instance_name: Option<InstanceName>,
#[doc="<p>The time at which the on-premises instance was registered.</p>"]
#[serde(rename="registerTime")]
pub register_time: Option<Timestamp>,
#[doc="<p>The tags currently associated with the on-premises instance.</p>"]
#[serde(rename="tags")]
pub tags: Option<TagList>,
            }
            
pub type InstanceInfoList = Vec<InstanceInfo>;
pub type InstanceName = String;
pub type InstanceNameList = Vec<InstanceName>;
pub type InstanceStatus = String;
pub type InstanceStatusList = Vec<InstanceStatus>;
#[doc="<p>Information about an instance in a deployment.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceSummary {
                #[doc="<p>The deployment ID.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: Option<DeploymentId>,
#[doc="<p>The instance ID.</p>"]
#[serde(rename="instanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>A timestamp indicating when the instance information was last updated.</p>"]
#[serde(rename="lastUpdatedAt")]
pub last_updated_at: Option<Timestamp>,
#[doc="<p>A list of lifecycle events for this instance.</p>"]
#[serde(rename="lifecycleEvents")]
pub lifecycle_events: Option<LifecycleEventList>,
#[doc="<p>The deployment status for this instance:</p> <ul> <li> <p>Pending: The deployment is pending for this instance.</p> </li> <li> <p>In Progress: The deployment is in progress for this instance.</p> </li> <li> <p>Succeeded: The deployment has succeeded for this instance.</p> </li> <li> <p>Failed: The deployment has failed for this instance.</p> </li> <li> <p>Skipped: The deployment has been skipped for this instance.</p> </li> <li> <p>Unknown: The deployment status is unknown for this instance.</p> </li> </ul>"]
#[serde(rename="status")]
pub status: Option<InstanceStatus>,
            }
            
pub type InstanceSummaryList = Vec<InstanceSummary>;
pub type InstancesList = Vec<InstanceId>;
pub type Key = String;
pub type LifecycleErrorCode = String;
#[doc="<p>Information about a deployment lifecycle event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct LifecycleEvent {
                #[doc="<p>Diagnostic information about the deployment lifecycle event.</p>"]
#[serde(rename="diagnostics")]
pub diagnostics: Option<Diagnostics>,
#[doc="<p>A timestamp indicating when the deployment lifecycle event ended.</p>"]
#[serde(rename="endTime")]
pub end_time: Option<Timestamp>,
#[doc="<p>The deployment lifecycle event name, such as ApplicationStop, BeforeInstall, AfterInstall, ApplicationStart, or ValidateService.</p>"]
#[serde(rename="lifecycleEventName")]
pub lifecycle_event_name: Option<LifecycleEventName>,
#[doc="<p>A timestamp indicating when the deployment lifecycle event started.</p>"]
#[serde(rename="startTime")]
pub start_time: Option<Timestamp>,
#[doc="<p>The deployment lifecycle event status:</p> <ul> <li> <p>Pending: The deployment lifecycle event is pending.</p> </li> <li> <p>InProgress: The deployment lifecycle event is in progress.</p> </li> <li> <p>Succeeded: The deployment lifecycle event ran successfully.</p> </li> <li> <p>Failed: The deployment lifecycle event has failed.</p> </li> <li> <p>Skipped: The deployment lifecycle event has been skipped.</p> </li> <li> <p>Unknown: The deployment lifecycle event is unknown.</p> </li> </ul>"]
#[serde(rename="status")]
pub status: Option<LifecycleEventStatus>,
            }
            
pub type LifecycleEventList = Vec<LifecycleEvent>;
pub type LifecycleEventName = String;
pub type LifecycleEventStatus = String;
pub type LifecycleMessage = String;
#[doc="<p>Represents the input of a list application revisions operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListApplicationRevisionsInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Whether to list revisions based on whether the revision is the target revision of an deployment group:</p> <ul> <li> <p>include: List revisions that are target revisions of a deployment group.</p> </li> <li> <p>exclude: Do not list revisions that are target revisions of a deployment group.</p> </li> <li> <p>ignore: List all revisions.</p> </li> </ul>"]
#[serde(rename="deployed")]
pub deployed: Option<ListStateFilterAction>,
#[doc="<p>An identifier returned from the previous list application revisions call. It can be used to return the next set of applications in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>An Amazon S3 bucket name to limit the search for revisions.</p> <p>If set to null, all of the user's buckets will be searched.</p>"]
#[serde(rename="s3Bucket")]
pub s_3_bucket: Option<S3Bucket>,
#[doc="<p>A key prefix for the set of Amazon S3 objects to limit the search for revisions.</p>"]
#[serde(rename="s3KeyPrefix")]
pub s_3_key_prefix: Option<S3Key>,
#[doc="<p>The column name to use to sort the list results:</p> <ul> <li> <p>registerTime: Sort by the time the revisions were registered with AWS CodeDeploy.</p> </li> <li> <p>firstUsedTime: Sort by the time the revisions were first used in a deployment.</p> </li> <li> <p>lastUsedTime: Sort by the time the revisions were last used in a deployment.</p> </li> </ul> <p>If not specified or set to null, the results will be returned in an arbitrary order.</p>"]
#[serde(rename="sortBy")]
pub sort_by: Option<ApplicationRevisionSortBy>,
#[doc="<p>The order in which to sort the list results:</p> <ul> <li> <p>ascending: ascending order.</p> </li> <li> <p>descending: descending order.</p> </li> </ul> <p>If not specified, the results will be sorted in ascending order.</p> <p>If set to null, the results will be sorted in an arbitrary order.</p>"]
#[serde(rename="sortOrder")]
pub sort_order: Option<SortOrder>,
            }
            
#[doc="<p>Represents the output of a list application revisions operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListApplicationRevisionsOutput {
                #[doc="<p>If a large amount of information is returned, an identifier will also be returned. It can be used in a subsequent list application revisions call to return the next set of application revisions in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>A list of locations that contain the matching revisions.</p>"]
#[serde(rename="revisions")]
pub revisions: Option<RevisionLocationList>,
            }
            
#[doc="<p>Represents the input of a list applications operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListApplicationsInput {
                #[doc="<p>An identifier returned from the previous list applications call. It can be used to return the next set of applications in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the output of a list applications operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListApplicationsOutput {
                #[doc="<p>A list of application names.</p>"]
#[serde(rename="applications")]
pub applications: Option<ApplicationsList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list applications call to return the next set of applications, will also be returned. in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the input of a list deployment configurations operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDeploymentConfigsInput {
                #[doc="<p>An identifier returned from the previous list deployment configurations call. It can be used to return the next set of deployment configurations in the list. </p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the output of a list deployment configurations operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDeploymentConfigsOutput {
                #[doc="<p>A list of deployment configurations, including built-in configurations such as CodeDeployDefault.OneAtATime.</p>"]
#[serde(rename="deploymentConfigsList")]
pub deployment_configs_list: Option<DeploymentConfigsList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment configurations call to return the next set of deployment configurations in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the input of a list deployment groups operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDeploymentGroupsInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the output of a list deployment groups operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDeploymentGroupsOutput {
                #[doc="<p>The application name.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>A list of corresponding deployment group names.</p>"]
#[serde(rename="deploymentGroups")]
pub deployment_groups: Option<DeploymentGroupsList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment groups call to return the next set of deployment groups in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the input of a list deployment instances operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDeploymentInstancesInput {
                #[doc="<p>The unique ID of a deployment.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: DeploymentId,
#[doc="<p>A subset of instances to list by status:</p> <ul> <li> <p>Pending: Include those instance with pending deployments.</p> </li> <li> <p>InProgress: Include those instance where deployments are still in progress.</p> </li> <li> <p>Succeeded: Include those instances with successful deployments.</p> </li> <li> <p>Failed: Include those instance with failed deployments.</p> </li> <li> <p>Skipped: Include those instance with skipped deployments.</p> </li> <li> <p>Unknown: Include those instance with deployments in an unknown state.</p> </li> </ul>"]
#[serde(rename="instanceStatusFilter")]
pub instance_status_filter: Option<InstanceStatusList>,
#[doc="<p>An identifier returned from the previous list deployment instances call. It can be used to return the next set of deployment instances in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the output of a list deployment instances operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDeploymentInstancesOutput {
                #[doc="<p>A list of instance IDs.</p>"]
#[serde(rename="instancesList")]
pub instances_list: Option<InstancesList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment instances call to return the next set of deployment instances in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the input of a list deployments operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDeploymentsInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>A time range (start and end) for returning a subset of the list of deployments.</p>"]
#[serde(rename="createTimeRange")]
pub create_time_range: Option<TimeRange>,
#[doc="<p>The name of an existing deployment group for the specified application.</p>"]
#[serde(rename="deploymentGroupName")]
pub deployment_group_name: Option<DeploymentGroupName>,
#[doc="<p>A subset of deployments to list by status:</p> <ul> <li> <p>Created: Include created deployments in the resulting list.</p> </li> <li> <p>Queued: Include queued deployments in the resulting list.</p> </li> <li> <p>In Progress: Include in-progress deployments in the resulting list.</p> </li> <li> <p>Succeeded: Include successful deployments in the resulting list.</p> </li> <li> <p>Failed: Include failed deployments in the resulting list.</p> </li> <li> <p>Stopped: Include stopped deployments in the resulting list.</p> </li> </ul>"]
#[serde(rename="includeOnlyStatuses")]
pub include_only_statuses: Option<DeploymentStatusList>,
#[doc="<p>An identifier returned from the previous list deployments call. It can be used to return the next set of deployments in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the output of a list deployments operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDeploymentsOutput {
                #[doc="<p>A list of deployment IDs.</p>"]
#[serde(rename="deployments")]
pub deployments: Option<DeploymentsList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployments call to return the next set of deployments in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>Represents the input of a list on-premises instances operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListOnPremisesInstancesInput {
                #[doc="<p>An identifier returned from the previous list on-premises instances call. It can be used to return the next set of on-premises instances in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The registration status of the on-premises instances:</p> <ul> <li> <p>Deregistered: Include deregistered on-premises instances in the resulting list.</p> </li> <li> <p>Registered: Include registered on-premises instances in the resulting list.</p> </li> </ul>"]
#[serde(rename="registrationStatus")]
pub registration_status: Option<RegistrationStatus>,
#[doc="<p>The on-premises instance tags that will be used to restrict the corresponding on-premises instance names returned.</p>"]
#[serde(rename="tagFilters")]
pub tag_filters: Option<TagFilterList>,
            }
            
#[doc="<p>Represents the output of list on-premises instances operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListOnPremisesInstancesOutput {
                #[doc="<p>The list of matching on-premises instance names.</p>"]
#[serde(rename="instanceNames")]
pub instance_names: Option<InstanceNameList>,
#[doc="<p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list on-premises instances call to return the next set of on-premises instances in the list.</p>"]
#[serde(rename="nextToken")]
pub next_token: Option<NextToken>,
            }
            
pub type ListStateFilterAction = String;
pub type LogTail = String;
pub type Message = String;
#[doc="<p>Information about minimum healthy instance.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct MinimumHealthyHosts {
                #[doc="<p>The minimum healthy instance type:</p> <ul> <li> <p>HOST_COUNT: The minimum number of healthy instance as an absolute value.</p> </li> <li> <p>FLEET_PERCENT: The minimum number of healthy instance as a percentage of the total number of instance in the deployment.</p> </li> </ul> <p>In an example of nine instance, if a HOST_COUNT of six is specified, deploy to up to three instances at a time. The deployment will be successful if six or more instances are deployed to successfully; otherwise, the deployment fails. If a FLEET_PERCENT of 40 is specified, deploy to up to five instance at a time. The deployment will be successful if four or more instance are deployed to successfully; otherwise, the deployment fails.</p> <note> <p>In a call to the get deployment configuration operation, CodeDeployDefault.OneAtATime will return a minimum healthy instance type of MOST_CONCURRENCY and a value of 1. This means a deployment to only one instance at a time. (You cannot set the type to MOST_CONCURRENCY, only to HOST_COUNT or FLEET_PERCENT.) In addition, with CodeDeployDefault.OneAtATime, AWS CodeDeploy will try to ensure that all instances but one are kept in a healthy state during the deployment. Although this allows one instance at a time to be taken offline for a new deployment, it also means that if the deployment to the last instance fails, the overall deployment still succeeds.</p> </note>"]
#[serde(rename="type")]
pub type_: Option<MinimumHealthyHostsType>,
#[doc="<p>The minimum healthy instance value.</p>"]
#[serde(rename="value")]
pub value: Option<MinimumHealthyHostsValue>,
            }
            
pub type MinimumHealthyHostsType = String;
pub type MinimumHealthyHostsValue = i64;
pub type NextToken = String;
pub type NullableBoolean = bool;
#[doc="<p>Represents the input of a register application revision operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterApplicationRevisionInput {
                #[doc="<p>The name of an AWS CodeDeploy application associated with the applicable IAM user or AWS account.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>A comment about the revision.</p>"]
#[serde(rename="description")]
pub description: Option<Description>,
#[doc="<p>Information about the application revision to register, including type and location.</p>"]
#[serde(rename="revision")]
pub revision: RevisionLocation,
            }
            
#[doc="<p>Represents the input of the register on-premises instance operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterOnPremisesInstanceInput {
                #[doc="<p>The ARN of the IAM session to associate with the on-premises instance.</p>"]
#[serde(rename="iamSessionArn")]
pub iam_session_arn: Option<IamSessionArn>,
#[doc="<p>The ARN of the IAM user to associate with the on-premises instance.</p>"]
#[serde(rename="iamUserArn")]
pub iam_user_arn: Option<IamUserArn>,
#[doc="<p>The name of the on-premises instance to register.</p>"]
#[serde(rename="instanceName")]
pub instance_name: InstanceName,
            }
            
pub type RegistrationStatus = String;
#[doc="<p>Represents the input of a remove tags from on-premises instances operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTagsFromOnPremisesInstancesInput {
                #[doc="<p>The names of the on-premises instances from which to remove tags.</p>"]
#[serde(rename="instanceNames")]
pub instance_names: InstanceNameList,
#[doc="<p>The tag key-value pairs to remove from the on-premises instances.</p>"]
#[serde(rename="tags")]
pub tags: TagList,
            }
            
pub type Repository = String;
#[doc="<p>Information about an application revision.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RevisionInfo {
                #[doc="<p>Information about an application revision, including usage details and currently associated deployment groups.</p>"]
#[serde(rename="genericRevisionInfo")]
pub generic_revision_info: Option<GenericRevisionInfo>,
#[doc="<p>Information about the location and type of an application revision.</p>"]
#[serde(rename="revisionLocation")]
pub revision_location: Option<RevisionLocation>,
            }
            
pub type RevisionInfoList = Vec<RevisionInfo>;
#[doc="<p>Information about the location of an application revision.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct RevisionLocation {
                #[doc="<p>Information about the location of application artifacts stored in GitHub.</p>"]
#[serde(rename="gitHubLocation")]
pub git_hub_location: Option<GitHubLocation>,
#[doc="<p>The type of application revision:</p> <ul> <li> <p>S3: An application revision stored in Amazon S3.</p> </li> <li> <p>GitHub: An application revision stored in GitHub.</p> </li> </ul>"]
#[serde(rename="revisionType")]
pub revision_type: Option<RevisionLocationType>,
#[doc="<p>Information about the location of application artifacts stored in Amazon S3. </p>"]
#[serde(rename="s3Location")]
pub s_3_location: Option<S3Location>,
            }
            
pub type RevisionLocationList = Vec<RevisionLocation>;
pub type RevisionLocationType = String;
pub type Role = String;
#[doc="<p>Information about a deployment rollback.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RollbackInfo {
                #[doc="<p>The ID of the deployment rollback.</p>"]
#[serde(rename="rollbackDeploymentId")]
pub rollback_deployment_id: Option<DeploymentId>,
#[doc="<p>Information describing the status of a deployment rollback; for example, whether the deployment can't be rolled back, is in progress, failed, or succeeded. </p>"]
#[serde(rename="rollbackMessage")]
pub rollback_message: Option<Description>,
#[doc="<p>The deployment ID of the deployment that was underway and triggered a rollback deployment because it failed or was stopped.</p>"]
#[serde(rename="rollbackTriggeringDeploymentId")]
pub rollback_triggering_deployment_id: Option<DeploymentId>,
            }
            
pub type S3Bucket = String;
pub type S3Key = String;
#[doc="<p>Information about the location of application artifacts stored in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct S3Location {
                #[doc="<p>The name of the Amazon S3 bucket where the application revision is stored.</p>"]
#[serde(rename="bucket")]
pub bucket: Option<S3Bucket>,
#[doc="<p>The file type of the application revision. Must be one of the following:</p> <ul> <li> <p>tar: A tar archive file.</p> </li> <li> <p>tgz: A compressed tar archive file.</p> </li> <li> <p>zip: A zip archive file.</p> </li> </ul>"]
#[serde(rename="bundleType")]
pub bundle_type: Option<BundleType>,
#[doc="<p>The ETag of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the ETag is not specified as an input parameter, ETag validation of the object will be skipped.</p>"]
#[serde(rename="eTag")]
pub e_tag: Option<ETag>,
#[doc="<p>The name of the Amazon S3 object that represents the bundled artifacts for the application revision.</p>"]
#[serde(rename="key")]
pub key: Option<S3Key>,
#[doc="<p>A specific version of the Amazon S3 object that represents the bundled artifacts for the application revision.</p> <p>If the version is not specified, the system will use the most recent version by default.</p>"]
#[serde(rename="version")]
pub version: Option<VersionId>,
            }
            
pub type ScriptName = String;
pub type SortOrder = String;
#[doc="<p>Represents the input of a stop deployment operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct StopDeploymentInput {
                #[doc="<p>Indicates, when a deployment is stopped, whether instances that have been updated should be rolled back to the previous version of the application revision.</p>"]
#[serde(rename="autoRollbackEnabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub auto_rollback_enabled: Option<NullableBoolean>,
#[doc="<p>The unique ID of a deployment.</p>"]
#[serde(rename="deploymentId")]
pub deployment_id: DeploymentId,
            }
            
#[doc="<p>Represents the output of a stop deployment operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StopDeploymentOutput {
                #[doc="<p>The status of the stop deployment operation:</p> <ul> <li> <p>Pending: The stop operation is pending.</p> </li> <li> <p>Succeeded: The stop operation was successful.</p> </li> </ul>"]
#[serde(rename="status")]
pub status: Option<StopStatus>,
#[doc="<p>An accompanying status message.</p>"]
#[serde(rename="statusMessage")]
pub status_message: Option<Message>,
            }
            
pub type StopStatus = String;
#[doc="<p>Information about a tag.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>The tag's key.</p>"]
#[serde(rename="Key")]
pub key: Option<Key>,
#[doc="<p>The tag's value.</p>"]
#[serde(rename="Value")]
pub value: Option<Value>,
            }
            
#[doc="<p>Information about an on-premises instance tag filter.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct TagFilter {
                #[doc="<p>The on-premises instance tag filter key.</p>"]
#[serde(rename="Key")]
pub key: Option<Key>,
#[doc="<p>The on-premises instance tag filter type:</p> <ul> <li> <p>KEY_ONLY: Key only.</p> </li> <li> <p>VALUE_ONLY: Value only.</p> </li> <li> <p>KEY_AND_VALUE: Key and value.</p> </li> </ul>"]
#[serde(rename="Type")]
pub type_: Option<TagFilterType>,
#[doc="<p>The on-premises instance tag filter value.</p>"]
#[serde(rename="Value")]
pub value: Option<Value>,
            }
            
pub type TagFilterList = Vec<TagFilter>;
pub type TagFilterType = String;
pub type TagList = Vec<Tag>;
#[doc="<p>Information about a time range.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct TimeRange {
                #[doc="<p>The end time of the time range.</p> <note> <p>Specify null to leave the end time open-ended.</p> </note>"]
#[serde(rename="end")]
pub end: Option<Timestamp>,
#[doc="<p>The start time of the time range.</p> <note> <p>Specify null to leave the start time open-ended.</p> </note>"]
#[serde(rename="start")]
pub start: Option<Timestamp>,
            }
            
pub type Timestamp = f64;
#[doc="<p>Information about notification triggers for the deployment group.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct TriggerConfig {
                #[doc="<p>The event type or types for which notifications are triggered.</p>"]
#[serde(rename="triggerEvents")]
pub trigger_events: Option<TriggerEventTypeList>,
#[doc="<p>The name of the notification trigger.</p>"]
#[serde(rename="triggerName")]
pub trigger_name: Option<TriggerName>,
#[doc="<p>The ARN of the Amazon Simple Notification Service topic through which notifications about deployment or instance events are sent.</p>"]
#[serde(rename="triggerTargetArn")]
pub trigger_target_arn: Option<TriggerTargetArn>,
            }
            
pub type TriggerConfigList = Vec<TriggerConfig>;
pub type TriggerEventType = String;
pub type TriggerEventTypeList = Vec<TriggerEventType>;
pub type TriggerName = String;
pub type TriggerTargetArn = String;
#[doc="<p>Represents the input of an update application operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateApplicationInput {
                #[doc="<p>The current name of the application you want to change.</p>"]
#[serde(rename="applicationName")]
pub application_name: Option<ApplicationName>,
#[doc="<p>The new name to give the application.</p>"]
#[serde(rename="newApplicationName")]
pub new_application_name: Option<ApplicationName>,
            }
            
#[doc="<p>Represents the input of an update deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateDeploymentGroupInput {
                #[doc="<p>Information to add or change about Amazon CloudWatch alarms when the deployment group is updated. </p>"]
#[serde(rename="alarmConfiguration")]
pub alarm_configuration: Option<AlarmConfiguration>,
#[doc="<p>The application name corresponding to the deployment group to update.</p>"]
#[serde(rename="applicationName")]
pub application_name: ApplicationName,
#[doc="<p>Information for an automatic rollback configuration that is added or changed when a deployment group is updated.</p>"]
#[serde(rename="autoRollbackConfiguration")]
pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
#[doc="<p>The replacement list of Auto Scaling groups to be included in the deployment group, if you want to change them. To keep the Auto Scaling groups, enter their names. To remove Auto Scaling groups, do not enter any Auto Scaling group names.</p>"]
#[serde(rename="autoScalingGroups")]
pub auto_scaling_groups: Option<AutoScalingGroupNameList>,
#[doc="<p>The current name of the deployment group.</p>"]
#[serde(rename="currentDeploymentGroupName")]
pub current_deployment_group_name: DeploymentGroupName,
#[doc="<p>The replacement deployment configuration name to use, if you want to change it.</p>"]
#[serde(rename="deploymentConfigName")]
pub deployment_config_name: Option<DeploymentConfigName>,
#[doc="<p>The replacement set of Amazon EC2 tags on which to filter, if you want to change them. To keep the existing tags, enter their names. To remove tags, do not enter any tag names.</p>"]
#[serde(rename="ec2TagFilters")]
pub ec_2_tag_filters: Option<EC2TagFilterList>,
#[doc="<p>The new name of the deployment group, if you want to change it.</p>"]
#[serde(rename="newDeploymentGroupName")]
pub new_deployment_group_name: Option<DeploymentGroupName>,
#[doc="<p>The replacement set of on-premises instance tags on which to filter, if you want to change them. To keep the existing tags, enter their names. To remove tags, do not enter any tag names.</p>"]
#[serde(rename="onPremisesInstanceTagFilters")]
pub on_premises_instance_tag_filters: Option<TagFilterList>,
#[doc="<p>A replacement ARN for the service role, if you want to change it.</p>"]
#[serde(rename="serviceRoleArn")]
pub service_role_arn: Option<Role>,
#[doc="<p>Information about triggers to change when the deployment group is updated. For examples, see <a href=\"http://docs.aws.amazon.com/codedeploy/latest/userguide/how-to-notify-edit.html\">Modify Triggers in an AWS CodeDeploy Deployment Group</a> in the AWS CodeDeploy User Guide.</p>"]
#[serde(rename="triggerConfigurations")]
pub trigger_configurations: Option<TriggerConfigList>,
            }
            
#[doc="<p>Represents the output of an update deployment group operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateDeploymentGroupOutput {
                #[doc="<p>If the output contains no data, and the corresponding deployment group contained at least one Auto Scaling group, AWS CodeDeploy successfully removed all corresponding Auto Scaling lifecycle event hooks from the AWS account. If the output contains data, AWS CodeDeploy could not remove some Auto Scaling lifecycle event hooks from the AWS account.</p>"]
#[serde(rename="hooksNotCleanedUp")]
pub hooks_not_cleaned_up: Option<AutoScalingGroupList>,
            }
            
pub type Value = String;
pub type VersionId = String;
/// Errors returned by AddTagsToOnPremisesInstances
                #[derive(Debug, PartialEq)]
                pub enum AddTagsToOnPremisesInstancesError {
                    
///<p>The maximum allowed number of tags was exceeded.</p>
TagLimitExceeded(String),
///<p>The maximum number of allowed on-premises instances in a single call was exceeded.</p>
InstanceLimitExceeded(String),
///<p>The specified on-premises instance is not registered.</p>
InstanceNotRegistered(String),
///<p>The specified tag was specified in an invalid format.</p>
InvalidTag(String),
///<p>A tag was not specified.</p>
TagRequired(String),
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddTagsToOnPremisesInstancesError {
                    pub fn from_body(body: &str) -> AddTagsToOnPremisesInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InstanceNameRequiredException" => AddTagsToOnPremisesInstancesError::InstanceNameRequired(String::from(error_message)),"TagLimitExceededException" => AddTagsToOnPremisesInstancesError::TagLimitExceeded(String::from(error_message)),"InstanceLimitExceededException" => AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(String::from(error_message)),"TagRequiredException" => AddTagsToOnPremisesInstancesError::TagRequired(String::from(error_message)),"InstanceNotRegisteredException" => AddTagsToOnPremisesInstancesError::InstanceNotRegistered(String::from(error_message)),"InvalidTagException" => AddTagsToOnPremisesInstancesError::InvalidTag(String::from(error_message)),"ValidationException" => AddTagsToOnPremisesInstancesError::Validation(error_message.to_string()),_ => AddTagsToOnPremisesInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddTagsToOnPremisesInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddTagsToOnPremisesInstancesError {
                    fn from(err: serde_json::error::Error) -> AddTagsToOnPremisesInstancesError {
                        AddTagsToOnPremisesInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for AddTagsToOnPremisesInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddTagsToOnPremisesInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            AddTagsToOnPremisesInstancesError::InvalidTag(ref cause) => cause,AddTagsToOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => cause,AddTagsToOnPremisesInstancesError::InstanceNotRegistered(ref cause) => cause,AddTagsToOnPremisesInstancesError::TagLimitExceeded(ref cause) => cause,AddTagsToOnPremisesInstancesError::TagRequired(ref cause) => cause,AddTagsToOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,AddTagsToOnPremisesInstancesError::Validation(ref cause) => cause,AddTagsToOnPremisesInstancesError::Credentials(ref err) => err.description(),AddTagsToOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),AddTagsToOnPremisesInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetApplicationRevisions
                #[derive(Debug, PartialEq)]
                pub enum BatchGetApplicationRevisionsError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The revision ID was not specified.</p>
RevisionRequired(String),
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The revision was specified in an invalid format.</p>
InvalidRevision(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetApplicationRevisionsError {
                    pub fn from_body(body: &str) -> BatchGetApplicationRevisionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationNameRequiredException" => BatchGetApplicationRevisionsError::ApplicationNameRequired(String::from(error_message)),"RevisionRequiredException" => BatchGetApplicationRevisionsError::RevisionRequired(String::from(error_message)),"InvalidRevisionException" => BatchGetApplicationRevisionsError::InvalidRevision(String::from(error_message)),"ApplicationDoesNotExistException" => BatchGetApplicationRevisionsError::ApplicationDoesNotExist(String::from(error_message)),"BatchLimitExceededException" => BatchGetApplicationRevisionsError::BatchLimitExceeded(String::from(error_message)),"InvalidApplicationNameException" => BatchGetApplicationRevisionsError::InvalidApplicationName(String::from(error_message)),"ValidationException" => BatchGetApplicationRevisionsError::Validation(error_message.to_string()),_ => BatchGetApplicationRevisionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetApplicationRevisionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetApplicationRevisionsError {
                    fn from(err: serde_json::error::Error) -> BatchGetApplicationRevisionsError {
                        BatchGetApplicationRevisionsError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetApplicationRevisionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetApplicationRevisionsError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetApplicationRevisionsError::BatchLimitExceeded(ref cause) => cause,BatchGetApplicationRevisionsError::RevisionRequired(ref cause) => cause,BatchGetApplicationRevisionsError::ApplicationNameRequired(ref cause) => cause,BatchGetApplicationRevisionsError::InvalidApplicationName(ref cause) => cause,BatchGetApplicationRevisionsError::InvalidRevision(ref cause) => cause,BatchGetApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => cause,BatchGetApplicationRevisionsError::Validation(ref cause) => cause,BatchGetApplicationRevisionsError::Credentials(ref err) => err.description(),BatchGetApplicationRevisionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetApplicationRevisionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetApplications
                #[derive(Debug, PartialEq)]
                pub enum BatchGetApplicationsError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetApplicationsError {
                    pub fn from_body(body: &str) -> BatchGetApplicationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "BatchLimitExceededException" => BatchGetApplicationsError::BatchLimitExceeded(String::from(error_message)),"ApplicationNameRequiredException" => BatchGetApplicationsError::ApplicationNameRequired(String::from(error_message)),"ApplicationDoesNotExistException" => BatchGetApplicationsError::ApplicationDoesNotExist(String::from(error_message)),"InvalidApplicationNameException" => BatchGetApplicationsError::InvalidApplicationName(String::from(error_message)),"ValidationException" => BatchGetApplicationsError::Validation(error_message.to_string()),_ => BatchGetApplicationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetApplicationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetApplicationsError {
                    fn from(err: serde_json::error::Error) -> BatchGetApplicationsError {
                        BatchGetApplicationsError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetApplicationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetApplicationsError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetApplicationsError::InvalidApplicationName(ref cause) => cause,BatchGetApplicationsError::ApplicationDoesNotExist(ref cause) => cause,BatchGetApplicationsError::ApplicationNameRequired(ref cause) => cause,BatchGetApplicationsError::BatchLimitExceeded(ref cause) => cause,BatchGetApplicationsError::Validation(ref cause) => cause,BatchGetApplicationsError::Credentials(ref err) => err.description(),BatchGetApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetApplicationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetDeploymentGroups
                #[derive(Debug, PartialEq)]
                pub enum BatchGetDeploymentGroupsError {
                    
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetDeploymentGroupsError {
                    pub fn from_body(body: &str) -> BatchGetDeploymentGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationNameRequiredException" => BatchGetDeploymentGroupsError::ApplicationNameRequired(String::from(error_message)),"InvalidApplicationNameException" => BatchGetDeploymentGroupsError::InvalidApplicationName(String::from(error_message)),"BatchLimitExceededException" => BatchGetDeploymentGroupsError::BatchLimitExceeded(String::from(error_message)),"ApplicationDoesNotExistException" => BatchGetDeploymentGroupsError::ApplicationDoesNotExist(String::from(error_message)),"InvalidDeploymentGroupNameException" => BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(String::from(error_message)),"DeploymentGroupNameRequiredException" => BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(String::from(error_message)),"ValidationException" => BatchGetDeploymentGroupsError::Validation(error_message.to_string()),_ => BatchGetDeploymentGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetDeploymentGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetDeploymentGroupsError {
                    fn from(err: serde_json::error::Error) -> BatchGetDeploymentGroupsError {
                        BatchGetDeploymentGroupsError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetDeploymentGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetDeploymentGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetDeploymentGroupsError::InvalidApplicationName(ref cause) => cause,BatchGetDeploymentGroupsError::ApplicationNameRequired(ref cause) => cause,BatchGetDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => cause,BatchGetDeploymentGroupsError::InvalidDeploymentGroupName(ref cause) => cause,BatchGetDeploymentGroupsError::DeploymentGroupNameRequired(ref cause) => cause,BatchGetDeploymentGroupsError::BatchLimitExceeded(ref cause) => cause,BatchGetDeploymentGroupsError::Validation(ref cause) => cause,BatchGetDeploymentGroupsError::Credentials(ref err) => err.description(),BatchGetDeploymentGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetDeploymentGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetDeploymentInstances
                #[derive(Debug, PartialEq)]
                pub enum BatchGetDeploymentInstancesError {
                    
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>The instance ID was not specified.</p>
InstanceIdRequired(String),
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),
///<p>The deployment does not exist with the applicable IAM user or AWS account.</p>
DeploymentDoesNotExist(String),
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetDeploymentInstancesError {
                    pub fn from_body(body: &str) -> BatchGetDeploymentInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentIdRequiredException" => BatchGetDeploymentInstancesError::DeploymentIdRequired(String::from(error_message)),"InstanceIdRequiredException" => BatchGetDeploymentInstancesError::InstanceIdRequired(String::from(error_message)),"InvalidInstanceNameException" => BatchGetDeploymentInstancesError::InvalidInstanceName(String::from(error_message)),"DeploymentDoesNotExistException" => BatchGetDeploymentInstancesError::DeploymentDoesNotExist(String::from(error_message)),"InvalidDeploymentIdException" => BatchGetDeploymentInstancesError::InvalidDeploymentId(String::from(error_message)),"BatchLimitExceededException" => BatchGetDeploymentInstancesError::BatchLimitExceeded(String::from(error_message)),"ValidationException" => BatchGetDeploymentInstancesError::Validation(error_message.to_string()),_ => BatchGetDeploymentInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetDeploymentInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetDeploymentInstancesError {
                    fn from(err: serde_json::error::Error) -> BatchGetDeploymentInstancesError {
                        BatchGetDeploymentInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetDeploymentInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetDeploymentInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetDeploymentInstancesError::BatchLimitExceeded(ref cause) => cause,BatchGetDeploymentInstancesError::InvalidDeploymentId(ref cause) => cause,BatchGetDeploymentInstancesError::InvalidInstanceName(ref cause) => cause,BatchGetDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => cause,BatchGetDeploymentInstancesError::DeploymentIdRequired(ref cause) => cause,BatchGetDeploymentInstancesError::InstanceIdRequired(ref cause) => cause,BatchGetDeploymentInstancesError::Validation(ref cause) => cause,BatchGetDeploymentInstancesError::Credentials(ref err) => err.description(),BatchGetDeploymentInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetDeploymentInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetDeployments
                #[derive(Debug, PartialEq)]
                pub enum BatchGetDeploymentsError {
                    
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetDeploymentsError {
                    pub fn from_body(body: &str) -> BatchGetDeploymentsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeploymentIdException" => BatchGetDeploymentsError::InvalidDeploymentId(String::from(error_message)),"BatchLimitExceededException" => BatchGetDeploymentsError::BatchLimitExceeded(String::from(error_message)),"DeploymentIdRequiredException" => BatchGetDeploymentsError::DeploymentIdRequired(String::from(error_message)),"ValidationException" => BatchGetDeploymentsError::Validation(error_message.to_string()),_ => BatchGetDeploymentsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetDeploymentsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetDeploymentsError {
                    fn from(err: serde_json::error::Error) -> BatchGetDeploymentsError {
                        BatchGetDeploymentsError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetDeploymentsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetDeploymentsError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetDeploymentsError::DeploymentIdRequired(ref cause) => cause,BatchGetDeploymentsError::InvalidDeploymentId(ref cause) => cause,BatchGetDeploymentsError::BatchLimitExceeded(ref cause) => cause,BatchGetDeploymentsError::Validation(ref cause) => cause,BatchGetDeploymentsError::Credentials(ref err) => err.description(),BatchGetDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetDeploymentsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchGetOnPremisesInstances
                #[derive(Debug, PartialEq)]
                pub enum BatchGetOnPremisesInstancesError {
                    
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),
///<p>The maximum number of names or IDs allowed for this request (100) was exceeded.</p>
BatchLimitExceeded(String),
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetOnPremisesInstancesError {
                    pub fn from_body(body: &str) -> BatchGetOnPremisesInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InstanceNameRequiredException" => BatchGetOnPremisesInstancesError::InstanceNameRequired(String::from(error_message)),"BatchLimitExceededException" => BatchGetOnPremisesInstancesError::BatchLimitExceeded(String::from(error_message)),"InvalidInstanceNameException" => BatchGetOnPremisesInstancesError::InvalidInstanceName(String::from(error_message)),"ValidationException" => BatchGetOnPremisesInstancesError::Validation(error_message.to_string()),_ => BatchGetOnPremisesInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetOnPremisesInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for BatchGetOnPremisesInstancesError {
                    fn from(err: serde_json::error::Error) -> BatchGetOnPremisesInstancesError {
                        BatchGetOnPremisesInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for BatchGetOnPremisesInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetOnPremisesInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetOnPremisesInstancesError::InvalidInstanceName(ref cause) => cause,BatchGetOnPremisesInstancesError::BatchLimitExceeded(ref cause) => cause,BatchGetOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,BatchGetOnPremisesInstancesError::Validation(ref cause) => cause,BatchGetOnPremisesInstancesError::Credentials(ref err) => err.description(),BatchGetOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetOnPremisesInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateApplication
                #[derive(Debug, PartialEq)]
                pub enum CreateApplicationError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>More applications were attempted to be created than are allowed.</p>
ApplicationLimitExceeded(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>An application with the specified name already exists with the applicable IAM user or AWS account.</p>
ApplicationAlreadyExists(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateApplicationError {
                    pub fn from_body(body: &str) -> CreateApplicationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationLimitExceededException" => CreateApplicationError::ApplicationLimitExceeded(String::from(error_message)),"ApplicationAlreadyExistsException" => CreateApplicationError::ApplicationAlreadyExists(String::from(error_message)),"InvalidApplicationNameException" => CreateApplicationError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => CreateApplicationError::ApplicationNameRequired(String::from(error_message)),"ValidationException" => CreateApplicationError::Validation(error_message.to_string()),_ => CreateApplicationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateApplicationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateApplicationError {
                    fn from(err: serde_json::error::Error) -> CreateApplicationError {
                        CreateApplicationError::Unknown(err.description().to_string())
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
                impl fmt::Display for CreateApplicationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateApplicationError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateApplicationError::ApplicationLimitExceeded(ref cause) => cause,CreateApplicationError::ApplicationNameRequired(ref cause) => cause,CreateApplicationError::InvalidApplicationName(ref cause) => cause,CreateApplicationError::ApplicationAlreadyExists(ref cause) => cause,CreateApplicationError::Validation(ref cause) => cause,CreateApplicationError::Credentials(ref err) => err.description(),CreateApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateApplicationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateDeployment
                #[derive(Debug, PartialEq)]
                pub enum CreateDeploymentError {
                    
///<p>The description is too long.</p>
DescriptionTooLong(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The number of allowed deployments was exceeded.</p>
DeploymentLimitExceeded(String),
///<p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
InvalidAutoRollbackConfig(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The named revision does not exist with the applicable IAM user or AWS account.</p>
RevisionDoesNotExist(String),
///<p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
DeploymentGroupDoesNotExist(String),
///<p>The revision ID was not specified.</p>
RevisionRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
DeploymentConfigDoesNotExist(String),
///<p>The revision was specified in an invalid format.</p>
InvalidRevision(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateDeploymentError {
                    pub fn from_body(body: &str) -> CreateDeploymentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidRevisionException" => CreateDeploymentError::InvalidRevision(String::from(error_message)),"DeploymentGroupDoesNotExistException" => CreateDeploymentError::DeploymentGroupDoesNotExist(String::from(error_message)),"ApplicationNameRequiredException" => CreateDeploymentError::ApplicationNameRequired(String::from(error_message)),"ApplicationDoesNotExistException" => CreateDeploymentError::ApplicationDoesNotExist(String::from(error_message)),"RevisionRequiredException" => CreateDeploymentError::RevisionRequired(String::from(error_message)),"RevisionDoesNotExistException" => CreateDeploymentError::RevisionDoesNotExist(String::from(error_message)),"DeploymentGroupNameRequiredException" => CreateDeploymentError::DeploymentGroupNameRequired(String::from(error_message)),"InvalidDeploymentConfigNameException" => CreateDeploymentError::InvalidDeploymentConfigName(String::from(error_message)),"InvalidApplicationNameException" => CreateDeploymentError::InvalidApplicationName(String::from(error_message)),"InvalidDeploymentGroupNameException" => CreateDeploymentError::InvalidDeploymentGroupName(String::from(error_message)),"DescriptionTooLongException" => CreateDeploymentError::DescriptionTooLong(String::from(error_message)),"DeploymentLimitExceededException" => CreateDeploymentError::DeploymentLimitExceeded(String::from(error_message)),"InvalidAutoRollbackConfigException" => CreateDeploymentError::InvalidAutoRollbackConfig(String::from(error_message)),"DeploymentConfigDoesNotExistException" => CreateDeploymentError::DeploymentConfigDoesNotExist(String::from(error_message)),"ValidationException" => CreateDeploymentError::Validation(error_message.to_string()),_ => CreateDeploymentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateDeploymentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateDeploymentError {
                    fn from(err: serde_json::error::Error) -> CreateDeploymentError {
                        CreateDeploymentError::Unknown(err.description().to_string())
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
                impl fmt::Display for CreateDeploymentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateDeploymentError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateDeploymentError::ApplicationDoesNotExist(ref cause) => cause,CreateDeploymentError::InvalidRevision(ref cause) => cause,CreateDeploymentError::ApplicationNameRequired(ref cause) => cause,CreateDeploymentError::DeploymentGroupDoesNotExist(ref cause) => cause,CreateDeploymentError::InvalidDeploymentGroupName(ref cause) => cause,CreateDeploymentError::DeploymentGroupNameRequired(ref cause) => cause,CreateDeploymentError::InvalidAutoRollbackConfig(ref cause) => cause,CreateDeploymentError::InvalidApplicationName(ref cause) => cause,CreateDeploymentError::DescriptionTooLong(ref cause) => cause,CreateDeploymentError::RevisionDoesNotExist(ref cause) => cause,CreateDeploymentError::InvalidDeploymentConfigName(ref cause) => cause,CreateDeploymentError::DeploymentConfigDoesNotExist(ref cause) => cause,CreateDeploymentError::DeploymentLimitExceeded(ref cause) => cause,CreateDeploymentError::RevisionRequired(ref cause) => cause,CreateDeploymentError::Validation(ref cause) => cause,CreateDeploymentError::Credentials(ref err) => err.description(),CreateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateDeploymentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateDeploymentConfig
                #[derive(Debug, PartialEq)]
                pub enum CreateDeploymentConfigError {
                    
///<p>The deployment configuration name was not specified.</p>
DeploymentConfigNameRequired(String),
///<p>The deployment configurations limit was exceeded.</p>
DeploymentConfigLimitExceeded(String),
///<p>The minimum healthy instance value was specified in an invalid format.</p>
InvalidMinimumHealthyHostValue(String),
///<p>A deployment configuration with the specified name already exists with the applicable IAM user or AWS account.</p>
DeploymentConfigAlreadyExists(String),
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateDeploymentConfigError {
                    pub fn from_body(body: &str) -> CreateDeploymentConfigError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidMinimumHealthyHostValueException" => CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(String::from(error_message)),"InvalidDeploymentConfigNameException" => CreateDeploymentConfigError::InvalidDeploymentConfigName(String::from(error_message)),"DeploymentConfigAlreadyExistsException" => CreateDeploymentConfigError::DeploymentConfigAlreadyExists(String::from(error_message)),"DeploymentConfigNameRequiredException" => CreateDeploymentConfigError::DeploymentConfigNameRequired(String::from(error_message)),"DeploymentConfigLimitExceededException" => CreateDeploymentConfigError::DeploymentConfigLimitExceeded(String::from(error_message)),"ValidationException" => CreateDeploymentConfigError::Validation(error_message.to_string()),_ => CreateDeploymentConfigError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateDeploymentConfigError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateDeploymentConfigError {
                    fn from(err: serde_json::error::Error) -> CreateDeploymentConfigError {
                        CreateDeploymentConfigError::Unknown(err.description().to_string())
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
                impl fmt::Display for CreateDeploymentConfigError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateDeploymentConfigError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateDeploymentConfigError::DeploymentConfigAlreadyExists(ref cause) => cause,CreateDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,CreateDeploymentConfigError::DeploymentConfigLimitExceeded(ref cause) => cause,CreateDeploymentConfigError::InvalidMinimumHealthyHostValue(ref cause) => cause,CreateDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,CreateDeploymentConfigError::Validation(ref cause) => cause,CreateDeploymentConfigError::Credentials(ref err) => err.description(),CreateDeploymentConfigError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateDeploymentConfigError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateDeploymentGroup
                #[derive(Debug, PartialEq)]
                pub enum CreateDeploymentGroupError {
                    
///<p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
InvalidRole(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
InvalidAutoRollbackConfig(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
DeploymentConfigDoesNotExist(String),
///<p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
AlarmsLimitExceeded(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The trigger was specified in an invalid format.</p>
InvalidTriggerConfig(String),
///<p>The tag was specified in an invalid format.</p>
InvalidEC2Tag(String),
///<p> The deployment groups limit was exceeded.</p>
DeploymentGroupLimitExceeded(String),
///<p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the 255 character limit.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled but the alarm list is empty.</p> </li> </ul>
InvalidAlarmConfig(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The maximum allowed number of triggers was exceeded.</p>
TriggerTargetsLimitExceeded(String),
///<p>A deployment group with the specified name already exists with the applicable IAM user or AWS account.</p>
DeploymentGroupAlreadyExists(String),
///<p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
InvalidAutoScalingGroup(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),
///<p>The role ID was not specified.</p>
RoleRequired(String),
///<p>The limit for lifecycle hooks was exceeded.</p>
LifecycleHookLimitExceeded(String),
///<p>The specified tag was specified in an invalid format.</p>
InvalidTag(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateDeploymentGroupError {
                    pub fn from_body(body: &str) -> CreateDeploymentGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeploymentConfigNameException" => CreateDeploymentGroupError::InvalidDeploymentConfigName(String::from(error_message)),"DeploymentGroupLimitExceededException" => CreateDeploymentGroupError::DeploymentGroupLimitExceeded(String::from(error_message)),"InvalidAlarmConfigException" => CreateDeploymentGroupError::InvalidAlarmConfig(String::from(error_message)),"ApplicationDoesNotExistException" => CreateDeploymentGroupError::ApplicationDoesNotExist(String::from(error_message)),"DeploymentGroupNameRequiredException" => CreateDeploymentGroupError::DeploymentGroupNameRequired(String::from(error_message)),"InvalidTagException" => CreateDeploymentGroupError::InvalidTag(String::from(error_message)),"InvalidRoleException" => CreateDeploymentGroupError::InvalidRole(String::from(error_message)),"InvalidDeploymentGroupNameException" => CreateDeploymentGroupError::InvalidDeploymentGroupName(String::from(error_message)),"ApplicationNameRequiredException" => CreateDeploymentGroupError::ApplicationNameRequired(String::from(error_message)),"DeploymentConfigDoesNotExistException" => CreateDeploymentGroupError::DeploymentConfigDoesNotExist(String::from(error_message)),"InvalidApplicationNameException" => CreateDeploymentGroupError::InvalidApplicationName(String::from(error_message)),"InvalidAutoScalingGroupException" => CreateDeploymentGroupError::InvalidAutoScalingGroup(String::from(error_message)),"InvalidEC2TagException" => CreateDeploymentGroupError::InvalidEC2Tag(String::from(error_message)),"RoleRequiredException" => CreateDeploymentGroupError::RoleRequired(String::from(error_message)),"LifecycleHookLimitExceededException" => CreateDeploymentGroupError::LifecycleHookLimitExceeded(String::from(error_message)),"InvalidAutoRollbackConfigException" => CreateDeploymentGroupError::InvalidAutoRollbackConfig(String::from(error_message)),"DeploymentGroupAlreadyExistsException" => CreateDeploymentGroupError::DeploymentGroupAlreadyExists(String::from(error_message)),"TriggerTargetsLimitExceededException" => CreateDeploymentGroupError::TriggerTargetsLimitExceeded(String::from(error_message)),"InvalidTriggerConfigException" => CreateDeploymentGroupError::InvalidTriggerConfig(String::from(error_message)),"AlarmsLimitExceededException" => CreateDeploymentGroupError::AlarmsLimitExceeded(String::from(error_message)),"ValidationException" => CreateDeploymentGroupError::Validation(error_message.to_string()),_ => CreateDeploymentGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateDeploymentGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateDeploymentGroupError {
                    fn from(err: serde_json::error::Error) -> CreateDeploymentGroupError {
                        CreateDeploymentGroupError::Unknown(err.description().to_string())
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
                impl fmt::Display for CreateDeploymentGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateDeploymentGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,CreateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => cause,CreateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => cause,CreateDeploymentGroupError::DeploymentGroupLimitExceeded(ref cause) => cause,CreateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => cause,CreateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => cause,CreateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => cause,CreateDeploymentGroupError::InvalidEC2Tag(ref cause) => cause,CreateDeploymentGroupError::InvalidTriggerConfig(ref cause) => cause,CreateDeploymentGroupError::InvalidApplicationName(ref cause) => cause,CreateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => cause,CreateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => cause,CreateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,CreateDeploymentGroupError::InvalidAlarmConfig(ref cause) => cause,CreateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => cause,CreateDeploymentGroupError::InvalidRole(ref cause) => cause,CreateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,CreateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,CreateDeploymentGroupError::InvalidTag(ref cause) => cause,CreateDeploymentGroupError::RoleRequired(ref cause) => cause,CreateDeploymentGroupError::Validation(ref cause) => cause,CreateDeploymentGroupError::Credentials(ref err) => err.description(),CreateDeploymentGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateDeploymentGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteApplication
                #[derive(Debug, PartialEq)]
                pub enum DeleteApplicationError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteApplicationError {
                    pub fn from_body(body: &str) -> DeleteApplicationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidApplicationNameException" => DeleteApplicationError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => DeleteApplicationError::ApplicationNameRequired(String::from(error_message)),"ValidationException" => DeleteApplicationError::Validation(error_message.to_string()),_ => DeleteApplicationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteApplicationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteApplicationError {
                    fn from(err: serde_json::error::Error) -> DeleteApplicationError {
                        DeleteApplicationError::Unknown(err.description().to_string())
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
                impl fmt::Display for DeleteApplicationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteApplicationError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteApplicationError::InvalidApplicationName(ref cause) => cause,DeleteApplicationError::ApplicationNameRequired(ref cause) => cause,DeleteApplicationError::Validation(ref cause) => cause,DeleteApplicationError::Credentials(ref err) => err.description(),DeleteApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteApplicationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteDeploymentConfig
                #[derive(Debug, PartialEq)]
                pub enum DeleteDeploymentConfigError {
                    
///<p>An invalid operation was detected.</p>
InvalidOperation(String),
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),
///<p>The deployment configuration name was not specified.</p>
DeploymentConfigNameRequired(String),
///<p>The deployment configuration is still in use.</p>
DeploymentConfigInUse(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteDeploymentConfigError {
                    pub fn from_body(body: &str) -> DeleteDeploymentConfigError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeploymentConfigNameException" => DeleteDeploymentConfigError::InvalidDeploymentConfigName(String::from(error_message)),"InvalidOperationException" => DeleteDeploymentConfigError::InvalidOperation(String::from(error_message)),"DeploymentConfigInUseException" => DeleteDeploymentConfigError::DeploymentConfigInUse(String::from(error_message)),"DeploymentConfigNameRequiredException" => DeleteDeploymentConfigError::DeploymentConfigNameRequired(String::from(error_message)),"ValidationException" => DeleteDeploymentConfigError::Validation(error_message.to_string()),_ => DeleteDeploymentConfigError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteDeploymentConfigError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteDeploymentConfigError {
                    fn from(err: serde_json::error::Error) -> DeleteDeploymentConfigError {
                        DeleteDeploymentConfigError::Unknown(err.description().to_string())
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
                impl fmt::Display for DeleteDeploymentConfigError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteDeploymentConfigError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteDeploymentConfigError::DeploymentConfigInUse(ref cause) => cause,DeleteDeploymentConfigError::InvalidOperation(ref cause) => cause,DeleteDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,DeleteDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,DeleteDeploymentConfigError::Validation(ref cause) => cause,DeleteDeploymentConfigError::Credentials(ref err) => err.description(),DeleteDeploymentConfigError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteDeploymentConfigError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteDeploymentGroup
                #[derive(Debug, PartialEq)]
                pub enum DeleteDeploymentGroupError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
InvalidRole(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteDeploymentGroupError {
                    pub fn from_body(body: &str) -> DeleteDeploymentGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentGroupNameRequiredException" => DeleteDeploymentGroupError::DeploymentGroupNameRequired(String::from(error_message)),"InvalidDeploymentGroupNameException" => DeleteDeploymentGroupError::InvalidDeploymentGroupName(String::from(error_message)),"InvalidApplicationNameException" => DeleteDeploymentGroupError::InvalidApplicationName(String::from(error_message)),"InvalidRoleException" => DeleteDeploymentGroupError::InvalidRole(String::from(error_message)),"ApplicationNameRequiredException" => DeleteDeploymentGroupError::ApplicationNameRequired(String::from(error_message)),"ValidationException" => DeleteDeploymentGroupError::Validation(error_message.to_string()),_ => DeleteDeploymentGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteDeploymentGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteDeploymentGroupError {
                    fn from(err: serde_json::error::Error) -> DeleteDeploymentGroupError {
                        DeleteDeploymentGroupError::Unknown(err.description().to_string())
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
                impl fmt::Display for DeleteDeploymentGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteDeploymentGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteDeploymentGroupError::InvalidApplicationName(ref cause) => cause,DeleteDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,DeleteDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,DeleteDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,DeleteDeploymentGroupError::InvalidRole(ref cause) => cause,DeleteDeploymentGroupError::Validation(ref cause) => cause,DeleteDeploymentGroupError::Credentials(ref err) => err.description(),DeleteDeploymentGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteDeploymentGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeregisterOnPremisesInstance
                #[derive(Debug, PartialEq)]
                pub enum DeregisterOnPremisesInstanceError {
                    
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeregisterOnPremisesInstanceError {
                    pub fn from_body(body: &str) -> DeregisterOnPremisesInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidInstanceNameException" => DeregisterOnPremisesInstanceError::InvalidInstanceName(String::from(error_message)),"InstanceNameRequiredException" => DeregisterOnPremisesInstanceError::InstanceNameRequired(String::from(error_message)),"ValidationException" => DeregisterOnPremisesInstanceError::Validation(error_message.to_string()),_ => DeregisterOnPremisesInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeregisterOnPremisesInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeregisterOnPremisesInstanceError {
                    fn from(err: serde_json::error::Error) -> DeregisterOnPremisesInstanceError {
                        DeregisterOnPremisesInstanceError::Unknown(err.description().to_string())
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
                impl fmt::Display for DeregisterOnPremisesInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeregisterOnPremisesInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            DeregisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,DeregisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,DeregisterOnPremisesInstanceError::Validation(ref cause) => cause,DeregisterOnPremisesInstanceError::Credentials(ref err) => err.description(),DeregisterOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeregisterOnPremisesInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetApplication
                #[derive(Debug, PartialEq)]
                pub enum GetApplicationError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetApplicationError {
                    pub fn from_body(body: &str) -> GetApplicationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidApplicationNameException" => GetApplicationError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => GetApplicationError::ApplicationNameRequired(String::from(error_message)),"ApplicationDoesNotExistException" => GetApplicationError::ApplicationDoesNotExist(String::from(error_message)),"ValidationException" => GetApplicationError::Validation(error_message.to_string()),_ => GetApplicationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetApplicationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetApplicationError {
                    fn from(err: serde_json::error::Error) -> GetApplicationError {
                        GetApplicationError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetApplicationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetApplicationError {
                    fn description(&self) -> &str {
                        match *self {
                            GetApplicationError::ApplicationNameRequired(ref cause) => cause,GetApplicationError::InvalidApplicationName(ref cause) => cause,GetApplicationError::ApplicationDoesNotExist(ref cause) => cause,GetApplicationError::Validation(ref cause) => cause,GetApplicationError::Credentials(ref err) => err.description(),GetApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetApplicationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetApplicationRevision
                #[derive(Debug, PartialEq)]
                pub enum GetApplicationRevisionError {
                    
///<p>The revision was specified in an invalid format.</p>
InvalidRevision(String),
///<p>The revision ID was not specified.</p>
RevisionRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The named revision does not exist with the applicable IAM user or AWS account.</p>
RevisionDoesNotExist(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetApplicationRevisionError {
                    pub fn from_body(body: &str) -> GetApplicationRevisionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationDoesNotExistException" => GetApplicationRevisionError::ApplicationDoesNotExist(String::from(error_message)),"InvalidApplicationNameException" => GetApplicationRevisionError::InvalidApplicationName(String::from(error_message)),"RevisionDoesNotExistException" => GetApplicationRevisionError::RevisionDoesNotExist(String::from(error_message)),"ApplicationNameRequiredException" => GetApplicationRevisionError::ApplicationNameRequired(String::from(error_message)),"RevisionRequiredException" => GetApplicationRevisionError::RevisionRequired(String::from(error_message)),"InvalidRevisionException" => GetApplicationRevisionError::InvalidRevision(String::from(error_message)),"ValidationException" => GetApplicationRevisionError::Validation(error_message.to_string()),_ => GetApplicationRevisionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetApplicationRevisionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetApplicationRevisionError {
                    fn from(err: serde_json::error::Error) -> GetApplicationRevisionError {
                        GetApplicationRevisionError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetApplicationRevisionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetApplicationRevisionError {
                    fn description(&self) -> &str {
                        match *self {
                            GetApplicationRevisionError::ApplicationDoesNotExist(ref cause) => cause,GetApplicationRevisionError::RevisionDoesNotExist(ref cause) => cause,GetApplicationRevisionError::InvalidApplicationName(ref cause) => cause,GetApplicationRevisionError::RevisionRequired(ref cause) => cause,GetApplicationRevisionError::ApplicationNameRequired(ref cause) => cause,GetApplicationRevisionError::InvalidRevision(ref cause) => cause,GetApplicationRevisionError::Validation(ref cause) => cause,GetApplicationRevisionError::Credentials(ref err) => err.description(),GetApplicationRevisionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetApplicationRevisionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDeployment
                #[derive(Debug, PartialEq)]
                pub enum GetDeploymentError {
                    
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),
///<p>The deployment does not exist with the applicable IAM user or AWS account.</p>
DeploymentDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDeploymentError {
                    pub fn from_body(body: &str) -> GetDeploymentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentDoesNotExistException" => GetDeploymentError::DeploymentDoesNotExist(String::from(error_message)),"InvalidDeploymentIdException" => GetDeploymentError::InvalidDeploymentId(String::from(error_message)),"DeploymentIdRequiredException" => GetDeploymentError::DeploymentIdRequired(String::from(error_message)),"ValidationException" => GetDeploymentError::Validation(error_message.to_string()),_ => GetDeploymentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDeploymentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDeploymentError {
                    fn from(err: serde_json::error::Error) -> GetDeploymentError {
                        GetDeploymentError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetDeploymentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDeploymentError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDeploymentError::DeploymentIdRequired(ref cause) => cause,GetDeploymentError::DeploymentDoesNotExist(ref cause) => cause,GetDeploymentError::InvalidDeploymentId(ref cause) => cause,GetDeploymentError::Validation(ref cause) => cause,GetDeploymentError::Credentials(ref err) => err.description(),GetDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDeploymentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDeploymentConfig
                #[derive(Debug, PartialEq)]
                pub enum GetDeploymentConfigError {
                    
///<p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
DeploymentConfigDoesNotExist(String),
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),
///<p>The deployment configuration name was not specified.</p>
DeploymentConfigNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDeploymentConfigError {
                    pub fn from_body(body: &str) -> GetDeploymentConfigError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeploymentConfigNameException" => GetDeploymentConfigError::InvalidDeploymentConfigName(String::from(error_message)),"DeploymentConfigDoesNotExistException" => GetDeploymentConfigError::DeploymentConfigDoesNotExist(String::from(error_message)),"DeploymentConfigNameRequiredException" => GetDeploymentConfigError::DeploymentConfigNameRequired(String::from(error_message)),"ValidationException" => GetDeploymentConfigError::Validation(error_message.to_string()),_ => GetDeploymentConfigError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDeploymentConfigError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDeploymentConfigError {
                    fn from(err: serde_json::error::Error) -> GetDeploymentConfigError {
                        GetDeploymentConfigError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetDeploymentConfigError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDeploymentConfigError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDeploymentConfigError::DeploymentConfigNameRequired(ref cause) => cause,GetDeploymentConfigError::DeploymentConfigDoesNotExist(ref cause) => cause,GetDeploymentConfigError::InvalidDeploymentConfigName(ref cause) => cause,GetDeploymentConfigError::Validation(ref cause) => cause,GetDeploymentConfigError::Credentials(ref err) => err.description(),GetDeploymentConfigError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDeploymentConfigError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDeploymentGroup
                #[derive(Debug, PartialEq)]
                pub enum GetDeploymentGroupError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
DeploymentGroupDoesNotExist(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDeploymentGroupError {
                    pub fn from_body(body: &str) -> GetDeploymentGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationDoesNotExistException" => GetDeploymentGroupError::ApplicationDoesNotExist(String::from(error_message)),"DeploymentGroupNameRequiredException" => GetDeploymentGroupError::DeploymentGroupNameRequired(String::from(error_message)),"DeploymentGroupDoesNotExistException" => GetDeploymentGroupError::DeploymentGroupDoesNotExist(String::from(error_message)),"InvalidDeploymentGroupNameException" => GetDeploymentGroupError::InvalidDeploymentGroupName(String::from(error_message)),"InvalidApplicationNameException" => GetDeploymentGroupError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => GetDeploymentGroupError::ApplicationNameRequired(String::from(error_message)),"ValidationException" => GetDeploymentGroupError::Validation(error_message.to_string()),_ => GetDeploymentGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDeploymentGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDeploymentGroupError {
                    fn from(err: serde_json::error::Error) -> GetDeploymentGroupError {
                        GetDeploymentGroupError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetDeploymentGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDeploymentGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,GetDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,GetDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,GetDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => cause,GetDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,GetDeploymentGroupError::InvalidApplicationName(ref cause) => cause,GetDeploymentGroupError::Validation(ref cause) => cause,GetDeploymentGroupError::Credentials(ref err) => err.description(),GetDeploymentGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDeploymentGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDeploymentInstance
                #[derive(Debug, PartialEq)]
                pub enum GetDeploymentInstanceError {
                    
///<p>The deployment does not exist with the applicable IAM user or AWS account.</p>
DeploymentDoesNotExist(String),
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),
///<p>The instance ID was not specified.</p>
InstanceIdRequired(String),
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),
///<p>The specified instance does not exist in the deployment group.</p>
InstanceDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDeploymentInstanceError {
                    pub fn from_body(body: &str) -> GetDeploymentInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentIdRequiredException" => GetDeploymentInstanceError::DeploymentIdRequired(String::from(error_message)),"InstanceDoesNotExistException" => GetDeploymentInstanceError::InstanceDoesNotExist(String::from(error_message)),"InvalidDeploymentIdException" => GetDeploymentInstanceError::InvalidDeploymentId(String::from(error_message)),"DeploymentDoesNotExistException" => GetDeploymentInstanceError::DeploymentDoesNotExist(String::from(error_message)),"InstanceIdRequiredException" => GetDeploymentInstanceError::InstanceIdRequired(String::from(error_message)),"InvalidInstanceNameException" => GetDeploymentInstanceError::InvalidInstanceName(String::from(error_message)),"ValidationException" => GetDeploymentInstanceError::Validation(error_message.to_string()),_ => GetDeploymentInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDeploymentInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDeploymentInstanceError {
                    fn from(err: serde_json::error::Error) -> GetDeploymentInstanceError {
                        GetDeploymentInstanceError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetDeploymentInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDeploymentInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDeploymentInstanceError::DeploymentDoesNotExist(ref cause) => cause,GetDeploymentInstanceError::InstanceDoesNotExist(ref cause) => cause,GetDeploymentInstanceError::InstanceIdRequired(ref cause) => cause,GetDeploymentInstanceError::DeploymentIdRequired(ref cause) => cause,GetDeploymentInstanceError::InvalidInstanceName(ref cause) => cause,GetDeploymentInstanceError::InvalidDeploymentId(ref cause) => cause,GetDeploymentInstanceError::Validation(ref cause) => cause,GetDeploymentInstanceError::Credentials(ref err) => err.description(),GetDeploymentInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDeploymentInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetOnPremisesInstance
                #[derive(Debug, PartialEq)]
                pub enum GetOnPremisesInstanceError {
                    
///<p>The specified on-premises instance is not registered.</p>
InstanceNotRegistered(String),
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetOnPremisesInstanceError {
                    pub fn from_body(body: &str) -> GetOnPremisesInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InstanceNotRegisteredException" => GetOnPremisesInstanceError::InstanceNotRegistered(String::from(error_message)),"InvalidInstanceNameException" => GetOnPremisesInstanceError::InvalidInstanceName(String::from(error_message)),"InstanceNameRequiredException" => GetOnPremisesInstanceError::InstanceNameRequired(String::from(error_message)),"ValidationException" => GetOnPremisesInstanceError::Validation(error_message.to_string()),_ => GetOnPremisesInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetOnPremisesInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetOnPremisesInstanceError {
                    fn from(err: serde_json::error::Error) -> GetOnPremisesInstanceError {
                        GetOnPremisesInstanceError::Unknown(err.description().to_string())
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
                impl fmt::Display for GetOnPremisesInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetOnPremisesInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            GetOnPremisesInstanceError::InstanceNotRegistered(ref cause) => cause,GetOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,GetOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,GetOnPremisesInstanceError::Validation(ref cause) => cause,GetOnPremisesInstanceError::Credentials(ref err) => err.description(),GetOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetOnPremisesInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListApplicationRevisions
                #[derive(Debug, PartialEq)]
                pub enum ListApplicationRevisionsError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The column name to sort by is either not present or was specified in an invalid format.</p>
InvalidSortBy(String),
///<p>The specified key prefix filter was specified in an invalid format.</p>
InvalidKeyPrefixFilter(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The bucket name either doesn't exist or was specified in an invalid format.</p>
InvalidBucketNameFilter(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The sort order was specified in an invalid format.</p>
InvalidSortOrder(String),
///<p>The deployed state filter was specified in an invalid format.</p>
InvalidDeployedStateFilter(String),
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),
///<p>A bucket name is required, but was not provided.</p>
BucketNameFilterRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListApplicationRevisionsError {
                    pub fn from_body(body: &str) -> ListApplicationRevisionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeployedStateFilterException" => ListApplicationRevisionsError::InvalidDeployedStateFilter(String::from(error_message)),"InvalidNextTokenException" => ListApplicationRevisionsError::InvalidNextToken(String::from(error_message)),"InvalidSortOrderException" => ListApplicationRevisionsError::InvalidSortOrder(String::from(error_message)),"InvalidBucketNameFilterException" => ListApplicationRevisionsError::InvalidBucketNameFilter(String::from(error_message)),"BucketNameFilterRequiredException" => ListApplicationRevisionsError::BucketNameFilterRequired(String::from(error_message)),"ApplicationDoesNotExistException" => ListApplicationRevisionsError::ApplicationDoesNotExist(String::from(error_message)),"InvalidKeyPrefixFilterException" => ListApplicationRevisionsError::InvalidKeyPrefixFilter(String::from(error_message)),"InvalidSortByException" => ListApplicationRevisionsError::InvalidSortBy(String::from(error_message)),"ApplicationNameRequiredException" => ListApplicationRevisionsError::ApplicationNameRequired(String::from(error_message)),"InvalidApplicationNameException" => ListApplicationRevisionsError::InvalidApplicationName(String::from(error_message)),"ValidationException" => ListApplicationRevisionsError::Validation(error_message.to_string()),_ => ListApplicationRevisionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListApplicationRevisionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListApplicationRevisionsError {
                    fn from(err: serde_json::error::Error) -> ListApplicationRevisionsError {
                        ListApplicationRevisionsError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListApplicationRevisionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListApplicationRevisionsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListApplicationRevisionsError::InvalidDeployedStateFilter(ref cause) => cause,ListApplicationRevisionsError::InvalidNextToken(ref cause) => cause,ListApplicationRevisionsError::InvalidBucketNameFilter(ref cause) => cause,ListApplicationRevisionsError::InvalidSortOrder(ref cause) => cause,ListApplicationRevisionsError::InvalidApplicationName(ref cause) => cause,ListApplicationRevisionsError::InvalidSortBy(ref cause) => cause,ListApplicationRevisionsError::BucketNameFilterRequired(ref cause) => cause,ListApplicationRevisionsError::InvalidKeyPrefixFilter(ref cause) => cause,ListApplicationRevisionsError::ApplicationNameRequired(ref cause) => cause,ListApplicationRevisionsError::ApplicationDoesNotExist(ref cause) => cause,ListApplicationRevisionsError::Validation(ref cause) => cause,ListApplicationRevisionsError::Credentials(ref err) => err.description(),ListApplicationRevisionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListApplicationRevisionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListApplications
                #[derive(Debug, PartialEq)]
                pub enum ListApplicationsError {
                    
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListApplicationsError {
                    pub fn from_body(body: &str) -> ListApplicationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidNextTokenException" => ListApplicationsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListApplicationsError::Validation(error_message.to_string()),_ => ListApplicationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListApplicationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListApplicationsError {
                    fn from(err: serde_json::error::Error) -> ListApplicationsError {
                        ListApplicationsError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListApplicationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListApplicationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListApplicationsError::InvalidNextToken(ref cause) => cause,ListApplicationsError::Validation(ref cause) => cause,ListApplicationsError::Credentials(ref err) => err.description(),ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListApplicationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDeploymentConfigs
                #[derive(Debug, PartialEq)]
                pub enum ListDeploymentConfigsError {
                    
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDeploymentConfigsError {
                    pub fn from_body(body: &str) -> ListDeploymentConfigsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidNextTokenException" => ListDeploymentConfigsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListDeploymentConfigsError::Validation(error_message.to_string()),_ => ListDeploymentConfigsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDeploymentConfigsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDeploymentConfigsError {
                    fn from(err: serde_json::error::Error) -> ListDeploymentConfigsError {
                        ListDeploymentConfigsError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListDeploymentConfigsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDeploymentConfigsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDeploymentConfigsError::InvalidNextToken(ref cause) => cause,ListDeploymentConfigsError::Validation(ref cause) => cause,ListDeploymentConfigsError::Credentials(ref err) => err.description(),ListDeploymentConfigsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDeploymentConfigsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDeploymentGroups
                #[derive(Debug, PartialEq)]
                pub enum ListDeploymentGroupsError {
                    
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDeploymentGroupsError {
                    pub fn from_body(body: &str) -> ListDeploymentGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationDoesNotExistException" => ListDeploymentGroupsError::ApplicationDoesNotExist(String::from(error_message)),"InvalidApplicationNameException" => ListDeploymentGroupsError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => ListDeploymentGroupsError::ApplicationNameRequired(String::from(error_message)),"InvalidNextTokenException" => ListDeploymentGroupsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListDeploymentGroupsError::Validation(error_message.to_string()),_ => ListDeploymentGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDeploymentGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDeploymentGroupsError {
                    fn from(err: serde_json::error::Error) -> ListDeploymentGroupsError {
                        ListDeploymentGroupsError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListDeploymentGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDeploymentGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDeploymentGroupsError::ApplicationNameRequired(ref cause) => cause,ListDeploymentGroupsError::InvalidApplicationName(ref cause) => cause,ListDeploymentGroupsError::InvalidNextToken(ref cause) => cause,ListDeploymentGroupsError::ApplicationDoesNotExist(ref cause) => cause,ListDeploymentGroupsError::Validation(ref cause) => cause,ListDeploymentGroupsError::Credentials(ref err) => err.description(),ListDeploymentGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDeploymentGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDeploymentInstances
                #[derive(Debug, PartialEq)]
                pub enum ListDeploymentInstancesError {
                    
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),
///<p>The specified instance status does not exist.</p>
InvalidInstanceStatus(String),
///<p>The specified deployment has not started.</p>
DeploymentNotStarted(String),
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),
///<p>The deployment does not exist with the applicable IAM user or AWS account.</p>
DeploymentDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDeploymentInstancesError {
                    pub fn from_body(body: &str) -> ListDeploymentInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDeploymentIdException" => ListDeploymentInstancesError::InvalidDeploymentId(String::from(error_message)),"DeploymentDoesNotExistException" => ListDeploymentInstancesError::DeploymentDoesNotExist(String::from(error_message)),"InvalidInstanceStatusException" => ListDeploymentInstancesError::InvalidInstanceStatus(String::from(error_message)),"InvalidNextTokenException" => ListDeploymentInstancesError::InvalidNextToken(String::from(error_message)),"DeploymentIdRequiredException" => ListDeploymentInstancesError::DeploymentIdRequired(String::from(error_message)),"DeploymentNotStartedException" => ListDeploymentInstancesError::DeploymentNotStarted(String::from(error_message)),"ValidationException" => ListDeploymentInstancesError::Validation(error_message.to_string()),_ => ListDeploymentInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDeploymentInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDeploymentInstancesError {
                    fn from(err: serde_json::error::Error) -> ListDeploymentInstancesError {
                        ListDeploymentInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListDeploymentInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDeploymentInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDeploymentInstancesError::InvalidInstanceStatus(ref cause) => cause,ListDeploymentInstancesError::DeploymentDoesNotExist(ref cause) => cause,ListDeploymentInstancesError::InvalidNextToken(ref cause) => cause,ListDeploymentInstancesError::InvalidDeploymentId(ref cause) => cause,ListDeploymentInstancesError::DeploymentIdRequired(ref cause) => cause,ListDeploymentInstancesError::DeploymentNotStarted(ref cause) => cause,ListDeploymentInstancesError::Validation(ref cause) => cause,ListDeploymentInstancesError::Credentials(ref err) => err.description(),ListDeploymentInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDeploymentInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDeployments
                #[derive(Debug, PartialEq)]
                pub enum ListDeploymentsError {
                    
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),
///<p>The specified deployment status doesn't exist or cannot be determined.</p>
InvalidDeploymentStatus(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
DeploymentGroupDoesNotExist(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The specified time range was specified in an invalid format.</p>
InvalidTimeRange(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDeploymentsError {
                    pub fn from_body(body: &str) -> ListDeploymentsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentGroupDoesNotExistException" => ListDeploymentsError::DeploymentGroupDoesNotExist(String::from(error_message)),"InvalidNextTokenException" => ListDeploymentsError::InvalidNextToken(String::from(error_message)),"InvalidDeploymentGroupNameException" => ListDeploymentsError::InvalidDeploymentGroupName(String::from(error_message)),"DeploymentGroupNameRequiredException" => ListDeploymentsError::DeploymentGroupNameRequired(String::from(error_message)),"ApplicationDoesNotExistException" => ListDeploymentsError::ApplicationDoesNotExist(String::from(error_message)),"ApplicationNameRequiredException" => ListDeploymentsError::ApplicationNameRequired(String::from(error_message)),"InvalidDeploymentStatusException" => ListDeploymentsError::InvalidDeploymentStatus(String::from(error_message)),"InvalidApplicationNameException" => ListDeploymentsError::InvalidApplicationName(String::from(error_message)),"InvalidTimeRangeException" => ListDeploymentsError::InvalidTimeRange(String::from(error_message)),"ValidationException" => ListDeploymentsError::Validation(error_message.to_string()),_ => ListDeploymentsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDeploymentsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDeploymentsError {
                    fn from(err: serde_json::error::Error) -> ListDeploymentsError {
                        ListDeploymentsError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListDeploymentsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDeploymentsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDeploymentsError::ApplicationNameRequired(ref cause) => cause,ListDeploymentsError::ApplicationDoesNotExist(ref cause) => cause,ListDeploymentsError::InvalidTimeRange(ref cause) => cause,ListDeploymentsError::InvalidDeploymentGroupName(ref cause) => cause,ListDeploymentsError::InvalidDeploymentStatus(ref cause) => cause,ListDeploymentsError::InvalidNextToken(ref cause) => cause,ListDeploymentsError::DeploymentGroupDoesNotExist(ref cause) => cause,ListDeploymentsError::DeploymentGroupNameRequired(ref cause) => cause,ListDeploymentsError::InvalidApplicationName(ref cause) => cause,ListDeploymentsError::Validation(ref cause) => cause,ListDeploymentsError::Credentials(ref err) => err.description(),ListDeploymentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDeploymentsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListOnPremisesInstances
                #[derive(Debug, PartialEq)]
                pub enum ListOnPremisesInstancesError {
                    
///<p>The next token was specified in an invalid format.</p>
InvalidNextToken(String),
///<p>The specified tag filter was specified in an invalid format.</p>
InvalidTagFilter(String),
///<p>The registration status was specified in an invalid format.</p>
InvalidRegistrationStatus(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListOnPremisesInstancesError {
                    pub fn from_body(body: &str) -> ListOnPremisesInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidTagFilterException" => ListOnPremisesInstancesError::InvalidTagFilter(String::from(error_message)),"InvalidRegistrationStatusException" => ListOnPremisesInstancesError::InvalidRegistrationStatus(String::from(error_message)),"InvalidNextTokenException" => ListOnPremisesInstancesError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListOnPremisesInstancesError::Validation(error_message.to_string()),_ => ListOnPremisesInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListOnPremisesInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListOnPremisesInstancesError {
                    fn from(err: serde_json::error::Error) -> ListOnPremisesInstancesError {
                        ListOnPremisesInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for ListOnPremisesInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListOnPremisesInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListOnPremisesInstancesError::InvalidTagFilter(ref cause) => cause,ListOnPremisesInstancesError::InvalidNextToken(ref cause) => cause,ListOnPremisesInstancesError::InvalidRegistrationStatus(ref cause) => cause,ListOnPremisesInstancesError::Validation(ref cause) => cause,ListOnPremisesInstancesError::Credentials(ref err) => err.description(),ListOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListOnPremisesInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterApplicationRevision
                #[derive(Debug, PartialEq)]
                pub enum RegisterApplicationRevisionError {
                    
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The revision ID was not specified.</p>
RevisionRequired(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The revision was specified in an invalid format.</p>
InvalidRevision(String),
///<p>The description is too long.</p>
DescriptionTooLong(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterApplicationRevisionError {
                    pub fn from_body(body: &str) -> RegisterApplicationRevisionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidApplicationNameException" => RegisterApplicationRevisionError::InvalidApplicationName(String::from(error_message)),"RevisionRequiredException" => RegisterApplicationRevisionError::RevisionRequired(String::from(error_message)),"InvalidRevisionException" => RegisterApplicationRevisionError::InvalidRevision(String::from(error_message)),"DescriptionTooLongException" => RegisterApplicationRevisionError::DescriptionTooLong(String::from(error_message)),"ApplicationDoesNotExistException" => RegisterApplicationRevisionError::ApplicationDoesNotExist(String::from(error_message)),"ApplicationNameRequiredException" => RegisterApplicationRevisionError::ApplicationNameRequired(String::from(error_message)),"ValidationException" => RegisterApplicationRevisionError::Validation(error_message.to_string()),_ => RegisterApplicationRevisionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterApplicationRevisionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterApplicationRevisionError {
                    fn from(err: serde_json::error::Error) -> RegisterApplicationRevisionError {
                        RegisterApplicationRevisionError::Unknown(err.description().to_string())
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
                impl fmt::Display for RegisterApplicationRevisionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterApplicationRevisionError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterApplicationRevisionError::InvalidRevision(ref cause) => cause,RegisterApplicationRevisionError::DescriptionTooLong(ref cause) => cause,RegisterApplicationRevisionError::RevisionRequired(ref cause) => cause,RegisterApplicationRevisionError::ApplicationDoesNotExist(ref cause) => cause,RegisterApplicationRevisionError::ApplicationNameRequired(ref cause) => cause,RegisterApplicationRevisionError::InvalidApplicationName(ref cause) => cause,RegisterApplicationRevisionError::Validation(ref cause) => cause,RegisterApplicationRevisionError::Credentials(ref err) => err.description(),RegisterApplicationRevisionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterApplicationRevisionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterOnPremisesInstance
                #[derive(Debug, PartialEq)]
                pub enum RegisterOnPremisesInstanceError {
                    
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),
///<p>An IAM user ARN was not specified.</p>
IamUserArnRequired(String),
///<p>The IAM session ARN was specified in an invalid format.</p>
InvalidIamSessionArn(String),
///<p>The specified on-premises instance name was specified in an invalid format.</p>
InvalidInstanceName(String),
///<p>The specified IAM user ARN is already registered with an on-premises instance.</p>
IamUserArnAlreadyRegistered(String),
///<p>No IAM ARN was included in the request. You must use an IAM session ARN or IAM user ARN in the request.</p>
IamArnRequired(String),
///<p>The request included an IAM session ARN that has already been used to register a different instance.</p>
IamSessionArnAlreadyRegistered(String),
///<p>The IAM user ARN was specified in an invalid format.</p>
InvalidIamUserArn(String),
///<p>Both an IAM user ARN and an IAM session ARN were included in the request. Use only one ARN type.</p>
MultipleIamArnsProvided(String),
///<p>The specified on-premises instance name is already registered.</p>
InstanceNameAlreadyRegistered(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterOnPremisesInstanceError {
                    pub fn from_body(body: &str) -> RegisterOnPremisesInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "IamSessionArnAlreadyRegisteredException" => RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(String::from(error_message)),"IamUserArnRequiredException" => RegisterOnPremisesInstanceError::IamUserArnRequired(String::from(error_message)),"InvalidIamSessionArnException" => RegisterOnPremisesInstanceError::InvalidIamSessionArn(String::from(error_message)),"InvalidIamUserArnException" => RegisterOnPremisesInstanceError::InvalidIamUserArn(String::from(error_message)),"MultipleIamArnsProvidedException" => RegisterOnPremisesInstanceError::MultipleIamArnsProvided(String::from(error_message)),"InstanceNameRequiredException" => RegisterOnPremisesInstanceError::InstanceNameRequired(String::from(error_message)),"IamArnRequiredException" => RegisterOnPremisesInstanceError::IamArnRequired(String::from(error_message)),"InstanceNameAlreadyRegisteredException" => RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(String::from(error_message)),"InvalidInstanceNameException" => RegisterOnPremisesInstanceError::InvalidInstanceName(String::from(error_message)),"IamUserArnAlreadyRegisteredException" => RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(String::from(error_message)),"ValidationException" => RegisterOnPremisesInstanceError::Validation(error_message.to_string()),_ => RegisterOnPremisesInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterOnPremisesInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterOnPremisesInstanceError {
                    fn from(err: serde_json::error::Error) -> RegisterOnPremisesInstanceError {
                        RegisterOnPremisesInstanceError::Unknown(err.description().to_string())
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
                impl fmt::Display for RegisterOnPremisesInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterOnPremisesInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterOnPremisesInstanceError::InvalidIamUserArn(ref cause) => cause,RegisterOnPremisesInstanceError::IamSessionArnAlreadyRegistered(ref cause) => cause,RegisterOnPremisesInstanceError::InstanceNameAlreadyRegistered(ref cause) => cause,RegisterOnPremisesInstanceError::InvalidInstanceName(ref cause) => cause,RegisterOnPremisesInstanceError::IamUserArnAlreadyRegistered(ref cause) => cause,RegisterOnPremisesInstanceError::IamUserArnRequired(ref cause) => cause,RegisterOnPremisesInstanceError::InstanceNameRequired(ref cause) => cause,RegisterOnPremisesInstanceError::InvalidIamSessionArn(ref cause) => cause,RegisterOnPremisesInstanceError::IamArnRequired(ref cause) => cause,RegisterOnPremisesInstanceError::MultipleIamArnsProvided(ref cause) => cause,RegisterOnPremisesInstanceError::Validation(ref cause) => cause,RegisterOnPremisesInstanceError::Credentials(ref err) => err.description(),RegisterOnPremisesInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterOnPremisesInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTagsFromOnPremisesInstances
                #[derive(Debug, PartialEq)]
                pub enum RemoveTagsFromOnPremisesInstancesError {
                    
///<p>An on-premises instance name was not specified.</p>
InstanceNameRequired(String),
///<p>The specified tag was specified in an invalid format.</p>
InvalidTag(String),
///<p>The maximum number of allowed on-premises instances in a single call was exceeded.</p>
InstanceLimitExceeded(String),
///<p>A tag was not specified.</p>
TagRequired(String),
///<p>The maximum allowed number of tags was exceeded.</p>
TagLimitExceeded(String),
///<p>The specified on-premises instance is not registered.</p>
InstanceNotRegistered(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTagsFromOnPremisesInstancesError {
                    pub fn from_body(body: &str) -> RemoveTagsFromOnPremisesInstancesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidTagException" => RemoveTagsFromOnPremisesInstancesError::InvalidTag(String::from(error_message)),"InstanceNameRequiredException" => RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(String::from(error_message)),"InstanceNotRegisteredException" => RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(String::from(error_message)),"TagRequiredException" => RemoveTagsFromOnPremisesInstancesError::TagRequired(String::from(error_message)),"InstanceLimitExceededException" => RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(String::from(error_message)),"TagLimitExceededException" => RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(String::from(error_message)),"ValidationException" => RemoveTagsFromOnPremisesInstancesError::Validation(error_message.to_string()),_ => RemoveTagsFromOnPremisesInstancesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTagsFromOnPremisesInstancesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTagsFromOnPremisesInstancesError {
                    fn from(err: serde_json::error::Error) -> RemoveTagsFromOnPremisesInstancesError {
                        RemoveTagsFromOnPremisesInstancesError::Unknown(err.description().to_string())
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
                impl fmt::Display for RemoveTagsFromOnPremisesInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTagsFromOnPremisesInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTagsFromOnPremisesInstancesError::InstanceNameRequired(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::InstanceLimitExceeded(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::InstanceNotRegistered(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::InvalidTag(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::TagRequired(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::TagLimitExceeded(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::Validation(ref cause) => cause,RemoveTagsFromOnPremisesInstancesError::Credentials(ref err) => err.description(),RemoveTagsFromOnPremisesInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RemoveTagsFromOnPremisesInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by StopDeployment
                #[derive(Debug, PartialEq)]
                pub enum StopDeploymentError {
                    
///<p>At least one of the deployment IDs was specified in an invalid format.</p>
InvalidDeploymentId(String),
///<p>At least one deployment ID must be specified.</p>
DeploymentIdRequired(String),
///<p>The deployment is already complete.</p>
DeploymentAlreadyCompleted(String),
///<p>The deployment does not exist with the applicable IAM user or AWS account.</p>
DeploymentDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl StopDeploymentError {
                    pub fn from_body(body: &str) -> StopDeploymentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DeploymentDoesNotExistException" => StopDeploymentError::DeploymentDoesNotExist(String::from(error_message)),"DeploymentIdRequiredException" => StopDeploymentError::DeploymentIdRequired(String::from(error_message)),"InvalidDeploymentIdException" => StopDeploymentError::InvalidDeploymentId(String::from(error_message)),"DeploymentAlreadyCompletedException" => StopDeploymentError::DeploymentAlreadyCompleted(String::from(error_message)),"ValidationException" => StopDeploymentError::Validation(error_message.to_string()),_ => StopDeploymentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => StopDeploymentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for StopDeploymentError {
                    fn from(err: serde_json::error::Error) -> StopDeploymentError {
                        StopDeploymentError::Unknown(err.description().to_string())
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
                impl fmt::Display for StopDeploymentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for StopDeploymentError {
                    fn description(&self) -> &str {
                        match *self {
                            StopDeploymentError::DeploymentDoesNotExist(ref cause) => cause,StopDeploymentError::InvalidDeploymentId(ref cause) => cause,StopDeploymentError::DeploymentIdRequired(ref cause) => cause,StopDeploymentError::DeploymentAlreadyCompleted(ref cause) => cause,StopDeploymentError::Validation(ref cause) => cause,StopDeploymentError::Credentials(ref err) => err.description(),StopDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),StopDeploymentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateApplication
                #[derive(Debug, PartialEq)]
                pub enum UpdateApplicationError {
                    
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>An application with the specified name already exists with the applicable IAM user or AWS account.</p>
ApplicationAlreadyExists(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateApplicationError {
                    pub fn from_body(body: &str) -> UpdateApplicationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ApplicationDoesNotExistException" => UpdateApplicationError::ApplicationDoesNotExist(String::from(error_message)),"InvalidApplicationNameException" => UpdateApplicationError::InvalidApplicationName(String::from(error_message)),"ApplicationNameRequiredException" => UpdateApplicationError::ApplicationNameRequired(String::from(error_message)),"ApplicationAlreadyExistsException" => UpdateApplicationError::ApplicationAlreadyExists(String::from(error_message)),"ValidationException" => UpdateApplicationError::Validation(error_message.to_string()),_ => UpdateApplicationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateApplicationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateApplicationError {
                    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
                        UpdateApplicationError::Unknown(err.description().to_string())
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
                impl fmt::Display for UpdateApplicationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateApplicationError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateApplicationError::ApplicationDoesNotExist(ref cause) => cause,UpdateApplicationError::ApplicationAlreadyExists(ref cause) => cause,UpdateApplicationError::InvalidApplicationName(ref cause) => cause,UpdateApplicationError::ApplicationNameRequired(ref cause) => cause,UpdateApplicationError::Validation(ref cause) => cause,UpdateApplicationError::Credentials(ref err) => err.description(),UpdateApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateApplicationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateDeploymentGroup
                #[derive(Debug, PartialEq)]
                pub enum UpdateDeploymentGroupError {
                    
///<p>The deployment configuration name was specified in an invalid format.</p>
InvalidDeploymentConfigName(String),
///<p>The application name was specified in an invalid format.</p>
InvalidApplicationName(String),
///<p>The deployment group name was specified in an invalid format.</p>
InvalidDeploymentGroupName(String),
///<p>The application does not exist with the applicable IAM user or AWS account.</p>
ApplicationDoesNotExist(String),
///<p>The deployment group name was not specified.</p>
DeploymentGroupNameRequired(String),
///<p>The trigger was specified in an invalid format.</p>
InvalidTriggerConfig(String),
///<p>The minimum number of required application names was not specified.</p>
ApplicationNameRequired(String),
///<p>The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled but an invalid triggering event type or no event types were listed.</p>
InvalidAutoRollbackConfig(String),
///<p>The format of the alarm configuration is invalid. Possible causes include:</p> <ul> <li> <p>The alarm list is null.</p> </li> <li> <p>The alarm object is null.</p> </li> <li> <p>The alarm name is empty or null or exceeds the 255 character limit.</p> </li> <li> <p>Two alarms with the same name have been specified.</p> </li> <li> <p>The alarm configuration is enabled but the alarm list is empty.</p> </li> </ul>
InvalidAlarmConfig(String),
///<p>The Auto Scaling group was specified in an invalid format or does not exist.</p>
InvalidAutoScalingGroup(String),
///<p>The limit for lifecycle hooks was exceeded.</p>
LifecycleHookLimitExceeded(String),
///<p>The tag was specified in an invalid format.</p>
InvalidEC2Tag(String),
///<p>The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropriate permissions to Auto Scaling.</p>
InvalidRole(String),
///<p>A deployment group with the specified name already exists with the applicable IAM user or AWS account.</p>
DeploymentGroupAlreadyExists(String),
///<p>The specified tag was specified in an invalid format.</p>
InvalidTag(String),
///<p>The deployment configuration does not exist with the applicable IAM user or AWS account.</p>
DeploymentConfigDoesNotExist(String),
///<p>The named deployment group does not exist with the applicable IAM user or AWS account.</p>
DeploymentGroupDoesNotExist(String),
///<p>The maximum allowed number of triggers was exceeded.</p>
TriggerTargetsLimitExceeded(String),
///<p>The maximum number of alarms for a deployment group (10) was exceeded.</p>
AlarmsLimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateDeploymentGroupError {
                    pub fn from_body(body: &str) -> UpdateDeploymentGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidAlarmConfigException" => UpdateDeploymentGroupError::InvalidAlarmConfig(String::from(error_message)),"InvalidRoleException" => UpdateDeploymentGroupError::InvalidRole(String::from(error_message)),"ApplicationDoesNotExistException" => UpdateDeploymentGroupError::ApplicationDoesNotExist(String::from(error_message)),"DeploymentGroupNameRequiredException" => UpdateDeploymentGroupError::DeploymentGroupNameRequired(String::from(error_message)),"InvalidTriggerConfigException" => UpdateDeploymentGroupError::InvalidTriggerConfig(String::from(error_message)),"InvalidDeploymentConfigNameException" => UpdateDeploymentGroupError::InvalidDeploymentConfigName(String::from(error_message)),"ApplicationNameRequiredException" => UpdateDeploymentGroupError::ApplicationNameRequired(String::from(error_message)),"InvalidEC2TagException" => UpdateDeploymentGroupError::InvalidEC2Tag(String::from(error_message)),"InvalidAutoRollbackConfigException" => UpdateDeploymentGroupError::InvalidAutoRollbackConfig(String::from(error_message)),"InvalidApplicationNameException" => UpdateDeploymentGroupError::InvalidApplicationName(String::from(error_message)),"InvalidAutoScalingGroupException" => UpdateDeploymentGroupError::InvalidAutoScalingGroup(String::from(error_message)),"LifecycleHookLimitExceededException" => UpdateDeploymentGroupError::LifecycleHookLimitExceeded(String::from(error_message)),"AlarmsLimitExceededException" => UpdateDeploymentGroupError::AlarmsLimitExceeded(String::from(error_message)),"InvalidDeploymentGroupNameException" => UpdateDeploymentGroupError::InvalidDeploymentGroupName(String::from(error_message)),"DeploymentGroupDoesNotExistException" => UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(String::from(error_message)),"DeploymentGroupAlreadyExistsException" => UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(String::from(error_message)),"DeploymentConfigDoesNotExistException" => UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(String::from(error_message)),"InvalidTagException" => UpdateDeploymentGroupError::InvalidTag(String::from(error_message)),"TriggerTargetsLimitExceededException" => UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(String::from(error_message)),"ValidationException" => UpdateDeploymentGroupError::Validation(error_message.to_string()),_ => UpdateDeploymentGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateDeploymentGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateDeploymentGroupError {
                    fn from(err: serde_json::error::Error) -> UpdateDeploymentGroupError {
                        UpdateDeploymentGroupError::Unknown(err.description().to_string())
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
                impl fmt::Display for UpdateDeploymentGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateDeploymentGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateDeploymentGroupError::DeploymentGroupNameRequired(ref cause) => cause,UpdateDeploymentGroupError::InvalidEC2Tag(ref cause) => cause,UpdateDeploymentGroupError::InvalidTag(ref cause) => cause,UpdateDeploymentGroupError::DeploymentGroupAlreadyExists(ref cause) => cause,UpdateDeploymentGroupError::InvalidAutoRollbackConfig(ref cause) => cause,UpdateDeploymentGroupError::TriggerTargetsLimitExceeded(ref cause) => cause,UpdateDeploymentGroupError::DeploymentGroupDoesNotExist(ref cause) => cause,UpdateDeploymentGroupError::LifecycleHookLimitExceeded(ref cause) => cause,UpdateDeploymentGroupError::ApplicationDoesNotExist(ref cause) => cause,UpdateDeploymentGroupError::InvalidAlarmConfig(ref cause) => cause,UpdateDeploymentGroupError::ApplicationNameRequired(ref cause) => cause,UpdateDeploymentGroupError::InvalidDeploymentConfigName(ref cause) => cause,UpdateDeploymentGroupError::InvalidApplicationName(ref cause) => cause,UpdateDeploymentGroupError::DeploymentConfigDoesNotExist(ref cause) => cause,UpdateDeploymentGroupError::InvalidAutoScalingGroup(ref cause) => cause,UpdateDeploymentGroupError::AlarmsLimitExceeded(ref cause) => cause,UpdateDeploymentGroupError::InvalidTriggerConfig(ref cause) => cause,UpdateDeploymentGroupError::InvalidDeploymentGroupName(ref cause) => cause,UpdateDeploymentGroupError::InvalidRole(ref cause) => cause,UpdateDeploymentGroupError::Validation(ref cause) => cause,UpdateDeploymentGroupError::Credentials(ref err) => err.description(),UpdateDeploymentGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateDeploymentGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the CodeDeploy API. CodeDeploy clients implement this trait.
        pub trait CodeDeploy {
        

                #[doc="<p>Adds tags to on-premises instances.</p>"]
                fn add_tags_to_on_premises_instances(&self, input: &AddTagsToOnPremisesInstancesInput)  -> Result<(), AddTagsToOnPremisesInstancesError>;
                

                #[doc="<p>Gets information about one or more application revisions.</p>"]
                fn batch_get_application_revisions(&self, input: &BatchGetApplicationRevisionsInput)  -> Result<BatchGetApplicationRevisionsOutput, BatchGetApplicationRevisionsError>;
                

                #[doc="<p>Gets information about one or more applications.</p>"]
                fn batch_get_applications(&self, input: &BatchGetApplicationsInput)  -> Result<BatchGetApplicationsOutput, BatchGetApplicationsError>;
                

                #[doc="<p>Gets information about one or more deployment groups.</p>"]
                fn batch_get_deployment_groups(&self, input: &BatchGetDeploymentGroupsInput)  -> Result<BatchGetDeploymentGroupsOutput, BatchGetDeploymentGroupsError>;
                

                #[doc="<p>Gets information about one or more instance that are part of a deployment group.</p>"]
                fn batch_get_deployment_instances(&self, input: &BatchGetDeploymentInstancesInput)  -> Result<BatchGetDeploymentInstancesOutput, BatchGetDeploymentInstancesError>;
                

                #[doc="<p>Gets information about one or more deployments.</p>"]
                fn batch_get_deployments(&self, input: &BatchGetDeploymentsInput)  -> Result<BatchGetDeploymentsOutput, BatchGetDeploymentsError>;
                

                #[doc="<p>Gets information about one or more on-premises instances.</p>"]
                fn batch_get_on_premises_instances(&self, input: &BatchGetOnPremisesInstancesInput)  -> Result<BatchGetOnPremisesInstancesOutput, BatchGetOnPremisesInstancesError>;
                

                #[doc="<p>Creates an application.</p>"]
                fn create_application(&self, input: &CreateApplicationInput)  -> Result<CreateApplicationOutput, CreateApplicationError>;
                

                #[doc="<p>Deploys an application revision through the specified deployment group.</p>"]
                fn create_deployment(&self, input: &CreateDeploymentInput)  -> Result<CreateDeploymentOutput, CreateDeploymentError>;
                

                #[doc="<p>Creates a deployment configuration.</p>"]
                fn create_deployment_config(&self, input: &CreateDeploymentConfigInput)  -> Result<CreateDeploymentConfigOutput, CreateDeploymentConfigError>;
                

                #[doc="<p>Creates a deployment group to which application revisions will be deployed.</p>"]
                fn create_deployment_group(&self, input: &CreateDeploymentGroupInput)  -> Result<CreateDeploymentGroupOutput, CreateDeploymentGroupError>;
                

                #[doc="<p>Deletes an application.</p>"]
                fn delete_application(&self, input: &DeleteApplicationInput)  -> Result<(), DeleteApplicationError>;
                

                #[doc="<p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note>"]
                fn delete_deployment_config(&self, input: &DeleteDeploymentConfigInput)  -> Result<(), DeleteDeploymentConfigError>;
                

                #[doc="<p>Deletes a deployment group.</p>"]
                fn delete_deployment_group(&self, input: &DeleteDeploymentGroupInput)  -> Result<DeleteDeploymentGroupOutput, DeleteDeploymentGroupError>;
                

                #[doc="<p>Deregisters an on-premises instance.</p>"]
                fn deregister_on_premises_instance(&self, input: &DeregisterOnPremisesInstanceInput)  -> Result<(), DeregisterOnPremisesInstanceError>;
                

                #[doc="<p>Gets information about an application.</p>"]
                fn get_application(&self, input: &GetApplicationInput)  -> Result<GetApplicationOutput, GetApplicationError>;
                

                #[doc="<p>Gets information about an application revision.</p>"]
                fn get_application_revision(&self, input: &GetApplicationRevisionInput)  -> Result<GetApplicationRevisionOutput, GetApplicationRevisionError>;
                

                #[doc="<p>Gets information about a deployment.</p>"]
                fn get_deployment(&self, input: &GetDeploymentInput)  -> Result<GetDeploymentOutput, GetDeploymentError>;
                

                #[doc="<p>Gets information about a deployment configuration.</p>"]
                fn get_deployment_config(&self, input: &GetDeploymentConfigInput)  -> Result<GetDeploymentConfigOutput, GetDeploymentConfigError>;
                

                #[doc="<p>Gets information about a deployment group.</p>"]
                fn get_deployment_group(&self, input: &GetDeploymentGroupInput)  -> Result<GetDeploymentGroupOutput, GetDeploymentGroupError>;
                

                #[doc="<p>Gets information about an instance as part of a deployment.</p>"]
                fn get_deployment_instance(&self, input: &GetDeploymentInstanceInput)  -> Result<GetDeploymentInstanceOutput, GetDeploymentInstanceError>;
                

                #[doc="<p>Gets information about an on-premises instance.</p>"]
                fn get_on_premises_instance(&self, input: &GetOnPremisesInstanceInput)  -> Result<GetOnPremisesInstanceOutput, GetOnPremisesInstanceError>;
                

                #[doc="<p>Lists information about revisions for an application.</p>"]
                fn list_application_revisions(&self, input: &ListApplicationRevisionsInput)  -> Result<ListApplicationRevisionsOutput, ListApplicationRevisionsError>;
                

                #[doc="<p>Lists the applications registered with the applicable IAM user or AWS account.</p>"]
                fn list_applications(&self, input: &ListApplicationsInput)  -> Result<ListApplicationsOutput, ListApplicationsError>;
                

                #[doc="<p>Lists the deployment configurations with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_configs(&self, input: &ListDeploymentConfigsInput)  -> Result<ListDeploymentConfigsOutput, ListDeploymentConfigsError>;
                

                #[doc="<p>Lists the deployment groups for an application registered with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_groups(&self, input: &ListDeploymentGroupsInput)  -> Result<ListDeploymentGroupsOutput, ListDeploymentGroupsError>;
                

                #[doc="<p>Lists the instance for a deployment associated with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_instances(&self, input: &ListDeploymentInstancesInput)  -> Result<ListDeploymentInstancesOutput, ListDeploymentInstancesError>;
                

                #[doc="<p>Lists the deployments in a deployment group for an application registered with the applicable IAM user or AWS account.</p>"]
                fn list_deployments(&self, input: &ListDeploymentsInput)  -> Result<ListDeploymentsOutput, ListDeploymentsError>;
                

                #[doc="<p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names will be listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>"]
                fn list_on_premises_instances(&self, input: &ListOnPremisesInstancesInput)  -> Result<ListOnPremisesInstancesOutput, ListOnPremisesInstancesError>;
                

                #[doc="<p>Registers with AWS CodeDeploy a revision for the specified application.</p>"]
                fn register_application_revision(&self, input: &RegisterApplicationRevisionInput)  -> Result<(), RegisterApplicationRevisionError>;
                

                #[doc="<p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note>"]
                fn register_on_premises_instance(&self, input: &RegisterOnPremisesInstanceInput)  -> Result<(), RegisterOnPremisesInstanceError>;
                

                #[doc="<p>Removes one or more tags from one or more on-premises instances.</p>"]
                fn remove_tags_from_on_premises_instances(&self, input: &RemoveTagsFromOnPremisesInstancesInput)  -> Result<(), RemoveTagsFromOnPremisesInstancesError>;
                

                #[doc="<p>Attempts to stop an ongoing deployment.</p>"]
                fn stop_deployment(&self, input: &StopDeploymentInput)  -> Result<StopDeploymentOutput, StopDeploymentError>;
                

                #[doc="<p>Changes the name of an application.</p>"]
                fn update_application(&self, input: &UpdateApplicationInput)  -> Result<(), UpdateApplicationError>;
                

                #[doc="<p>Changes information about a deployment group.</p>"]
                fn update_deployment_group(&self, input: &UpdateDeploymentGroupInput)  -> Result<UpdateDeploymentGroupOutput, UpdateDeploymentGroupError>;
                
}
/// A client for the CodeDeploy API.
        pub struct CodeDeployClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> CodeDeployClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  CodeDeployClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> CodeDeploy for CodeDeployClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p>Adds tags to on-premises instances.</p>"]
                fn add_tags_to_on_premises_instances(&self, input: &AddTagsToOnPremisesInstancesInput)  -> Result<(), AddTagsToOnPremisesInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.AddTagsToOnPremisesInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(AddTagsToOnPremisesInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more application revisions.</p>"]
                fn batch_get_application_revisions(&self, input: &BatchGetApplicationRevisionsInput)  -> Result<BatchGetApplicationRevisionsOutput, BatchGetApplicationRevisionsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetApplicationRevisions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetApplicationRevisionsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetApplicationRevisionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more applications.</p>"]
                fn batch_get_applications(&self, input: &BatchGetApplicationsInput)  -> Result<BatchGetApplicationsOutput, BatchGetApplicationsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetApplications");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetApplicationsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetApplicationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more deployment groups.</p>"]
                fn batch_get_deployment_groups(&self, input: &BatchGetDeploymentGroupsInput)  -> Result<BatchGetDeploymentGroupsOutput, BatchGetDeploymentGroupsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetDeploymentGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetDeploymentGroupsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetDeploymentGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more instance that are part of a deployment group.</p>"]
                fn batch_get_deployment_instances(&self, input: &BatchGetDeploymentInstancesInput)  -> Result<BatchGetDeploymentInstancesOutput, BatchGetDeploymentInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetDeploymentInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetDeploymentInstancesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetDeploymentInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more deployments.</p>"]
                fn batch_get_deployments(&self, input: &BatchGetDeploymentsInput)  -> Result<BatchGetDeploymentsOutput, BatchGetDeploymentsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetDeployments");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetDeploymentsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetDeploymentsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about one or more on-premises instances.</p>"]
                fn batch_get_on_premises_instances(&self, input: &BatchGetOnPremisesInstancesInput)  -> Result<BatchGetOnPremisesInstancesOutput, BatchGetOnPremisesInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.BatchGetOnPremisesInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetOnPremisesInstancesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetOnPremisesInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates an application.</p>"]
                fn create_application(&self, input: &CreateApplicationInput)  -> Result<CreateApplicationOutput, CreateApplicationError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.CreateApplication");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateApplicationOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateApplicationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deploys an application revision through the specified deployment group.</p>"]
                fn create_deployment(&self, input: &CreateDeploymentInput)  -> Result<CreateDeploymentOutput, CreateDeploymentError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeployment");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDeploymentOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateDeploymentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a deployment configuration.</p>"]
                fn create_deployment_config(&self, input: &CreateDeploymentConfigInput)  -> Result<CreateDeploymentConfigOutput, CreateDeploymentConfigError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentConfig");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDeploymentConfigOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateDeploymentConfigError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a deployment group to which application revisions will be deployed.</p>"]
                fn create_deployment_group(&self, input: &CreateDeploymentGroupInput)  -> Result<CreateDeploymentGroupOutput, CreateDeploymentGroupError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.CreateDeploymentGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDeploymentGroupOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateDeploymentGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes an application.</p>"]
                fn delete_application(&self, input: &DeleteApplicationInput)  -> Result<(), DeleteApplicationError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteApplication");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DeleteApplicationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a deployment configuration.</p> <note> <p>A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted.</p> </note>"]
                fn delete_deployment_config(&self, input: &DeleteDeploymentConfigInput)  -> Result<(), DeleteDeploymentConfigError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentConfig");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DeleteDeploymentConfigError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a deployment group.</p>"]
                fn delete_deployment_group(&self, input: &DeleteDeploymentGroupInput)  -> Result<DeleteDeploymentGroupOutput, DeleteDeploymentGroupError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.DeleteDeploymentGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteDeploymentGroupOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteDeploymentGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deregisters an on-premises instance.</p>"]
                fn deregister_on_premises_instance(&self, input: &DeregisterOnPremisesInstanceInput)  -> Result<(), DeregisterOnPremisesInstanceError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.DeregisterOnPremisesInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DeregisterOnPremisesInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about an application.</p>"]
                fn get_application(&self, input: &GetApplicationInput)  -> Result<GetApplicationOutput, GetApplicationError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplication");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetApplicationOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetApplicationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about an application revision.</p>"]
                fn get_application_revision(&self, input: &GetApplicationRevisionInput)  -> Result<GetApplicationRevisionOutput, GetApplicationRevisionError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetApplicationRevision");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetApplicationRevisionOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetApplicationRevisionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about a deployment.</p>"]
                fn get_deployment(&self, input: &GetDeploymentInput)  -> Result<GetDeploymentOutput, GetDeploymentError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeployment");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeploymentOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDeploymentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about a deployment configuration.</p>"]
                fn get_deployment_config(&self, input: &GetDeploymentConfigInput)  -> Result<GetDeploymentConfigOutput, GetDeploymentConfigError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentConfig");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeploymentConfigOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDeploymentConfigError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about a deployment group.</p>"]
                fn get_deployment_group(&self, input: &GetDeploymentGroupInput)  -> Result<GetDeploymentGroupOutput, GetDeploymentGroupError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeploymentGroupOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDeploymentGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about an instance as part of a deployment.</p>"]
                fn get_deployment_instance(&self, input: &GetDeploymentInstanceInput)  -> Result<GetDeploymentInstanceOutput, GetDeploymentInstanceError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetDeploymentInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeploymentInstanceOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDeploymentInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets information about an on-premises instance.</p>"]
                fn get_on_premises_instance(&self, input: &GetOnPremisesInstanceInput)  -> Result<GetOnPremisesInstanceOutput, GetOnPremisesInstanceError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.GetOnPremisesInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetOnPremisesInstanceOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetOnPremisesInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists information about revisions for an application.</p>"]
                fn list_application_revisions(&self, input: &ListApplicationRevisionsInput)  -> Result<ListApplicationRevisionsOutput, ListApplicationRevisionsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListApplicationRevisions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListApplicationRevisionsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListApplicationRevisionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the applications registered with the applicable IAM user or AWS account.</p>"]
                fn list_applications(&self, input: &ListApplicationsInput)  -> Result<ListApplicationsOutput, ListApplicationsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListApplications");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListApplicationsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListApplicationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the deployment configurations with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_configs(&self, input: &ListDeploymentConfigsInput)  -> Result<ListDeploymentConfigsOutput, ListDeploymentConfigsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentConfigs");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDeploymentConfigsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDeploymentConfigsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the deployment groups for an application registered with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_groups(&self, input: &ListDeploymentGroupsInput)  -> Result<ListDeploymentGroupsOutput, ListDeploymentGroupsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDeploymentGroupsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDeploymentGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the instance for a deployment associated with the applicable IAM user or AWS account.</p>"]
                fn list_deployment_instances(&self, input: &ListDeploymentInstancesInput)  -> Result<ListDeploymentInstancesOutput, ListDeploymentInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeploymentInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDeploymentInstancesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDeploymentInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the deployments in a deployment group for an application registered with the applicable IAM user or AWS account.</p>"]
                fn list_deployments(&self, input: &ListDeploymentsInput)  -> Result<ListDeploymentsOutput, ListDeploymentsError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListDeployments");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDeploymentsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDeploymentsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets a list of names for one or more on-premises instances.</p> <p>Unless otherwise specified, both registered and deregistered on-premises instance names will be listed. To list only registered or deregistered on-premises instance names, use the registration status parameter.</p>"]
                fn list_on_premises_instances(&self, input: &ListOnPremisesInstancesInput)  -> Result<ListOnPremisesInstancesOutput, ListOnPremisesInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.ListOnPremisesInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListOnPremisesInstancesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListOnPremisesInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Registers with AWS CodeDeploy a revision for the specified application.</p>"]
                fn register_application_revision(&self, input: &RegisterApplicationRevisionInput)  -> Result<(), RegisterApplicationRevisionError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.RegisterApplicationRevision");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(RegisterApplicationRevisionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Registers an on-premises instance.</p> <note> <p>Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both.</p> </note>"]
                fn register_on_premises_instance(&self, input: &RegisterOnPremisesInstanceInput)  -> Result<(), RegisterOnPremisesInstanceError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.RegisterOnPremisesInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(RegisterOnPremisesInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes one or more tags from one or more on-premises instances.</p>"]
                fn remove_tags_from_on_premises_instances(&self, input: &RemoveTagsFromOnPremisesInstancesInput)  -> Result<(), RemoveTagsFromOnPremisesInstancesError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.RemoveTagsFromOnPremisesInstances");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(RemoveTagsFromOnPremisesInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Attempts to stop an ongoing deployment.</p>"]
                fn stop_deployment(&self, input: &StopDeploymentInput)  -> Result<StopDeploymentOutput, StopDeploymentError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.StopDeployment");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<StopDeploymentOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(StopDeploymentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Changes the name of an application.</p>"]
                fn update_application(&self, input: &UpdateApplicationInput)  -> Result<(), UpdateApplicationError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateApplication");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(UpdateApplicationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Changes information about a deployment group.</p>"]
                fn update_deployment_group(&self, input: &UpdateDeploymentGroupInput)  -> Result<UpdateDeploymentGroupOutput, UpdateDeploymentGroupError> {
                    let mut request = SignedRequest::new("POST", "codedeploy", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CodeDeploy_20141006.UpdateDeploymentGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateDeploymentGroupOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateDeploymentGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
