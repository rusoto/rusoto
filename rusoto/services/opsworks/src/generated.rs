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
/// <p>Describes an agent version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentVersion {
    /// <p>The configuration manager.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>The agent version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>A description of the app.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct App {
    /// <p>The app ID.</p>
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>A <code>Source</code> object that describes the app repository.</p>
    #[serde(rename = "AppSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_source: Option<Source>,
    /// <p>The stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>When the app was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The app's data sources.</p>
    #[serde(rename = "DataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>A description of the app.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The app vhost settings with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// <p>Whether to enable SSL for the app.</p>
    #[serde(rename = "EnableSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<bool>,
    /// <p><p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>. </p> <note> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variable names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases, but if you do exceed it, you will cause an exception (API) with an &quot;Environment: is too large (maximum is 20 KB)&quot; message.</p> </note></p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    /// <p>The app name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The app's short name.</p>
    #[serde(rename = "Shortname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    #[serde(rename = "SslConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    /// <p>The app stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The app type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssignInstanceRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The layer ID, which must correspond to a custom layer. You cannot assign a registered instance to a built-in layer.</p>
    #[serde(rename = "LayerIds")]
    pub layer_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssignVolumeRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The volume ID.</p>
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateElasticIpRequest {
    /// <p>The Elastic IP address.</p>
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: String,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachElasticLoadBalancerRequest {
    /// <p>The Elastic Load Balancing instance's name.</p>
    #[serde(rename = "ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    /// <p>The ID of the layer to which the Elastic Load Balancing instance is to be attached.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
}

/// <p>Describes a load-based auto scaling upscaling or downscaling threshold configuration, which specifies when AWS OpsWorks Stacks starts or stops load-based instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingThresholds {
    /// <p><p>Custom Cloudwatch auto scaling alarms, to be used as thresholds. This parameter takes a list of up to five alarm names, which are case sensitive and must be in the same region as the stack.</p> <note> <p>To use custom alarms, you must update your service role to allow <code>cloudwatch:DescribeAlarms</code>. You can either have AWS OpsWorks Stacks update the role for you when you first use this feature or you can edit the role manually. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-servicerole.html">Allowing AWS OpsWorks Stacks to Act on Your Behalf</a>.</p> </note></p>
    #[serde(rename = "Alarms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<String>>,
    /// <p>The CPU utilization threshold, as a percent of the available CPU. A value of -1 disables the threshold.</p>
    #[serde(rename = "CpuThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_threshold: Option<f64>,
    /// <p>The amount of time (in minutes) after a scaling event occurs that AWS OpsWorks Stacks should ignore metrics and suppress additional scaling events. For example, AWS OpsWorks Stacks adds new instances following an upscaling event but the instances won't start reducing the load until they have been booted and configured. There is no point in raising additional scaling events during that operation, which typically takes several minutes. <code>IgnoreMetricsTime</code> allows you to direct AWS OpsWorks Stacks to suppress scaling events long enough to get the new instances online.</p>
    #[serde(rename = "IgnoreMetricsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_metrics_time: Option<i64>,
    /// <p>The number of instances to add or remove when the load exceeds a threshold.</p>
    #[serde(rename = "InstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The load threshold. A value of -1 disables the threshold. For more information about how load is computed, see <a href="http://en.wikipedia.org/wiki/Load_%28computing%29">Load (computing)</a>.</p>
    #[serde(rename = "LoadThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_threshold: Option<f64>,
    /// <p>The memory utilization threshold, as a percent of the available memory. A value of -1 disables the threshold.</p>
    #[serde(rename = "MemoryThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_threshold: Option<f64>,
    /// <p>The amount of time, in minutes, that the load must exceed a threshold before more instances are added or removed.</p>
    #[serde(rename = "ThresholdsWaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds_wait_time: Option<i64>,
}

/// <p>Describes a block device mapping. This data type maps directly to the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_BlockDeviceMapping.html">BlockDeviceMapping</a> data type. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDeviceMapping {
    /// <p>The device name that is exposed to the instance, such as <code>/dev/sdh</code>. For the root device, you can use the explicit device name or you can set this parameter to <code>ROOT_DEVICE</code> and AWS OpsWorks Stacks will provide the correct device name.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>An <code>EBSBlockDevice</code> that defines how to configure an Amazon EBS volume when the instance is launched.</p>
    #[serde(rename = "Ebs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsBlockDevice>,
    /// <p>Suppresses the specified device included in the AMI's block device mapping.</p>
    #[serde(rename = "NoDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<String>,
    /// <p>The virtual device name. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_BlockDeviceMapping.html">BlockDeviceMapping</a>.</p>
    #[serde(rename = "VirtualName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

/// <p>Describes the Chef configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChefConfiguration {
    /// <p>The Berkshelf version.</p>
    #[serde(rename = "BerkshelfVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub berkshelf_version: Option<String>,
    /// <p>Whether to enable Berkshelf.</p>
    #[serde(rename = "ManageBerkshelf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_berkshelf: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloneStackRequest {
    /// <p><p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack&#39;s instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack&#39;s instances.</p> </li> </ul> <p>The default setting is <code>LATEST</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack&#39;s default setting.</p> </note></p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>A list of stack attributes and values as key/value pairs to be added to the cloned stack.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "ChefConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    /// <p>A list of source stack app IDs to be included in the cloned stack.</p>
    #[serde(rename = "CloneAppIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_app_ids: Option<Vec<String>>,
    /// <p>Whether to clone the source stack's permissions.</p>
    #[serde(rename = "ClonePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_permissions: Option<bool>,
    /// <p>The configuration manager. When you clone a stack we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 12.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Adding Apps</a> or <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html">Cookbooks and Recipes</a>.</p>
    #[serde(rename = "CustomCookbooksSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    /// <p>A string that contains user-defined, custom JSON. It is used to override the corresponding default stack configuration JSON values. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> </p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The cloned stack's default Availability Zone, which must be in the specified region. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see the <code>VpcId</code> parameter description. </p>
    #[serde(rename = "DefaultAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    /// <p><p>The stack&#39;s operating system, which must be set to one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2018.03</code>, <code>Amazon Linux 2017.09</code>, <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p> <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information about how to use custom AMIs with OpsWorks, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the parent stack&#39;s operating system. For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">AWS OpsWorks Stacks Operating Systems</a>.</p> <note> <p>You can specify a different Linux operating system for the cloned stack, but you cannot change from Linux to Windows or Windows to Linux.</p> </note></p>
    #[serde(rename = "DefaultOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_os: Option<String>,
    /// <p>The default root device type. This value is used by default for all instances in the cloned stack, but you can override it when you create an instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "DefaultRootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<String>,
    /// <p>A default Amazon EC2 key pair name. The default value is none. If you specify a key pair name, AWS OpsWorks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html"> Using SSH to Communicate with an Instance</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html"> create an instance</a>. </p>
    #[serde(rename = "DefaultSshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    /// <p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>
    #[serde(rename = "DefaultSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<String>,
    /// <p>The stack's host name theme, with spaces are replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities_and_Titans</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>
    #[serde(rename = "HostnameTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<String>,
    /// <p>The cloned stack name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The cloned stack AWS region, such as "ap-northeast-2". For more information about AWS regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p><p>The stack AWS Identity and Access Management (IAM) role, which allows AWS OpsWorks Stacks to work with AWS resources on your behalf. You must set this parameter to the Amazon Resource Name (ARN) for an existing IAM role. If you create a stack by using the AWS OpsWorks Stacks console, it creates the role for you. You can obtain an existing stack&#39;s IAM ARN programmatically by calling <a>DescribePermissions</a>. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p> <note> <p>You must set this parameter to a valid service role ARN or the action will fail; there is no default value. You can specify the source stack&#39;s service role ARN, if you prefer, but you must do so explicitly.</p> </note></p>
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// <p>The source stack ID.</p>
    #[serde(rename = "SourceStackId")]
    pub source_stack_id: String,
    /// <p>Whether to use custom cookbooks.</p>
    #[serde(rename = "UseCustomCookbooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    /// <p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. With <code>UseOpsworksSecurityGroups</code> you can instead provide your own custom security groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate Amazon Elastic Compute Cloud (Amazon EC2) security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on creation; custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    /// <p>The ID of the VPC that the cloned stack is to be launched into. It must be in the specified region. All instances are launched into this VPC, and you cannot change the ID later.</p> <ul> <li> <p>If your account supports EC2 Classic, the default value is no VPC.</p> </li> <li> <p>If your account does not support EC2 Classic, the default value is the default VPC for the specified region.</p> </li> </ul> <p>If the VPC ID corresponds to a default VPC and you have specified either the <code>DefaultAvailabilityZone</code> or the <code>DefaultSubnetId</code> parameter only, AWS OpsWorks Stacks infers the value of the other parameter. If you specify neither parameter, AWS OpsWorks Stacks sets these parameters to the first valid Availability Zone for the specified region and the corresponding default VPC subnet ID, respectively. </p> <p>If you specify a nondefault VPC ID, note the following:</p> <ul> <li> <p>It must belong to a VPC in your account that is in the specified region.</p> </li> <li> <p>You must specify a value for <code>DefaultSubnetId</code>.</p> </li> </ul> <p>For more information about how to use AWS OpsWorks Stacks with a VPC, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-vpc.html">Running a Stack in a VPC</a>. For more information about default VPC and EC2 Classic, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html">Supported Platforms</a>. </p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains the response to a <code>CloneStack</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloneStackResult {
    /// <p>The cloned stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Describes the Amazon CloudWatch logs configuration for a layer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLogsConfiguration {
    /// <p>Whether CloudWatch Logs is enabled for a layer.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A list of configuration options for CloudWatch Logs.</p>
    #[serde(rename = "LogStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<CloudWatchLogsLogStream>>,
}

/// <p>Describes the Amazon CloudWatch logs configuration for a layer. For detailed information about members of this data type, see the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html">CloudWatch Logs Agent Reference</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLogsLogStream {
    /// <p>Specifies the max number of log events in a batch, up to 10000. The default value is 1000.</p>
    #[serde(rename = "BatchCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_count: Option<i64>,
    /// <p>Specifies the maximum size of log events in a batch, in bytes, up to 1048576 bytes. The default value is 32768 bytes. This size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>Specifies the time duration for the batching of log events. The minimum value is 5000ms and default value is 5000ms.</p>
    #[serde(rename = "BufferDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_duration: Option<i64>,
    /// <p>Specifies how the time stamp is extracted from logs. For more information, see the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html">CloudWatch Logs Agent Reference</a>.</p>
    #[serde(rename = "DatetimeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_format: Option<String>,
    /// <p>Specifies the encoding of the log file so that the file can be read correctly. The default is <code>utf_8</code>. Encodings supported by Python <code>codecs.decode()</code> can be used here.</p>
    #[serde(rename = "Encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// <p>Specifies log files that you want to push to CloudWatch Logs.</p> <p> <code>File</code> can point to a specific file or multiple files (by using wild card characters such as <code>/var/log/system.log*</code>). Only the latest file is pushed to CloudWatch Logs, based on file modification time. We recommend that you use wild card characters to specify a series of files of the same type, such as <code>access_log.2014-06-01-01</code>, <code>access_log.2014-06-01-02</code>, and so on by using a pattern like <code>access_log.*</code>. Don't use a wildcard to match multiple file types, such as <code>access_log_80</code> and <code>access_log_443</code>. To specify multiple, different file types, add another log stream entry to the configuration file, so that each log file type is stored in a different log group.</p> <p>Zipped files are not supported.</p>
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// <p>Specifies the range of lines for identifying a file. The valid values are one number, or two dash-delimited numbers, such as '1', '2-5'. The default value is '1', meaning the first line is used to calculate the fingerprint. Fingerprint lines are not sent to CloudWatch Logs unless all specified lines are available.</p>
    #[serde(rename = "FileFingerprintLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_fingerprint_lines: Option<String>,
    /// <p>Specifies where to start to read data (start_of_file or end_of_file). The default is start_of_file. This setting is only used if there is no state persisted for that log stream.</p>
    #[serde(rename = "InitialPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_position: Option<String>,
    /// <p>Specifies the destination log group. A log group is created automatically if it doesn't already exist. Log group names can be between 1 and 512 characters long. Allowed characters include a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>Specifies the pattern for identifying the start of a log message.</p>
    #[serde(rename = "MultiLineStartPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_line_start_pattern: Option<String>,
    /// <p>Specifies the time zone of log event time stamps.</p>
    #[serde(rename = "TimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// <p>Describes a command.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Command {
    /// <p>Date and time when the command was acknowledged.</p>
    #[serde(rename = "AcknowledgedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    /// <p>The command ID.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>Date when the command completed.</p>
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// <p>Date and time when the command was run.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The command deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The command exit code.</p>
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The ID of the instance where the command was executed.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The URL of the command log.</p>
    #[serde(rename = "LogUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// <p><p>The command status:</p> <ul> <li> <p>failed</p> </li> <li> <p>successful</p> </li> <li> <p>skipped</p> </li> <li> <p>pending</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The command type:</p> <ul> <li> <p> <code>configure</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>execute<em>recipes</code> </p> </li> <li> <p> <code>install</em>dependencies</code> </p> </li> <li> <p> <code>restart</code> </p> </li> <li> <p> <code>rollback</code> </p> </li> <li> <p> <code>setup</code> </p> </li> <li> <p> <code>start</code> </p> </li> <li> <p> <code>stop</code> </p> </li> <li> <p> <code>undeploy</code> </p> </li> <li> <p> <code>update<em>custom</em>cookbooks</code> </p> </li> <li> <p> <code>update_dependencies</code> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAppRequest {
    /// <p>A <code>Source</code> object that specifies the app repository.</p>
    #[serde(rename = "AppSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_source: Option<Source>,
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app's data source.</p>
    #[serde(rename = "DataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>A description of the app.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The app virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// <p>Whether to enable SSL for the app.</p>
    #[serde(rename = "EnableSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<bool>,
    /// <p><p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>.</p> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables&#39; names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, &quot;Environment: is too large (maximum is 20KB).&quot;</p> <note> <p>If you have specified one or more environment variables, you cannot modify the stack&#39;s Chef version.</p> </note></p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    /// <p>The app name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The app's short name.</p>
    #[serde(rename = "Shortname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    #[serde(rename = "SslConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// <p>The app type. Each supported type is associated with a particular layer. For example, PHP applications are associated with a PHP layer. AWS OpsWorks Stacks deploys an application to those instances that are members of the corresponding layer. If your app isn't one of the standard types, or you prefer to implement your own Deploy recipes, specify <code>other</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Contains the response to a <code>CreateApp</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAppResult {
    /// <p>The app ID.</p>
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeploymentRequest {
    /// <p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>
    #[serde(rename = "Command")]
    pub command: DeploymentCommand,
    /// <p>A user-defined comment.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>A string that contains user-defined, custom JSON. You can use this parameter to override some corresponding default stack configuration JSON values. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html">Overriding Attributes With Custom JSON</a>.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The instance IDs for the deployment targets.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>The layer IDs for the deployment targets.</p>
    #[serde(rename = "LayerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>CreateDeployment</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeploymentResult {
    /// <p>The deployment ID, which can be used with other requests to identify the deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstanceRequest {
    /// <p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p> <code>INHERIT</code> - Use the stack's default agent version setting.</p> </li> <li> <p> <i>version_number</i> - Use the specified agent version. This value overrides the stack's default setting. To update the agent version, edit the instance configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the instance.</p> </li> </ul> <p>The default setting is <code>INHERIT</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p><p>A custom AMI ID to be used to create the instance. The AMI should be based on one of the supported operating systems. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Using Custom AMIs</a>.</p> <note> <p>If you specify a custom AMI, you must set <code>Os</code> to <code>Custom</code>.</p> </note></p>
    #[serde(rename = "AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The instance architecture. The default option is <code>x86_64</code>. Instance types do not necessarily support both architectures. For a list of the architectures that are supported by the different instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance Families and Types</a>.</p>
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>For load-based or time-based instances, the type. Windows stacks can use only time-based instances.</p>
    #[serde(rename = "AutoScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<String>,
    /// <p>The instance Availability Zone. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>An array of <code>BlockDeviceMapping</code> objects that specify the instance's block devices. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block Device Mapping</a>. Note that block device mappings are not supported for custom AMIs.</p>
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>Whether to create an Amazon EBS-optimized instance.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// <p>The instance host name.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code> to ensure that your instances have the latest security updates.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>The instance type, such as <code>t2.micro</code>. For a list of supported instance types, open the stack in the console, choose <b>Instances</b>, and choose <b>+ Instance</b>. The <b>Size</b> list contains the currently supported types. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance Families and Types</a>. The parameter values that you use to specify the various types are in the <b>API Name</b> column of the <b>Available Instance Types</b> table.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>An array that contains the instance's layer IDs.</p>
    #[serde(rename = "LayerIds")]
    pub layer_ids: Vec<String>,
    /// <p>The instance's operating system, which must be set to one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2018.03</code>, <code>Amazon Linux 2017.09</code>, <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>.</p> </li> </ul> <p>For more information about the supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">AWS OpsWorks Stacks Operating Systems</a>.</p> <p>The default option is the current Amazon Linux version. If you set this parameter to <code>Custom</code>, you must use the <a>CreateInstance</a> action's AmiId parameter to specify the custom AMI that you want to use. Block device mappings are not supported if the value is <code>Custom</code>. For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">Operating Systems</a>For more information about how to use custom AMIs with AWS OpsWorks Stacks, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Using Custom AMIs</a>.</p>
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// <p>The instance root device type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "RootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    /// <p>The instance's Amazon EC2 key-pair name.</p>
    #[serde(rename = "SshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// <p>The ID of the instance's subnet. If the stack is running in a VPC, you can use this parameter to override the stack's default subnet ID value and direct AWS OpsWorks Stacks to launch the instance in a different subnet.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The instance's tenancy option. The default option is no tenancy, or if the instance is running in a VPC, inherit tenancy settings from the VPC. The following are valid values for this parameter: <code>dedicated</code>, <code>default</code>, or <code>host</code>. Because there are costs associated with changes in tenancy options, we recommend that you research tenancy options before choosing them for your instances. For more information about dedicated hosts, see <a href="http://aws.amazon.com/ec2/dedicated-hosts/">Dedicated Hosts Overview</a> and <a href="http://aws.amazon.com/ec2/dedicated-hosts/">Amazon EC2 Dedicated Hosts</a>. For more information about dedicated instances, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/dedicated-instance.html">Dedicated Instances</a> and <a href="http://aws.amazon.com/ec2/purchasing-options/dedicated-instances/">Amazon EC2 Dedicated Instances</a>.</p>
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    /// <p>The instance's virtualization type, <code>paravirtual</code> or <code>hvm</code>.</p>
    #[serde(rename = "VirtualizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,
}

/// <p>Contains the response to a <code>CreateInstance</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstanceResult {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLayerRequest {
    /// <p>One or more user-defined key-value pairs to be added to the stack attributes.</p> <p>To create a cluster layer, set the <code>EcsClusterArn</code> attribute to the cluster's ARN.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Whether to automatically assign an <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP address</a> to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    /// <p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignPublicIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    /// <p>Specifies CloudWatch Logs configuration options for the layer. For more information, see <a>CloudWatchLogsLogStream</a>.</p>
    #[serde(rename = "CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    /// <p>The ARN of an IAM profile to be used for the layer's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "CustomInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    /// <p>A JSON-formatted string containing custom stack configuration and deployment attributes to be installed on the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html"> Using Custom JSON</a>. This feature is supported as of version 1.7.42 of the AWS CLI. </p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>A <code>LayerCustomRecipes</code> object that specifies the layer custom recipes.</p>
    #[serde(rename = "CustomRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    /// <p>An array containing the layer custom security group IDs.</p>
    #[serde(rename = "CustomSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    /// <p>Whether to disable auto healing for the layer.</p>
    #[serde(rename = "EnableAutoHealing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>To ensure that your instances have the latest security updates, we strongly recommend using the default value of <code>true</code>.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>A <code>LifeCycleEventConfiguration</code> object that you can use to configure the Shutdown event to specify an execution timeout and enable or disable Elastic Load Balancer connection draining.</p>
    #[serde(rename = "LifecycleEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    /// <p>The layer name, which is used by the console.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An array of <code>Package</code> objects that describes the layer packages.</p>
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// <p>For custom layers only, use this parameter to specify the layer's short name, which is used internally by AWS OpsWorks Stacks and by Chef recipes. The short name is also used as the name for the directory where your app files are installed. It can have a maximum of 200 characters, which are limited to the alphanumeric characters, '-', '_', and '.'.</p> <p>The built-in layers' short names are defined by AWS OpsWorks Stacks. For more information, see the <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/layers.html">Layer Reference</a>.</p>
    #[serde(rename = "Shortname")]
    pub shortname: String,
    /// <p>The layer stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// <p>The layer type. A stack cannot have more than one built-in layer of the same type. It can have any number of custom layers. Built-in layers are not available in Chef 12 stacks.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>Whether to use Amazon EBS-optimized instances.</p>
    #[serde(rename = "UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    /// <p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>
    #[serde(rename = "VolumeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

/// <p>Contains the response to a <code>CreateLayer</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLayerResult {
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStackRequest {
    /// <p><p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack&#39;s instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack&#39;s instances.</p> </li> </ul> <p>The default setting is the most recent release of the agent. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack&#39;s default setting.</p> </note></p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>One or more user-defined key-value pairs to be added to the stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "ChefConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    /// <p>The configuration manager. When you create a stack we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 12.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Adding Apps</a> or <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html">Cookbooks and Recipes</a>.</p>
    #[serde(rename = "CustomCookbooksSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    /// <p>A string that contains user-defined, custom JSON. It can be used to override the corresponding default stack configuration attribute values or to pass data to recipes. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The stack's default Availability Zone, which must be in the specified region. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see the <code>VpcId</code> parameter description. </p>
    #[serde(rename = "DefaultAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    /// <p>The stack's default operating system, which is installed on every instance unless you specify a different operating system when you create the instance. You can specify one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2018.03</code>, <code>Amazon Linux 2017.09</code>, <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html"> Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the current Amazon Linux version. For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">AWS OpsWorks Stacks Operating Systems</a>.</p>
    #[serde(rename = "DefaultOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_os: Option<String>,
    /// <p>The default root device type. This value is the default for all instances in the stack, but you can override it when you create an instance. The default option is <code>instance-store</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "DefaultRootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<String>,
    /// <p>A default Amazon EC2 key pair name. The default value is none. If you specify a key pair name, AWS OpsWorks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html"> Using SSH to Communicate with an Instance</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html"> create an instance</a>. </p>
    #[serde(rename = "DefaultSshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    /// <p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>
    #[serde(rename = "DefaultSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<String>,
    /// <p>The stack's host name theme, with spaces replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities_and_Titans</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>
    #[serde(rename = "HostnameTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<String>,
    /// <p>The stack name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The stack&#39;s AWS region, such as <code>ap-south-1</code>. For more information about Amazon regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p> <note> <p>In the AWS CLI, this API maps to the <code>--stack-region</code> parameter. If the <code>--stack-region</code> parameter and the AWS CLI common parameter <code>--region</code> are set to the same value, the stack uses a <i>regional</i> endpoint. If the <code>--stack-region</code> parameter is not set, but the AWS CLI <code>--region</code> parameter is, this also results in a stack with a <i>regional</i> endpoint. However, if the <code>--region</code> parameter is set to <code>us-east-1</code>, and the <code>--stack-region</code> parameter is set to one of the following, then the stack uses a legacy or <i>classic</i> region: <code>us-west-1, us-west-2, sa-east-1, eu-central-1, eu-west-1, ap-northeast-1, ap-southeast-1, ap-southeast-2</code>. In this case, the actual API endpoint of the stack is in <code>us-east-1</code>. Only the preceding regions are supported as classic regions in the <code>us-east-1</code> API endpoint. Because it is a best practice to choose the regional endpoint that is closest to where you manage AWS, we recommend that you use regional endpoints for new stacks. The AWS CLI common <code>--region</code> parameter always specifies a regional API endpoint; it cannot be used to specify a classic AWS OpsWorks Stacks region.</p> </note></p>
    #[serde(rename = "Region")]
    pub region: String,
    /// <p>The stack's AWS Identity and Access Management (IAM) role, which allows AWS OpsWorks Stacks to work with AWS resources on your behalf. You must set this parameter to the Amazon Resource Name (ARN) for an existing IAM role. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// <p>Whether the stack uses custom cookbooks.</p>
    #[serde(rename = "UseCustomCookbooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    /// <p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. With <code>UseOpsworksSecurityGroups</code> you can instead provide your own custom security groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it, but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate EC2 security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on creation; custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    /// <p>The ID of the VPC that the stack is to be launched into. The VPC must be in the stack's region. All instances are launched into this VPC. You cannot change the ID later.</p> <ul> <li> <p>If your account supports EC2-Classic, the default value is <code>no VPC</code>.</p> </li> <li> <p>If your account does not support EC2-Classic, the default value is the default VPC for the specified region.</p> </li> </ul> <p>If the VPC ID corresponds to a default VPC and you have specified either the <code>DefaultAvailabilityZone</code> or the <code>DefaultSubnetId</code> parameter only, AWS OpsWorks Stacks infers the value of the other parameter. If you specify neither parameter, AWS OpsWorks Stacks sets these parameters to the first valid Availability Zone for the specified region and the corresponding default VPC subnet ID, respectively.</p> <p>If you specify a nondefault VPC ID, note the following:</p> <ul> <li> <p>It must belong to a VPC in your account that is in the specified region.</p> </li> <li> <p>You must specify a value for <code>DefaultSubnetId</code>.</p> </li> </ul> <p>For more information about how to use AWS OpsWorks Stacks with a VPC, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-vpc.html">Running a Stack in a VPC</a>. For more information about default VPC and EC2-Classic, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html">Supported Platforms</a>. </p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains the response to a <code>CreateStack</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStackResult {
    /// <p>The stack ID, which is an opaque string that you use to identify the stack when performing actions such as <code>DescribeStacks</code>.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserProfileRequest {
    /// <p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html">Setting an IAM User's Public SSH Key</a>.</p>
    #[serde(rename = "AllowSelfManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<bool>,
    /// <p>The user's IAM ARN; this can also be a federated user's ARN.</p>
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: String,
    /// <p>The user's public SSH key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The user's SSH user name. The allowable characters are [a-z], [A-Z], [0-9], '-', and '_'. If the specified name includes other punctuation marks, AWS OpsWorks Stacks removes them. For example, <code>my.name</code> will be changed to <code>myname</code>. If you do not specify an SSH user name, AWS OpsWorks Stacks generates one from the IAM user name. </p>
    #[serde(rename = "SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
}

/// <p>Contains the response to a <code>CreateUserProfile</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserProfileResult {
    /// <p>The user's IAM ARN.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
}

/// <p>Describes an app's data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    /// <p>The data source's ARN.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The database name.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The data source's type, <code>AutoSelectOpsworksMysqlInstance</code>, <code>OpsworksMysqlInstance</code>, <code>RdsDbInstance</code>, or <code>None</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppRequest {
    /// <p>The app ID.</p>
    #[serde(rename = "AppId")]
    pub app_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInstanceRequest {
    /// <p>Whether to delete the instance Elastic IP address.</p>
    #[serde(rename = "DeleteElasticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_elastic_ip: Option<bool>,
    /// <p>Whether to delete the instance's Amazon EBS volumes.</p>
    #[serde(rename = "DeleteVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_volumes: Option<bool>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLayerRequest {
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStackRequest {
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserProfileRequest {
    /// <p>The user's IAM ARN. This can also be a federated user's ARN.</p>
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: String,
}

/// <p>Describes a deployment of a stack or app.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Deployment {
    /// <p>The app ID.</p>
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Used to specify a stack or deployment command.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<DeploymentCommand>,
    /// <p>A user-defined comment.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>Date when the deployment completed.</p>
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// <p>Date when the deployment was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>A string that contains user-defined custom JSON. It can be used to override the corresponding default stack configuration attribute values for stack or to pass data to recipes. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information on custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The deployment ID.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The deployment duration.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The user's IAM ARN.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The IDs of the target instances.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p><p>The deployment status:</p> <ul> <li> <p>running</p> </li> <li> <p>successful</p> </li> <li> <p>failed</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Used to specify a stack or deployment command.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentCommand {
    /// <p>The arguments of those commands that take arguments. It should be set to a JSON object with the following format:</p> <p> <code>{"arg_name1" : ["value1", "value2", ...], "arg_name2" : ["value1", "value2", ...], ...}</code> </p> <p>The <code>update_dependencies</code> command takes two arguments:</p> <ul> <li> <p> <code>upgrade_os_to</code> - Specifies the desired Amazon Linux version for instances whose OS you want to upgrade, such as <code>Amazon Linux 2016.09</code>. You must also set the <code>allow_reboot</code> argument to true.</p> </li> <li> <p> <code>allow_reboot</code> - Specifies whether to allow AWS OpsWorks Stacks to reboot the instances if necessary, after installing the updates. This argument can be set to either <code>true</code> or <code>false</code>. The default value is <code>false</code>.</p> </li> </ul> <p>For example, to upgrade an instance to Amazon Linux 2016.09, set <code>Args</code> to the following.</p> <p> <code> { "upgrade_os_to":["Amazon Linux 2016.09"], "allow_reboot":["true"] } </code> </p>
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p><p>Specifies the operation. You can specify only one command.</p> <p>For stacks, the following commands are available:</p> <ul> <li> <p> <code>execute<em>recipes</code>: Execute one or more recipes. To specify the recipes, set an <code>Args</code> parameter named <code>recipes</code> to the list of recipes to be executed. For example, to execute <code>phpapp::appsetup</code>, set <code>Args</code> to <code>{&quot;recipes&quot;:[&quot;phpapp::appsetup&quot;]}</code>.</p> </li> <li> <p> <code>install</em>dependencies</code>: Install the stack&#39;s dependencies.</p> </li> <li> <p> <code>update<em>custom</em>cookbooks</code>: Update the stack&#39;s custom cookbooks.</p> </li> <li> <p> <code>update<em>dependencies</code>: Update the stack&#39;s dependencies.</p> </li> </ul> <note> <p>The update</em>dependencies and install_dependencies commands are supported only for Linux instances. You can run the commands successfully on Windows instances, but they do nothing.</p> </note> <p>For apps, the following commands are available:</p> <ul> <li> <p> <code>deploy</code>: Deploy an app. Ruby on Rails apps have an optional <code>Args</code> parameter named <code>migrate</code>. Set <code>Args</code> to {&quot;migrate&quot;:[&quot;true&quot;]} to migrate the database. The default setting is {&quot;migrate&quot;:[&quot;false&quot;]}.</p> </li> <li> <p> <code>rollback</code> Roll the app back to the previous version. When you update an app, AWS OpsWorks Stacks stores the previous version, up to a maximum of five versions. You can use this command to roll an app back as many as four versions.</p> </li> <li> <p> <code>start</code>: Start the app&#39;s web or application server.</p> </li> <li> <p> <code>stop</code>: Stop the app&#39;s web or application server.</p> </li> <li> <p> <code>restart</code>: Restart the app&#39;s web or application server.</p> </li> <li> <p> <code>undeploy</code>: Undeploy the app.</p> </li> </ul></p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterEcsClusterRequest {
    /// <p>The cluster's Amazon Resource Number (ARN).</p>
    #[serde(rename = "EcsClusterArn")]
    pub ecs_cluster_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterElasticIpRequest {
    /// <p>The Elastic IP address.</p>
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterInstanceRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterRdsDbInstanceRequest {
    /// <p>The Amazon RDS instance's ARN.</p>
    #[serde(rename = "RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterVolumeRequest {
    /// <p>The AWS OpsWorks Stacks volume ID, which is the GUID that AWS OpsWorks Stacks assigned to the instance when you registered the volume with the stack, not the Amazon EC2 volume ID.</p>
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAgentVersionsRequest {
    /// <p>The configuration manager.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeAgentVersions</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAgentVersionsResult {
    /// <p>The agent versions for the specified stack or configuration manager. Note that this value is the complete version number, not the abbreviated number used by the console.</p>
    #[serde(rename = "AgentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<Vec<AgentVersion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAppsRequest {
    /// <p>An array of app IDs for the apps to be described. If you use this parameter, <code>DescribeApps</code> returns a description of the specified apps. Otherwise, it returns a description of every app.</p>
    #[serde(rename = "AppIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// <p>The app stack ID. If you use this parameter, <code>DescribeApps</code> returns a description of the apps in the specified stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeApps</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAppsResult {
    /// <p>An array of <code>App</code> objects that describe the specified apps. </p>
    #[serde(rename = "Apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<App>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCommandsRequest {
    /// <p>An array of command IDs. If you include this parameter, <code>DescribeCommands</code> returns a description of the specified commands. Otherwise, it returns a description of every command.</p>
    #[serde(rename = "CommandIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_ids: Option<Vec<String>>,
    /// <p>The deployment ID. If you include this parameter, <code>DescribeCommands</code> returns a description of the commands associated with the specified deployment.</p>
    #[serde(rename = "DeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The instance ID. If you include this parameter, <code>DescribeCommands</code> returns a description of the commands associated with the specified instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeCommands</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCommandsResult {
    /// <p>An array of <code>Command</code> objects that describe each of the specified commands.</p>
    #[serde(rename = "Commands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeploymentsRequest {
    /// <p>The app ID. If you include this parameter, the command returns a description of the commands associated with the specified app.</p>
    #[serde(rename = "AppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>An array of deployment IDs to be described. If you include this parameter, the command returns a description of the specified deployments. Otherwise, it returns a description of every deployment.</p>
    #[serde(rename = "DeploymentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_ids: Option<Vec<String>>,
    /// <p>The stack ID. If you include this parameter, the command returns a description of the commands associated with the specified stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeDeployments</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeploymentsResult {
    /// <p>An array of <code>Deployment</code> objects that describe the deployments.</p>
    #[serde(rename = "Deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEcsClustersRequest {
    /// <p>A list of ARNs, one for each cluster to be described.</p>
    #[serde(rename = "EcsClusterArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arns: Option<Vec<String>>,
    /// <p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request did not return all of the remaining results, the response object's<code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>DescribeEcsClusters</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A stack ID. <code>DescribeEcsClusters</code> returns a description of the cluster that is registered with the stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeEcsClusters</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEcsClustersResult {
    /// <p>A list of <code>EcsCluster</code> objects containing the cluster descriptions.</p>
    #[serde(rename = "EcsClusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_clusters: Option<Vec<EcsCluster>>,
    /// <p>If a paginated request does not return all of the remaining results, this parameter is set to a token that you can assign to the request object's <code>NextToken</code> parameter to retrieve the next set of results. If the previous paginated request returned all of the remaining results, this parameter is set to <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticIpsRequest {
    /// <p>The instance ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses associated with the specified instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>An array of Elastic IP addresses to be described. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the specified Elastic IP addresses. Otherwise, it returns a description of every Elastic IP address.</p>
    #[serde(rename = "Ips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<String>>,
    /// <p>A stack ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses that are registered with the specified stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeElasticIps</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticIpsResult {
    /// <p>An <code>ElasticIps</code> object that describes the specified Elastic IP addresses.</p>
    #[serde(rename = "ElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<Vec<ElasticIp>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeElasticLoadBalancersRequest {
    /// <p>A list of layer IDs. The action describes the Elastic Load Balancing instances for the specified layers.</p>
    #[serde(rename = "LayerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    /// <p>A stack ID. The action describes the stack's Elastic Load Balancing instances.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeElasticLoadBalancers</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeElasticLoadBalancersResult {
    /// <p>A list of <code>ElasticLoadBalancer</code> objects that describe the specified Elastic Load Balancing instances.</p>
    #[serde(rename = "ElasticLoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancers: Option<Vec<ElasticLoadBalancer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstancesRequest {
    /// <p>An array of instance IDs to be described. If you use this parameter, <code>DescribeInstances</code> returns a description of the specified instances. Otherwise, it returns a description of every instance.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>A layer ID. If you use this parameter, <code>DescribeInstances</code> returns descriptions of the instances associated with the specified layer.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
    /// <p>A stack ID. If you use this parameter, <code>DescribeInstances</code> returns descriptions of the instances associated with the specified stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeInstances</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstancesResult {
    /// <p>An array of <code>Instance</code> objects that describe the instances.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLayersRequest {
    /// <p>An array of layer IDs that specify the layers to be described. If you omit this parameter, <code>DescribeLayers</code> returns a description of every layer in the specified stack.</p>
    #[serde(rename = "LayerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeLayers</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLayersResult {
    /// <p>An array of <code>Layer</code> objects that describe the layers.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoadBasedAutoScalingRequest {
    /// <p>An array of layer IDs.</p>
    #[serde(rename = "LayerIds")]
    pub layer_ids: Vec<String>,
}

/// <p>Contains the response to a <code>DescribeLoadBasedAutoScaling</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLoadBasedAutoScalingResult {
    /// <p>An array of <code>LoadBasedAutoScalingConfiguration</code> objects that describe each layer's configuration.</p>
    #[serde(rename = "LoadBasedAutoScalingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_based_auto_scaling_configurations: Option<Vec<LoadBasedAutoScalingConfiguration>>,
}

/// <p>Contains the response to a <code>DescribeMyUserProfile</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMyUserProfileResult {
    /// <p>A <code>UserProfile</code> object that describes the user's SSH information.</p>
    #[serde(rename = "UserProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile: Option<SelfUserProfile>,
}

/// <p>The response to a <code>DescribeOperatingSystems</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOperatingSystemsResponse {
    /// <p>Contains information in response to a <code>DescribeOperatingSystems</code> request.</p>
    #[serde(rename = "OperatingSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_systems: Option<Vec<OperatingSystem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePermissionsRequest {
    /// <p>The user's IAM ARN. This can also be a federated user's ARN. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribePermissions</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePermissionsResult {
    /// <p><p>An array of <code>Permission</code> objects that describe the stack permissions.</p> <ul> <li> <p>If the request object contains only a stack ID, the array contains a <code>Permission</code> object with permissions for each of the stack IAM ARNs.</p> </li> <li> <p>If the request object contains only an IAM ARN, the array contains a <code>Permission</code> object with permissions for each of the user&#39;s stack IDs.</p> </li> <li> <p>If the request contains a stack ID and an IAM ARN, the array contains a single <code>Permission</code> object with permissions for the specified stack and IAM ARN.</p> </li> </ul></p>
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRaidArraysRequest {
    /// <p>The instance ID. If you use this parameter, <code>DescribeRaidArrays</code> returns descriptions of the RAID arrays associated with the specified instance. </p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>An array of RAID array IDs. If you use this parameter, <code>DescribeRaidArrays</code> returns descriptions of the specified arrays. Otherwise, it returns a description of every array.</p>
    #[serde(rename = "RaidArrayIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_array_ids: Option<Vec<String>>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeRaidArrays</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRaidArraysResult {
    /// <p>A <code>RaidArrays</code> object that describes the specified RAID arrays.</p>
    #[serde(rename = "RaidArrays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_arrays: Option<Vec<RaidArray>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRdsDbInstancesRequest {
    /// <p>An array containing the ARNs of the instances to be described.</p>
    #[serde(rename = "RdsDbInstanceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_instance_arns: Option<Vec<String>>,
    /// <p>The ID of the stack with which the instances are registered. The operation returns descriptions of all registered Amazon RDS instances.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>DescribeRdsDbInstances</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRdsDbInstancesResult {
    /// <p>An a array of <code>RdsDbInstance</code> objects that describe the instances.</p>
    #[serde(rename = "RdsDbInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_instances: Option<Vec<RdsDbInstance>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServiceErrorsRequest {
    /// <p>The instance ID. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the errors associated with the specified instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>An array of service error IDs. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the specified errors. Otherwise, it returns a description of every error.</p>
    #[serde(rename = "ServiceErrorIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_error_ids: Option<Vec<String>>,
    /// <p>The stack ID. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the errors associated with the specified stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Contains the response to a <code>DescribeServiceErrors</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServiceErrorsResult {
    /// <p>An array of <code>ServiceError</code> objects that describe the specified service errors.</p>
    #[serde(rename = "ServiceErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_errors: Option<Vec<ServiceError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStackProvisioningParametersRequest {
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>DescribeStackProvisioningParameters</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStackProvisioningParametersResult {
    /// <p>The AWS OpsWorks Stacks agent installer's URL.</p>
    #[serde(rename = "AgentInstallerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_url: Option<String>,
    /// <p>An embedded object that contains the provisioning parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStackSummaryRequest {
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>DescribeStackSummary</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStackSummaryResult {
    /// <p>A <code>StackSummary</code> object that contains the results.</p>
    #[serde(rename = "StackSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_summary: Option<StackSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStacksRequest {
    /// <p>An array of stack IDs that specify the stacks to be described. If you omit this parameter, <code>DescribeStacks</code> returns a description of every stack.</p>
    #[serde(rename = "StackIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_ids: Option<Vec<String>>,
}

/// <p>Contains the response to a <code>DescribeStacks</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStacksResult {
    /// <p>An array of <code>Stack</code> objects that describe the stacks.</p>
    #[serde(rename = "Stacks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<Vec<Stack>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTimeBasedAutoScalingRequest {
    /// <p>An array of instance IDs.</p>
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

/// <p>Contains the response to a <code>DescribeTimeBasedAutoScaling</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTimeBasedAutoScalingResult {
    /// <p>An array of <code>TimeBasedAutoScalingConfiguration</code> objects that describe the configuration for the specified instances.</p>
    #[serde(rename = "TimeBasedAutoScalingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_auto_scaling_configurations: Option<Vec<TimeBasedAutoScalingConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserProfilesRequest {
    /// <p>An array of IAM or federated user ARNs that identify the users to be described.</p>
    #[serde(rename = "IamUserArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arns: Option<Vec<String>>,
}

/// <p>Contains the response to a <code>DescribeUserProfiles</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserProfilesResult {
    /// <p>A <code>Users</code> object that describes the specified users.</p>
    #[serde(rename = "UserProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profiles: Option<Vec<UserProfile>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVolumesRequest {
    /// <p>The instance ID. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the volumes associated with the specified instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The RAID array ID. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the volumes associated with the specified RAID array.</p>
    #[serde(rename = "RaidArrayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_array_id: Option<String>,
    /// <p>A stack ID. The action describes the stack's registered Amazon EBS volumes.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>Am array of volume IDs. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the specified volumes. Otherwise, it returns a description of every volume.</p>
    #[serde(rename = "VolumeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_ids: Option<Vec<String>>,
}

/// <p>Contains the response to a <code>DescribeVolumes</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVolumesResult {
    /// <p>An array of volume IDs.</p>
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachElasticLoadBalancerRequest {
    /// <p>The Elastic Load Balancing instance's name.</p>
    #[serde(rename = "ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    /// <p>The ID of the layer that the Elastic Load Balancing instance is attached to.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateElasticIpRequest {
    /// <p>The Elastic IP address.</p>
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: String,
}

/// <p>Describes an Amazon EBS volume. This data type maps directly to the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html">EbsBlockDevice</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EbsBlockDevice {
    /// <p>Whether the volume is deleted on instance termination.</p>
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    /// <p>The number of I/O operations per second (IOPS) that the volume supports. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html">EbsBlockDevice</a>.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The snapshot ID.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The volume size, in GiB. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html">EbsBlockDevice</a>.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
    /// <p>The volume type. <code>gp2</code> for General Purpose (SSD) volumes, <code>io1</code> for Provisioned IOPS (SSD) volumes, <code>st1</code> for Throughput Optimized hard disk drives (HDD), <code>sc1</code> for Cold HDD,and <code>standard</code> for Magnetic volumes.</p> <p>If you specify the <code>io1</code> volume type, you must also specify a value for the <code>Iops</code> attribute. The maximum ratio of provisioned IOPS to requested volume size (in GiB) is 50:1. AWS uses the default volume size (in GiB) specified in the AMI attributes to set IOPS to 50 x (volume size).</p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes a registered Amazon ECS cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EcsCluster {
    /// <p>The cluster's ARN.</p>
    #[serde(rename = "EcsClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    /// <p>The cluster name.</p>
    #[serde(rename = "EcsClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name: Option<String>,
    /// <p>The time and date that the cluster was registered with the stack.</p>
    #[serde(rename = "RegisteredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Describes an Elastic IP address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticIp {
    /// <p>The domain.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The ID of the instance that the address is attached to.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The IP address.</p>
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p>The name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS region. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>Describes an Elastic Load Balancing instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticLoadBalancer {
    /// <p>A list of Availability Zones.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The instance's public DNS name.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>
    #[serde(rename = "Ec2InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_ids: Option<Vec<String>>,
    /// <p>The Elastic Load Balancing instance's name.</p>
    #[serde(rename = "ElasticLoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancer_name: Option<String>,
    /// <p>The ID of the layer that the instance is attached to.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
    /// <p>The instance's AWS region.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ID of the stack that the instance is associated with.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>A list of subnet IDs, if the stack is running in a VPC.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The VPC ID.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Represents an app's environment variable.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    /// <p>(Required) The environment variable's name, which can consist of up to 64 characters and must be specified. The name can contain upper- and lowercase letters, numbers, and underscores (_), but it must start with a letter or underscore.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>(Optional) Whether the variable's value will be returned by the <a>DescribeApps</a> action. To conceal an environment variable's value, set <code>Secure</code> to <code>true</code>. <code>DescribeApps</code> then returns <code>*****FILTERED*****</code> instead of the actual value. The default value for <code>Secure</code> is <code>false</code>. </p>
    #[serde(rename = "Secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// <p>(Optional) The environment variable's value, which can be left empty. If you specify a value, it can contain up to 256 characters, which must all be printable.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHostnameSuggestionRequest {
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
}

/// <p>Contains the response to a <code>GetHostnameSuggestion</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetHostnameSuggestionResult {
    /// <p>The generated host name.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantAccessRequest {
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires at the end of this period, the user will no longer be able to use the credentials to log in. If the user is logged in at the time, he or she automatically will be logged out.</p>
    #[serde(rename = "ValidForInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for_in_minutes: Option<i64>,
}

/// <p>Contains the response to a <code>GrantAccess</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantAccessResult {
    /// <p>A <code>TemporaryCredential</code> object that contains the data needed to log in to the instance by RDP clients, such as the Microsoft Remote Desktop Connection.</p>
    #[serde(rename = "TemporaryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_credential: Option<TemporaryCredential>,
}

/// <p>Describes an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>The agent version. This parameter is set to <code>INHERIT</code> if the instance inherits the default stack setting or to a a version number for a fixed agent version.</p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>A custom AMI ID to be used to create the instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Instances</a> </p>
    #[serde(rename = "AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The instance architecture: "i386" or "x86_64".</p>
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The instance's Amazon Resource Number (ARN).</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>For load-based or time-based instances, the type.</p>
    #[serde(rename = "AutoScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<String>,
    /// <p>The instance Availability Zone. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>An array of <code>BlockDeviceMapping</code> objects that specify the instance's block device mappings.</p>
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// <p>The time that the instance was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>Whether this is an Amazon EBS-optimized instance.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// <p>The ID of the associated Amazon EC2 instance.</p>
    #[serde(rename = "Ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>For container instances, the Amazon ECS cluster's ARN.</p>
    #[serde(rename = "EcsClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    /// <p>For container instances, the instance's ARN.</p>
    #[serde(rename = "EcsContainerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_container_instance_arn: Option<String>,
    /// <p>The instance <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP address </a>.</p>
    #[serde(rename = "ElasticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    /// <p>The instance host name.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>For registered instances, the infrastructure class: <code>ec2</code> or <code>on-premises</code>.</p>
    #[serde(rename = "InfrastructureClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_class: Option<String>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. If this value is set to <code>false</code>, you must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The ARN of the instance's IAM profile. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "InstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
    /// <p>The instance type, such as <code>t2.micro</code>.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The ID of the last service error. For more information, call <a>DescribeServiceErrors</a>.</p>
    #[serde(rename = "LastServiceErrorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_service_error_id: Option<String>,
    /// <p>An array containing the instance layer IDs.</p>
    #[serde(rename = "LayerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    /// <p>The instance's operating system.</p>
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// <p>The instance's platform.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The instance's private DNS name.</p>
    #[serde(rename = "PrivateDns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns: Option<String>,
    /// <p>The instance's private IP address.</p>
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// <p>The instance public DNS name.</p>
    #[serde(rename = "PublicDns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns: Option<String>,
    /// <p>The instance public IP address.</p>
    #[serde(rename = "PublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>For registered instances, who performed the registration.</p>
    #[serde(rename = "RegisteredBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    /// <p>The instance's reported AWS OpsWorks Stacks agent version.</p>
    #[serde(rename = "ReportedAgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_agent_version: Option<String>,
    /// <p>For registered instances, the reported operating system.</p>
    #[serde(rename = "ReportedOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_os: Option<ReportedOs>,
    /// <p>The instance's root device type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "RootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    /// <p>The root device volume ID.</p>
    #[serde(rename = "RootDeviceVolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_volume_id: Option<String>,
    /// <p>An array containing the instance security group IDs.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The SSH key's Deep Security Agent (DSA) fingerprint.</p>
    #[serde(rename = "SshHostDsaKeyFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_host_dsa_key_fingerprint: Option<String>,
    /// <p>The SSH key's RSA fingerprint.</p>
    #[serde(rename = "SshHostRsaKeyFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_host_rsa_key_fingerprint: Option<String>,
    /// <p>The instance's Amazon EC2 key-pair name.</p>
    #[serde(rename = "SshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p><p>The instance status:</p> <ul> <li> <p> <code>booting</code> </p> </li> <li> <p> <code>connection<em>lost</code> </p> </li> <li> <p> <code>online</code> </p> </li> <li> <p> <code>pending</code> </p> </li> <li> <p> <code>rebooting</code> </p> </li> <li> <p> <code>requested</code> </p> </li> <li> <p> <code>running</em>setup</code> </p> </li> <li> <p> <code>setup<em>failed</code> </p> </li> <li> <p> <code>shutting</em>down</code> </p> </li> <li> <p> <code>start<em>failed</code> </p> </li> <li> <p> <code>stop</em>failed</code> </p> </li> <li> <p> <code>stopped</code> </p> </li> <li> <p> <code>stopping</code> </p> </li> <li> <p> <code>terminated</code> </p> </li> <li> <p> <code>terminating</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The instance's subnet ID; applicable only if the stack is running in a VPC.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The instance's tenancy option, such as <code>dedicated</code> or <code>host</code>.</p>
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    /// <p>The instance's virtualization type: <code>paravirtual</code> or <code>hvm</code>.</p>
    #[serde(rename = "VirtualizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,
}

/// <p>Contains a description of an Amazon EC2 instance from the Amazon EC2 metadata service. For more information, see <a href="https://docs.aws.amazon.com/sdkfornet/latest/apidocs/Index.html">Instance Metadata and User Data</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceIdentity {
    /// <p>A JSON document that contains the metadata.</p>
    #[serde(rename = "Document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>A signature that can be used to verify the document's accuracy and authenticity.</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// <p>Describes how many instances a stack has for each status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstancesCount {
    /// <p>The number of instances in the Assigning state.</p>
    #[serde(rename = "Assigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigning: Option<i64>,
    /// <p>The number of instances with <code>booting</code> status.</p>
    #[serde(rename = "Booting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booting: Option<i64>,
    /// <p>The number of instances with <code>connection_lost</code> status.</p>
    #[serde(rename = "ConnectionLost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_lost: Option<i64>,
    /// <p>The number of instances in the Deregistering state.</p>
    #[serde(rename = "Deregistering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistering: Option<i64>,
    /// <p>The number of instances with <code>online</code> status.</p>
    #[serde(rename = "Online")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<i64>,
    /// <p>The number of instances with <code>pending</code> status.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// <p>The number of instances with <code>rebooting</code> status.</p>
    #[serde(rename = "Rebooting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebooting: Option<i64>,
    /// <p>The number of instances in the Registered state.</p>
    #[serde(rename = "Registered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered: Option<i64>,
    /// <p>The number of instances in the Registering state.</p>
    #[serde(rename = "Registering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registering: Option<i64>,
    /// <p>The number of instances with <code>requested</code> status.</p>
    #[serde(rename = "Requested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<i64>,
    /// <p>The number of instances with <code>running_setup</code> status.</p>
    #[serde(rename = "RunningSetup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_setup: Option<i64>,
    /// <p>The number of instances with <code>setup_failed</code> status.</p>
    #[serde(rename = "SetupFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_failed: Option<i64>,
    /// <p>The number of instances with <code>shutting_down</code> status.</p>
    #[serde(rename = "ShuttingDown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutting_down: Option<i64>,
    /// <p>The number of instances with <code>start_failed</code> status.</p>
    #[serde(rename = "StartFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_failed: Option<i64>,
    /// <p>The number of instances with <code>stop_failed</code> status.</p>
    #[serde(rename = "StopFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_failed: Option<i64>,
    /// <p>The number of instances with <code>stopped</code> status.</p>
    #[serde(rename = "Stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
    /// <p>The number of instances with <code>stopping</code> status.</p>
    #[serde(rename = "Stopping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping: Option<i64>,
    /// <p>The number of instances with <code>terminated</code> status.</p>
    #[serde(rename = "Terminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<i64>,
    /// <p>The number of instances with <code>terminating</code> status.</p>
    #[serde(rename = "Terminating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i64>,
    /// <p>The number of instances in the Unassigning state.</p>
    #[serde(rename = "Unassigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unassigning: Option<i64>,
}

/// <p>Describes a layer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Layer {
    /// <p>The Amazon Resource Number (ARN) of a layer.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The layer attributes.</p> <p>For the <code>HaproxyStatsPassword</code>, <code>MysqlRootPassword</code>, and <code>GangliaPassword</code> attributes, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value</p> <p>For an ECS Cluster layer, AWS OpsWorks Stacks the <code>EcsClusterArn</code> attribute is set to the cluster's ARN.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Whether to automatically assign an <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP address</a> to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    /// <p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignPublicIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    /// <p>The Amazon CloudWatch Logs configuration settings for the layer.</p>
    #[serde(rename = "CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    /// <p>Date when the layer was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ARN of the default IAM profile to be used for the layer's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "CustomInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    /// <p>A JSON formatted string containing the layer's custom stack configuration and deployment attributes.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>A <code>LayerCustomRecipes</code> object that specifies the layer's custom recipes.</p>
    #[serde(rename = "CustomRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    /// <p>An array containing the layer's custom security group IDs.</p>
    #[serde(rename = "CustomSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    /// <p>AWS OpsWorks Stacks supports five lifecycle events: <b>setup</b>, <b>configuration</b>, <b>deploy</b>, <b>undeploy</b>, and <b>shutdown</b>. For each layer, AWS OpsWorks Stacks runs a set of standard recipes for each event. You can also provide custom recipes for any or all layers and events. AWS OpsWorks Stacks runs custom event recipes after the standard recipes. <code>LayerCustomRecipes</code> specifies the custom recipes for a particular layer to be run in response to each of the five events.</p> <p>To specify a recipe, use the cookbook's directory name in the repository followed by two colons and the recipe name, which is the recipe's file name without the <code>.rb</code> extension. For example: <code>phpapp2::dbsetup</code> specifies the <code>dbsetup.rb</code> recipe in the repository's <code>phpapp2</code> folder.</p>
    #[serde(rename = "DefaultRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_recipes: Option<Recipes>,
    /// <p>An array containing the layer's security group names.</p>
    #[serde(rename = "DefaultSecurityGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_security_group_names: Option<Vec<String>>,
    /// <p>Whether auto healing is disabled for the layer.</p>
    #[serde(rename = "EnableAutoHealing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. If this value is set to <code>false</code>, you must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
    /// <p>A <code>LifeCycleEventConfiguration</code> object that specifies the Shutdown event configuration.</p>
    #[serde(rename = "LifecycleEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    /// <p>The layer name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of <code>Package</code> objects that describe the layer's packages.</p>
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// <p>The layer short name.</p>
    #[serde(rename = "Shortname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// <p>The layer stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The layer type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Whether the layer uses Amazon EBS-optimized instances.</p>
    #[serde(rename = "UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    /// <p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>
    #[serde(rename = "VolumeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

/// <p>Specifies the lifecycle event configuration</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEventConfiguration {
    /// <p>A <code>ShutdownEventConfiguration</code> object that specifies the Shutdown event configuration.</p>
    #[serde(rename = "Shutdown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown: Option<ShutdownEventConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p>Do not use. A validation exception occurs if you add a <code>MaxResults</code> parameter to a <code>ListTagsRequest</code> call. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Do not use. A validation exception occurs if you add a <code>NextToken</code> parameter to a <code>ListTagsRequest</code> call. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The stack or layer's Amazon Resource Number (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p>Contains the response to a <code>ListTags</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResult {
    /// <p>If a paginated request does not return all of the remaining results, this parameter is set to a token that you can assign to the request object's <code>NextToken</code> parameter to get the next set of results. If the previous paginated request returned all of the remaining results, this parameter is set to <code>null</code>. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A set of key-value pairs that contain tag keys and tag values that are attached to a stack or layer.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes a layer's load-based auto scaling configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBasedAutoScalingConfiguration {
    /// <p>An <code>AutoScalingThresholds</code> object that describes the downscaling configuration, which defines how and when AWS OpsWorks Stacks reduces the number of instances.</p>
    #[serde(rename = "DownScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_scaling: Option<AutoScalingThresholds>,
    /// <p>Whether load-based auto scaling is enabled for the layer.</p>
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
    /// <p>An <code>AutoScalingThresholds</code> object that describes the upscaling configuration, which defines how and when AWS OpsWorks Stacks increases the number of instances.</p>
    #[serde(rename = "UpScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_scaling: Option<AutoScalingThresholds>,
}

/// <p>Describes supported operating systems in AWS OpsWorks Stacks.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperatingSystem {
    /// <p>Supported configuration manager name and versions for an AWS OpsWorks Stacks operating system.</p>
    #[serde(rename = "ConfigurationManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_managers: Option<Vec<OperatingSystemConfigurationManager>>,
    /// <p>The ID of a supported operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the operating system, such as <code>Amazon Linux 2018.03</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A short name for the operating system manufacturer.</p>
    #[serde(rename = "ReportedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_name: Option<String>,
    /// <p>The version of the operating system, including the release and edition, if applicable.</p>
    #[serde(rename = "ReportedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_version: Option<String>,
    /// <p>Indicates that an operating system is not supported for new instances.</p>
    #[serde(rename = "Supported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
    /// <p>The type of a supported operating system, either <code>Linux</code> or <code>Windows</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A block that contains information about the configuration manager (Chef) and the versions of the configuration manager that are supported for an operating system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperatingSystemConfigurationManager {
    /// <p>The name of the configuration manager, which is Chef.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The versions of the configuration manager that are supported by an operating system.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Describes stack or user permissions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Permission {
    /// <p>Whether the user can use SSH.</p>
    #[serde(rename = "AllowSsh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ssh: Option<bool>,
    /// <p>Whether the user can use <b>sudo</b>.</p>
    #[serde(rename = "AllowSudo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sudo: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for an AWS Identity and Access Management (IAM) role. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The user's permission level, which must be the following:</p> <ul> <li> <p> <code>deny</code> </p> </li> <li> <p> <code>show</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>manage</code> </p> </li> <li> <p> <code>iam_only</code> </p> </li> </ul> <p>For more information on the permissions associated with these levels, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a> </p>
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>A stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

/// <p>Describes an instance's RAID array.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RaidArray {
    /// <p>The array's Availability Zone. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>When the RAID array was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The array's Linux device. For example /dev/mdadm0.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>For PIOPS volumes, the IOPS per disk.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The array's mount point.</p>
    #[serde(rename = "MountPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    /// <p>The array name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of disks in the array.</p>
    #[serde(rename = "NumberOfDisks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_disks: Option<i64>,
    /// <p>The array ID.</p>
    #[serde(rename = "RaidArrayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_array_id: Option<String>,
    /// <p>The <a href="http://en.wikipedia.org/wiki/Standard_RAID_levels">RAID level</a>.</p>
    #[serde(rename = "RaidLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_level: Option<i64>,
    /// <p>The array's size.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The volume type, standard or PIOPS.</p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes an Amazon RDS instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RdsDbInstance {
    /// <p>The instance's address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The DB instance identifier.</p>
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>
    #[serde(rename = "DbPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    /// <p>The master user name.</p>
    #[serde(rename = "DbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    /// <p>The instance's database engine.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>Set to <code>true</code> if AWS OpsWorks Stacks is unable to discover the Amazon RDS instance. AWS OpsWorks Stacks attempts to discover the instance only once. If this value is set to <code>true</code>, you must deregister the instance, and then register it again.</p>
    #[serde(rename = "MissingOnRds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_on_rds: Option<bool>,
    /// <p>The instance's ARN.</p>
    #[serde(rename = "RdsDbInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_instance_arn: Option<String>,
    /// <p>The instance's AWS region.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ID of the stack with which the instance is registered.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootInstanceRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

/// <p>AWS OpsWorks Stacks supports five lifecycle events: <b>setup</b>, <b>configuration</b>, <b>deploy</b>, <b>undeploy</b>, and <b>shutdown</b>. For each layer, AWS OpsWorks Stacks runs a set of standard recipes for each event. In addition, you can provide custom recipes for any or all layers and events. AWS OpsWorks Stacks runs custom event recipes after the standard recipes. <code>LayerCustomRecipes</code> specifies the custom recipes for a particular layer to be run in response to each of the five events. </p> <p>To specify a recipe, use the cookbook's directory name in the repository followed by two colons and the recipe name, which is the recipe's file name without the .rb extension. For example: phpapp2::dbsetup specifies the dbsetup.rb recipe in the repository's phpapp2 folder.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipes {
    /// <p>An array of custom recipe names to be run following a <code>configure</code> event.</p>
    #[serde(rename = "Configure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure: Option<Vec<String>>,
    /// <p>An array of custom recipe names to be run following a <code>deploy</code> event.</p>
    #[serde(rename = "Deploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Vec<String>>,
    /// <p>An array of custom recipe names to be run following a <code>setup</code> event.</p>
    #[serde(rename = "Setup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<Vec<String>>,
    /// <p>An array of custom recipe names to be run following a <code>shutdown</code> event.</p>
    #[serde(rename = "Shutdown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown: Option<Vec<String>>,
    /// <p>An array of custom recipe names to be run following a <code>undeploy</code> event.</p>
    #[serde(rename = "Undeploy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undeploy: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterEcsClusterRequest {
    /// <p>The cluster's ARN.</p>
    #[serde(rename = "EcsClusterArn")]
    pub ecs_cluster_arn: String,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>RegisterEcsCluster</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterEcsClusterResult {
    /// <p>The cluster's ARN.</p>
    #[serde(rename = "EcsClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterElasticIpRequest {
    /// <p>The Elastic IP address.</p>
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: String,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>RegisterElasticIp</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterElasticIpResult {
    /// <p>The Elastic IP address.</p>
    #[serde(rename = "ElasticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterInstanceRequest {
    /// <p>The instance's hostname.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>An InstanceIdentity object that contains the instance's identity.</p>
    #[serde(rename = "InstanceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity: Option<InstanceIdentity>,
    /// <p>The instance's private IP address.</p>
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// <p>The instance's public IP address.</p>
    #[serde(rename = "PublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>The instances public RSA key. This key is used to encrypt communication between the instance and the service.</p>
    #[serde(rename = "RsaPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsa_public_key: Option<String>,
    /// <p>The instances public RSA key fingerprint.</p>
    #[serde(rename = "RsaPublicKeyFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsa_public_key_fingerprint: Option<String>,
    /// <p>The ID of the stack that the instance is to be registered with.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>RegisterInstanceResult</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterInstanceResult {
    /// <p>The registered instance's AWS OpsWorks Stacks ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterRdsDbInstanceRequest {
    /// <p>The database password.</p>
    #[serde(rename = "DbPassword")]
    pub db_password: String,
    /// <p>The database's master user name.</p>
    #[serde(rename = "DbUser")]
    pub db_user: String,
    /// <p>The Amazon RDS instance's ARN.</p>
    #[serde(rename = "RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterVolumeRequest {
    /// <p>The Amazon EBS volume ID.</p>
    #[serde(rename = "Ec2VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_volume_id: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

/// <p>Contains the response to a <code>RegisterVolume</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterVolumeResult {
    /// <p>The volume ID.</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

/// <p>A registered instance's reported operating system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReportedOs {
    /// <p>The operating system family.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The operating system name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Describes a user's SSH information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SelfUserProfile {
    /// <p>The user's IAM ARN.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The user's name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The user's SSH public key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The user's SSH user name.</p>
    #[serde(rename = "SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
}

/// <p>Describes an AWS OpsWorks Stacks service error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceError {
    /// <p>When the error occurred.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>A message that describes the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The error ID.</p>
    #[serde(rename = "ServiceErrorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_error_id: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The error type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetLoadBasedAutoScalingRequest {
    /// <p>An <code>AutoScalingThresholds</code> object with the downscaling threshold configuration. If the load falls below these thresholds for a specified amount of time, AWS OpsWorks Stacks stops a specified number of instances.</p>
    #[serde(rename = "DownScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_scaling: Option<AutoScalingThresholds>,
    /// <p>Enables load-based auto scaling for the layer.</p>
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
    /// <p>An <code>AutoScalingThresholds</code> object with the upscaling threshold configuration. If the load exceeds these thresholds for a specified amount of time, AWS OpsWorks Stacks starts a specified number of instances.</p>
    #[serde(rename = "UpScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_scaling: Option<AutoScalingThresholds>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetPermissionRequest {
    /// <p>The user is allowed to use SSH to communicate with the instance.</p>
    #[serde(rename = "AllowSsh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ssh: Option<bool>,
    /// <p>The user is allowed to use <b>sudo</b> to elevate privileges.</p>
    #[serde(rename = "AllowSudo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sudo: Option<bool>,
    /// <p>The user's IAM ARN. This can also be a federated user's ARN.</p>
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: String,
    /// <p>The user's permission level, which must be set to one of the following strings. You cannot set your own permissions level.</p> <ul> <li> <p> <code>deny</code> </p> </li> <li> <p> <code>show</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>manage</code> </p> </li> <li> <p> <code>iam_only</code> </p> </li> </ul> <p>For more information about the permissions associated with these levels, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetTimeBasedAutoScalingRequest {
    /// <p>An <code>AutoScalingSchedule</code> with the instance schedule.</p>
    #[serde(rename = "AutoScalingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_schedule: Option<WeeklyAutoScalingSchedule>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

/// <p>The Shutdown event configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShutdownEventConfiguration {
    /// <p>Whether to enable Elastic Load Balancing connection draining. For more information, see <a href="https://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/TerminologyandKeyConcepts.html#conn-drain">Connection Draining</a> </p>
    #[serde(rename = "DelayUntilElbConnectionsDrained")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_until_elb_connections_drained: Option<bool>,
    /// <p>The time, in seconds, that AWS OpsWorks Stacks will wait after triggering a Shutdown event before shutting down an instance.</p>
    #[serde(rename = "ExecutionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<i64>,
}

/// <p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Creating Apps</a> or <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html">Custom Recipes and Cookbooks</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// <p>When included in a request, the parameter depends on the repository type.</p> <ul> <li> <p>For Amazon S3 bundles, set <code>Password</code> to the appropriate IAM secret access key.</p> </li> <li> <p>For HTTP bundles and Subversion repositories, set <code>Password</code> to the password.</p> </li> </ul> <p>For more information on how to safely handle IAM credentials, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html">https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html</a>.</p> <p>In responses, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The application's version. AWS OpsWorks Stacks enables you to easily deploy new versions of an application. One of the simplest approaches is to have branches or revisions in your repository that represent different versions that can potentially be deployed.</p>
    #[serde(rename = "Revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// <p>In requests, the repository's SSH key.</p> <p>In responses, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>
    #[serde(rename = "SshKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    /// <p>The repository type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The source URL. The following is an example of an Amazon S3 source URL: <code>https://s3.amazonaws.com/opsworks-demo-bucket/opsworks_cookbook_demo.tar.gz</code>.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p><p>This parameter depends on the repository type.</p> <ul> <li> <p>For Amazon S3 bundles, set <code>Username</code> to the appropriate IAM access key ID.</p> </li> <li> <p>For HTTP bundles, Git repositories, and Subversion repositories, set <code>Username</code> to the user name.</p> </li> </ul></p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes an app's SSL configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SslConfiguration {
    /// <p>The contents of the certificate's domain.crt file.</p>
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// <p>Optional. Can be used to specify an intermediate certificate authority key or client authentication.</p>
    #[serde(rename = "Chain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// <p>The private key; the contents of the certificate's domain.kex file.</p>
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
}

/// <p>Describes a stack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Stack {
    /// <p>The agent version. This parameter is set to <code>LATEST</code> for auto-update. or a version number for a fixed agent version.</p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The stack's ARN.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The stack's attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "ChefConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    /// <p>The configuration manager.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>The date when the stack was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Adding Apps</a> or <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html">Cookbooks and Recipes</a>.</p>
    #[serde(rename = "CustomCookbooksSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    /// <p>A JSON object that contains user-defined attributes to be added to the stack configuration and deployment attributes. You can use custom JSON to override the corresponding default stack configuration attribute values or to pass data to recipes. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information on custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The stack's default Availability Zone. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "DefaultAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<String>,
    /// <p>The ARN of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    /// <p>The stack's default operating system.</p>
    #[serde(rename = "DefaultOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_os: Option<String>,
    /// <p>The default root device type. This value is used by default for all instances in the stack, but you can override it when you create an instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "DefaultRootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<String>,
    /// <p>A default Amazon EC2 key pair for the stack's instances. You can override this value when you create or update an instance.</p>
    #[serde(rename = "DefaultSshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    /// <p>The default subnet ID; applicable only if the stack is running in a VPC.</p>
    #[serde(rename = "DefaultSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<String>,
    /// <p>The stack host name theme, with spaces replaced by underscores.</p>
    #[serde(rename = "HostnameTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<String>,
    /// <p>The stack name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The stack AWS region, such as "ap-northeast-2". For more information about AWS regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The stack AWS Identity and Access Management (IAM) role.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>Whether the stack uses custom cookbooks.</p>
    #[serde(rename = "UseCustomCookbooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    /// <p>Whether the stack automatically associates the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p>
    #[serde(rename = "UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    /// <p>The VPC ID; applicable only if the stack is running in a VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Describes the configuration manager.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackConfigurationManager {
    /// <p>The name. This parameter must be set to "Chef".</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Chef version. This parameter must be set to 12, 11.10, or 11.4 for Linux stacks, and to 12.2 for Windows stacks. The default value for Linux stacks is 11.4.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Summarizes the number of layers, instances, and apps in a stack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StackSummary {
    /// <p>The number of apps.</p>
    #[serde(rename = "AppsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_count: Option<i64>,
    /// <p>The stack's ARN.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An <code>InstancesCount</code> object with the number of instances in each status.</p>
    #[serde(rename = "InstancesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_count: Option<InstancesCount>,
    /// <p>The number of layers.</p>
    #[serde(rename = "LayersCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers_count: Option<i64>,
    /// <p>The stack name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartInstanceRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartStackRequest {
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopInstanceRequest {
    /// <p>Specifies whether to force an instance to stop. If the instance's root device type is <code>ebs</code>, or EBS-backed, adding the <code>Force</code> parameter to the <code>StopInstances</code> API call disassociates the AWS OpsWorks Stacks instance from EC2, and forces deletion of <i>only</i> the OpsWorks Stacks instance. You must also delete the formerly-associated instance in EC2 after troubleshooting and replacing the AWS OpsWorks Stacks instance with a new one.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopStackRequest {
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The stack or layer's Amazon Resource Number (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p><p>A map that contains tag keys and tag values that are attached to a stack or layer.</p> <ul> <li> <p>The key cannot be empty.</p> </li> <li> <p>The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>Leading and trailing white spaces are trimmed from both the key and value.</p> </li> <li> <p>A maximum of 40 tags is allowed for any resource.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Contains the data needed by RDP clients such as the Microsoft Remote Desktop Connection to log in to the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemporaryCredential {
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>
    #[serde(rename = "ValidForInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for_in_minutes: Option<i64>,
}

/// <p>Describes an instance's time-based auto scaling configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TimeBasedAutoScalingConfiguration {
    /// <p>A <code>WeeklyAutoScalingSchedule</code> object with the instance schedule.</p>
    #[serde(rename = "AutoScalingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_schedule: Option<WeeklyAutoScalingSchedule>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnassignInstanceRequest {
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnassignVolumeRequest {
    /// <p>The volume ID.</p>
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The stack or layer's Amazon Resource Number (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of the keys of tags to be removed from a stack or layer.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAppRequest {
    /// <p>The app ID.</p>
    #[serde(rename = "AppId")]
    pub app_id: String,
    /// <p>A <code>Source</code> object that specifies the app repository.</p>
    #[serde(rename = "AppSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_source: Option<Source>,
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app's data sources.</p>
    #[serde(rename = "DataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>A description of the app.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The app's virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// <p>Whether SSL is enabled for the app.</p>
    #[serde(rename = "EnableSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<bool>,
    /// <p><p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances.For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment"> Environment Variables</a>.</p> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables&#39; names, values, and protected flag values - cannot exceed 20 KB. This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, &quot;Environment: is too large (maximum is 20 KB).&quot;</p> <note> <p>If you have specified one or more environment variables, you cannot modify the stack&#39;s Chef version.</p> </note></p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    /// <p>The app name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An <code>SslConfiguration</code> object with the SSL configuration.</p>
    #[serde(rename = "SslConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    /// <p>The app type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateElasticIpRequest {
    /// <p>The IP address for which you want to update the name.</p>
    #[serde(rename = "ElasticIp")]
    pub elastic_ip: String,
    /// <p>The new name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstanceRequest {
    /// <p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p> <code>INHERIT</code> - Use the stack's default agent version setting.</p> </li> <li> <p> <i>version_number</i> - Use the specified agent version. This value overrides the stack's default setting. To update the agent version, you must edit the instance configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the instance.</p> </li> </ul> <p>The default setting is <code>INHERIT</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>.</p> <p>AgentVersion cannot be set to Chef 12.2.</p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The ID of the AMI that was used to create the instance. The value of this parameter must be the same AMI ID that the instance is already using. You cannot apply a new AMI to an instance by running UpdateInstance. UpdateInstance does not work on instances that are using custom AMIs. </p>
    #[serde(rename = "AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The instance architecture. Instance types do not necessarily support both architectures. For a list of the architectures that are supported by the different instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance Families and Types</a>.</p>
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>For load-based or time-based instances, the type. Windows stacks can use only time-based instances.</p>
    #[serde(rename = "AutoScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<String>,
    /// <p>This property cannot be updated.</p>
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// <p>The instance host name.</p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The instance type, such as <code>t2.micro</code>. For a list of supported instance types, open the stack in the console, choose <b>Instances</b>, and choose <b>+ Instance</b>. The <b>Size</b> list contains the currently supported types. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance Families and Types</a>. The parameter values that you use to specify the various types are in the <b>API Name</b> column of the <b>Available Instance Types</b> table.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The instance's layer IDs.</p>
    #[serde(rename = "LayerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    /// <p><p>The instance&#39;s operating system, which must be set to one of the following. You cannot update an instance that is using a custom AMI.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2018.03</code>, <code>Amazon Linux 2017.09</code>, <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> </ul> <p>For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">AWS OpsWorks Stacks Operating Systems</a>.</p> <p>The default option is the current Amazon Linux version. If you set this parameter to <code>Custom</code>, you must use the AmiId parameter to specify the custom AMI that you want to use. For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">Operating Systems</a>. For more information about how to use custom AMIs with OpsWorks, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Using Custom AMIs</a>.</p> <note> <p>You can specify a different Linux operating system for the updated stack, but you cannot change from Linux to Windows or Windows to Linux.</p> </note></p>
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// <p>The instance's Amazon EC2 key name.</p>
    #[serde(rename = "SshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLayerRequest {
    /// <p>One or more user-defined key/value pairs to be added to the stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Whether to automatically assign an <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP address</a> to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    /// <p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html">How to Edit a Layer</a>.</p>
    #[serde(rename = "AutoAssignPublicIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    /// <p>Specifies CloudWatch Logs configuration options for the layer. For more information, see <a>CloudWatchLogsLogStream</a>.</p>
    #[serde(rename = "CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    /// <p>The ARN of an IAM profile to be used for all of the layer's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "CustomInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    /// <p>A JSON-formatted string containing custom stack configuration and deployment attributes to be installed on the layer's instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html"> Using Custom JSON</a>. </p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>A <code>LayerCustomRecipes</code> object that specifies the layer's custom recipes.</p>
    #[serde(rename = "CustomRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    /// <p>An array containing the layer's custom security group IDs.</p>
    #[serde(rename = "CustomSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    /// <p>Whether to disable auto healing for the layer.</p>
    #[serde(rename = "EnableAutoHealing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    /// <p><p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note></p>
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// <p>The layer ID.</p>
    #[serde(rename = "LayerId")]
    pub layer_id: String,
    /// <p><p/></p>
    #[serde(rename = "LifecycleEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    /// <p>The layer name, which is used by the console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of <code>Package</code> objects that describe the layer's packages.</p>
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// <p>For custom layers only, use this parameter to specify the layer's short name, which is used internally by AWS OpsWorks Stacks and by Chef. The short name is also used as the name for the directory where your app files are installed. It can have a maximum of 200 characters and must be in the following format: /\A[a-z0-9\-\_\.]+\Z/.</p> <p>The built-in layers' short names are defined by AWS OpsWorks Stacks. For more information, see the <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/layers.html">Layer Reference</a> </p>
    #[serde(rename = "Shortname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// <p>Whether to use Amazon EBS-optimized instances.</p>
    #[serde(rename = "UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    /// <p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>
    #[serde(rename = "VolumeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMyUserProfileRequest {
    /// <p>The user's SSH public key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRdsDbInstanceRequest {
    /// <p>The database password.</p>
    #[serde(rename = "DbPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    /// <p>The master user name.</p>
    #[serde(rename = "DbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    /// <p>The Amazon RDS instance's ARN.</p>
    #[serde(rename = "RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStackRequest {
    /// <p><p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack&#39;s instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack&#39;s instances.</p> </li> </ul> <p>The default setting is <code>LATEST</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack&#39;s default setting.</p> </note></p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>One or more user-defined key-value pairs to be added to the stack attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "ChefConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    /// <p>The configuration manager. When you update a stack, we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 12.</p>
    #[serde(rename = "ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// <p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Adding Apps</a> or <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html">Cookbooks and Recipes</a>.</p>
    #[serde(rename = "CustomCookbooksSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    /// <p>A string that contains user-defined, custom JSON. It can be used to override the corresponding default stack configuration JSON values or to pass data to recipes. The string should be in the following format:</p> <p> <code>"{\"key1\": \"value1\", \"key2\": \"value2\",...}"</code> </p> <p>For more information about custom JSON, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>
    #[serde(rename = "CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<String>,
    /// <p>The stack's default Availability Zone, which must be in the stack's region. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see <a>CreateStack</a>. </p>
    #[serde(rename = "DefaultAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<String>,
    /// <p>The ARN of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">Using Identifiers</a>.</p>
    #[serde(rename = "DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    /// <p>The stack's operating system, which must be set to one of the following:</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2018.03</code>, <code>Amazon Linux 2017.09</code>, <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information about how to use custom AMIs with OpsWorks, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html">Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the stack's current operating system. For more information about supported operating systems, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html">AWS OpsWorks Stacks Operating Systems</a>.</p>
    #[serde(rename = "DefaultOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_os: Option<String>,
    /// <p>The default root device type. This value is used by default for all instances in the stack, but you can override it when you create an instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device">Storage for the Root Device</a>.</p>
    #[serde(rename = "DefaultRootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<String>,
    /// <p>A default Amazon EC2 key-pair name. The default value is <code>none</code>. If you specify a key-pair name, AWS OpsWorks Stacks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html"> Using SSH to Communicate with an Instance</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html"> create an instance</a>. </p>
    #[serde(rename = "DefaultSshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    /// <p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>
    #[serde(rename = "DefaultSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<String>,
    /// <p>The stack's new host name theme, with spaces replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities_and_Titans</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>
    #[serde(rename = "HostnameTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<String>,
    /// <p>The stack's new name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Do not use this parameter. You cannot update a stack's service role.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The stack ID.</p>
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// <p>Whether the stack uses custom cookbooks.</p>
    #[serde(rename = "UseCustomCookbooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    /// <p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. <code>UseOpsworksSecurityGroups</code> allows you to provide your own custom security groups instead of using the built-in groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it, but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate EC2 security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on. Custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html">Create a New Stack</a>.</p>
    #[serde(rename = "UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserProfileRequest {
    /// <p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html">Managing User Permissions</a>.</p>
    #[serde(rename = "AllowSelfManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<bool>,
    /// <p>The user IAM ARN. This can also be a federated user's ARN.</p>
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: String,
    /// <p>The user's new SSH public key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The user's SSH user name. The allowable characters are [a-z], [A-Z], [0-9], '-', and '_'. If the specified name includes other punctuation marks, AWS OpsWorks Stacks removes them. For example, <code>my.name</code> will be changed to <code>myname</code>. If you do not specify an SSH user name, AWS OpsWorks Stacks generates one from the IAM user name. </p>
    #[serde(rename = "SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVolumeRequest {
    /// <p>The new mount point.</p>
    #[serde(rename = "MountPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    /// <p>The new name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The volume ID.</p>
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

/// <p>Describes a user's SSH information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserProfile {
    /// <p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html">Managing User Permissions</a>.</p>
    #[serde(rename = "AllowSelfManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<bool>,
    /// <p>The user's IAM ARN.</p>
    #[serde(rename = "IamUserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    /// <p>The user's name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The user's SSH public key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The user's SSH user name.</p>
    #[serde(rename = "SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
}

/// <p>Describes an instance's Amazon EBS volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Volume {
    /// <p>The volume Availability Zone. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The device name.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// <p>The Amazon EC2 volume ID.</p>
    #[serde(rename = "Ec2VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_volume_id: Option<String>,
    /// <p>Specifies whether an Amazon EBS volume is encrypted. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS Encryption</a>.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>For PIOPS volumes, the IOPS per disk.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The volume mount point. For example, "/mnt/disk1".</p>
    #[serde(rename = "MountPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    /// <p>The volume name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The RAID array ID.</p>
    #[serde(rename = "RaidArrayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_array_id: Option<String>,
    /// <p>The AWS region. For more information about AWS regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html">Regions and Endpoints</a>.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The volume size.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>The value returned by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeVolumes.html">DescribeVolumes</a>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The volume ID.</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p><p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html"> Amazon EBS Volume Types</a>.</p> <ul> <li> <p> <code>standard</code> - Magnetic. Magnetic volumes must have a minimum size of 1 GiB and a maximum size of 1024 GiB.</p> </li> <li> <p> <code>io1</code> - Provisioned IOPS (SSD). PIOPS volumes must have a minimum size of 4 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>gp2</code> - General Purpose (SSD). General purpose volumes must have a minimum size of 1 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>st1</code> - Throughput Optimized hard disk drive (HDD). Throughput optimized HDD volumes must have a minimum size of 500 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>sc1</code> - Cold HDD. Cold HDD volumes must have a minimum size of 500 GiB and a maximum size of 16384 GiB.</p> </li> </ul></p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes an Amazon EBS volume configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeConfiguration {
    /// <p>Specifies whether an Amazon EBS volume is encrypted. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS Encryption</a>.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>For PIOPS volumes, the IOPS per disk.</p>
    #[serde(rename = "Iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>The volume mount point. For example "/dev/sdh".</p>
    #[serde(rename = "MountPoint")]
    pub mount_point: String,
    /// <p>The number of disks in the volume.</p>
    #[serde(rename = "NumberOfDisks")]
    pub number_of_disks: i64,
    /// <p>The volume <a href="http://en.wikipedia.org/wiki/Standard_RAID_levels">RAID level</a>.</p>
    #[serde(rename = "RaidLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_level: Option<i64>,
    /// <p>The volume size.</p>
    #[serde(rename = "Size")]
    pub size: i64,
    /// <p><p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html"> Amazon EBS Volume Types</a>.</p> <ul> <li> <p> <code>standard</code> - Magnetic. Magnetic volumes must have a minimum size of 1 GiB and a maximum size of 1024 GiB.</p> </li> <li> <p> <code>io1</code> - Provisioned IOPS (SSD). PIOPS volumes must have a minimum size of 4 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>gp2</code> - General Purpose (SSD). General purpose volumes must have a minimum size of 1 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>st1</code> - Throughput Optimized hard disk drive (HDD). Throughput optimized HDD volumes must have a minimum size of 500 GiB and a maximum size of 16384 GiB.</p> </li> <li> <p> <code>sc1</code> - Cold HDD. Cold HDD volumes must have a minimum size of 500 GiB and a maximum size of 16384 GiB.</p> </li> </ul></p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes a time-based instance's auto scaling schedule. The schedule consists of a set of key-value pairs.</p> <ul> <li> <p>The key is the time period (a UTC hour) and must be an integer from 0 - 23.</p> </li> <li> <p>The value indicates whether the instance should be online or offline for the specified period, and must be set to "on" or "off"</p> </li> </ul> <p>The default setting for all time periods is off, so you use the following parameters primarily to specify the online periods. You don't have to explicitly specify offline periods unless you want to change an online period to an offline period.</p> <p>The following example specifies that the instance should be online for four hours, from UTC 1200 - 1600. It will be off for the remainder of the day.</p> <p> <code> { "12":"on", "13":"on", "14":"on", "15":"on" } </code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeeklyAutoScalingSchedule {
    /// <p>The schedule for Friday.</p>
    #[serde(rename = "Friday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Monday.</p>
    #[serde(rename = "Monday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Saturday.</p>
    #[serde(rename = "Saturday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Sunday.</p>
    #[serde(rename = "Sunday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Thursday.</p>
    #[serde(rename = "Thursday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Tuesday.</p>
    #[serde(rename = "Tuesday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schedule for Wednesday.</p>
    #[serde(rename = "Wednesday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by AssignInstance
#[derive(Debug, PartialEq)]
pub enum AssignInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl AssignInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssignInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssignInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssignInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssignInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssignInstanceError {}
/// Errors returned by AssignVolume
#[derive(Debug, PartialEq)]
pub enum AssignVolumeError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl AssignVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssignVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssignVolumeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssignVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssignVolumeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssignVolumeError {}
/// Errors returned by AssociateElasticIp
#[derive(Debug, PartialEq)]
pub enum AssociateElasticIpError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl AssociateElasticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateElasticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateElasticIpError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateElasticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateElasticIpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateElasticIpError {}
/// Errors returned by AttachElasticLoadBalancer
#[derive(Debug, PartialEq)]
pub enum AttachElasticLoadBalancerError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl AttachElasticLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachElasticLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AttachElasticLoadBalancerError::ResourceNotFound(
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
impl fmt::Display for AttachElasticLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachElasticLoadBalancerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachElasticLoadBalancerError {}
/// Errors returned by CloneStack
#[derive(Debug, PartialEq)]
pub enum CloneStackError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl CloneStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CloneStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CloneStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CloneStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CloneStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CloneStackError {}
/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAppError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAppError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAppError {}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDeploymentError::ResourceNotFound(err.msg))
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
            CreateDeploymentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeploymentError {}
/// Errors returned by CreateInstance
#[derive(Debug, PartialEq)]
pub enum CreateInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstanceError {}
/// Errors returned by CreateLayer
#[derive(Debug, PartialEq)]
pub enum CreateLayerError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateLayerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLayerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLayerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLayerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLayerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLayerError {}
/// Errors returned by CreateStack
#[derive(Debug, PartialEq)]
pub enum CreateStackError {}

impl CreateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateStackError {}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {}

impl CreateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateUserProfileError {}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAppError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppError {}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInstanceError {}
/// Errors returned by DeleteLayer
#[derive(Debug, PartialEq)]
pub enum DeleteLayerError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteLayerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLayerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLayerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLayerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLayerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLayerError {}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStackError {}
/// Errors returned by DeleteUserProfile
#[derive(Debug, PartialEq)]
pub enum DeleteUserProfileError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserProfileError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserProfileError {}
/// Errors returned by DeregisterEcsCluster
#[derive(Debug, PartialEq)]
pub enum DeregisterEcsClusterError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeregisterEcsClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterEcsClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterEcsClusterError::ResourceNotFound(
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
impl fmt::Display for DeregisterEcsClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterEcsClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterEcsClusterError {}
/// Errors returned by DeregisterElasticIp
#[derive(Debug, PartialEq)]
pub enum DeregisterElasticIpError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeregisterElasticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterElasticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterElasticIpError::ResourceNotFound(
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
impl fmt::Display for DeregisterElasticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterElasticIpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterElasticIpError {}
/// Errors returned by DeregisterInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeregisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterInstanceError {}
/// Errors returned by DeregisterRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterRdsDbInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeregisterRdsDbInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterRdsDbInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterRdsDbInstanceError::ResourceNotFound(
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
impl fmt::Display for DeregisterRdsDbInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterRdsDbInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterRdsDbInstanceError {}
/// Errors returned by DeregisterVolume
#[derive(Debug, PartialEq)]
pub enum DeregisterVolumeError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DeregisterVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterVolumeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterVolumeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterVolumeError {}
/// Errors returned by DescribeAgentVersions
#[derive(Debug, PartialEq)]
pub enum DescribeAgentVersionsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeAgentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAgentVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAgentVersionsError::ResourceNotFound(
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
impl fmt::Display for DescribeAgentVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAgentVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAgentVersionsError {}
/// Errors returned by DescribeApps
#[derive(Debug, PartialEq)]
pub enum DescribeAppsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAppsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAppsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAppsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAppsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAppsError {}
/// Errors returned by DescribeCommands
#[derive(Debug, PartialEq)]
pub enum DescribeCommandsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeCommandsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCommandsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCommandsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCommandsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCommandsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCommandsError {}
/// Errors returned by DescribeDeployments
#[derive(Debug, PartialEq)]
pub enum DescribeDeploymentsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeploymentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDeploymentsError::ResourceNotFound(
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
impl fmt::Display for DescribeDeploymentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeploymentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDeploymentsError {}
/// Errors returned by DescribeEcsClusters
#[derive(Debug, PartialEq)]
pub enum DescribeEcsClustersError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeEcsClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEcsClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEcsClustersError::ResourceNotFound(
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
impl fmt::Display for DescribeEcsClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEcsClustersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEcsClustersError {}
/// Errors returned by DescribeElasticIps
#[derive(Debug, PartialEq)]
pub enum DescribeElasticIpsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeElasticIpsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeElasticIpsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeElasticIpsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeElasticIpsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticIpsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeElasticIpsError {}
/// Errors returned by DescribeElasticLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeElasticLoadBalancersError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeElasticLoadBalancersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeElasticLoadBalancersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeElasticLoadBalancersError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeElasticLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeElasticLoadBalancersError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeElasticLoadBalancersError {}
/// Errors returned by DescribeInstances
#[derive(Debug, PartialEq)]
pub enum DescribeInstancesError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeInstancesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstancesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstancesError {}
/// Errors returned by DescribeLayers
#[derive(Debug, PartialEq)]
pub enum DescribeLayersError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeLayersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLayersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLayersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLayersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLayersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLayersError {}
/// Errors returned by DescribeLoadBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBasedAutoScalingError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeLoadBasedAutoScalingError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLoadBasedAutoScalingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeLoadBasedAutoScalingError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLoadBasedAutoScalingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoadBasedAutoScalingError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLoadBasedAutoScalingError {}
/// Errors returned by DescribeMyUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeMyUserProfileError {}

impl DescribeMyUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMyUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMyUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeMyUserProfileError {}
/// Errors returned by DescribeOperatingSystems
#[derive(Debug, PartialEq)]
pub enum DescribeOperatingSystemsError {}

impl DescribeOperatingSystemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOperatingSystemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOperatingSystemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeOperatingSystemsError {}
/// Errors returned by DescribePermissions
#[derive(Debug, PartialEq)]
pub enum DescribePermissionsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribePermissionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePermissionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePermissionsError::ResourceNotFound(
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
impl fmt::Display for DescribePermissionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePermissionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePermissionsError {}
/// Errors returned by DescribeRaidArrays
#[derive(Debug, PartialEq)]
pub enum DescribeRaidArraysError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeRaidArraysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRaidArraysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRaidArraysError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRaidArraysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRaidArraysError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRaidArraysError {}
/// Errors returned by DescribeRdsDbInstances
#[derive(Debug, PartialEq)]
pub enum DescribeRdsDbInstancesError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeRdsDbInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRdsDbInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRdsDbInstancesError::ResourceNotFound(
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
impl fmt::Display for DescribeRdsDbInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRdsDbInstancesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRdsDbInstancesError {}
/// Errors returned by DescribeServiceErrors
#[derive(Debug, PartialEq)]
pub enum DescribeServiceErrorsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeServiceErrorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServiceErrorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeServiceErrorsError::ResourceNotFound(
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
impl fmt::Display for DescribeServiceErrorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceErrorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServiceErrorsError {}
/// Errors returned by DescribeStackProvisioningParameters
#[derive(Debug, PartialEq)]
pub enum DescribeStackProvisioningParametersError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeStackProvisioningParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeStackProvisioningParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeStackProvisioningParametersError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStackProvisioningParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStackProvisioningParametersError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeStackProvisioningParametersError {}
/// Errors returned by DescribeStackSummary
#[derive(Debug, PartialEq)]
pub enum DescribeStackSummaryError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeStackSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStackSummaryError::ResourceNotFound(
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
impl fmt::Display for DescribeStackSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStackSummaryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStackSummaryError {}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeStacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStacksError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStacksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStacksError {}
/// Errors returned by DescribeTimeBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum DescribeTimeBasedAutoScalingError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeTimeBasedAutoScalingError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTimeBasedAutoScalingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeTimeBasedAutoScalingError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTimeBasedAutoScalingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTimeBasedAutoScalingError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTimeBasedAutoScalingError {}
/// Errors returned by DescribeUserProfiles
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfilesError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeUserProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserProfilesError::ResourceNotFound(
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
impl fmt::Display for DescribeUserProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserProfilesError {}
/// Errors returned by DescribeVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeVolumesError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeVolumesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVolumesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeVolumesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVolumesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVolumesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVolumesError {}
/// Errors returned by DetachElasticLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DetachElasticLoadBalancerError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DetachElasticLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachElasticLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetachElasticLoadBalancerError::ResourceNotFound(
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
impl fmt::Display for DetachElasticLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachElasticLoadBalancerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachElasticLoadBalancerError {}
/// Errors returned by DisassociateElasticIp
#[derive(Debug, PartialEq)]
pub enum DisassociateElasticIpError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl DisassociateElasticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateElasticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateElasticIpError::ResourceNotFound(
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
impl fmt::Display for DisassociateElasticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateElasticIpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateElasticIpError {}
/// Errors returned by GetHostnameSuggestion
#[derive(Debug, PartialEq)]
pub enum GetHostnameSuggestionError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl GetHostnameSuggestionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHostnameSuggestionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetHostnameSuggestionError::ResourceNotFound(
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
impl fmt::Display for GetHostnameSuggestionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHostnameSuggestionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHostnameSuggestionError {}
/// Errors returned by GrantAccess
#[derive(Debug, PartialEq)]
pub enum GrantAccessError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl GrantAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GrantAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GrantAccessError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GrantAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrantAccessError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GrantAccessError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by RebootInstance
#[derive(Debug, PartialEq)]
pub enum RebootInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RebootInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RebootInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebootInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RebootInstanceError {}
/// Errors returned by RegisterEcsCluster
#[derive(Debug, PartialEq)]
pub enum RegisterEcsClusterError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RegisterEcsClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterEcsClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterEcsClusterError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterEcsClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterEcsClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterEcsClusterError {}
/// Errors returned by RegisterElasticIp
#[derive(Debug, PartialEq)]
pub enum RegisterElasticIpError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RegisterElasticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterElasticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterElasticIpError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterElasticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterElasticIpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterElasticIpError {}
/// Errors returned by RegisterInstance
#[derive(Debug, PartialEq)]
pub enum RegisterInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RegisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterInstanceError {}
/// Errors returned by RegisterRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum RegisterRdsDbInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RegisterRdsDbInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterRdsDbInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterRdsDbInstanceError::ResourceNotFound(
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
impl fmt::Display for RegisterRdsDbInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterRdsDbInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterRdsDbInstanceError {}
/// Errors returned by RegisterVolume
#[derive(Debug, PartialEq)]
pub enum RegisterVolumeError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl RegisterVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterVolumeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterVolumeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterVolumeError {}
/// Errors returned by SetLoadBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum SetLoadBasedAutoScalingError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl SetLoadBasedAutoScalingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetLoadBasedAutoScalingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetLoadBasedAutoScalingError::ResourceNotFound(
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
impl fmt::Display for SetLoadBasedAutoScalingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLoadBasedAutoScalingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetLoadBasedAutoScalingError {}
/// Errors returned by SetPermission
#[derive(Debug, PartialEq)]
pub enum SetPermissionError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl SetPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetPermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetPermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetPermissionError {}
/// Errors returned by SetTimeBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum SetTimeBasedAutoScalingError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl SetTimeBasedAutoScalingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetTimeBasedAutoScalingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetTimeBasedAutoScalingError::ResourceNotFound(
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
impl fmt::Display for SetTimeBasedAutoScalingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetTimeBasedAutoScalingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetTimeBasedAutoScalingError {}
/// Errors returned by StartInstance
#[derive(Debug, PartialEq)]
pub enum StartInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl StartInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartInstanceError {}
/// Errors returned by StartStack
#[derive(Debug, PartialEq)]
pub enum StartStackError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl StartStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartStackError {}
/// Errors returned by StopInstance
#[derive(Debug, PartialEq)]
pub enum StopInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl StopInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopInstanceError {}
/// Errors returned by StopStack
#[derive(Debug, PartialEq)]
pub enum StopStackError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl StopStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopStackError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UnassignInstance
#[derive(Debug, PartialEq)]
pub enum UnassignInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UnassignInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnassignInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnassignInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UnassignInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnassignInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnassignInstanceError {}
/// Errors returned by UnassignVolume
#[derive(Debug, PartialEq)]
pub enum UnassignVolumeError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UnassignVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnassignVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnassignVolumeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UnassignVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnassignVolumeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnassignVolumeError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAppError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAppError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAppError {}
/// Errors returned by UpdateElasticIp
#[derive(Debug, PartialEq)]
pub enum UpdateElasticIpError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateElasticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateElasticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateElasticIpError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateElasticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateElasticIpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateElasticIpError {}
/// Errors returned by UpdateInstance
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateInstanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInstanceError {}
/// Errors returned by UpdateLayer
#[derive(Debug, PartialEq)]
pub enum UpdateLayerError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateLayerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLayerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLayerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLayerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLayerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLayerError {}
/// Errors returned by UpdateMyUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateMyUserProfileError {}

impl UpdateMyUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMyUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMyUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateMyUserProfileError {}
/// Errors returned by UpdateRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum UpdateRdsDbInstanceError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateRdsDbInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRdsDbInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRdsDbInstanceError::ResourceNotFound(
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
impl fmt::Display for UpdateRdsDbInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRdsDbInstanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRdsDbInstanceError {}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateStackError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStackError {}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserProfileError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserProfileError {}
/// Errors returned by UpdateVolume
#[derive(Debug, PartialEq)]
pub enum UpdateVolumeError {
    /// <p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateVolumeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVolumeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVolumeError {}
/// Trait representing the capabilities of the AWS OpsWorks API. AWS OpsWorks clients implement this trait.
#[async_trait]
pub trait OpsWorks {
    /// <p>Assign a registered instance to a layer.</p> <ul> <li> <p>You can assign registered on-premises instances to any layer type.</p> </li> <li> <p>You can assign registered Amazon EC2 instances only to custom layers.</p> </li> <li> <p>You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> </li> </ul> <p> <b>Required Permissions</b>: To use this action, an AWS Identity and Access Management (IAM) user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn assign_instance(
        &self,
        input: AssignInstanceRequest,
    ) -> Result<(), RusotoError<AssignInstanceError>>;

    /// <p>Assigns one of the stack's registered Amazon EBS volumes to a specified instance. The volume must first be registered with the stack by calling <a>RegisterVolume</a>. After you register the volume, you must call <a>UpdateVolume</a> to specify a mount point before calling <code>AssignVolume</code>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn assign_volume(
        &self,
        input: AssignVolumeRequest,
    ) -> Result<(), RusotoError<AssignVolumeError>>;

    /// <p>Associates one of the stack's registered Elastic IP addresses with a specified instance. The address must first be registered with the stack by calling <a>RegisterElasticIp</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn associate_elastic_ip(
        &self,
        input: AssociateElasticIpRequest,
    ) -> Result<(), RusotoError<AssociateElasticIpError>>;

    /// <p>Attaches an Elastic Load Balancing load balancer to a specified layer. AWS OpsWorks Stacks does not support Application Load Balancer. You can only use Classic Load Balancer with AWS OpsWorks Stacks. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/layers-elb.html">Elastic Load Balancing</a>.</p> <note> <p>You must create the Elastic Load Balancing instance separately, by using the Elastic Load Balancing console, API, or CLI. For more information, see <a href="https://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/Welcome.html"> Elastic Load Balancing Developer Guide</a>.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn attach_elastic_load_balancer(
        &self,
        input: AttachElasticLoadBalancerRequest,
    ) -> Result<(), RusotoError<AttachElasticLoadBalancerError>>;

    /// <p>Creates a clone of a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-cloning.html">Clone a Stack</a>. By default, all parameters are set to the values used by the parent stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn clone_stack(
        &self,
        input: CloneStackRequest,
    ) -> Result<CloneStackResult, RusotoError<CloneStackError>>;

    /// <p>Creates an app for a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Creating Apps</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResult, RusotoError<CreateAppError>>;

    /// <p>Runs deployment or stack commands. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html">Deploying Apps</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html">Run Stack Commands</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResult, RusotoError<CreateDeploymentError>>;

    /// <p>Creates an instance in a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html">Adding an Instance to a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_instance(
        &self,
        input: CreateInstanceRequest,
    ) -> Result<CreateInstanceResult, RusotoError<CreateInstanceError>>;

    /// <p>Creates a layer. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-create.html">How to Create a Layer</a>.</p> <note> <p>You should use <b>CreateLayer</b> for noncustom layer types such as PHP App Server only if the stack does not have an existing layer of that type. A stack can have at most one instance of each noncustom layer; if you attempt to create a second instance, <b>CreateLayer</b> fails. A stack can have an arbitrary number of custom layers, so you can call <b>CreateLayer</b> as many times as you like for that layer type.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_layer(
        &self,
        input: CreateLayerRequest,
    ) -> Result<CreateLayerResult, RusotoError<CreateLayerError>>;

    /// <p>Creates a new stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-edit.html">Create a New Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> Result<CreateStackResult, RusotoError<CreateStackError>>;

    /// <p>Creates a new user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResult, RusotoError<CreateUserProfileError>>;

    /// <p>Deletes a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_app(&self, input: DeleteAppRequest) -> Result<(), RusotoError<DeleteAppError>>;

    /// <p>Deletes a specified instance, which terminates the associated Amazon EC2 instance. You must stop an instance before you can delete it.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-delete.html">Deleting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<(), RusotoError<DeleteInstanceError>>;

    /// <p>Deletes a specified layer. You must first stop and then delete all associated instances or unassign registered instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-delete.html">How to Delete a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_layer(
        &self,
        input: DeleteLayerRequest,
    ) -> Result<(), RusotoError<DeleteLayerError>>;

    /// <p>Deletes a specified stack. You must first delete all instances, layers, and apps or deregister registered instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-shutting.html">Shut Down a Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> Result<(), RusotoError<DeleteStackError>>;

    /// <p>Deletes a user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<(), RusotoError<DeleteUserProfileError>>;

    /// <p>Deregisters a specified Amazon ECS cluster from a stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html#workinglayers-ecscluster-delete"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html</a>.</p>
    async fn deregister_ecs_cluster(
        &self,
        input: DeregisterEcsClusterRequest,
    ) -> Result<(), RusotoError<DeregisterEcsClusterError>>;

    /// <p>Deregisters a specified Elastic IP address. The address can then be registered by another stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_elastic_ip(
        &self,
        input: DeregisterElasticIpRequest,
    ) -> Result<(), RusotoError<DeregisterElasticIpError>>;

    /// <p>Deregister a registered Amazon EC2 or on-premises instance. This action removes the instance from the stack and returns it to your control. This action cannot be used with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> Result<(), RusotoError<DeregisterInstanceError>>;

    /// <p>Deregisters an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_rds_db_instance(
        &self,
        input: DeregisterRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<DeregisterRdsDbInstanceError>>;

    /// <p>Deregisters an Amazon EBS volume. The volume can then be registered by another stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_volume(
        &self,
        input: DeregisterVolumeRequest,
    ) -> Result<(), RusotoError<DeregisterVolumeError>>;

    /// <p>Describes the available AWS OpsWorks Stacks agent versions. You must specify a stack ID or a configuration manager. <code>DescribeAgentVersions</code> returns a list of available agent versions for the specified stack or configuration manager.</p>
    async fn describe_agent_versions(
        &self,
        input: DescribeAgentVersionsRequest,
    ) -> Result<DescribeAgentVersionsResult, RusotoError<DescribeAgentVersionsError>>;

    /// <p>Requests a description of a specified set of apps.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_apps(
        &self,
        input: DescribeAppsRequest,
    ) -> Result<DescribeAppsResult, RusotoError<DescribeAppsError>>;

    /// <p>Describes the results of specified commands.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_commands(
        &self,
        input: DescribeCommandsRequest,
    ) -> Result<DescribeCommandsResult, RusotoError<DescribeCommandsError>>;

    /// <p>Requests a description of a specified set of deployments.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_deployments(
        &self,
        input: DescribeDeploymentsRequest,
    ) -> Result<DescribeDeploymentsResult, RusotoError<DescribeDeploymentsError>>;

    /// <p>Describes Amazon ECS clusters that are registered with a stack. If you specify only a stack ID, you can use the <code>MaxResults</code> and <code>NextToken</code> parameters to paginate the response. However, AWS OpsWorks Stacks currently supports only one cluster per layer, so the result set has a maximum of one element.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permission. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_ecs_clusters(
        &self,
        input: DescribeEcsClustersRequest,
    ) -> Result<DescribeEcsClustersResult, RusotoError<DescribeEcsClustersError>>;

    /// <p>Describes <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP addresses</a>.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_elastic_ips(
        &self,
        input: DescribeElasticIpsRequest,
    ) -> Result<DescribeElasticIpsResult, RusotoError<DescribeElasticIpsError>>;

    /// <p>Describes a stack's Elastic Load Balancing instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_elastic_load_balancers(
        &self,
        input: DescribeElasticLoadBalancersRequest,
    ) -> Result<DescribeElasticLoadBalancersResult, RusotoError<DescribeElasticLoadBalancersError>>;

    /// <p>Requests a description of a set of instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_instances(
        &self,
        input: DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResult, RusotoError<DescribeInstancesError>>;

    /// <p>Requests a description of one or more layers in a specified stack.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_layers(
        &self,
        input: DescribeLayersRequest,
    ) -> Result<DescribeLayersResult, RusotoError<DescribeLayersError>>;

    /// <p>Describes load-based auto scaling configurations for specified layers.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_load_based_auto_scaling(
        &self,
        input: DescribeLoadBasedAutoScalingRequest,
    ) -> Result<DescribeLoadBasedAutoScalingResult, RusotoError<DescribeLoadBasedAutoScalingError>>;

    /// <p>Describes a user's SSH information.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_my_user_profile(
        &self,
    ) -> Result<DescribeMyUserProfileResult, RusotoError<DescribeMyUserProfileError>>;

    /// <p>Describes the operating systems that are supported by AWS OpsWorks Stacks.</p>
    async fn describe_operating_systems(
        &self,
    ) -> Result<DescribeOperatingSystemsResponse, RusotoError<DescribeOperatingSystemsError>>;

    /// <p>Describes the permissions for a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_permissions(
        &self,
        input: DescribePermissionsRequest,
    ) -> Result<DescribePermissionsResult, RusotoError<DescribePermissionsError>>;

    /// <p>Describe an instance's RAID arrays.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_raid_arrays(
        &self,
        input: DescribeRaidArraysRequest,
    ) -> Result<DescribeRaidArraysResult, RusotoError<DescribeRaidArraysError>>;

    /// <p>Describes Amazon RDS instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_rds_db_instances(
        &self,
        input: DescribeRdsDbInstancesRequest,
    ) -> Result<DescribeRdsDbInstancesResult, RusotoError<DescribeRdsDbInstancesError>>;

    /// <p>Describes AWS OpsWorks Stacks service errors.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_service_errors(
        &self,
        input: DescribeServiceErrorsRequest,
    ) -> Result<DescribeServiceErrorsResult, RusotoError<DescribeServiceErrorsError>>;

    /// <p>Requests a description of a stack's provisioning parameters.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stack_provisioning_parameters(
        &self,
        input: DescribeStackProvisioningParametersRequest,
    ) -> Result<
        DescribeStackProvisioningParametersResult,
        RusotoError<DescribeStackProvisioningParametersError>,
    >;

    /// <p>Describes the number of layers and apps in a specified stack, and the number of instances in each state, such as <code>running_setup</code> or <code>online</code>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stack_summary(
        &self,
        input: DescribeStackSummaryRequest,
    ) -> Result<DescribeStackSummaryResult, RusotoError<DescribeStackSummaryError>>;

    /// <p>Requests a description of one or more stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> Result<DescribeStacksResult, RusotoError<DescribeStacksError>>;

    /// <p>Describes time-based auto scaling configurations for specified instances.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_time_based_auto_scaling(
        &self,
        input: DescribeTimeBasedAutoScalingRequest,
    ) -> Result<DescribeTimeBasedAutoScalingResult, RusotoError<DescribeTimeBasedAutoScalingError>>;

    /// <p>Describe specified users.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_user_profiles(
        &self,
        input: DescribeUserProfilesRequest,
    ) -> Result<DescribeUserProfilesResult, RusotoError<DescribeUserProfilesError>>;

    /// <p>Describes an instance's Amazon EBS volumes.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_volumes(
        &self,
        input: DescribeVolumesRequest,
    ) -> Result<DescribeVolumesResult, RusotoError<DescribeVolumesError>>;

    /// <p>Detaches a specified Elastic Load Balancing instance from its layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn detach_elastic_load_balancer(
        &self,
        input: DetachElasticLoadBalancerRequest,
    ) -> Result<(), RusotoError<DetachElasticLoadBalancerError>>;

    /// <p>Disassociates an Elastic IP address from its instance. The address remains registered with the stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn disassociate_elastic_ip(
        &self,
        input: DisassociateElasticIpRequest,
    ) -> Result<(), RusotoError<DisassociateElasticIpError>>;

    /// <p>Gets a generated host name for the specified layer, based on the current host name theme.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn get_hostname_suggestion(
        &self,
        input: GetHostnameSuggestionRequest,
    ) -> Result<GetHostnameSuggestionResult, RusotoError<GetHostnameSuggestionError>>;

    /// <p><note> <p>This action can be used only with Windows stacks.</p> </note> <p>Grants RDP access to a Windows instance for a specified time period.</p></p>
    async fn grant_access(
        &self,
        input: GrantAccessRequest,
    ) -> Result<GrantAccessResult, RusotoError<GrantAccessError>>;

    /// <p>Returns a list of tags that are applied to the specified stack or layer.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResult, RusotoError<ListTagsError>>;

    /// <p>Reboots a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> Result<(), RusotoError<RebootInstanceError>>;

    /// <p>Registers a specified Amazon ECS cluster with a stack. You can register only one cluster with a stack. A cluster can be registered with only one stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html"> Managing User Permissions</a>.</p>
    async fn register_ecs_cluster(
        &self,
        input: RegisterEcsClusterRequest,
    ) -> Result<RegisterEcsClusterResult, RusotoError<RegisterEcsClusterError>>;

    /// <p>Registers an Elastic IP address with a specified stack. An address can be registered with only one stack at a time. If the address is already registered, you must first deregister it by calling <a>DeregisterElasticIp</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_elastic_ip(
        &self,
        input: RegisterElasticIpRequest,
    ) -> Result<RegisterElasticIpResult, RusotoError<RegisterElasticIpError>>;

    /// <p>Registers instances that were created outside of AWS OpsWorks Stacks with a specified stack.</p> <note> <p>We do not recommend using this action to register instances. The complete registration operation includes two tasks: installing the AWS OpsWorks Stacks agent on the instance, and registering the instance with the stack. <code>RegisterInstance</code> handles only the second step. You should instead use the AWS CLI <code>register</code> command, which performs the entire registration operation. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register.html"> Registering an Instance with an AWS OpsWorks Stacks Stack</a>.</p> </note> <p>Registered instances have the same requirements as instances that are created by using the <a>CreateInstance</a> API. For example, registered instances must be running a supported Linux-based operating system, and they must have a supported instance type. For more information about requirements for instances that you want to register, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register-registering-preparer.html"> Preparing the Instance</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> Result<RegisterInstanceResult, RusotoError<RegisterInstanceError>>;

    /// <p>Registers an Amazon RDS instance with a stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_rds_db_instance(
        &self,
        input: RegisterRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<RegisterRdsDbInstanceError>>;

    /// <p>Registers an Amazon EBS volume with a specified stack. A volume can be registered with only one stack at a time. If the volume is already registered, you must first deregister it by calling <a>DeregisterVolume</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_volume(
        &self,
        input: RegisterVolumeRequest,
    ) -> Result<RegisterVolumeResult, RusotoError<RegisterVolumeError>>;

    /// <p>Specify the load-based auto scaling configuration for a specified layer. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html">Managing Load with Time-based and Load-based Instances</a>.</p> <note> <p>To use load-based auto scaling, you must create a set of load-based auto scaling instances. Load-based auto scaling operates only on the instances from that set, so you must ensure that you have created enough instances to handle the maximum anticipated load.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_load_based_auto_scaling(
        &self,
        input: SetLoadBasedAutoScalingRequest,
    ) -> Result<(), RusotoError<SetLoadBasedAutoScalingError>>;

    /// <p>Specifies a user's permissions. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingsecurity.html">Security and Permissions</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_permission(
        &self,
        input: SetPermissionRequest,
    ) -> Result<(), RusotoError<SetPermissionError>>;

    /// <p>Specify the time-based auto scaling configuration for a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html">Managing Load with Time-based and Load-based Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_time_based_auto_scaling(
        &self,
        input: SetTimeBasedAutoScalingRequest,
    ) -> Result<(), RusotoError<SetTimeBasedAutoScalingError>>;

    /// <p>Starts a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> Result<(), RusotoError<StartInstanceError>>;

    /// <p>Starts a stack's instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn start_stack(
        &self,
        input: StartStackRequest,
    ) -> Result<(), RusotoError<StartStackError>>;

    /// <p>Stops a specified instance. When you stop a standard instance, the data disappears and must be reinstalled when you restart the instance. You can stop an Amazon EBS-backed instance without losing data. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> Result<(), RusotoError<StopInstanceError>>;

    /// <p>Stops a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn stop_stack(&self, input: StopStackRequest) -> Result<(), RusotoError<StopStackError>>;

    /// <p>Apply cost-allocation tags to a specified stack or layer in AWS OpsWorks Stacks. For more information about how tagging works, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/tagging.html">Tags</a> in the AWS OpsWorks User Guide.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Unassigns a registered instance from all layers that are using the instance. The instance remains in the stack as an unassigned instance, and can be assigned to another layer as needed. You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn unassign_instance(
        &self,
        input: UnassignInstanceRequest,
    ) -> Result<(), RusotoError<UnassignInstanceError>>;

    /// <p>Unassigns an assigned Amazon EBS volume. The volume remains registered with the stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn unassign_volume(
        &self,
        input: UnassignVolumeRequest,
    ) -> Result<(), RusotoError<UnassignVolumeError>>;

    /// <p>Removes tags from a specified stack or layer.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_app(&self, input: UpdateAppRequest) -> Result<(), RusotoError<UpdateAppError>>;

    /// <p>Updates a registered Elastic IP address's name. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_elastic_ip(
        &self,
        input: UpdateElasticIpRequest,
    ) -> Result<(), RusotoError<UpdateElasticIpError>>;

    /// <p>Updates a specified instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_instance(
        &self,
        input: UpdateInstanceRequest,
    ) -> Result<(), RusotoError<UpdateInstanceError>>;

    /// <p>Updates a specified layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_layer(
        &self,
        input: UpdateLayerRequest,
    ) -> Result<(), RusotoError<UpdateLayerError>>;

    /// <p>Updates a user's SSH public key.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_my_user_profile(
        &self,
        input: UpdateMyUserProfileRequest,
    ) -> Result<(), RusotoError<UpdateMyUserProfileError>>;

    /// <p>Updates an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_rds_db_instance(
        &self,
        input: UpdateRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<UpdateRdsDbInstanceError>>;

    /// <p>Updates a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> Result<(), RusotoError<UpdateStackError>>;

    /// <p>Updates a specified user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserProfileError>>;

    /// <p>Updates an Amazon EBS volume's name or mount point. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_volume(
        &self,
        input: UpdateVolumeRequest,
    ) -> Result<(), RusotoError<UpdateVolumeError>>;
}
/// A client for the AWS OpsWorks API.
#[derive(Clone)]
pub struct OpsWorksClient {
    client: Client,
    region: region::Region,
}

impl OpsWorksClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OpsWorksClient {
        OpsWorksClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OpsWorksClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        OpsWorksClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> OpsWorksClient {
        OpsWorksClient { client, region }
    }
}

#[async_trait]
impl OpsWorks for OpsWorksClient {
    /// <p>Assign a registered instance to a layer.</p> <ul> <li> <p>You can assign registered on-premises instances to any layer type.</p> </li> <li> <p>You can assign registered Amazon EC2 instances only to custom layers.</p> </li> <li> <p>You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> </li> </ul> <p> <b>Required Permissions</b>: To use this action, an AWS Identity and Access Management (IAM) user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn assign_instance(
        &self,
        input: AssignInstanceRequest,
    ) -> Result<(), RusotoError<AssignInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssignInstance");
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
            Err(AssignInstanceError::from_response(response))
        }
    }

    /// <p>Assigns one of the stack's registered Amazon EBS volumes to a specified instance. The volume must first be registered with the stack by calling <a>RegisterVolume</a>. After you register the volume, you must call <a>UpdateVolume</a> to specify a mount point before calling <code>AssignVolume</code>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn assign_volume(
        &self,
        input: AssignVolumeRequest,
    ) -> Result<(), RusotoError<AssignVolumeError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssignVolume");
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
            Err(AssignVolumeError::from_response(response))
        }
    }

    /// <p>Associates one of the stack's registered Elastic IP addresses with a specified instance. The address must first be registered with the stack by calling <a>RegisterElasticIp</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn associate_elastic_ip(
        &self,
        input: AssociateElasticIpRequest,
    ) -> Result<(), RusotoError<AssociateElasticIpError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssociateElasticIp");
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
            Err(AssociateElasticIpError::from_response(response))
        }
    }

    /// <p>Attaches an Elastic Load Balancing load balancer to a specified layer. AWS OpsWorks Stacks does not support Application Load Balancer. You can only use Classic Load Balancer with AWS OpsWorks Stacks. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/layers-elb.html">Elastic Load Balancing</a>.</p> <note> <p>You must create the Elastic Load Balancing instance separately, by using the Elastic Load Balancing console, API, or CLI. For more information, see <a href="https://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/Welcome.html"> Elastic Load Balancing Developer Guide</a>.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn attach_elastic_load_balancer(
        &self,
        input: AttachElasticLoadBalancerRequest,
    ) -> Result<(), RusotoError<AttachElasticLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.AttachElasticLoadBalancer",
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
            Err(AttachElasticLoadBalancerError::from_response(response))
        }
    }

    /// <p>Creates a clone of a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-cloning.html">Clone a Stack</a>. By default, all parameters are set to the values used by the parent stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn clone_stack(
        &self,
        input: CloneStackRequest,
    ) -> Result<CloneStackResult, RusotoError<CloneStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CloneStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CloneStackResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CloneStackError::from_response(response))
        }
    }

    /// <p>Creates an app for a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html">Creating Apps</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResult, RusotoError<CreateAppError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateApp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateAppResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAppError::from_response(response))
        }
    }

    /// <p>Runs deployment or stack commands. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html">Deploying Apps</a> and <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html">Run Stack Commands</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> Result<CreateDeploymentResult, RusotoError<CreateDeploymentError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateDeployment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDeploymentResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeploymentError::from_response(response))
        }
    }

    /// <p>Creates an instance in a specified stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html">Adding an Instance to a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_instance(
        &self,
        input: CreateInstanceRequest,
    ) -> Result<CreateInstanceResult, RusotoError<CreateInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInstanceError::from_response(response))
        }
    }

    /// <p>Creates a layer. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-create.html">How to Create a Layer</a>.</p> <note> <p>You should use <b>CreateLayer</b> for noncustom layer types such as PHP App Server only if the stack does not have an existing layer of that type. A stack can have at most one instance of each noncustom layer; if you attempt to create a second instance, <b>CreateLayer</b> fails. A stack can have an arbitrary number of custom layers, so you can call <b>CreateLayer</b> as many times as you like for that layer type.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_layer(
        &self,
        input: CreateLayerRequest,
    ) -> Result<CreateLayerResult, RusotoError<CreateLayerError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateLayer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateLayerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLayerError::from_response(response))
        }
    }

    /// <p>Creates a new stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-edit.html">Create a New Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_stack(
        &self,
        input: CreateStackRequest,
    ) -> Result<CreateStackResult, RusotoError<CreateStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateStackResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStackError::from_response(response))
        }
    }

    /// <p>Creates a new user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResult, RusotoError<CreateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateUserProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserProfileError::from_response(response))
        }
    }

    /// <p>Deletes a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_app(&self, input: DeleteAppRequest) -> Result<(), RusotoError<DeleteAppError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteApp");
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
            Err(DeleteAppError::from_response(response))
        }
    }

    /// <p>Deletes a specified instance, which terminates the associated Amazon EC2 instance. You must stop an instance before you can delete it.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-delete.html">Deleting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<(), RusotoError<DeleteInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteInstance");
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
            Err(DeleteInstanceError::from_response(response))
        }
    }

    /// <p>Deletes a specified layer. You must first stop and then delete all associated instances or unassign registered instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-delete.html">How to Delete a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_layer(
        &self,
        input: DeleteLayerRequest,
    ) -> Result<(), RusotoError<DeleteLayerError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteLayer");
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
            Err(DeleteLayerError::from_response(response))
        }
    }

    /// <p>Deletes a specified stack. You must first delete all instances, layers, and apps or deregister registered instances. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-shutting.html">Shut Down a Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_stack(
        &self,
        input: DeleteStackRequest,
    ) -> Result<(), RusotoError<DeleteStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteStack");
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
            Err(DeleteStackError::from_response(response))
        }
    }

    /// <p>Deletes a user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<(), RusotoError<DeleteUserProfileError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteUserProfile");
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
            Err(DeleteUserProfileError::from_response(response))
        }
    }

    /// <p>Deregisters a specified Amazon ECS cluster from a stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html#workinglayers-ecscluster-delete"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html</a>.</p>
    async fn deregister_ecs_cluster(
        &self,
        input: DeregisterEcsClusterRequest,
    ) -> Result<(), RusotoError<DeregisterEcsClusterError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterEcsCluster");
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
            Err(DeregisterEcsClusterError::from_response(response))
        }
    }

    /// <p>Deregisters a specified Elastic IP address. The address can then be registered by another stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_elastic_ip(
        &self,
        input: DeregisterElasticIpRequest,
    ) -> Result<(), RusotoError<DeregisterElasticIpError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterElasticIp");
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
            Err(DeregisterElasticIpError::from_response(response))
        }
    }

    /// <p>Deregister a registered Amazon EC2 or on-premises instance. This action removes the instance from the stack and returns it to your control. This action cannot be used with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> Result<(), RusotoError<DeregisterInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterInstance");
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
            Err(DeregisterInstanceError::from_response(response))
        }
    }

    /// <p>Deregisters an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_rds_db_instance(
        &self,
        input: DeregisterRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<DeregisterRdsDbInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterRdsDbInstance");
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
            Err(DeregisterRdsDbInstanceError::from_response(response))
        }
    }

    /// <p>Deregisters an Amazon EBS volume. The volume can then be registered by another stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn deregister_volume(
        &self,
        input: DeregisterVolumeRequest,
    ) -> Result<(), RusotoError<DeregisterVolumeError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterVolume");
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
            Err(DeregisterVolumeError::from_response(response))
        }
    }

    /// <p>Describes the available AWS OpsWorks Stacks agent versions. You must specify a stack ID or a configuration manager. <code>DescribeAgentVersions</code> returns a list of available agent versions for the specified stack or configuration manager.</p>
    async fn describe_agent_versions(
        &self,
        input: DescribeAgentVersionsRequest,
    ) -> Result<DescribeAgentVersionsResult, RusotoError<DescribeAgentVersionsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeAgentVersions");
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
                .deserialize::<DescribeAgentVersionsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAgentVersionsError::from_response(response))
        }
    }

    /// <p>Requests a description of a specified set of apps.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_apps(
        &self,
        input: DescribeAppsRequest,
    ) -> Result<DescribeAppsResult, RusotoError<DescribeAppsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeApps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAppsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAppsError::from_response(response))
        }
    }

    /// <p>Describes the results of specified commands.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_commands(
        &self,
        input: DescribeCommandsRequest,
    ) -> Result<DescribeCommandsResult, RusotoError<DescribeCommandsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeCommands");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeCommandsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCommandsError::from_response(response))
        }
    }

    /// <p>Requests a description of a specified set of deployments.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_deployments(
        &self,
        input: DescribeDeploymentsRequest,
    ) -> Result<DescribeDeploymentsResult, RusotoError<DescribeDeploymentsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeDeployments");
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
                .deserialize::<DescribeDeploymentsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDeploymentsError::from_response(response))
        }
    }

    /// <p>Describes Amazon ECS clusters that are registered with a stack. If you specify only a stack ID, you can use the <code>MaxResults</code> and <code>NextToken</code> parameters to paginate the response. However, AWS OpsWorks Stacks currently supports only one cluster per layer, so the result set has a maximum of one element.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permission. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_ecs_clusters(
        &self,
        input: DescribeEcsClustersRequest,
    ) -> Result<DescribeEcsClustersResult, RusotoError<DescribeEcsClustersError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeEcsClusters");
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
                .deserialize::<DescribeEcsClustersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEcsClustersError::from_response(response))
        }
    }

    /// <p>Describes <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP addresses</a>.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_elastic_ips(
        &self,
        input: DescribeElasticIpsRequest,
    ) -> Result<DescribeElasticIpsResult, RusotoError<DescribeElasticIpsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeElasticIps");
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
                .deserialize::<DescribeElasticIpsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeElasticIpsError::from_response(response))
        }
    }

    /// <p>Describes a stack's Elastic Load Balancing instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_elastic_load_balancers(
        &self,
        input: DescribeElasticLoadBalancersRequest,
    ) -> Result<DescribeElasticLoadBalancersResult, RusotoError<DescribeElasticLoadBalancersError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.DescribeElasticLoadBalancers",
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
                .deserialize::<DescribeElasticLoadBalancersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeElasticLoadBalancersError::from_response(response))
        }
    }

    /// <p>Requests a description of a set of instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_instances(
        &self,
        input: DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResult, RusotoError<DescribeInstancesError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeInstancesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInstancesError::from_response(response))
        }
    }

    /// <p>Requests a description of one or more layers in a specified stack.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_layers(
        &self,
        input: DescribeLayersRequest,
    ) -> Result<DescribeLayersResult, RusotoError<DescribeLayersError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeLayers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeLayersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLayersError::from_response(response))
        }
    }

    /// <p>Describes load-based auto scaling configurations for specified layers.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_load_based_auto_scaling(
        &self,
        input: DescribeLoadBasedAutoScalingRequest,
    ) -> Result<DescribeLoadBasedAutoScalingResult, RusotoError<DescribeLoadBasedAutoScalingError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.DescribeLoadBasedAutoScaling",
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
                .deserialize::<DescribeLoadBasedAutoScalingResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLoadBasedAutoScalingError::from_response(response))
        }
    }

    /// <p>Describes a user's SSH information.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_my_user_profile(
        &self,
    ) -> Result<DescribeMyUserProfileResult, RusotoError<DescribeMyUserProfileError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeMyUserProfile");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMyUserProfileResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMyUserProfileError::from_response(response))
        }
    }

    /// <p>Describes the operating systems that are supported by AWS OpsWorks Stacks.</p>
    async fn describe_operating_systems(
        &self,
    ) -> Result<DescribeOperatingSystemsResponse, RusotoError<DescribeOperatingSystemsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeOperatingSystems");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOperatingSystemsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOperatingSystemsError::from_response(response))
        }
    }

    /// <p>Describes the permissions for a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_permissions(
        &self,
        input: DescribePermissionsRequest,
    ) -> Result<DescribePermissionsResult, RusotoError<DescribePermissionsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribePermissions");
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
                .deserialize::<DescribePermissionsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePermissionsError::from_response(response))
        }
    }

    /// <p>Describe an instance's RAID arrays.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_raid_arrays(
        &self,
        input: DescribeRaidArraysRequest,
    ) -> Result<DescribeRaidArraysResult, RusotoError<DescribeRaidArraysError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeRaidArrays");
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
                .deserialize::<DescribeRaidArraysResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRaidArraysError::from_response(response))
        }
    }

    /// <p>Describes Amazon RDS instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_rds_db_instances(
        &self,
        input: DescribeRdsDbInstancesRequest,
    ) -> Result<DescribeRdsDbInstancesResult, RusotoError<DescribeRdsDbInstancesError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeRdsDbInstances");
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
                .deserialize::<DescribeRdsDbInstancesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRdsDbInstancesError::from_response(response))
        }
    }

    /// <p>Describes AWS OpsWorks Stacks service errors.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>
    async fn describe_service_errors(
        &self,
        input: DescribeServiceErrorsRequest,
    ) -> Result<DescribeServiceErrorsResult, RusotoError<DescribeServiceErrorsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeServiceErrors");
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
                .deserialize::<DescribeServiceErrorsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeServiceErrorsError::from_response(response))
        }
    }

    /// <p>Requests a description of a stack's provisioning parameters.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stack_provisioning_parameters(
        &self,
        input: DescribeStackProvisioningParametersRequest,
    ) -> Result<
        DescribeStackProvisioningParametersResult,
        RusotoError<DescribeStackProvisioningParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.DescribeStackProvisioningParameters",
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
                .deserialize::<DescribeStackProvisioningParametersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStackProvisioningParametersError::from_response(
                response,
            ))
        }
    }

    /// <p>Describes the number of layers and apps in a specified stack, and the number of instances in each state, such as <code>running_setup</code> or <code>online</code>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stack_summary(
        &self,
        input: DescribeStackSummaryRequest,
    ) -> Result<DescribeStackSummaryResult, RusotoError<DescribeStackSummaryError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeStackSummary");
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
                .deserialize::<DescribeStackSummaryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStackSummaryError::from_response(response))
        }
    }

    /// <p>Requests a description of one or more stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_stacks(
        &self,
        input: DescribeStacksRequest,
    ) -> Result<DescribeStacksResult, RusotoError<DescribeStacksError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeStacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeStacksResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStacksError::from_response(response))
        }
    }

    /// <p>Describes time-based auto scaling configurations for specified instances.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_time_based_auto_scaling(
        &self,
        input: DescribeTimeBasedAutoScalingRequest,
    ) -> Result<DescribeTimeBasedAutoScalingResult, RusotoError<DescribeTimeBasedAutoScalingError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.DescribeTimeBasedAutoScaling",
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
                .deserialize::<DescribeTimeBasedAutoScalingResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTimeBasedAutoScalingError::from_response(response))
        }
    }

    /// <p>Describe specified users.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_user_profiles(
        &self,
        input: DescribeUserProfilesRequest,
    ) -> Result<DescribeUserProfilesResult, RusotoError<DescribeUserProfilesError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeUserProfiles");
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
                .deserialize::<DescribeUserProfilesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserProfilesError::from_response(response))
        }
    }

    /// <p>Describes an instance's Amazon EBS volumes.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn describe_volumes(
        &self,
        input: DescribeVolumesRequest,
    ) -> Result<DescribeVolumesResult, RusotoError<DescribeVolumesError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeVolumes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeVolumesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVolumesError::from_response(response))
        }
    }

    /// <p>Detaches a specified Elastic Load Balancing instance from its layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn detach_elastic_load_balancer(
        &self,
        input: DetachElasticLoadBalancerRequest,
    ) -> Result<(), RusotoError<DetachElasticLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorks_20130218.DetachElasticLoadBalancer",
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
            Err(DetachElasticLoadBalancerError::from_response(response))
        }
    }

    /// <p>Disassociates an Elastic IP address from its instance. The address remains registered with the stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn disassociate_elastic_ip(
        &self,
        input: DisassociateElasticIpRequest,
    ) -> Result<(), RusotoError<DisassociateElasticIpError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DisassociateElasticIp");
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
            Err(DisassociateElasticIpError::from_response(response))
        }
    }

    /// <p>Gets a generated host name for the specified layer, based on the current host name theme.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn get_hostname_suggestion(
        &self,
        input: GetHostnameSuggestionRequest,
    ) -> Result<GetHostnameSuggestionResult, RusotoError<GetHostnameSuggestionError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.GetHostnameSuggestion");
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
                .deserialize::<GetHostnameSuggestionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetHostnameSuggestionError::from_response(response))
        }
    }

    /// <p><note> <p>This action can be used only with Windows stacks.</p> </note> <p>Grants RDP access to a Windows instance for a specified time period.</p></p>
    async fn grant_access(
        &self,
        input: GrantAccessRequest,
    ) -> Result<GrantAccessResult, RusotoError<GrantAccessError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.GrantAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GrantAccessResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GrantAccessError::from_response(response))
        }
    }

    /// <p>Returns a list of tags that are applied to the specified stack or layer.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResult, RusotoError<ListTagsError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTagsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Reboots a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> Result<(), RusotoError<RebootInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RebootInstance");
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
            Err(RebootInstanceError::from_response(response))
        }
    }

    /// <p>Registers a specified Amazon ECS cluster with a stack. You can register only one cluster with a stack. A cluster can be registered with only one stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html"> Managing User Permissions</a>.</p>
    async fn register_ecs_cluster(
        &self,
        input: RegisterEcsClusterRequest,
    ) -> Result<RegisterEcsClusterResult, RusotoError<RegisterEcsClusterError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterEcsCluster");
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
                .deserialize::<RegisterEcsClusterResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterEcsClusterError::from_response(response))
        }
    }

    /// <p>Registers an Elastic IP address with a specified stack. An address can be registered with only one stack at a time. If the address is already registered, you must first deregister it by calling <a>DeregisterElasticIp</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_elastic_ip(
        &self,
        input: RegisterElasticIpRequest,
    ) -> Result<RegisterElasticIpResult, RusotoError<RegisterElasticIpError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterElasticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RegisterElasticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterElasticIpError::from_response(response))
        }
    }

    /// <p>Registers instances that were created outside of AWS OpsWorks Stacks with a specified stack.</p> <note> <p>We do not recommend using this action to register instances. The complete registration operation includes two tasks: installing the AWS OpsWorks Stacks agent on the instance, and registering the instance with the stack. <code>RegisterInstance</code> handles only the second step. You should instead use the AWS CLI <code>register</code> command, which performs the entire registration operation. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register.html"> Registering an Instance with an AWS OpsWorks Stacks Stack</a>.</p> </note> <p>Registered instances have the same requirements as instances that are created by using the <a>CreateInstance</a> API. For example, registered instances must be running a supported Linux-based operating system, and they must have a supported instance type. For more information about requirements for instances that you want to register, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register-registering-preparer.html"> Preparing the Instance</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> Result<RegisterInstanceResult, RusotoError<RegisterInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RegisterInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterInstanceError::from_response(response))
        }
    }

    /// <p>Registers an Amazon RDS instance with a stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_rds_db_instance(
        &self,
        input: RegisterRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<RegisterRdsDbInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterRdsDbInstance");
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
            Err(RegisterRdsDbInstanceError::from_response(response))
        }
    }

    /// <p>Registers an Amazon EBS volume with a specified stack. A volume can be registered with only one stack at a time. If the volume is already registered, you must first deregister it by calling <a>DeregisterVolume</a>. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn register_volume(
        &self,
        input: RegisterVolumeRequest,
    ) -> Result<RegisterVolumeResult, RusotoError<RegisterVolumeError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterVolume");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RegisterVolumeResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterVolumeError::from_response(response))
        }
    }

    /// <p>Specify the load-based auto scaling configuration for a specified layer. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html">Managing Load with Time-based and Load-based Instances</a>.</p> <note> <p>To use load-based auto scaling, you must create a set of load-based auto scaling instances. Load-based auto scaling operates only on the instances from that set, so you must ensure that you have created enough instances to handle the maximum anticipated load.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_load_based_auto_scaling(
        &self,
        input: SetLoadBasedAutoScalingRequest,
    ) -> Result<(), RusotoError<SetLoadBasedAutoScalingError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetLoadBasedAutoScaling");
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
            Err(SetLoadBasedAutoScalingError::from_response(response))
        }
    }

    /// <p>Specifies a user's permissions. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingsecurity.html">Security and Permissions</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_permission(
        &self,
        input: SetPermissionRequest,
    ) -> Result<(), RusotoError<SetPermissionError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetPermission");
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
            Err(SetPermissionError::from_response(response))
        }
    }

    /// <p>Specify the time-based auto scaling configuration for a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html">Managing Load with Time-based and Load-based Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn set_time_based_auto_scaling(
        &self,
        input: SetTimeBasedAutoScalingRequest,
    ) -> Result<(), RusotoError<SetTimeBasedAutoScalingError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetTimeBasedAutoScaling");
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
            Err(SetTimeBasedAutoScalingError::from_response(response))
        }
    }

    /// <p>Starts a specified instance. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> Result<(), RusotoError<StartInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StartInstance");
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
            Err(StartInstanceError::from_response(response))
        }
    }

    /// <p>Starts a stack's instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn start_stack(
        &self,
        input: StartStackRequest,
    ) -> Result<(), RusotoError<StartStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StartStack");
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
            Err(StartStackError::from_response(response))
        }
    }

    /// <p>Stops a specified instance. When you stop a standard instance, the data disappears and must be reinstalled when you restart the instance. You can stop an Amazon EBS-backed instance without losing data. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> Result<(), RusotoError<StopInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StopInstance");
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
            Err(StopInstanceError::from_response(response))
        }
    }

    /// <p>Stops a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn stop_stack(&self, input: StopStackRequest) -> Result<(), RusotoError<StopStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StopStack");
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
            Err(StopStackError::from_response(response))
        }
    }

    /// <p>Apply cost-allocation tags to a specified stack or layer in AWS OpsWorks Stacks. For more information about how tagging works, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/tagging.html">Tags</a> in the AWS OpsWorks User Guide.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.TagResource");
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
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Unassigns a registered instance from all layers that are using the instance. The instance remains in the stack as an unassigned instance, and can be assigned to another layer as needed. You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn unassign_instance(
        &self,
        input: UnassignInstanceRequest,
    ) -> Result<(), RusotoError<UnassignInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UnassignInstance");
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
            Err(UnassignInstanceError::from_response(response))
        }
    }

    /// <p>Unassigns an assigned Amazon EBS volume. The volume remains registered with the stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn unassign_volume(
        &self,
        input: UnassignVolumeRequest,
    ) -> Result<(), RusotoError<UnassignVolumeError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UnassignVolume");
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
            Err(UnassignVolumeError::from_response(response))
        }
    }

    /// <p>Removes tags from a specified stack or layer.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UntagResource");
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
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_app(&self, input: UpdateAppRequest) -> Result<(), RusotoError<UpdateAppError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateApp");
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
            Err(UpdateAppError::from_response(response))
        }
    }

    /// <p>Updates a registered Elastic IP address's name. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_elastic_ip(
        &self,
        input: UpdateElasticIpRequest,
    ) -> Result<(), RusotoError<UpdateElasticIpError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateElasticIp");
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
            Err(UpdateElasticIpError::from_response(response))
        }
    }

    /// <p>Updates a specified instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_instance(
        &self,
        input: UpdateInstanceRequest,
    ) -> Result<(), RusotoError<UpdateInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateInstance");
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
            Err(UpdateInstanceError::from_response(response))
        }
    }

    /// <p>Updates a specified layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_layer(
        &self,
        input: UpdateLayerRequest,
    ) -> Result<(), RusotoError<UpdateLayerError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateLayer");
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
            Err(UpdateLayerError::from_response(response))
        }
    }

    /// <p>Updates a user's SSH public key.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_my_user_profile(
        &self,
        input: UpdateMyUserProfileRequest,
    ) -> Result<(), RusotoError<UpdateMyUserProfileError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateMyUserProfile");
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
            Err(UpdateMyUserProfileError::from_response(response))
        }
    }

    /// <p>Updates an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_rds_db_instance(
        &self,
        input: UpdateRdsDbInstanceRequest,
    ) -> Result<(), RusotoError<UpdateRdsDbInstanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateRdsDbInstance");
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
            Err(UpdateRdsDbInstanceError::from_response(response))
        }
    }

    /// <p>Updates a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_stack(
        &self,
        input: UpdateStackRequest,
    ) -> Result<(), RusotoError<UpdateStackError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateStack");
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
            Err(UpdateStackError::from_response(response))
        }
    }

    /// <p>Updates a specified user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information about user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateUserProfile");
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
            Err(UpdateUserProfileError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EBS volume's name or mount point. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
    async fn update_volume(
        &self,
        input: UpdateVolumeRequest,
    ) -> Result<(), RusotoError<UpdateVolumeError>> {
        let mut request = SignedRequest::new("POST", "opsworks", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateVolume");
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
            Err(UpdateVolumeError::from_response(response))
        }
    }
}
