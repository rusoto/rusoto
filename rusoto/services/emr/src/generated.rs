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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddInstanceFleetInput {
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>Specifies the configuration of the instance fleet.</p>
    #[serde(rename = "InstanceFleet")]
    pub instance_fleet: InstanceFleetConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddInstanceFleetOutput {
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The unique identifier of the instance fleet.</p>
    #[serde(rename = "InstanceFleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
}

/// <p>Input to an AddInstanceGroups call.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddInstanceGroupsInput {
    /// <p>Instance groups to add.</p>
    #[serde(rename = "InstanceGroups")]
    pub instance_groups: Vec<InstanceGroupConfig>,
    /// <p>Job flow in which to add the instance groups.</p>
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,
}

/// <p>Output from an AddInstanceGroups call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddInstanceGroupsOutput {
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>Instance group IDs of the newly created instance groups.</p>
    #[serde(rename = "InstanceGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_ids: Option<Vec<String>>,
    /// <p>The job flow ID in which the instance groups are added.</p>
    #[serde(rename = "JobFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_id: Option<String>,
}

/// <p> The input argument to the <a>AddJobFlowSteps</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddJobFlowStepsInput {
    /// <p>A string that uniquely identifies the job flow. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>ListClusters</a>. </p>
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,
    /// <p> A list of <a>StepConfig</a> to be executed by the job flow. </p>
    #[serde(rename = "Steps")]
    pub steps: Vec<StepConfig>,
}

/// <p> The output for the <a>AddJobFlowSteps</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddJobFlowStepsOutput {
    /// <p>The identifiers of the list of steps added to the job flow.</p>
    #[serde(rename = "StepIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_ids: Option<Vec<String>>,
}

/// <p>This input identifies a cluster and a list of tags to attach.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsInput {
    /// <p>The Amazon EMR resource identifier to which tags will be added. This value must be a cluster identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of tags to associate with a cluster and propagate to EC2 instances. Tags are user-defined key/value pairs that consist of a required key string with a maximum of 128 characters, and an optional value string with a maximum of 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>This output indicates the result of adding tags to a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsOutput {}

/// <p>With Amazon EMR release version 4.0 and later, the only accepted parameter is the application name. To pass arguments to applications, you use configuration classifications specified using configuration JSON objects. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-configure-apps.html">Configuring Applications</a>.</p> <p>With earlier Amazon EMR releases, the application is any Amazon or third-party software that you can add to the cluster. This structure contains a list of strings that indicates the software to use with the cluster and accepts a user argument list. Amazon EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action argument.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    /// <p>This option is for advanced users only. This is meta information about third-party applications that third-party vendors use for testing purposes.</p>
    #[serde(rename = "AdditionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<::std::collections::HashMap<String, String>>,
    /// <p>Arguments for Amazon EMR to pass to the application.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>The name of the application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. An automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AutoScalingPolicy {
    /// <p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>
    #[serde(rename = "Constraints")]
    pub constraints: ScalingConstraints,
    /// <p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>
    #[serde(rename = "Rules")]
    pub rules: Vec<ScalingRule>,
}

/// <p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingPolicyDescription {
    /// <p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<ScalingConstraints>,
    /// <p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ScalingRule>>,
    /// <p>The status of an automatic scaling policy. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutoScalingPolicyStatus>,
}

/// <p>The reason for an <a>AutoScalingPolicyStatus</a> change.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingPolicyStateChangeReason {
    /// <p>The code indicating the reason for the change in status.<code>USER_REQUEST</code> indicates that the scaling policy status was changed by a user. <code>PROVISION_FAILURE</code> indicates that the status change was because the policy failed to provision. <code>CLEANUP_FAILURE</code> indicates an error.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>A friendly, more verbose message that accompanies an automatic scaling policy state change.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The status of an automatic scaling policy. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingPolicyStatus {
    /// <p>Indicates the status of the automatic scaling policy.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason for a change in status.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<AutoScalingPolicyStateChangeReason>,
}

/// <p>A configuration for Amazon EMR block public access. When <code>BlockPublicSecurityGroupRules</code> is set to <code>true</code>, Amazon EMR prevents cluster creation if one of the cluster's security groups has a rule that allows inbound traffic from 0.0.0.0/0 or ::/0 on a port, unless the port is specified as an exception using <code>PermittedPublicSecurityGroupRuleRanges</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockPublicAccessConfiguration {
    /// <p>Indicates whether EMR block public access is enabled (<code>true</code>) or disabled (<code>false</code>). By default, the value is <code>false</code> for accounts that have created EMR clusters before July 2019. For accounts created after this, the default is <code>true</code>.</p>
    #[serde(rename = "BlockPublicSecurityGroupRules")]
    pub block_public_security_group_rules: bool,
    /// <p>Specifies ports and port ranges that are permitted to have security group rules that allow inbound traffic from all public sources. For example, if Port 23 (Telnet) is specified for <code>PermittedPublicSecurityGroupRuleRanges</code>, Amazon EMR allows cluster creation if a security group associated with the cluster has a rule that allows inbound traffic on Port 23 from IPv4 0.0.0.0/0 or IPv6 port ::/0 as the source.</p> <p>By default, Port 22, which is used for SSH access to the cluster EC2 instances, is in the list of <code>PermittedPublicSecurityGroupRuleRanges</code>.</p>
    #[serde(rename = "PermittedPublicSecurityGroupRuleRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted_public_security_group_rule_ranges: Option<Vec<PortRange>>,
}

/// <p>Properties that describe the AWS principal that created the <code>BlockPublicAccessConfiguration</code> using the <code>PutBlockPublicAccessConfiguration</code> action as well as the date and time that the configuration was created. Each time a configuration for block public access is updated, Amazon EMR updates this metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BlockPublicAccessConfigurationMetadata {
    /// <p>The Amazon Resource Name that created or last modified the configuration.</p>
    #[serde(rename = "CreatedByArn")]
    pub created_by_arn: String,
    /// <p>The date and time that the configuration was created.</p>
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
}

/// <p>Configuration of a bootstrap action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BootstrapActionConfig {
    /// <p>The name of the bootstrap action.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The script run by the bootstrap action.</p>
    #[serde(rename = "ScriptBootstrapAction")]
    pub script_bootstrap_action: ScriptBootstrapActionConfig,
}

/// <p>Reports the configuration of a bootstrap action in a cluster (job flow).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BootstrapActionDetail {
    /// <p>A description of the bootstrap action.</p>
    #[serde(rename = "BootstrapActionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_action_config: Option<BootstrapActionConfig>,
}

/// <p>Specification of the status of a CancelSteps request. Available only in Amazon EMR version 4.8.0 and later, excluding version 5.0.0.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelStepsInfo {
    /// <p>The reason for the failure if the CancelSteps request fails.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of a CancelSteps Request. The value may be SUBMITTED or FAILED.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The encrypted StepId of a step.</p>
    #[serde(rename = "StepId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
}

/// <p>The input argument to the <a>CancelSteps</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelStepsInput {
    /// <p>The <code>ClusterID</code> for which specified steps will be canceled. Use <a>RunJobFlow</a> and <a>ListClusters</a> to get ClusterIDs. </p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The option to choose for cancelling <code>RUNNING</code> steps. By default, the value is <code>SEND_INTERRUPT</code>.</p>
    #[serde(rename = "StepCancellationOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_cancellation_option: Option<String>,
    /// <p>The list of <code>StepIDs</code> to cancel. Use <a>ListSteps</a> to get steps and their states for the specified cluster.</p>
    #[serde(rename = "StepIds")]
    pub step_ids: Vec<String>,
}

/// <p> The output for the <a>CancelSteps</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelStepsOutput {
    /// <p>A list of <a>CancelStepsInfo</a>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>
    #[serde(rename = "CancelStepsInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_steps_info_list: Option<Vec<CancelStepsInfo>>,
}

/// <p>The definition of a CloudWatch metric alarm, which determines when an automatic scaling activity is triggered. When the defined alarm conditions are satisfied, scaling activity begins.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchAlarmDefinition {
    /// <p>Determines how the metric specified by <code>MetricName</code> is compared to the value specified by <code>Threshold</code>.</p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>A CloudWatch metric dimension.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    /// <p>The number of periods, in five-minute increments, during which the alarm condition must exist before the alarm triggers automatic scaling activity. The default value is <code>1</code>.</p>
    #[serde(rename = "EvaluationPeriods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i64>,
    /// <p>The name of the CloudWatch metric that is watched to determine an alarm condition.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>The namespace for the CloudWatch metric. The default is <code>AWS/ElasticMapReduce</code>.</p>
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The period, in seconds, over which the statistic is applied. EMR CloudWatch metrics are emitted every five minutes (300 seconds), so if an EMR CloudWatch metric is specified, specify <code>300</code>.</p>
    #[serde(rename = "Period")]
    pub period: i64,
    /// <p>The statistic to apply to the metric associated with the alarm. The default is <code>AVERAGE</code>.</p>
    #[serde(rename = "Statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    /// <p>The value against which the specified statistic is compared.</p>
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    /// <p>The unit of measure associated with the CloudWatch metric being watched. The value specified for <code>Unit</code> must correspond to the units specified in the CloudWatch metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>The detailed description of the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Cluster {
    /// <p>The applications installed on this cluster.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    /// <p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>
    #[serde(rename = "AutoScalingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    /// <p>Specifies whether the cluster should terminate after completing all steps.</p>
    #[serde(rename = "AutoTerminate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminate: Option<bool>,
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>Applies only to Amazon EMR releases 4.x and later. The list of Configurations supplied to the EMR cluster.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>Available only in Amazon EMR version 5.7.0 and later. The ID of a custom Amazon EBS-backed Linux AMI if the cluster uses a custom AMI.</p>
    #[serde(rename = "CustomAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    /// <p>The size, in GiB, of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.</p>
    #[serde(rename = "EbsRootVolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_size: Option<i64>,
    /// <p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>
    #[serde(rename = "Ec2InstanceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_attributes: Option<Ec2InstanceAttributes>,
    /// <p>The unique identifier for the cluster.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note> <p>The instance group configuration of the cluster. A value of <code>INSTANCE<em>GROUP</code> indicates a uniform instance group configuration. A value of <code>INSTANCE</em>FLEET</code> indicates an instance fleets configuration.</p></p>
    #[serde(rename = "InstanceCollectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_collection_type: Option<String>,
    /// <p>Attributes for Kerberos configuration when Kerberos authentication is enabled using a security configuration. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-kerberos.html">Use Kerberos Authentication</a> in the <i>EMR Management Guide</i>.</p>
    #[serde(rename = "KerberosAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_attributes: Option<KerberosAttributes>,
    /// <p>The path to the Amazon S3 location where logs for this cluster are stored.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The DNS name of the master node. If the cluster is on a private subnet, this is the private DNS name. On a public subnet, this is the public DNS name.</p>
    #[serde(rename = "MasterPublicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_public_dns_name: Option<String>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    /// <p> The Amazon Resource Name (ARN) of the Outpost where the cluster is launched. </p>
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    /// <p>The Amazon EMR release label, which determines the version of open-source application packages installed on the cluster. Release labels are in the form <code>emr-x.x.x</code>, where x.x.x is an Amazon EMR release version such as <code>emr-5.14.0</code>. For more information about Amazon EMR release versions and included application versions and features, see <a href="https://docs.aws.amazon.com/emr/latest/ReleaseGuide/">https://docs.aws.amazon.com/emr/latest/ReleaseGuide/</a>. The release label applies only to Amazon EMR releases version 4.0 and later. Earlier versions use <code>AmiVersion</code>.</p>
    #[serde(rename = "ReleaseLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    /// <p>Applies only when <code>CustomAmiID</code> is used. Specifies the type of updates that are applied from the Amazon Linux AMI package repositories when an instance boots using the AMI.</p>
    #[serde(rename = "RepoUpgradeOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    /// <p>The AMI version requested for this cluster.</p>
    #[serde(rename = "RequestedAmiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ami_version: Option<String>,
    /// <p>The AMI version running on this cluster.</p>
    #[serde(rename = "RunningAmiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_ami_version: Option<String>,
    /// <p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> is available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    /// <p>The name of the security configuration applied to the cluster.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The current status details about the cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,
    /// <p>Specifies the number of steps that can be executed concurrently.</p>
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i64>,
    /// <p>A list of tags associated with a cluster.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Indicates whether Amazon EMR will lock the cluster to prevent the EC2 instances from being terminated by an API call or user intervention, or in the event of a cluster error.</p>
    #[serde(rename = "TerminationProtected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
    /// <p>Indicates whether the cluster is visible to all IAM users of the AWS account associated with the cluster. The default value, <code>true</code>, indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. If this value is <code>false</code>, only the IAM user that created the cluster can perform actions. This value can be changed on a running cluster by using the <a>SetVisibleToAllUsers</a> action. You can override the default value of <code>true</code> when you create a cluster by using the <code>VisibleToAllUsers</code> parameter of the <code>RunJobFlow</code> action.</p>
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

/// <p>The reason that the cluster changed to its current state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterStateChangeReason {
    /// <p>The programmatic code for the state change reason.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The descriptive message for the state change reason.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The detailed status of the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterStatus {
    /// <p>The current state of the cluster.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason for the cluster status change.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<ClusterStateChangeReason>,
    /// <p>A timeline that represents the status of a cluster over the lifetime of the cluster.</p>
    #[serde(rename = "Timeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<ClusterTimeline>,
}

/// <p>The summary description of the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterSummary {
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The unique identifier for the cluster.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    /// <p> The Amazon Resource Name (ARN) of the Outpost where the cluster is launched. </p>
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    /// <p>The details about the current status of the cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,
}

/// <p>Represents the timeline of the cluster's lifecycle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClusterTimeline {
    /// <p>The creation date and time of the cluster.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time when the cluster was terminated.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The date and time when the cluster was ready to execute steps.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

/// <p>An entity describing an executable that runs on a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Command {
    /// <p>Arguments for Amazon EMR to pass to the command for execution.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>The name of the command.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon S3 location of the command script.</p>
    #[serde(rename = "ScriptPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_path: Option<String>,
}

/// <p><note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>An optional configuration specification to be used when provisioning cluster instances, which can include configurations for applications and software bundled with Amazon EMR. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-configure-apps.html">Configuring Applications</a>.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    /// <p>The classification within a configuration.</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>A list of additional configurations to apply within a configuration object.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>A set of properties specified within a configuration classification.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSecurityConfigurationInput {
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The security configuration details in JSON format. For JSON parameters and examples, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-security-configurations.html">Use Security Configurations to Set Up Cluster Security</a> in the <i>Amazon EMR Management Guide</i>.</p>
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSecurityConfigurationOutput {
    /// <p>The date and time the security configuration was created.</p>
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSecurityConfigurationInput {
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSecurityConfigurationOutput {}

/// <p>This input determines which cluster to describe.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClusterInput {
    /// <p>The identifier of the cluster to describe.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

/// <p>This output contains the description of the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterOutput {
    /// <p>This output contains the details for the requested cluster.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

/// <p> The input for the <a>DescribeJobFlows</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobFlowsInput {
    /// <p>Return only job flows created after this date and time.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>Return only job flows created before this date and time.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>Return only job flows whose job flow ID is contained in this list.</p>
    #[serde(rename = "JobFlowIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_ids: Option<Vec<String>>,
    /// <p>Return only job flows whose state is contained in this list.</p>
    #[serde(rename = "JobFlowStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_states: Option<Vec<String>>,
}

/// <p> The output for the <a>DescribeJobFlows</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobFlowsOutput {
    /// <p>A list of job flows matching the parameters supplied.</p>
    #[serde(rename = "JobFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flows: Option<Vec<JobFlowDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSecurityConfigurationInput {
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSecurityConfigurationOutput {
    /// <p>The date and time the security configuration was created</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The security configuration details in JSON format.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
}

/// <p>This input determines which step to describe.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStepInput {
    /// <p>The identifier of the cluster with steps to describe.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The identifier of the step to describe.</p>
    #[serde(rename = "StepId")]
    pub step_id: String,
}

/// <p>This output contains the description of the cluster step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStepOutput {
    /// <p>The step details for the requested step identifier.</p>
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<Step>,
}

/// <p>Configuration of requested EBS block device associated with the instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EbsBlockDevice {
    /// <p>The device name that is exposed to the instance, such as /dev/sdh.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// <p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>
    #[serde(rename = "VolumeSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_specification: Option<VolumeSpecification>,
}

/// <p>Configuration of requested EBS block device associated with the instance group with count of volumes that will be associated to every instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EbsBlockDeviceConfig {
    /// <p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,
    /// <p>Number of EBS volumes with a specific volume configuration that will be associated with every instance in the instance group</p>
    #[serde(rename = "VolumesPerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_per_instance: Option<i64>,
}

/// <p>The Amazon EBS configuration of a cluster instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EbsConfiguration {
    /// <p>An array of Amazon EBS volume specifications attached to a cluster instance.</p>
    #[serde(rename = "EbsBlockDeviceConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,
    /// <p>Indicates whether an Amazon EBS volume is EBS-optimized.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
}

/// <p>EBS block device that's attached to an EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EbsVolume {
    /// <p>The device name that is exposed to the instance, such as /dev/sdh.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// <p>The volume identifier of the EBS volume.</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

/// <p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Ec2InstanceAttributes {
    /// <p>A list of additional Amazon EC2 security group IDs for the master node.</p>
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    /// <p>A list of additional Amazon EC2 security group IDs for the core and task nodes.</p>
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    /// <p>The Availability Zone in which the cluster will run. </p>
    #[serde(rename = "Ec2AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_availability_zone: Option<String>,
    /// <p>The name of the Amazon EC2 key pair to use when connecting with SSH into the master node as a user named "hadoop".</p>
    #[serde(rename = "Ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>Set this parameter to the identifier of the Amazon VPC subnet where you want the cluster to launch. If you do not specify this value, and your account supports EC2-Classic, the cluster launches in EC2-Classic.</p>
    #[serde(rename = "Ec2SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    /// <p>The identifier of the Amazon EC2 security group for the master node.</p>
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    /// <p>The identifier of the Amazon EC2 security group for the core and task nodes.</p>
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    /// <p>The IAM role that was specified when the cluster was launched. The EC2 instances of the cluster assume this role.</p>
    #[serde(rename = "IamInstanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    /// <p>Applies to clusters configured with the instance fleets option. Specifies one or more Availability Zones in which to launch EC2 cluster instances when the EC2-Classic network configuration is supported. Amazon EMR chooses the Availability Zone with the best fit from among the list of <code>RequestedEc2AvailabilityZones</code>, and then launches all cluster instances within that Availability Zone. If you do not specify this value, Amazon EMR chooses the Availability Zone for you. <code>RequestedEc2SubnetIDs</code> and <code>RequestedEc2AvailabilityZones</code> cannot be specified together.</p>
    #[serde(rename = "RequestedEc2AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ec_2_availability_zones: Option<Vec<String>>,
    /// <p>Applies to clusters configured with the instance fleets option. Specifies the unique identifier of one or more Amazon EC2 subnets in which to launch EC2 cluster instances. Subnets must exist within the same VPC. Amazon EMR chooses the EC2 subnet with the best fit from among the list of <code>RequestedEc2SubnetIds</code>, and then launches all cluster instances within that Subnet. If this value is not specified, and the account and Region support EC2-Classic networks, the cluster launches instances in the EC2-Classic network and uses <code>RequestedEc2AvailabilityZones</code> instead of this setting. If EC2-Classic is not supported, and no Subnet is specified, Amazon EMR chooses the subnet for you. <code>RequestedEc2SubnetIDs</code> and <code>RequestedEc2AvailabilityZones</code> cannot be specified together.</p>
    #[serde(rename = "RequestedEc2SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_ec_2_subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>
    #[serde(rename = "ServiceAccessSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_security_group: Option<String>,
}

/// <p>The details of the step failure. The service attempts to detect the root cause for many common failures.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureDetails {
    /// <p>The path to the log file where the step failure root cause was originally recorded.</p>
    #[serde(rename = "LogFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    /// <p>The descriptive message including the error the EMR service has identified as the cause of step failure. This is text from an error log that describes the root cause of the failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The reason for the step failure. In the case where the service cannot successfully determine the root cause of the failure, it returns "Unknown Error" as a reason.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBlockPublicAccessConfigurationInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBlockPublicAccessConfigurationOutput {
    /// <p>A configuration for Amazon EMR block public access. The configuration applies to all clusters created in your account for the current Region. The configuration specifies whether block public access is enabled. If block public access is enabled, security groups associated with the cluster cannot have rules that allow inbound traffic from 0.0.0.0/0 or ::/0 on a port, unless the port is specified as an exception using <code>PermittedPublicSecurityGroupRuleRanges</code> in the <code>BlockPublicAccessConfiguration</code>. By default, Port 22 (SSH) is an exception, and public access is allowed on this port. You can change this by updating the block public access configuration to remove the exception.</p>
    #[serde(rename = "BlockPublicAccessConfiguration")]
    pub block_public_access_configuration: BlockPublicAccessConfiguration,
    /// <p>Properties that describe the AWS principal that created the <code>BlockPublicAccessConfiguration</code> using the <code>PutBlockPublicAccessConfiguration</code> action as well as the date and time that the configuration was created. Each time a configuration for block public access is updated, Amazon EMR updates this metadata.</p>
    #[serde(rename = "BlockPublicAccessConfigurationMetadata")]
    pub block_public_access_configuration_metadata: BlockPublicAccessConfigurationMetadata,
}

/// <p>A job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HadoopJarStepConfig {
    /// <p>A list of command line arguments passed to the JAR file's main function when executed.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>A path to a JAR file run during the step.</p>
    #[serde(rename = "Jar")]
    pub jar: String,
    /// <p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a Main-Class in its manifest file.</p>
    #[serde(rename = "MainClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    /// <p>A list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<KeyValue>>,
}

/// <p>A cluster step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HadoopStepConfig {
    /// <p>The list of command line arguments to pass to the JAR file's main function for execution.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>The path to the JAR file that runs during the step.</p>
    #[serde(rename = "Jar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jar: Option<String>,
    /// <p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a main class in its manifest file.</p>
    #[serde(rename = "MainClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    /// <p>The list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents an EC2 instance provisioned as part of cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>The list of EBS volumes that are attached to this instance.</p>
    #[serde(rename = "EbsVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<Vec<EbsVolume>>,
    /// <p>The unique identifier of the instance in Amazon EC2.</p>
    #[serde(rename = "Ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>The unique identifier for the instance in Amazon EMR.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The unique identifier of the instance fleet to which an EC2 instance belongs.</p>
    #[serde(rename = "InstanceFleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
    /// <p>The identifier of the instance group to which this instance belongs.</p>
    #[serde(rename = "InstanceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    /// <p>The EC2 instance type, for example <code>m3.xlarge</code>.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The instance purchasing option. Valid values are <code>ON_DEMAND</code> or <code>SPOT</code>. </p>
    #[serde(rename = "Market")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// <p>The private DNS name of the instance.</p>
    #[serde(rename = "PrivateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The private IP address of the instance.</p>
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>The public DNS name of the instance.</p>
    #[serde(rename = "PublicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// <p>The public IP address of the instance.</p>
    #[serde(rename = "PublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    /// <p>The current status of the instance.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceStatus>,
}

/// <p><p>Describes an instance fleet, which is a group of EC2 instances that host a particular node type (master, core, or task) in an Amazon EMR cluster. Instance fleets can consist of a mix of instance types and On-Demand and Spot instances, which are provisioned to meet a defined target capacity. </p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceFleet {
    /// <p>The unique identifier of the instance fleet.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The node type that the instance fleet hosts. Valid values are MASTER, CORE, or TASK. </p>
    #[serde(rename = "InstanceFleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_type: Option<String>,
    /// <p>The specification for the instance types that comprise an instance fleet. Up to five unique instance specifications may be defined for each instance fleet. </p>
    #[serde(rename = "InstanceTypeSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_specifications: Option<Vec<InstanceTypeSpecification>>,
    /// <p>Describes the launch specification for an instance fleet. </p>
    #[serde(rename = "LaunchSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    /// <p>A friendly name for the instance fleet.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of On-Demand units that have been provisioned for the instance fleet to fulfill <code>TargetOnDemandCapacity</code>. This provisioned capacity might be less than or greater than <code>TargetOnDemandCapacity</code>.</p>
    #[serde(rename = "ProvisionedOnDemandCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_on_demand_capacity: Option<i64>,
    /// <p>The number of Spot units that have been provisioned for this instance fleet to fulfill <code>TargetSpotCapacity</code>. This provisioned capacity might be less than or greater than <code>TargetSpotCapacity</code>.</p>
    #[serde(rename = "ProvisionedSpotCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_spot_capacity: Option<i64>,
    /// <p>The current status of the instance fleet. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceFleetStatus>,
    /// <p><p>The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision. When the instance fleet launches, Amazon EMR tries to provision On-Demand instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When an On-Demand instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units. You can use <a>InstanceFleet$ProvisionedOnDemandCapacity</a> to determine the Spot capacity units that have been provisioned for the instance fleet.</p> <note> <p>If not specified or set to 0, only Spot instances are provisioned for the instance fleet using <code>TargetSpotCapacity</code>. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note></p>
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    /// <p><p>The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision. When the instance fleet launches, Amazon EMR tries to provision Spot instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When a Spot instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units. You can use <a>InstanceFleet$ProvisionedSpotCapacity</a> to determine the Spot capacity units that have been provisioned for the instance fleet.</p> <note> <p>If not specified or set to 0, only On-Demand instances are provisioned for the instance fleet. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note></p>
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}

/// <p><p>The configuration that defines an instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceFleetConfig {
    /// <p>The node type that the instance fleet hosts. Valid values are MASTER,CORE,and TASK.</p>
    #[serde(rename = "InstanceFleetType")]
    pub instance_fleet_type: String,
    /// <p>The instance type configurations that define the EC2 instances in the instance fleet.</p>
    #[serde(rename = "InstanceTypeConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    /// <p>The launch specification for the instance fleet.</p>
    #[serde(rename = "LaunchSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    /// <p>The friendly name of the instance fleet.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision. When the instance fleet launches, Amazon EMR tries to provision On-Demand instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When an On-Demand instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.</p> <note> <p>If not specified or set to 0, only Spot instances are provisioned for the instance fleet using <code>TargetSpotCapacity</code>. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note></p>
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    /// <p><p>The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision. When the instance fleet launches, Amazon EMR tries to provision Spot instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When a Spot instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.</p> <note> <p>If not specified or set to 0, only On-Demand instances are provisioned for the instance fleet. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note></p>
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}

/// <p><p>Configuration parameters for an instance fleet modification request.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceFleetModifyConfig {
    /// <p>A unique identifier for the instance fleet.</p>
    #[serde(rename = "InstanceFleetId")]
    pub instance_fleet_id: String,
    /// <p>The target capacity of On-Demand units for the instance fleet. For more information see <a>InstanceFleetConfig$TargetOnDemandCapacity</a>.</p>
    #[serde(rename = "TargetOnDemandCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    /// <p>The target capacity of Spot units for the instance fleet. For more information, see <a>InstanceFleetConfig$TargetSpotCapacity</a>.</p>
    #[serde(rename = "TargetSpotCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}

/// <p><p>The launch specification for Spot instances in the fleet, which determines the defined duration and provisioning timeout behavior.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceFleetProvisioningSpecifications {
    /// <p>The launch specification for Spot instances in the fleet, which determines the defined duration and provisioning timeout behavior.</p>
    #[serde(rename = "SpotSpecification")]
    pub spot_specification: SpotProvisioningSpecification,
}

/// <p><p>Provides status change reason details for the instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceFleetStateChangeReason {
    /// <p>A code corresponding to the reason the state change occurred.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>An explanatory message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p><p>The status of the instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceFleetStatus {
    /// <p><p>A code representing the instance fleet status.</p> <ul> <li> <p> <code>PROVISIONING</code>—The instance fleet is provisioning EC2 resources and is not yet ready to run jobs.</p> </li> <li> <p> <code>BOOTSTRAPPING</code>—EC2 instances and other resources have been provisioned and the bootstrap actions specified for the instances are underway.</p> </li> <li> <p> <code>RUNNING</code>—EC2 instances and other resources are running. They are either executing jobs or waiting to execute jobs.</p> </li> <li> <p> <code>RESIZING</code>—A resize operation is underway. EC2 instances are either being added or removed.</p> </li> <li> <p> <code>SUSPENDED</code>—A resize operation could not complete. Existing EC2 instances are running, but instances can&#39;t be added or removed.</p> </li> <li> <p> <code>TERMINATING</code>—The instance fleet is terminating EC2 instances.</p> </li> <li> <p> <code>TERMINATED</code>—The instance fleet is no longer active, and all EC2 instances have been terminated.</p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Provides status change reason details for the instance fleet.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceFleetStateChangeReason>,
    /// <p>Provides historical timestamps for the instance fleet, including the time of creation, the time it became ready to run jobs, and the time of termination.</p>
    #[serde(rename = "Timeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceFleetTimeline>,
}

/// <p><p>Provides historical timestamps for the instance fleet, including the time of creation, the time it became ready to run jobs, and the time of termination.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceFleetTimeline {
    /// <p>The time and date the instance fleet was created.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The time and date the instance fleet terminated.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The time and date the instance fleet was ready to run jobs.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

/// <p>This entity represents an instance group, which is a group of instances that have common purpose. For example, CORE instance group is used for HDFS.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceGroup {
    /// <p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See PutAutoScalingPolicy.</p>
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    /// <p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    #[serde(rename = "BidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// <p><note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p></p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The version number of the requested configuration specification for this instance group.</p>
    #[serde(rename = "ConfigurationsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations_version: Option<i64>,
    /// <p>The EBS block devices that are mapped to this instance group.</p>
    #[serde(rename = "EbsBlockDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    /// <p>If the instance group is EBS-optimized. An Amazon EBS-optimized instance uses an optimized configuration stack and provides additional, dedicated capacity for Amazon EBS I/O.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// <p>The identifier of the instance group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of the instance group. Valid values are MASTER, CORE or TASK.</p>
    #[serde(rename = "InstanceGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_type: Option<String>,
    /// <p>The EC2 instance type for all instances in the instance group.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>A list of configurations that were successfully applied for an instance group last time.</p>
    #[serde(rename = "LastSuccessfullyAppliedConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successfully_applied_configurations: Option<Vec<Configuration>>,
    /// <p>The version number of a configuration specification that was successfully applied for an instance group last time. </p>
    #[serde(rename = "LastSuccessfullyAppliedConfigurationsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successfully_applied_configurations_version: Option<i64>,
    /// <p>The marketplace to provision instances for this group. Valid values are ON_DEMAND or SPOT.</p>
    #[serde(rename = "Market")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// <p>The name of the instance group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The target number of instances for the instance group.</p>
    #[serde(rename = "RequestedInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_instance_count: Option<i64>,
    /// <p>The number of instances currently running in this instance group.</p>
    #[serde(rename = "RunningInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_instance_count: Option<i64>,
    /// <p>Policy for customizing shrink operations.</p>
    #[serde(rename = "ShrinkPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
    /// <p>The current status of the instance group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceGroupStatus>,
}

/// <p>Configuration defining a new instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceGroupConfig {
    /// <p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    /// <p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    #[serde(rename = "BidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// <p><note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p></p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>EBS configurations that will be attached to each EC2 instance in the instance group.</p>
    #[serde(rename = "EbsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    /// <p>Target number of instances for the instance group.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The role of the instance group in the cluster.</p>
    #[serde(rename = "InstanceRole")]
    pub instance_role: String,
    /// <p>The EC2 instance type for all instances in the instance group.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>Market type of the EC2 instances used to create a cluster node.</p>
    #[serde(rename = "Market")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// <p>Friendly name given to the instance group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Detailed information about an instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceGroupDetail {
    /// <p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    #[serde(rename = "BidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// <p>The date/time the instance group was created.</p>
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
    /// <p>The date/time the instance group was terminated.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>Unique identifier for the instance group.</p>
    #[serde(rename = "InstanceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    /// <p>Target number of instances to run in the instance group.</p>
    #[serde(rename = "InstanceRequestCount")]
    pub instance_request_count: i64,
    /// <p>Instance group role in the cluster</p>
    #[serde(rename = "InstanceRole")]
    pub instance_role: String,
    /// <p>Actual count of running instances.</p>
    #[serde(rename = "InstanceRunningCount")]
    pub instance_running_count: i64,
    /// <p>EC2 instance type.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>Details regarding the state of the instance group.</p>
    #[serde(rename = "LastStateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    /// <p>Market type of the EC2 instances used to create a cluster node.</p>
    #[serde(rename = "Market")]
    pub market: String,
    /// <p>Friendly name for the instance group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The date/time the instance group was available to the cluster.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
    /// <p>The date/time the instance group was started.</p>
    #[serde(rename = "StartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    /// <p>State of instance group. The following values are deprecated: STARTING, TERMINATED, and FAILED.</p>
    #[serde(rename = "State")]
    pub state: String,
}

/// <p>Modify the size or configurations of an instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceGroupModifyConfig {
    /// <p>A list of new or modified configurations to apply for an instance group.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The EC2 InstanceIds to terminate. After you terminate the instances, the instance group will not return to its original requested size.</p>
    #[serde(rename = "EC2InstanceIdsToTerminate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_ids_to_terminate: Option<Vec<String>>,
    /// <p>Target size for the instance group.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>Unique ID of the instance group to expand or shrink.</p>
    #[serde(rename = "InstanceGroupId")]
    pub instance_group_id: String,
    /// <p>Policy for customizing shrink operations.</p>
    #[serde(rename = "ShrinkPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
}

/// <p>The status change reason details for the instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceGroupStateChangeReason {
    /// <p>The programmable code for the state change reason.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The status change reason description.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The details of the instance group status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceGroupStatus {
    /// <p>The current state of the instance group.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The status change reason details for the instance group.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceGroupStateChangeReason>,
    /// <p>The timeline of the instance group status over time.</p>
    #[serde(rename = "Timeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceGroupTimeline>,
}

/// <p>The timeline of the instance group lifecycle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceGroupTimeline {
    /// <p>The creation date and time of the instance group.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time when the instance group terminated.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The date and time when the instance group became ready to perform tasks.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

/// <p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceResizePolicy {
    /// <p>Decommissioning timeout override for the specific list of instances to be terminated.</p>
    #[serde(rename = "InstanceTerminationTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_termination_timeout: Option<i64>,
    /// <p>Specific list of instances to be protected when shrinking an instance group.</p>
    #[serde(rename = "InstancesToProtect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_protect: Option<Vec<String>>,
    /// <p>Specific list of instances to be terminated when shrinking an instance group.</p>
    #[serde(rename = "InstancesToTerminate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_terminate: Option<Vec<String>>,
}

/// <p>The details of the status change reason for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceStateChangeReason {
    /// <p>The programmable code for the state change reason.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The status change reason description.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The instance status details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceStatus {
    /// <p>The current state of the instance.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The details of the status change reason for the instance.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<InstanceStateChangeReason>,
    /// <p>The timeline of the instance status over time.</p>
    #[serde(rename = "Timeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<InstanceTimeline>,
}

/// <p>The timeline of the instance lifecycle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceTimeline {
    /// <p>The creation date and time of the instance.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time when the instance was terminated.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The date and time when the instance was ready to perform tasks.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
}

/// <p><p>An instance type configuration for each instance type in an instance fleet, which determines the EC2 instances Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities. There can be a maximum of 5 instance type configurations in a fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceTypeConfig {
    /// <p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>
    #[serde(rename = "BidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// <p>The bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The configuration of Amazon Elastic Block Storage (EBS) attached to each instance as defined by <code>InstanceType</code>. </p>
    #[serde(rename = "EbsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    /// <p>An EC2 instance type, such as <code>m3.xlarge</code>. </p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <a>InstanceFleetConfig</a>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>
    #[serde(rename = "WeightedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<i64>,
}

/// <p><p>The configuration specification for each instance type in an instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceTypeSpecification {
    /// <p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD.</p>
    #[serde(rename = "BidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<String>,
    /// <p>The bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%).</p>
    #[serde(rename = "BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    /// <p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software bundled with Amazon EMR.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The configuration of Amazon Elastic Block Storage (EBS) attached to each instance as defined by <code>InstanceType</code>.</p>
    #[serde(rename = "EbsBlockDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    /// <p>Evaluates to <code>TRUE</code> when the specified <code>InstanceType</code> is EBS-optimized.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// <p>The EC2 instance type, for example <code>m3.xlarge</code>.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <a>InstanceFleetConfig</a>. Capacity values represent performance characteristics such as vCPUs, memory, or I/O. If not specified, the default value is 1.</p>
    #[serde(rename = "WeightedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<i64>,
}

/// <p>A description of a cluster (job flow).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobFlowDetail {
    /// <p>Applies only to Amazon EMR AMI versions 3.x and 2.x. For Amazon EMR releases 4.0 and later, <code>ReleaseLabel</code> is used. To specify a custom AMI, use <code>CustomAmiID</code>.</p>
    #[serde(rename = "AmiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_version: Option<String>,
    /// <p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides a way for the automatic scaling feature to get the required permissions it needs to launch and terminate EC2 instances in an instance group.</p>
    #[serde(rename = "AutoScalingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    /// <p>A list of the bootstrap actions run by the job flow.</p>
    #[serde(rename = "BootstrapActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionDetail>>,
    /// <p>Describes the execution status of the job flow.</p>
    #[serde(rename = "ExecutionStatusDetail")]
    pub execution_status_detail: JobFlowExecutionStatusDetail,
    /// <p>Describes the Amazon EC2 instances of the job flow.</p>
    #[serde(rename = "Instances")]
    pub instances: JobFlowInstancesDetail,
    /// <p>The job flow identifier.</p>
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,
    /// <p>The IAM role that was specified when the job flow was launched. The EC2 instances of the job flow assume this role.</p>
    #[serde(rename = "JobFlowRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_role: Option<String>,
    /// <p>The location in Amazon S3 where log files for the job are stored.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The name of the job flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    /// <p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>A list of steps run by the job flow.</p>
    #[serde(rename = "Steps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepDetail>>,
    /// <p>A list of strings set by third party software when the job flow is launched. If you are not using third party software to manage the job flow this value is empty.</p>
    #[serde(rename = "SupportedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    /// <p>Indicates whether the cluster is visible to all IAM users of the AWS account associated with the cluster. The default value, <code>true</code>, indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. If this value is <code>false</code>, only the IAM user that created the cluster can perform actions. This value can be changed on a running cluster by using the <a>SetVisibleToAllUsers</a> action. You can override the default value of <code>true</code> when you create a cluster by using the <code>VisibleToAllUsers</code> parameter of the <code>RunJobFlow</code> action.</p>
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

/// <p>Describes the status of the cluster (job flow).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobFlowExecutionStatusDetail {
    /// <p>The creation date and time of the job flow.</p>
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
    /// <p>The completion date and time of the job flow.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>Description of the job flow last changed state.</p>
    #[serde(rename = "LastStateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    /// <p>The date and time when the job flow was ready to start running bootstrap actions.</p>
    #[serde(rename = "ReadyDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_date_time: Option<f64>,
    /// <p>The start date and time of the job flow.</p>
    #[serde(rename = "StartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    /// <p>The state of the job flow.</p>
    #[serde(rename = "State")]
    pub state: String,
}

/// <p>A description of the Amazon EC2 instance on which the cluster (job flow) runs. A valid JobFlowInstancesConfig must contain either InstanceGroups or InstanceFleets, which is the recommended configuration. They cannot be used together. You may also have MasterInstanceType, SlaveInstanceType, and InstanceCount (all three must be present), but we don't recommend this configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct JobFlowInstancesConfig {
    /// <p>A list of additional Amazon EC2 security group IDs for the master node.</p>
    #[serde(rename = "AdditionalMasterSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    /// <p>A list of additional Amazon EC2 security group IDs for the core and task nodes.</p>
    #[serde(rename = "AdditionalSlaveSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    /// <p>The name of the EC2 key pair that can be used to ssh to the master node as the user called "hadoop."</p>
    #[serde(rename = "Ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>Applies to clusters that use the uniform instance group configuration. To launch the cluster in Amazon Virtual Private Cloud (Amazon VPC), set this parameter to the identifier of the Amazon VPC subnet where you want the cluster to launch. If you do not specify this value and your account supports EC2-Classic, the cluster launches in EC2-Classic.</p>
    #[serde(rename = "Ec2SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    /// <p><p>Applies to clusters that use the instance fleet configuration. When multiple EC2 subnet IDs are specified, Amazon EMR evaluates them and launches instances in the optimal subnet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    #[serde(rename = "Ec2SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the Amazon EC2 security group for the master node.</p>
    #[serde(rename = "EmrManagedMasterSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    /// <p>The identifier of the Amazon EC2 security group for the core and task nodes.</p>
    #[serde(rename = "EmrManagedSlaveSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    /// <p>Applies only to Amazon EMR release versions earlier than 4.0. The Hadoop version for the cluster. Valid inputs are "0.18" (deprecated), "0.20" (deprecated), "0.20.205" (deprecated), "1.0.3", "2.2.0", or "2.4.0". If you do not set this value, the default of 0.18 is used, unless the <code>AmiVersion</code> parameter is set in the RunJobFlow call, in which case the default version of Hadoop for that AMI version is used.</p>
    #[serde(rename = "HadoopVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hadoop_version: Option<String>,
    /// <p>The number of EC2 instances in the cluster.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p><note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note> <p>Describes the EC2 instances and instance configurations for clusters that use the instance fleet configuration.</p></p>
    #[serde(rename = "InstanceFleets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleetConfig>>,
    /// <p>Configuration for the instance groups in a cluster.</p>
    #[serde(rename = "InstanceGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupConfig>>,
    /// <p>Specifies whether the cluster should remain available after completing all steps.</p>
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    /// <p>The EC2 instance type of the master node.</p>
    #[serde(rename = "MasterInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_type: Option<String>,
    /// <p>The Availability Zone in which the cluster runs.</p>
    #[serde(rename = "Placement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementType>,
    /// <p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>
    #[serde(rename = "ServiceAccessSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_security_group: Option<String>,
    /// <p>The EC2 instance type of the core and task nodes.</p>
    #[serde(rename = "SlaveInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_instance_type: Option<String>,
    /// <p>Specifies whether to lock the cluster to prevent the Amazon EC2 instances from being terminated by API call, user intervention, or in the event of a job-flow error.</p>
    #[serde(rename = "TerminationProtected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
}

/// <p>Specify the type of Amazon EC2 instances that the cluster (job flow) runs on.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobFlowInstancesDetail {
    /// <p>The name of an Amazon EC2 key pair that can be used to ssh to the master node.</p>
    #[serde(rename = "Ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>For clusters launched within Amazon Virtual Private Cloud, this is the identifier of the subnet where the cluster was launched.</p>
    #[serde(rename = "Ec2SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    /// <p>The Hadoop version for the cluster.</p>
    #[serde(rename = "HadoopVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hadoop_version: Option<String>,
    /// <p>The number of Amazon EC2 instances in the cluster. If the value is 1, the same instance serves as both the master and core and task node. If the value is greater than 1, one instance is the master node and all others are core and task nodes.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>Details about the instance groups in a cluster.</p>
    #[serde(rename = "InstanceGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupDetail>>,
    /// <p>Specifies whether the cluster should remain available after completing all steps.</p>
    #[serde(rename = "KeepJobFlowAliveWhenNoSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    /// <p>The Amazon EC2 instance identifier of the master node.</p>
    #[serde(rename = "MasterInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_id: Option<String>,
    /// <p>The Amazon EC2 master node instance type.</p>
    #[serde(rename = "MasterInstanceType")]
    pub master_instance_type: String,
    /// <p>The DNS name of the master node. If the cluster is on a private subnet, this is the private DNS name. On a public subnet, this is the public DNS name.</p>
    #[serde(rename = "MasterPublicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_public_dns_name: Option<String>,
    /// <p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour that an m1.small runs. Larger instances are weighted more, so an Amazon EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>
    #[serde(rename = "NormalizedInstanceHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    /// <p>The Amazon EC2 Availability Zone for the cluster.</p>
    #[serde(rename = "Placement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<PlacementType>,
    /// <p>The Amazon EC2 core and task node instance type.</p>
    #[serde(rename = "SlaveInstanceType")]
    pub slave_instance_type: String,
    /// <p>Specifies whether the Amazon EC2 instances in the cluster are protected from termination by API calls, user intervention, or in the event of a job-flow error.</p>
    #[serde(rename = "TerminationProtected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protected: Option<bool>,
}

/// <p>Attributes for Kerberos configuration when Kerberos authentication is enabled using a security configuration. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-kerberos.html">Use Kerberos Authentication</a> in the <i>EMR Management Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KerberosAttributes {
    /// <p>The Active Directory password for <code>ADDomainJoinUser</code>.</p>
    #[serde(rename = "ADDomainJoinPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_domain_join_password: Option<String>,
    /// <p>Required only when establishing a cross-realm trust with an Active Directory domain. A user with sufficient privileges to join resources to the domain.</p>
    #[serde(rename = "ADDomainJoinUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_domain_join_user: Option<String>,
    /// <p>Required only when establishing a cross-realm trust with a KDC in a different realm. The cross-realm principal password, which must be identical across realms.</p>
    #[serde(rename = "CrossRealmTrustPrincipalPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_realm_trust_principal_password: Option<String>,
    /// <p>The password used within the cluster for the kadmin service on the cluster-dedicated KDC, which maintains Kerberos principals, password policies, and keytabs for the cluster.</p>
    #[serde(rename = "KdcAdminPassword")]
    pub kdc_admin_password: String,
    /// <p>The name of the Kerberos realm to which all nodes in a cluster belong. For example, <code>EC2.INTERNAL</code>. </p>
    #[serde(rename = "Realm")]
    pub realm: String,
}

/// <p>A key value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    /// <p>The unique identifier of a key value pair.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value part of the identified key.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>This input determines which bootstrap actions to retrieve.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBootstrapActionsInput {
    /// <p>The cluster identifier for the bootstrap actions to list.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This output contains the bootstrap actions detail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBootstrapActionsOutput {
    /// <p>The bootstrap actions associated with the cluster.</p>
    #[serde(rename = "BootstrapActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<Command>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This input determines how the ListClusters action filters the list of clusters that it returns.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListClustersInput {
    /// <p>The cluster state filters to apply when listing clusters.</p>
    #[serde(rename = "ClusterStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_states: Option<Vec<String>>,
    /// <p>The creation date and time beginning value filter for listing clusters.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>The creation date and time end value filter for listing clusters.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This contains a ClusterSummaryList with the cluster details; for example, the cluster IDs, names, and status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClustersOutput {
    /// <p>The list of clusters for the account based on the given filters.</p>
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ClusterSummary>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstanceFleetsInput {
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstanceFleetsOutput {
    /// <p>The list of instance fleets for the cluster and given filters.</p>
    #[serde(rename = "InstanceFleets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleet>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This input determines which instance groups to retrieve.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstanceGroupsInput {
    /// <p>The identifier of the cluster for which to list the instance groups.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This input determines which instance groups to retrieve.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstanceGroupsOutput {
    /// <p>The list of instance groups for the cluster and given filters.</p>
    #[serde(rename = "InstanceGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroup>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This input determines which instances to list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstancesInput {
    /// <p>The identifier of the cluster for which to list the instances.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The unique identifier of the instance fleet.</p>
    #[serde(rename = "InstanceFleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_id: Option<String>,
    /// <p>The node type of the instance fleet. For example MASTER, CORE, or TASK.</p>
    #[serde(rename = "InstanceFleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_fleet_type: Option<String>,
    /// <p>The identifier of the instance group for which to list the instances.</p>
    #[serde(rename = "InstanceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
    /// <p>The type of instance group for which to list the instances.</p>
    #[serde(rename = "InstanceGroupTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_types: Option<Vec<String>>,
    /// <p>A list of instance states that will filter the instances returned with this request.</p>
    #[serde(rename = "InstanceStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_states: Option<Vec<String>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>This output contains the list of instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstancesOutput {
    /// <p>The list of instances for the cluster and given filters.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSecurityConfigurationsInput {
    /// <p>The pagination token that indicates the set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSecurityConfigurationsOutput {
    /// <p>A pagination token that indicates the next set of results to retrieve. Include the marker in the next ListSecurityConfiguration call to retrieve the next page of results, if required.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The creation date and time, and name, of each security configuration.</p>
    #[serde(rename = "SecurityConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfigurationSummary>>,
}

/// <p>This input determines which steps to list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStepsInput {
    /// <p>The identifier of the cluster for which to list the steps.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The filter to limit the step list based on the identifier of the steps. You can specify a maximum of ten Step IDs. The character constraint applies to the overall length of the array.</p>
    #[serde(rename = "StepIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_ids: Option<Vec<String>>,
    /// <p>The filter to limit the step list based on certain states.</p>
    #[serde(rename = "StepStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_states: Option<Vec<String>>,
}

/// <p>This output contains the list of steps returned in reverse order. This means that the last step is the first element in the list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStepsOutput {
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The filtered list of steps for the cluster.</p>
    #[serde(rename = "Steps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepSummary>>,
}

/// <p>A CloudWatch dimension, which is specified using a <code>Key</code> (known as a <code>Name</code> in CloudWatch), <code>Value</code> pair. By default, Amazon EMR uses one dimension whose <code>Key</code> is <code>JobFlowID</code> and <code>Value</code> is a variable representing the cluster ID, which is <code>${emr.clusterId}</code>. This enables the rule to bootstrap when the cluster ID becomes available.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The dimension name.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The dimension value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyClusterInput {
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The number of steps that can be executed concurrently. You can specify a maximum of 256 steps. </p>
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyClusterOutput {
    /// <p>The number of steps that can be executed concurrently.</p>
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyInstanceFleetInput {
    /// <p>The unique identifier of the cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The unique identifier of the instance fleet.</p>
    #[serde(rename = "InstanceFleet")]
    pub instance_fleet: InstanceFleetModifyConfig,
}

/// <p>Change the size of some instance groups.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyInstanceGroupsInput {
    /// <p>The ID of the cluster to which the instance group belongs.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>Instance groups to change.</p>
    #[serde(rename = "InstanceGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupModifyConfig>>,
}

/// <p>The Amazon EC2 Availability Zone configuration of the cluster (job flow).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementType {
    /// <p>The Amazon EC2 Availability Zone for the cluster. <code>AvailabilityZone</code> is used for uniform instance groups, while <code>AvailabilityZones</code> (plural) is used for instance fleets.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p><p>When multiple Availability Zones are specified, Amazon EMR evaluates them and launches instances in the optimal Availability Zone. <code>AvailabilityZones</code> is used for instance fleets, while <code>AvailabilityZone</code> (singular) is used for uniform instance groups.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
}

/// <p>A list of port ranges that are permitted to allow inbound traffic from all public IP addresses. To specify a single port, use the same value for <code>MinRange</code> and <code>MaxRange</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortRange {
    /// <p>The smallest port number in a specified range of port numbers.</p>
    #[serde(rename = "MaxRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range: Option<i64>,
    /// <p>The smallest port number in a specified range of port numbers.</p>
    #[serde(rename = "MinRange")]
    pub min_range: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAutoScalingPolicyInput {
    /// <p>Specifies the definition of the automatic scaling policy.</p>
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: AutoScalingPolicy,
    /// <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>Specifies the ID of the instance group to which the automatic scaling policy is applied.</p>
    #[serde(rename = "InstanceGroupId")]
    pub instance_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAutoScalingPolicyOutput {
    /// <p>The automatic scaling policy definition.</p>
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>Specifies the ID of the instance group to which the scaling policy is applied.</p>
    #[serde(rename = "InstanceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBlockPublicAccessConfigurationInput {
    /// <p>A configuration for Amazon EMR block public access. The configuration applies to all clusters created in your account for the current Region. The configuration specifies whether block public access is enabled. If block public access is enabled, security groups associated with the cluster cannot have rules that allow inbound traffic from 0.0.0.0/0 or ::/0 on a port, unless the port is specified as an exception using <code>PermittedPublicSecurityGroupRuleRanges</code> in the <code>BlockPublicAccessConfiguration</code>. By default, Port 22 (SSH) is an exception, and public access is allowed on this port. You can change this by updating <code>BlockPublicSecurityGroupRules</code> to remove the exception.</p>
    #[serde(rename = "BlockPublicAccessConfiguration")]
    pub block_public_access_configuration: BlockPublicAccessConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutBlockPublicAccessConfigurationOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveAutoScalingPolicyInput {
    /// <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>Specifies the ID of the instance group to which the scaling policy is applied.</p>
    #[serde(rename = "InstanceGroupId")]
    pub instance_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveAutoScalingPolicyOutput {}

/// <p>This input identifies a cluster and a list of tags to remove.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsInput {
    /// <p>The Amazon EMR resource identifier from which tags will be removed. This value must be a cluster identifier.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of tag keys to remove from a resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>This output indicates the result of removing tags from a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsOutput {}

/// <p> Input to the <a>RunJobFlow</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RunJobFlowInput {
    /// <p>A JSON string for selecting additional features.</p>
    #[serde(rename = "AdditionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// <p>Applies only to Amazon EMR AMI versions 3.x and 2.x. For Amazon EMR releases 4.0 and later, <code>ReleaseLabel</code> is used. To specify a custom AMI, use <code>CustomAmiID</code>.</p>
    #[serde(rename = "AmiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_version: Option<String>,
    /// <p>Applies to Amazon EMR releases 4.0 and later. A case-insensitive list of applications for Amazon EMR to install and configure when launching the cluster. For a list of applications available for each Amazon EMR release version, see the <a href="https://docs.aws.amazon.com/emr/latest/ReleaseGuide/">Amazon EMR Release Guide</a>.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    /// <p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>
    #[serde(rename = "AutoScalingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role: Option<String>,
    /// <p>A list of bootstrap actions to run before Hadoop starts on the cluster nodes.</p>
    #[serde(rename = "BootstrapActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionConfig>>,
    /// <p>For Amazon EMR releases 4.0 and later. The list of configurations supplied for the EMR cluster you are creating.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>Available only in Amazon EMR version 5.7.0 and later. The ID of a custom Amazon EBS-backed Linux AMI. If specified, Amazon EMR uses this AMI when it launches cluster EC2 instances. For more information about custom AMIs in Amazon EMR, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-custom-ami.html">Using a Custom AMI</a> in the <i>Amazon EMR Management Guide</i>. If omitted, the cluster uses the base Linux AMI for the <code>ReleaseLabel</code> specified. For Amazon EMR versions 2.x and 3.x, use <code>AmiVersion</code> instead.</p> <p>For information about creating a custom AMI, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/creating-an-ami-ebs.html">Creating an Amazon EBS-Backed Linux AMI</a> in the <i>Amazon Elastic Compute Cloud User Guide for Linux Instances</i>. For information about finding an AMI ID, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Finding a Linux AMI</a>. </p>
    #[serde(rename = "CustomAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_id: Option<String>,
    /// <p>The size, in GiB, of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.</p>
    #[serde(rename = "EbsRootVolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_root_volume_size: Option<i64>,
    /// <p>A specification of the number and type of Amazon EC2 instances.</p>
    #[serde(rename = "Instances")]
    pub instances: JobFlowInstancesConfig,
    /// <p>Also called instance profile and EC2 role. An IAM role for an EMR cluster. The EC2 instances of the cluster assume this role. The default role is <code>EMR_EC2_DefaultRole</code>. In order to use the default role, you must have already created it using the CLI or console.</p>
    #[serde(rename = "JobFlowRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_role: Option<String>,
    /// <p>Attributes for Kerberos configuration when Kerberos authentication is enabled using a security configuration. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-kerberos.html">Use Kerberos Authentication</a> in the <i>EMR Management Guide</i>.</p>
    #[serde(rename = "KerberosAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerberos_attributes: Option<KerberosAttributes>,
    /// <p>The location in Amazon S3 to write the log files of the job flow. If a value is not provided, logs are not created.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The name of the job flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and later, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use with the job flow that accepts a user argument list. EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action arguments. For more information, see &quot;Launch a Job Flow on the MapR Distribution for Hadoop&quot; in the <a href="https://docs.aws.amazon.com/emr/latest/DeveloperGuide/emr-dg.pdf">Amazon EMR Developer Guide</a>. Supported values are:</p> <ul> <li> <p>&quot;mapr-m3&quot; - launch the cluster using MapR M3 Edition.</p> </li> <li> <p>&quot;mapr-m5&quot; - launch the cluster using MapR M5 Edition.</p> </li> <li> <p>&quot;mapr&quot; with the user arguments specifying &quot;--edition,m3&quot; or &quot;--edition,m5&quot; - launch the job flow using MapR M3 or M5 Edition respectively.</p> </li> <li> <p>&quot;mapr-m7&quot; - launch the cluster using MapR M7 Edition.</p> </li> <li> <p>&quot;hunk&quot; - launch the cluster with the Hunk Big Data Analtics Platform.</p> </li> <li> <p>&quot;hue&quot;- launch the cluster with Hue installed.</p> </li> <li> <p>&quot;spark&quot; - launch the cluster with Apache Spark installed.</p> </li> <li> <p>&quot;ganglia&quot; - launch the cluster with the Ganglia Monitoring System installed.</p> </li> </ul></p>
    #[serde(rename = "NewSupportedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_supported_products: Option<Vec<SupportedProductConfig>>,
    /// <p>The Amazon EMR release label, which determines the version of open-source application packages installed on the cluster. Release labels are in the form <code>emr-x.x.x</code>, where x.x.x is an Amazon EMR release version such as <code>emr-5.14.0</code>. For more information about Amazon EMR release versions and included application versions and features, see <a href="https://docs.aws.amazon.com/emr/latest/ReleaseGuide/">https://docs.aws.amazon.com/emr/latest/ReleaseGuide/</a>. The release label applies only to Amazon EMR releases version 4.0 and later. Earlier versions use <code>AmiVersion</code>.</p>
    #[serde(rename = "ReleaseLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    /// <p>Applies only when <code>CustomAmiID</code> is used. Specifies which updates from the Amazon Linux AMI package repositories to apply automatically when the instance boots using the AMI. If omitted, the default is <code>SECURITY</code>, which indicates that only security updates are applied. If <code>NONE</code> is specified, no updates are applied, and all updates must be applied manually.</p>
    #[serde(rename = "RepoUpgradeOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    /// <p>Specifies the way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>
    #[serde(rename = "ScaleDownBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_behavior: Option<String>,
    /// <p>The name of a security configuration to apply to the cluster.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Specifies the number of steps that can be executed concurrently. The default value is <code>1</code>. The maximum value is <code>256</code>.</p>
    #[serde(rename = "StepConcurrencyLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_concurrency_level: Option<i64>,
    /// <p>A list of steps to run.</p>
    #[serde(rename = "Steps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepConfig>>,
    /// <p><note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and later, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use. For more information, see the <a href="https://docs.aws.amazon.com/emr/latest/DeveloperGuide/emr-dg.pdf">Amazon EMR Developer Guide</a>. Currently supported values are:</p> <ul> <li> <p>&quot;mapr-m3&quot; - launch the job flow using MapR M3 Edition.</p> </li> <li> <p>&quot;mapr-m5&quot; - launch the job flow using MapR M5 Edition.</p> </li> </ul></p>
    #[serde(rename = "SupportedProducts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    /// <p>A list of tags to associate with a cluster and propagate to Amazon EC2 instances.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A value of <code>true</code> indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. This is the default. A value of <code>false</code> indicates that only the IAM user who created the cluster can perform actions.</p>
    #[serde(rename = "VisibleToAllUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}

/// <p> The result of the <a>RunJobFlow</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RunJobFlowOutput {
    /// <p>The Amazon Resource Name of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>An unique identifier for the job flow.</p>
    #[serde(rename = "JobFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_flow_id: Option<String>,
}

/// <p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingAction {
    /// <p>Not available for instance groups. Instance groups use the market type specified for the group.</p>
    #[serde(rename = "Market")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// <p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
}

/// <p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or below these limits.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingConstraints {
    /// <p>The upper boundary of EC2 instances in an instance group beyond which scaling activities are not allowed to grow. Scale-out activities will not add instances beyond this boundary.</p>
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    /// <p>The lower boundary of EC2 instances in an instance group below which scaling activities are not allowed to shrink. Scale-in activities will not terminate instances below this boundary.</p>
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
}

/// <p>A scale-in or scale-out rule that defines scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingRule {
    /// <p>The conditions that trigger an automatic scaling activity.</p>
    #[serde(rename = "Action")]
    pub action: ScalingAction,
    /// <p>A friendly, more verbose description of the automatic scaling rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name used to identify an automatic scaling rule. Rule names must be unique within a scaling policy.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The CloudWatch alarm definition that determines when automatic scaling activity is triggered.</p>
    #[serde(rename = "Trigger")]
    pub trigger: ScalingTrigger,
}

/// <p>The conditions that trigger an automatic scaling activity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingTrigger {
    /// <p>The definition of a CloudWatch metric alarm. When the defined alarm conditions are met along with other trigger parameters, scaling activity begins.</p>
    #[serde(rename = "CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
}

/// <p>Configuration of the script to run during a bootstrap action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptBootstrapActionConfig {
    /// <p>A list of command line arguments to pass to the bootstrap action script.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>Location of the script to run during a bootstrap action. Can be either a location in Amazon S3 or on a local file system.</p>
    #[serde(rename = "Path")]
    pub path: String,
}

/// <p>The creation date and time, and name, of a security configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityConfigurationSummary {
    /// <p>The date and time the security configuration was created.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p> The input argument to the <a>TerminationProtection</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetTerminationProtectionInput {
    /// <p> A list of strings that uniquely identify the clusters to protect. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>DescribeJobFlows</a> . </p>
    #[serde(rename = "JobFlowIds")]
    pub job_flow_ids: Vec<String>,
    /// <p>A Boolean that indicates whether to protect the cluster and prevent the Amazon EC2 instances in the cluster from shutting down due to API calls, user intervention, or job-flow error.</p>
    #[serde(rename = "TerminationProtected")]
    pub termination_protected: bool,
}

/// <p>The input to the SetVisibleToAllUsers action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetVisibleToAllUsersInput {
    /// <p>The unique identifier of the job flow (cluster).</p>
    #[serde(rename = "JobFlowIds")]
    pub job_flow_ids: Vec<String>,
    /// <p>A value of <code>true</code> indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. This is the default. A value of <code>false</code> indicates that only the IAM user who created the cluster can perform actions.</p>
    #[serde(rename = "VisibleToAllUsers")]
    pub visible_to_all_users: bool,
}

/// <p>Policy for customizing shrink operations. Allows configuration of decommissioning timeout and targeted instance shrinking.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShrinkPolicy {
    /// <p>The desired timeout for decommissioning an instance. Overrides the default YARN decommissioning timeout.</p>
    #[serde(rename = "DecommissionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decommission_timeout: Option<i64>,
    /// <p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>
    #[serde(rename = "InstanceResizePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_resize_policy: Option<InstanceResizePolicy>,
}

/// <p>An automatic scaling configuration, which describes how the policy adds or removes instances, the cooldown period, and the number of EC2 instances that will be added each time the CloudWatch metric alarm condition is satisfied.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleScalingPolicyConfiguration {
    /// <p>The way in which EC2 instances are added (if <code>ScalingAdjustment</code> is a positive number) or terminated (if <code>ScalingAdjustment</code> is a negative number) each time the scaling activity is triggered. <code>CHANGE_IN_CAPACITY</code> is the default. <code>CHANGE_IN_CAPACITY</code> indicates that the EC2 instance count increments or decrements by <code>ScalingAdjustment</code>, which should be expressed as an integer. <code>PERCENT_CHANGE_IN_CAPACITY</code> indicates the instance count increments or decrements by the percentage specified by <code>ScalingAdjustment</code>, which should be expressed as an integer. For example, 20 indicates an increase in 20% increments of cluster capacity. <code>EXACT_CAPACITY</code> indicates the scaling activity results in an instance group with the number of EC2 instances specified by <code>ScalingAdjustment</code>, which should be expressed as a positive integer.</p>
    #[serde(rename = "AdjustmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    /// <p>The amount of time, in seconds, after a scaling activity completes before any further trigger-related scaling activities can start. The default value is 0.</p>
    #[serde(rename = "CoolDown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cool_down: Option<i64>,
    /// <p>The amount by which to scale in or scale out, based on the specified <code>AdjustmentType</code>. A positive value adds to the instance group's EC2 instance count while a negative number removes instances. If <code>AdjustmentType</code> is set to <code>EXACT_CAPACITY</code>, the number should only be a positive integer. If <code>AdjustmentType</code> is set to <code>PERCENT_CHANGE_IN_CAPACITY</code>, the value should express the percentage as an integer. For example, -20 indicates a decrease in 20% increments of cluster capacity.</p>
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,
}

/// <p><p>The launch specification for Spot instances in the instance fleet, which determines the defined duration and provisioning timeout behavior.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotProvisioningSpecification {
    /// <p>The defined duration for Spot instances (also known as Spot blocks) in minutes. When specified, the Spot instance does not terminate before the defined duration expires, and defined duration pricing for Spot instances applies. Valid values are 60, 120, 180, 240, 300, or 360. The duration period starts as soon as a Spot instance receives its instance ID. At the end of the duration, Amazon EC2 marks the Spot instance for termination and provides a Spot instance termination notice, which gives the instance a two-minute warning before it terminates. </p>
    #[serde(rename = "BlockDurationMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_duration_minutes: Option<i64>,
    /// <p>The action to take when <code>TargetSpotCapacity</code> has not been fulfilled when the <code>TimeoutDurationMinutes</code> has expired; that is, when all Spot instances could not be provisioned within the Spot provisioning timeout. Valid values are <code>TERMINATE_CLUSTER</code> and <code>SWITCH_TO_ON_DEMAND</code>. SWITCH_TO_ON_DEMAND specifies that if no Spot instances are available, On-Demand Instances should be provisioned to fulfill any remaining Spot capacity.</p>
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: String,
    /// <p>The spot provisioning timeout period in minutes. If Spot instances are not provisioned within this time period, the <code>TimeOutAction</code> is taken. Minimum value is 5 and maximum value is 1440. The timeout applies only during initial provisioning, when the cluster is first created.</p>
    #[serde(rename = "TimeoutDurationMinutes")]
    pub timeout_duration_minutes: i64,
}

/// <p>This represents a step in a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Step {
    /// <p>The action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE. TERMINATE_JOB_FLOW is provided for backward compatibility. We recommend using TERMINATE_CLUSTER instead.</p>
    #[serde(rename = "ActionOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    /// <p>The Hadoop job configuration of the cluster step.</p>
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    /// <p>The identifier of the cluster step.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the cluster step.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current execution status details of the cluster step.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StepStatus>,
}

/// <p>Specification of a cluster (job flow) step.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepConfig {
    /// <p>The action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE. TERMINATE_JOB_FLOW is provided for backward compatibility. We recommend using TERMINATE_CLUSTER instead.</p>
    #[serde(rename = "ActionOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    /// <p>The JAR file used for the step.</p>
    #[serde(rename = "HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,
    /// <p>The name of the step.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Combines the execution state and configuration of a step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepDetail {
    /// <p>The description of the step status.</p>
    #[serde(rename = "ExecutionStatusDetail")]
    pub execution_status_detail: StepExecutionStatusDetail,
    /// <p>The step configuration.</p>
    #[serde(rename = "StepConfig")]
    pub step_config: StepConfig,
}

/// <p>The execution state of a step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepExecutionStatusDetail {
    /// <p>The creation date and time of the step.</p>
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
    /// <p>The completion date and time of the step.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>A description of the step's current state.</p>
    #[serde(rename = "LastStateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_reason: Option<String>,
    /// <p>The start date and time of the step.</p>
    #[serde(rename = "StartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    /// <p>The state of the step.</p>
    #[serde(rename = "State")]
    pub state: String,
}

/// <p>The details of the step state change reason.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepStateChangeReason {
    /// <p>The programmable code for the state change reason. Note: Currently, the service provides no code for the state change.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The descriptive message for the state change reason.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The execution status details of the cluster step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepStatus {
    /// <p>The details for the step failure including reason, message, and log file path where the root cause was identified.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    /// <p>The execution state of the cluster step.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason for the step execution status change.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<StepStateChangeReason>,
    /// <p>The timeline of the cluster step status over time.</p>
    #[serde(rename = "Timeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<StepTimeline>,
}

/// <p>The summary of the cluster step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepSummary {
    /// <p>The action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE. TERMINATE_JOB_FLOW is available for backward compatibility. We recommend using TERMINATE_CLUSTER instead.</p>
    #[serde(rename = "ActionOnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_failure: Option<String>,
    /// <p>The Hadoop job configuration of the cluster step.</p>
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    /// <p>The identifier of the cluster step.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the cluster step.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current execution status details of the cluster step.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StepStatus>,
}

/// <p>The timeline of the cluster step lifecycle.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StepTimeline {
    /// <p>The date and time when the cluster step was created.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time when the cluster step execution completed or failed.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The date and time when the cluster step execution started.</p>
    #[serde(rename = "StartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
}

/// <p>The list of supported product configurations which allow user-supplied arguments. EMR accepts these arguments and forwards them to the corresponding installation script as bootstrap action arguments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SupportedProductConfig {
    /// <p>The list of user-supplied arguments.</p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// <p>The name of the product configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A key/value pair containing user-defined metadata that you can associate with an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A user-defined key, which is the minimum required information for a valid tag. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag </a>. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A user-defined value, which is optional in a tag. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p> Input to the <a>TerminateJobFlows</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateJobFlowsInput {
    /// <p>A list of job flows to be shutdown.</p>
    #[serde(rename = "JobFlowIds")]
    pub job_flow_ids: Vec<String>,
}

/// <p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeSpecification {
    /// <p>The number of I/O operations per second (IOPS) that the volume supports.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume type is EBS-optimized, the minimum value is 10.</p>
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: i64,
    /// <p>The volume type. Volume types supported are gp2, io1, standard.</p>
    #[serde(rename = "VolumeType")]
    pub volume_type: String,
}

/// Errors returned by AddInstanceFleet
#[derive(Debug, PartialEq)]
pub enum AddInstanceFleetError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl AddInstanceFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddInstanceFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(AddInstanceFleetError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddInstanceFleetError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddInstanceFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddInstanceFleetError::InternalServer(ref cause) => write!(f, "{}", cause),
            AddInstanceFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddInstanceFleetError {}
/// Errors returned by AddInstanceGroups
#[derive(Debug, PartialEq)]
pub enum AddInstanceGroupsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl AddInstanceGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddInstanceGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddInstanceGroupsError::InternalServerError(
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
impl fmt::Display for AddInstanceGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddInstanceGroupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddInstanceGroupsError {}
/// Errors returned by AddJobFlowSteps
#[derive(Debug, PartialEq)]
pub enum AddJobFlowStepsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl AddJobFlowStepsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddJobFlowStepsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddJobFlowStepsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddJobFlowStepsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddJobFlowStepsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddJobFlowStepsError {}
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(AddTagsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddTagsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::InternalServer(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CancelSteps
#[derive(Debug, PartialEq)]
pub enum CancelStepsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl CancelStepsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelStepsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CancelStepsError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CancelStepsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelStepsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelStepsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelStepsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelStepsError {}
/// Errors returned by CreateSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateSecurityConfigurationError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl CreateSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InvalidRequest(
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
impl fmt::Display for CreateSecurityConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSecurityConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateSecurityConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSecurityConfigurationError {}
/// Errors returned by DeleteSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteSecurityConfigurationError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl DeleteSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InvalidRequest(
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
impl fmt::Display for DeleteSecurityConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSecurityConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteSecurityConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSecurityConfigurationError {}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl DescribeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeClusterError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeClusterError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClusterError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClusterError {}
/// Errors returned by DescribeJobFlows
#[derive(Debug, PartialEq)]
pub enum DescribeJobFlowsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl DescribeJobFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobFlowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeJobFlowsError::InternalServerError(
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
impl fmt::Display for DescribeJobFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobFlowsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobFlowsError {}
/// Errors returned by DescribeSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeSecurityConfigurationError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl DescribeSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeSecurityConfigurationError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeSecurityConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSecurityConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSecurityConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeSecurityConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSecurityConfigurationError {}
/// Errors returned by DescribeStep
#[derive(Debug, PartialEq)]
pub enum DescribeStepError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl DescribeStepError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStepError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeStepError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeStepError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStepError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStepError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeStepError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStepError {}
/// Errors returned by GetBlockPublicAccessConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBlockPublicAccessConfigurationError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl GetBlockPublicAccessConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBlockPublicAccessConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetBlockPublicAccessConfigurationError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetBlockPublicAccessConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBlockPublicAccessConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlockPublicAccessConfigurationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetBlockPublicAccessConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetBlockPublicAccessConfigurationError {}
/// Errors returned by ListBootstrapActions
#[derive(Debug, PartialEq)]
pub enum ListBootstrapActionsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListBootstrapActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBootstrapActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListBootstrapActionsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListBootstrapActionsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBootstrapActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBootstrapActionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListBootstrapActionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBootstrapActionsError {}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListClustersError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListClustersError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClustersError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListClustersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListClustersError {}
/// Errors returned by ListInstanceFleets
#[derive(Debug, PartialEq)]
pub enum ListInstanceFleetsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListInstanceFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstanceFleetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListInstanceFleetsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstanceFleetsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstanceFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstanceFleetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListInstanceFleetsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstanceFleetsError {}
/// Errors returned by ListInstanceGroups
#[derive(Debug, PartialEq)]
pub enum ListInstanceGroupsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListInstanceGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstanceGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListInstanceGroupsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstanceGroupsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstanceGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstanceGroupsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListInstanceGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstanceGroupsError {}
/// Errors returned by ListInstances
#[derive(Debug, PartialEq)]
pub enum ListInstancesError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListInstancesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInstancesError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstancesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListInstancesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstancesError {}
/// Errors returned by ListSecurityConfigurations
#[derive(Debug, PartialEq)]
pub enum ListSecurityConfigurationsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListSecurityConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSecurityConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListSecurityConfigurationsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSecurityConfigurationsError::InvalidRequest(
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
impl fmt::Display for ListSecurityConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSecurityConfigurationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSecurityConfigurationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSecurityConfigurationsError {}
/// Errors returned by ListSteps
#[derive(Debug, PartialEq)]
pub enum ListStepsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ListStepsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStepsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListStepsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListStepsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListStepsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStepsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListStepsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStepsError {}
/// Errors returned by ModifyCluster
#[derive(Debug, PartialEq)]
pub enum ModifyClusterError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ModifyClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ModifyClusterError::InternalServerError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ModifyClusterError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyClusterError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ModifyClusterError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyClusterError {}
/// Errors returned by ModifyInstanceFleet
#[derive(Debug, PartialEq)]
pub enum ModifyInstanceFleetError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl ModifyInstanceFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyInstanceFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ModifyInstanceFleetError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ModifyInstanceFleetError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyInstanceFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyInstanceFleetError::InternalServer(ref cause) => write!(f, "{}", cause),
            ModifyInstanceFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyInstanceFleetError {}
/// Errors returned by ModifyInstanceGroups
#[derive(Debug, PartialEq)]
pub enum ModifyInstanceGroupsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl ModifyInstanceGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyInstanceGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ModifyInstanceGroupsError::InternalServerError(
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
impl fmt::Display for ModifyInstanceGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyInstanceGroupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyInstanceGroupsError {}
/// Errors returned by PutAutoScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutAutoScalingPolicyError {}

impl PutAutoScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAutoScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAutoScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for PutAutoScalingPolicyError {}
/// Errors returned by PutBlockPublicAccessConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBlockPublicAccessConfigurationError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl PutBlockPublicAccessConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBlockPublicAccessConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        PutBlockPublicAccessConfigurationError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        PutBlockPublicAccessConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutBlockPublicAccessConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutBlockPublicAccessConfigurationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            PutBlockPublicAccessConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutBlockPublicAccessConfigurationError {}
/// Errors returned by RemoveAutoScalingPolicy
#[derive(Debug, PartialEq)]
pub enum RemoveAutoScalingPolicyError {}

impl RemoveAutoScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveAutoScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveAutoScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for RemoveAutoScalingPolicyError {}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    /// <p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(RemoveTagsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RemoveTagsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::InternalServer(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
/// Errors returned by RunJobFlow
#[derive(Debug, PartialEq)]
pub enum RunJobFlowError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl RunJobFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RunJobFlowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(RunJobFlowError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RunJobFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunJobFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RunJobFlowError {}
/// Errors returned by SetTerminationProtection
#[derive(Debug, PartialEq)]
pub enum SetTerminationProtectionError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl SetTerminationProtectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetTerminationProtectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        SetTerminationProtectionError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetTerminationProtectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetTerminationProtectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetTerminationProtectionError {}
/// Errors returned by SetVisibleToAllUsers
#[derive(Debug, PartialEq)]
pub enum SetVisibleToAllUsersError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl SetVisibleToAllUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetVisibleToAllUsersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(SetVisibleToAllUsersError::InternalServerError(
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
impl fmt::Display for SetVisibleToAllUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetVisibleToAllUsersError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetVisibleToAllUsersError {}
/// Errors returned by TerminateJobFlows
#[derive(Debug, PartialEq)]
pub enum TerminateJobFlowsError {
    /// <p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
}

impl TerminateJobFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateJobFlowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(TerminateJobFlowsError::InternalServerError(
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
impl fmt::Display for TerminateJobFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateJobFlowsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TerminateJobFlowsError {}
/// Trait representing the capabilities of the Amazon EMR API. Amazon EMR clients implement this trait.
#[async_trait]
pub trait Emr {
    /// <p><p>Adds an instance fleet to a running cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x.</p> </note></p>
    async fn add_instance_fleet(
        &self,
        input: AddInstanceFleetInput,
    ) -> Result<AddInstanceFleetOutput, RusotoError<AddInstanceFleetError>>;

    /// <p>Adds one or more instance groups to a running cluster.</p>
    async fn add_instance_groups(
        &self,
        input: AddInstanceGroupsInput,
    ) -> Result<AddInstanceGroupsOutput, RusotoError<AddInstanceGroupsError>>;

    /// <p>AddJobFlowSteps adds new steps to a running cluster. A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using SSH to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/AddMoreThan256Steps.html">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the cluster or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a cluster that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>
    async fn add_job_flow_steps(
        &self,
        input: AddJobFlowStepsInput,
    ) -> Result<AddJobFlowStepsOutput, RusotoError<AddJobFlowStepsError>>;

    /// <p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>
    async fn cancel_steps(
        &self,
        input: CancelStepsInput,
    ) -> Result<CancelStepsOutput, RusotoError<CancelStepsError>>;

    /// <p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>
    async fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationInput,
    ) -> Result<CreateSecurityConfigurationOutput, RusotoError<CreateSecurityConfigurationError>>;

    /// <p>Deletes a security configuration.</p>
    async fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationInput,
    ) -> Result<DeleteSecurityConfigurationOutput, RusotoError<DeleteSecurityConfigurationError>>;

    /// <p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. </p>
    async fn describe_cluster(
        &self,
        input: DescribeClusterInput,
    ) -> Result<DescribeClusterOutput, RusotoError<DescribeClusterError>>;

    /// <p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>
    async fn describe_job_flows(
        &self,
        input: DescribeJobFlowsInput,
    ) -> Result<DescribeJobFlowsOutput, RusotoError<DescribeJobFlowsError>>;

    /// <p>Provides the details of a security configuration by returning the configuration JSON.</p>
    async fn describe_security_configuration(
        &self,
        input: DescribeSecurityConfigurationInput,
    ) -> Result<DescribeSecurityConfigurationOutput, RusotoError<DescribeSecurityConfigurationError>>;

    /// <p>Provides more detail about the cluster step.</p>
    async fn describe_step(
        &self,
        input: DescribeStepInput,
    ) -> Result<DescribeStepOutput, RusotoError<DescribeStepError>>;

    /// <p>Returns the Amazon EMR block public access configuration for your AWS account in the current Region. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/configure-block-public-access.html">Configure Block Public Access for Amazon EMR</a> in the <i>Amazon EMR Management Guide</i>.</p>
    async fn get_block_public_access_configuration(
        &self,
    ) -> Result<
        GetBlockPublicAccessConfigurationOutput,
        RusotoError<GetBlockPublicAccessConfigurationError>,
    >;

    /// <p>Provides information about the bootstrap actions associated with a cluster.</p>
    async fn list_bootstrap_actions(
        &self,
        input: ListBootstrapActionsInput,
    ) -> Result<ListBootstrapActionsOutput, RusotoError<ListBootstrapActionsError>>;

    /// <p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>
    async fn list_clusters(
        &self,
        input: ListClustersInput,
    ) -> Result<ListClustersOutput, RusotoError<ListClustersError>>;

    /// <p><p>Lists all available details about the instance fleets in a cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    async fn list_instance_fleets(
        &self,
        input: ListInstanceFleetsInput,
    ) -> Result<ListInstanceFleetsOutput, RusotoError<ListInstanceFleetsError>>;

    /// <p>Provides all available details about the instance groups in a cluster.</p>
    async fn list_instance_groups(
        &self,
        input: ListInstanceGroupsInput,
    ) -> Result<ListInstanceGroupsOutput, RusotoError<ListInstanceGroupsError>>;

    /// <p>Provides information for all active EC2 instances and EC2 instances terminated in the last 30 days, up to a maximum of 2,000. EC2 instances in any of the following states are considered active: AWAITING_FULFILLMENT, PROVISIONING, BOOTSTRAPPING, RUNNING.</p>
    async fn list_instances(
        &self,
        input: ListInstancesInput,
    ) -> Result<ListInstancesOutput, RusotoError<ListInstancesError>>;

    /// <p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>
    async fn list_security_configurations(
        &self,
        input: ListSecurityConfigurationsInput,
    ) -> Result<ListSecurityConfigurationsOutput, RusotoError<ListSecurityConfigurationsError>>;

    /// <p>Provides a list of steps for the cluster in reverse order unless you specify <code>stepIds</code> with the request of filter by <code>StepStates</code>. You can specify a maximum of ten <code>stepIDs</code>.</p>
    async fn list_steps(
        &self,
        input: ListStepsInput,
    ) -> Result<ListStepsOutput, RusotoError<ListStepsError>>;

    /// <p>Modifies the number of steps that can be executed concurrently for the cluster specified using ClusterID.</p>
    async fn modify_cluster(
        &self,
        input: ModifyClusterInput,
    ) -> Result<ModifyClusterOutput, RusotoError<ModifyClusterError>>;

    /// <p><p>Modifies the target On-Demand and target Spot capacities for the instance fleet with the specified InstanceFleetID within the cluster specified using ClusterID. The call either succeeds or fails atomically.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    async fn modify_instance_fleet(
        &self,
        input: ModifyInstanceFleetInput,
    ) -> Result<(), RusotoError<ModifyInstanceFleetError>>;

    /// <p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>
    async fn modify_instance_groups(
        &self,
        input: ModifyInstanceGroupsInput,
    ) -> Result<(), RusotoError<ModifyInstanceGroupsError>>;

    /// <p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>
    async fn put_auto_scaling_policy(
        &self,
        input: PutAutoScalingPolicyInput,
    ) -> Result<PutAutoScalingPolicyOutput, RusotoError<PutAutoScalingPolicyError>>;

    /// <p>Creates or updates an Amazon EMR block public access configuration for your AWS account in the current Region. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/configure-block-public-access.html">Configure Block Public Access for Amazon EMR</a> in the <i>Amazon EMR Management Guide</i>.</p>
    async fn put_block_public_access_configuration(
        &self,
        input: PutBlockPublicAccessConfigurationInput,
    ) -> Result<
        PutBlockPublicAccessConfigurationOutput,
        RusotoError<PutBlockPublicAccessConfigurationError>,
    >;

    /// <p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>
    async fn remove_auto_scaling_policy(
        &self,
        input: RemoveAutoScalingPolicyInput,
    ) -> Result<RemoveAutoScalingPolicyOutput, RusotoError<RemoveAutoScalingPolicyError>>;

    /// <p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>>;

    /// <p><p>RunJobFlow creates and starts running a new cluster (job flow). The cluster runs the steps specified. After the steps complete, the cluster stops and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the cluster transitions to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the cluster and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/AddMoreThan256Steps.html">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running clusters, we recommend that you periodically store your results.</p> <note> <p>The instance fleets configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions. The RunJobFlow request can contain InstanceFleets parameters or InstanceGroups parameters, but not both.</p> </note></p>
    async fn run_job_flow(
        &self,
        input: RunJobFlowInput,
    ) -> Result<RunJobFlowOutput, RusotoError<RunJobFlowError>>;

    /// <p>SetTerminationProtection locks a cluster (job flow) so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling <code>SetTerminationProtection</code> on a cluster is similar to calling the Amazon EC2 <code>DisableAPITermination</code> API on all EC2 instances in a cluster.</p> <p> <code>SetTerminationProtection</code> is used to prevent accidental termination of a cluster and to ensure that in the event of an error, the instances persist so that you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a cluster that has been locked by setting <code>SetTerminationProtection</code> to <code>true</code>, you must first unlock the job flow by a subsequent call to <code>SetTerminationProtection</code> in which you set the value to <code>false</code>. </p> <p> For more information, see<a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/UsingEMR_TerminationProtection.html">Managing Cluster Termination</a> in the <i>Amazon EMR Management Guide</i>. </p>
    async fn set_termination_protection(
        &self,
        input: SetTerminationProtectionInput,
    ) -> Result<(), RusotoError<SetTerminationProtectionError>>;

    /// <p>Sets the <a>Cluster$VisibleToAllUsers</a> value, which determines whether the cluster is visible to all IAM users of the AWS account associated with the cluster. Only the IAM user who created the cluster or the AWS account root user can call this action. The default value, <code>true</code>, indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. If set to <code>false</code>, only the IAM user that created the cluster can perform actions. This action works on running clusters. You can override the default <code>true</code> setting when you create a cluster by using the <code>VisibleToAllUsers</code> parameter with <code>RunJobFlow</code>.</p>
    async fn set_visible_to_all_users(
        &self,
        input: SetVisibleToAllUsersInput,
    ) -> Result<(), RusotoError<SetVisibleToAllUsersError>>;

    /// <p>TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the cluster is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the cluster was created.</p> <p>The maximum number of clusters allowed is 10. The call to <code>TerminateJobFlows</code> is asynchronous. Depending on the configuration of the cluster, it may take up to 1-5 minutes for the cluster to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>
    async fn terminate_job_flows(
        &self,
        input: TerminateJobFlowsInput,
    ) -> Result<(), RusotoError<TerminateJobFlowsError>>;
}
/// A client for the Amazon EMR API.
#[derive(Clone)]
pub struct EmrClient {
    client: Client,
    region: region::Region,
}

impl EmrClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EmrClient {
        EmrClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EmrClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EmrClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EmrClient {
        EmrClient { client, region }
    }
}

#[async_trait]
impl Emr for EmrClient {
    /// <p><p>Adds an instance fleet to a running cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x.</p> </note></p>
    async fn add_instance_fleet(
        &self,
        input: AddInstanceFleetInput,
    ) -> Result<AddInstanceFleetOutput, RusotoError<AddInstanceFleetError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddInstanceFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AddInstanceFleetOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddInstanceFleetError::from_response(response))
        }
    }

    /// <p>Adds one or more instance groups to a running cluster.</p>
    async fn add_instance_groups(
        &self,
        input: AddInstanceGroupsInput,
    ) -> Result<AddInstanceGroupsOutput, RusotoError<AddInstanceGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddInstanceGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AddInstanceGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddInstanceGroupsError::from_response(response))
        }
    }

    /// <p>AddJobFlowSteps adds new steps to a running cluster. A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using SSH to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/AddMoreThan256Steps.html">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the cluster or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a cluster that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>
    async fn add_job_flow_steps(
        &self,
        input: AddJobFlowStepsInput,
    ) -> Result<AddJobFlowStepsOutput, RusotoError<AddJobFlowStepsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddJobFlowSteps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AddJobFlowStepsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddJobFlowStepsError::from_response(response))
        }
    }

    /// <p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AddTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsError::from_response(response))
        }
    }

    /// <p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>
    async fn cancel_steps(
        &self,
        input: CancelStepsInput,
    ) -> Result<CancelStepsOutput, RusotoError<CancelStepsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.CancelSteps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CancelStepsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CancelStepsError::from_response(response))
        }
    }

    /// <p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>
    async fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationInput,
    ) -> Result<CreateSecurityConfigurationOutput, RusotoError<CreateSecurityConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.CreateSecurityConfiguration",
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
                .deserialize::<CreateSecurityConfigurationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSecurityConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes a security configuration.</p>
    async fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationInput,
    ) -> Result<DeleteSecurityConfigurationOutput, RusotoError<DeleteSecurityConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.DeleteSecurityConfiguration",
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
                .deserialize::<DeleteSecurityConfigurationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSecurityConfigurationError::from_response(response))
        }
    }

    /// <p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. </p>
    async fn describe_cluster(
        &self,
        input: DescribeClusterInput,
    ) -> Result<DescribeClusterOutput, RusotoError<DescribeClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeClusterOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClusterError::from_response(response))
        }
    }

    /// <p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>
    async fn describe_job_flows(
        &self,
        input: DescribeJobFlowsInput,
    ) -> Result<DescribeJobFlowsOutput, RusotoError<DescribeJobFlowsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeJobFlows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeJobFlowsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobFlowsError::from_response(response))
        }
    }

    /// <p>Provides the details of a security configuration by returning the configuration JSON.</p>
    async fn describe_security_configuration(
        &self,
        input: DescribeSecurityConfigurationInput,
    ) -> Result<DescribeSecurityConfigurationOutput, RusotoError<DescribeSecurityConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.DescribeSecurityConfiguration",
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
                .deserialize::<DescribeSecurityConfigurationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSecurityConfigurationError::from_response(response))
        }
    }

    /// <p>Provides more detail about the cluster step.</p>
    async fn describe_step(
        &self,
        input: DescribeStepInput,
    ) -> Result<DescribeStepOutput, RusotoError<DescribeStepError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeStep");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeStepOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStepError::from_response(response))
        }
    }

    /// <p>Returns the Amazon EMR block public access configuration for your AWS account in the current Region. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/configure-block-public-access.html">Configure Block Public Access for Amazon EMR</a> in the <i>Amazon EMR Management Guide</i>.</p>
    async fn get_block_public_access_configuration(
        &self,
    ) -> Result<
        GetBlockPublicAccessConfigurationOutput,
        RusotoError<GetBlockPublicAccessConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.GetBlockPublicAccessConfiguration",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBlockPublicAccessConfigurationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlockPublicAccessConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Provides information about the bootstrap actions associated with a cluster.</p>
    async fn list_bootstrap_actions(
        &self,
        input: ListBootstrapActionsInput,
    ) -> Result<ListBootstrapActionsOutput, RusotoError<ListBootstrapActionsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListBootstrapActions");
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
                .deserialize::<ListBootstrapActionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListBootstrapActionsError::from_response(response))
        }
    }

    /// <p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>
    async fn list_clusters(
        &self,
        input: ListClustersInput,
    ) -> Result<ListClustersOutput, RusotoError<ListClustersError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListClustersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListClustersError::from_response(response))
        }
    }

    /// <p><p>Lists all available details about the instance fleets in a cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    async fn list_instance_fleets(
        &self,
        input: ListInstanceFleetsInput,
    ) -> Result<ListInstanceFleetsOutput, RusotoError<ListInstanceFleetsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstanceFleets");
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
                .deserialize::<ListInstanceFleetsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstanceFleetsError::from_response(response))
        }
    }

    /// <p>Provides all available details about the instance groups in a cluster.</p>
    async fn list_instance_groups(
        &self,
        input: ListInstanceGroupsInput,
    ) -> Result<ListInstanceGroupsOutput, RusotoError<ListInstanceGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstanceGroups");
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
                .deserialize::<ListInstanceGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstanceGroupsError::from_response(response))
        }
    }

    /// <p>Provides information for all active EC2 instances and EC2 instances terminated in the last 30 days, up to a maximum of 2,000. EC2 instances in any of the following states are considered active: AWAITING_FULFILLMENT, PROVISIONING, BOOTSTRAPPING, RUNNING.</p>
    async fn list_instances(
        &self,
        input: ListInstancesInput,
    ) -> Result<ListInstancesOutput, RusotoError<ListInstancesError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListInstancesError::from_response(response))
        }
    }

    /// <p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>
    async fn list_security_configurations(
        &self,
        input: ListSecurityConfigurationsInput,
    ) -> Result<ListSecurityConfigurationsOutput, RusotoError<ListSecurityConfigurationsError>>
    {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.ListSecurityConfigurations",
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
                .deserialize::<ListSecurityConfigurationsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSecurityConfigurationsError::from_response(response))
        }
    }

    /// <p>Provides a list of steps for the cluster in reverse order unless you specify <code>stepIds</code> with the request of filter by <code>StepStates</code>. You can specify a maximum of ten <code>stepIDs</code>.</p>
    async fn list_steps(
        &self,
        input: ListStepsInput,
    ) -> Result<ListStepsOutput, RusotoError<ListStepsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListSteps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListStepsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListStepsError::from_response(response))
        }
    }

    /// <p>Modifies the number of steps that can be executed concurrently for the cluster specified using ClusterID.</p>
    async fn modify_cluster(
        &self,
        input: ModifyClusterInput,
    ) -> Result<ModifyClusterOutput, RusotoError<ModifyClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ModifyCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ModifyClusterOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyClusterError::from_response(response))
        }
    }

    /// <p><p>Modifies the target On-Demand and target Spot capacities for the instance fleet with the specified InstanceFleetID within the cluster specified using ClusterID. The call either succeeds or fails atomically.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note></p>
    async fn modify_instance_fleet(
        &self,
        input: ModifyInstanceFleetInput,
    ) -> Result<(), RusotoError<ModifyInstanceFleetError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ModifyInstanceFleet");
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
            Err(ModifyInstanceFleetError::from_response(response))
        }
    }

    /// <p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>
    async fn modify_instance_groups(
        &self,
        input: ModifyInstanceGroupsInput,
    ) -> Result<(), RusotoError<ModifyInstanceGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ModifyInstanceGroups");
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
            Err(ModifyInstanceGroupsError::from_response(response))
        }
    }

    /// <p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>
    async fn put_auto_scaling_policy(
        &self,
        input: PutAutoScalingPolicyInput,
    ) -> Result<PutAutoScalingPolicyOutput, RusotoError<PutAutoScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.PutAutoScalingPolicy");
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
                .deserialize::<PutAutoScalingPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutAutoScalingPolicyError::from_response(response))
        }
    }

    /// <p>Creates or updates an Amazon EMR block public access configuration for your AWS account in the current Region. For more information see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/configure-block-public-access.html">Configure Block Public Access for Amazon EMR</a> in the <i>Amazon EMR Management Guide</i>.</p>
    async fn put_block_public_access_configuration(
        &self,
        input: PutBlockPublicAccessConfigurationInput,
    ) -> Result<
        PutBlockPublicAccessConfigurationOutput,
        RusotoError<PutBlockPublicAccessConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ElasticMapReduce.PutBlockPublicAccessConfiguration",
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
                .deserialize::<PutBlockPublicAccessConfigurationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutBlockPublicAccessConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>
    async fn remove_auto_scaling_policy(
        &self,
        input: RemoveAutoScalingPolicyInput,
    ) -> Result<RemoveAutoScalingPolicyOutput, RusotoError<RemoveAutoScalingPolicyError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RemoveAutoScalingPolicy");
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
                .deserialize::<RemoveAutoScalingPolicyOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveAutoScalingPolicyError::from_response(response))
        }
    }

    /// <p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-tags.html">Tag Clusters</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RemoveTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RemoveTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsError::from_response(response))
        }
    }

    /// <p><p>RunJobFlow creates and starts running a new cluster (job flow). The cluster runs the steps specified. After the steps complete, the cluster stops and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the cluster transitions to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the cluster and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/AddMoreThan256Steps.html">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running clusters, we recommend that you periodically store your results.</p> <note> <p>The instance fleets configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions. The RunJobFlow request can contain InstanceFleets parameters or InstanceGroups parameters, but not both.</p> </note></p>
    async fn run_job_flow(
        &self,
        input: RunJobFlowInput,
    ) -> Result<RunJobFlowOutput, RusotoError<RunJobFlowError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RunJobFlow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RunJobFlowOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RunJobFlowError::from_response(response))
        }
    }

    /// <p>SetTerminationProtection locks a cluster (job flow) so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling <code>SetTerminationProtection</code> on a cluster is similar to calling the Amazon EC2 <code>DisableAPITermination</code> API on all EC2 instances in a cluster.</p> <p> <code>SetTerminationProtection</code> is used to prevent accidental termination of a cluster and to ensure that in the event of an error, the instances persist so that you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a cluster that has been locked by setting <code>SetTerminationProtection</code> to <code>true</code>, you must first unlock the job flow by a subsequent call to <code>SetTerminationProtection</code> in which you set the value to <code>false</code>. </p> <p> For more information, see<a href="https://docs.aws.amazon.com/emr/latest/ManagementGuide/UsingEMR_TerminationProtection.html">Managing Cluster Termination</a> in the <i>Amazon EMR Management Guide</i>. </p>
    async fn set_termination_protection(
        &self,
        input: SetTerminationProtectionInput,
    ) -> Result<(), RusotoError<SetTerminationProtectionError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.SetTerminationProtection");
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
            Err(SetTerminationProtectionError::from_response(response))
        }
    }

    /// <p>Sets the <a>Cluster$VisibleToAllUsers</a> value, which determines whether the cluster is visible to all IAM users of the AWS account associated with the cluster. Only the IAM user who created the cluster or the AWS account root user can call this action. The default value, <code>true</code>, indicates that all IAM users in the AWS account can perform cluster actions if they have the proper IAM policy permissions. If set to <code>false</code>, only the IAM user that created the cluster can perform actions. This action works on running clusters. You can override the default <code>true</code> setting when you create a cluster by using the <code>VisibleToAllUsers</code> parameter with <code>RunJobFlow</code>.</p>
    async fn set_visible_to_all_users(
        &self,
        input: SetVisibleToAllUsersInput,
    ) -> Result<(), RusotoError<SetVisibleToAllUsersError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.SetVisibleToAllUsers");
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
            Err(SetVisibleToAllUsersError::from_response(response))
        }
    }

    /// <p>TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the cluster is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the cluster was created.</p> <p>The maximum number of clusters allowed is 10. The call to <code>TerminateJobFlows</code> is asynchronous. Depending on the configuration of the cluster, it may take up to 1-5 minutes for the cluster to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>
    async fn terminate_job_flows(
        &self,
        input: TerminateJobFlowsInput,
    ) -> Result<(), RusotoError<TerminateJobFlowsError>> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.TerminateJobFlows");
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
            Err(TerminateJobFlowsError::from_response(response))
        }
    }
}
