
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
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Describes an agent version.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AgentVersion {
    #[doc="<p>The configuration manager.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[doc="<p>The agent version.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>A description of the app.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct App {
    #[doc="<p>The app ID.</p>"]
    #[serde(rename="AppId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<String>,
    #[doc="<p>A <code>Source</code> object that describes the app repository.</p>"]
    #[serde(rename="AppSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_source: Option<Source>,
    #[doc="<p>The stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>When the app was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>The app's data sources.</p>"]
    #[serde(rename="DataSources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[doc="<p>A description of the app.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The app vhost settings with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>"]
    #[serde(rename="Domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[doc="<p>Whether to enable SSL for the app.</p>"]
    #[serde(rename="EnableSsl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_ssl: Option<bool>,
    #[doc="<p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment\"> Environment Variables</a>. </p> <note> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variable names, values, and protected flag values - cannot exceed 10 KB (10240 Bytes). This limit should accommodate most if not all use cases, but if you do exceed it, you will cause an exception (API) with an \"Environment: is too large (maximum is 10KB)\" message.</p> </note>"]
    #[serde(rename="Environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    #[doc="<p>The app name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The app's short name.</p>"]
    #[serde(rename="Shortname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shortname: Option<String>,
    #[doc="<p>An <code>SslConfiguration</code> object with the SSL configuration.</p>"]
    #[serde(rename="SslConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    #[doc="<p>The app stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The app type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AssignInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[doc="<p>The layer ID, which must correspond to a custom layer. You cannot assign a registered instance to a built-in layer.</p>"]
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AssignVolumeRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The volume ID.</p>"]
    #[serde(rename="VolumeId")]
    pub volume_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateElasticIpRequest {
    #[doc="<p>The Elastic IP address.</p>"]
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachElasticLoadBalancerRequest {
    #[doc="<p>The Elastic Load Balancing instance's name.</p>"]
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    #[doc="<p>The ID of the layer that the Elastic Load Balancing instance is to be attached to.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

#[doc="<p>Describes a load-based auto scaling upscaling or downscaling threshold configuration, which specifies when AWS OpsWorks Stacks starts or stops load-based instances.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AutoScalingThresholds {
    #[doc="<p>Custom Cloudwatch auto scaling alarms, to be used as thresholds. This parameter takes a list of up to five alarm names, which are case sensitive and must be in the same region as the stack.</p> <note> <p>To use custom alarms, you must update your service role to allow <code>cloudwatch:DescribeAlarms</code>. You can either have AWS OpsWorks Stacks update the role for you when you first use this feature or you can edit the role manually. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-servicerole.html\">Allowing AWS OpsWorks Stacks to Act on Your Behalf</a>.</p> </note>"]
    #[serde(rename="Alarms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alarms: Option<Vec<String>>,
    #[doc="<p>The CPU utilization threshold, as a percent of the available CPU. A value of -1 disables the threshold.</p>"]
    #[serde(rename="CpuThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu_threshold: Option<f64>,
    #[doc="<p>The amount of time (in minutes) after a scaling event occurs that AWS OpsWorks Stacks should ignore metrics and suppress additional scaling events. For example, AWS OpsWorks Stacks adds new instances following an upscaling event but the instances won't start reducing the load until they have been booted and configured. There is no point in raising additional scaling events during that operation, which typically takes several minutes. <code>IgnoreMetricsTime</code> allows you to direct AWS OpsWorks Stacks to suppress scaling events long enough to get the new instances online.</p>"]
    #[serde(rename="IgnoreMetricsTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignore_metrics_time: Option<i64>,
    #[doc="<p>The number of instances to add or remove when the load exceeds a threshold.</p>"]
    #[serde(rename="InstanceCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_count: Option<i64>,
    #[doc="<p>The load threshold. A value of -1 disables the threshold. For more information about how load is computed, see <a href=\"http://en.wikipedia.org/wiki/Load_%28computing%29\">Load (computing)</a>.</p>"]
    #[serde(rename="LoadThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub load_threshold: Option<f64>,
    #[doc="<p>The memory utilization threshold, as a percent of the available memory. A value of -1 disables the threshold.</p>"]
    #[serde(rename="MemoryThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub memory_threshold: Option<f64>,
    #[doc="<p>The amount of time, in minutes, that the load must exceed a threshold before more instances are added or removed.</p>"]
    #[serde(rename="ThresholdsWaitTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thresholds_wait_time: Option<i64>,
}

#[doc="<p>Describes a block device mapping. This data type maps directly to the Amazon EC2 <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_BlockDeviceMapping.html\">BlockDeviceMapping</a> data type. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct BlockDeviceMapping {
    #[doc="<p>The device name that is exposed to the instance, such as <code>/dev/sdh</code>. For the root device, you can use the explicit device name or you can set this parameter to <code>ROOT_DEVICE</code> and AWS OpsWorks Stacks will provide the correct device name.</p>"]
    #[serde(rename="DeviceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_name: Option<String>,
    #[doc="<p>An <code>EBSBlockDevice</code> that defines how to configure an Amazon EBS volume when the instance is launched.</p>"]
    #[serde(rename="Ebs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs: Option<EbsBlockDevice>,
    #[doc="<p>Suppresses the specified device included in the AMI's block device mapping.</p>"]
    #[serde(rename="NoDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub no_device: Option<String>,
    #[doc="<p>The virtual device name. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_BlockDeviceMapping.html\">BlockDeviceMapping</a>.</p>"]
    #[serde(rename="VirtualName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtual_name: Option<String>,
}

#[doc="<p>Describes the Chef configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ChefConfiguration {
    #[doc="<p>The Berkshelf version.</p>"]
    #[serde(rename="BerkshelfVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub berkshelf_version: Option<String>,
    #[doc="<p>Whether to enable Berkshelf.</p>"]
    #[serde(rename="ManageBerkshelf")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manage_berkshelf: Option<bool>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CloneStackRequest {
    #[doc="<p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack's instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack's instances.</p> </li> </ul> <p>The default setting is <code>LATEST</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack's default setting.</p> </note>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>A list of stack attributes and values as key/value pairs to be added to the cloned stack.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="ChefConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    #[doc="<p>A list of source stack app IDs to be included in the cloned stack.</p>"]
    #[serde(rename="CloneAppIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_app_ids: Option<Vec<String>>,
    #[doc="<p>Whether to clone the source stack's permissions.</p>"]
    #[serde(rename="ClonePermissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_permissions: Option<bool>,
    #[doc="<p>The configuration manager. When you clone a stack we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 12.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[serde(rename="CustomCookbooksSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    #[doc="<p>A string that contains user-defined, custom JSON. It is used to override the corresponding default stack configuration JSON values. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a> </p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The cloned stack's default Availability Zone, which must be in the specified region. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see the <code>VpcId</code> parameter description. </p>"]
    #[serde(rename="DefaultAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_availability_zone: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    #[doc="<p>The stack's operating system, which must be set to one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p> <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information on how to use custom AMIs with OpsWorks, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the parent stack's operating system. For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">AWS OpsWorks Stacks Operating Systems</a>.</p> <note> <p>You can specify a different Linux operating system for the cloned stack, but you cannot change from Linux to Windows or Windows to Linux.</p> </note>"]
    #[serde(rename="DefaultOs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_os: Option<String>,
    #[doc="<p>The default root device type. This value is used by default for all instances in the cloned stack, but you can override it when you create an instance. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="DefaultRootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_root_device_type: Option<String>,
    #[doc="<p>A default Amazon EC2 key pair name. The default value is none. If you specify a key pair name, AWS OpsWorks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html\"> Using SSH to Communicate with an Instance</a> and <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html\"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html\"> create an instance</a>. </p>"]
    #[serde(rename="DefaultSshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    #[doc="<p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>"]
    #[serde(rename="DefaultSubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_subnet_id: Option<String>,
    #[doc="<p>The stack's host name theme, with spaces are replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>"]
    #[serde(rename="HostnameTheme")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname_theme: Option<String>,
    #[doc="<p>The cloned stack name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The cloned stack AWS region, such as \"ap-northeast-2\". For more information about AWS regions, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[doc="<p>The stack AWS Identity and Access Management (IAM) role, which allows AWS OpsWorks Stacks to work with AWS resources on your behalf. You must set this parameter to the Amazon Resource Name (ARN) for an existing IAM role. If you create a stack by using the AWS OpsWorks Stacks console, it creates the role for you. You can obtain an existing stack's IAM ARN programmatically by calling <a>DescribePermissions</a>. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p> <note> <p>You must set this parameter to a valid service role ARN or the action will fail; there is no default value. You can specify the source stack's service role ARN, if you prefer, but you must do so explicitly.</p> </note>"]
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[doc="<p>The source stack ID.</p>"]
    #[serde(rename="SourceStackId")]
    pub source_stack_id: String,
    #[doc="<p>Whether to use custom cookbooks.</p>"]
    #[serde(rename="UseCustomCookbooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    #[doc="<p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. With <code>UseOpsworksSecurityGroups</code> you can instead provide your own custom security groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate Amazon Elastic Compute Cloud (Amazon EC2) security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on creation; custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    #[doc="<p>The ID of the VPC that the cloned stack is to be launched into. It must be in the specified region. All instances are launched into this VPC, and you cannot change the ID later.</p> <ul> <li> <p>If your account supports EC2 Classic, the default value is no VPC.</p> </li> <li> <p>If your account does not support EC2 Classic, the default value is the default VPC for the specified region.</p> </li> </ul> <p>If the VPC ID corresponds to a default VPC and you have specified either the <code>DefaultAvailabilityZone</code> or the <code>DefaultSubnetId</code> parameter only, AWS OpsWorks Stacks infers the value of the other parameter. If you specify neither parameter, AWS OpsWorks Stacks sets these parameters to the first valid Availability Zone for the specified region and the corresponding default VPC subnet ID, respectively. </p> <p>If you specify a nondefault VPC ID, note the following:</p> <ul> <li> <p>It must belong to a VPC in your account that is in the specified region.</p> </li> <li> <p>You must specify a value for <code>DefaultSubnetId</code>.</p> </li> </ul> <p>For more information on how to use AWS OpsWorks Stacks with a VPC, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-vpc.html\">Running a Stack in a VPC</a>. For more information on default VPC and EC2 Classic, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html\">Supported Platforms</a>. </p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>CloneStack</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CloneStackResult {
    #[doc="<p>The cloned stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Describes the Amazon CloudWatch logs configuration for a layer.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudWatchLogsConfiguration {
    #[doc="<p>Whether CloudWatch Logs is enabled for a layer.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="<p>A list of configuration options for CloudWatch Logs.</p>"]
    #[serde(rename="LogStreams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_streams: Option<Vec<CloudWatchLogsLogStream>>,
}

#[doc="<p>Specifies the encoding of the log file so that the file can be read correctly. The default is <code>utf_8</code>. Encodings supported by Python <code>codecs.decode()</code> can be used here.</p>"]
#[doc="<p>Specifies where to start to read data (start_of_file or end_of_file). The default is start_of_file. It's only used if there is no state persisted for that log stream.</p>"]
#[doc="<p>Describes the Amazon CloudWatch logs configuration for a layer. For detailed information about members of this data type, see the <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html\">CloudWatch Logs Agent Reference</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudWatchLogsLogStream {
    #[doc="<p>Specifies the max number of log events in a batch, up to 10000. The default value is 1000.</p>"]
    #[serde(rename="BatchCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub batch_count: Option<i64>,
    #[doc="<p>Specifies the maximum size of log events in a batch, in bytes, up to 1048576 bytes. The default value is 32768 bytes. This size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p>"]
    #[serde(rename="BatchSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub batch_size: Option<i64>,
    #[doc="<p>Specifies the time duration for the batching of log events. The minimum value is 5000ms and default value is 5000ms.</p>"]
    #[serde(rename="BufferDuration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffer_duration: Option<i64>,
    #[doc="<p>Specifies how the time stamp is extracted from logs. For more information, see the <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AgentReference.html\">CloudWatch Logs Agent Reference</a>.</p>"]
    #[serde(rename="DatetimeFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub datetime_format: Option<String>,
    #[doc="<p>Specifies the encoding of the log file so that the file can be read correctly. The default is <code>utf_8</code>. Encodings supported by Python <code>codecs.decode()</code> can be used here.</p>"]
    #[serde(rename="Encoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,
    #[doc="<p>Specifies log files that you want to push to CloudWatch Logs.</p> <p> <code>File</code> can point to a specific file or multiple files (by using wild card characters such as <code>/var/log/system.log*</code>). Only the latest file is pushed to CloudWatch Logs, based on file modification time. We recommend that you use wild card characters to specify a series of files of the same type, such as <code>access_log.2014-06-01-01</code>, <code>access_log.2014-06-01-02</code>, and so on by using a pattern like <code>access_log.*</code>. Don't use a wildcard to match multiple file types, such as <code>access_log_80</code> and <code>access_log_443</code>. To specify multiple, different file types, add another log stream entry to the configuration file, so that each log file type is stored in a different log group.</p> <p>Zipped files are not supported.</p>"]
    #[serde(rename="File")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file: Option<String>,
    #[doc="<p>Specifies the range of lines for identifying a file. The valid values are one number, or two dash-delimited numbers, such as '1', '2-5'. The default value is '1', meaning the first line is used to calculate the fingerprint. Fingerprint lines are not sent to CloudWatch Logs unless all specified lines are available.</p>"]
    #[serde(rename="FileFingerprintLines")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_fingerprint_lines: Option<String>,
    #[doc="<p>Specifies where to start to read data (start_of_file or end_of_file). The default is start_of_file. This setting is only used if there is no state persisted for that log stream.</p>"]
    #[serde(rename="InitialPosition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub initial_position: Option<String>,
    #[doc="<p>Specifies the destination log group. A log group is created automatically if it doesn't already exist. Log group names can be between 1 and 512 characters long. Allowed characters include a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p>"]
    #[serde(rename="LogGroupName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_group_name: Option<String>,
    #[doc="<p>Specifies the pattern for identifying the start of a log message.</p>"]
    #[serde(rename="MultiLineStartPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_line_start_pattern: Option<String>,
    #[doc="<p>Specifies the time zone of log event time stamps.</p>"]
    #[serde(rename="TimeZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time_zone: Option<String>,
}

#[doc="<p>Describes the Amazon CloudWatch logs configuration for a layer.</p>"]
#[doc="<p>The preferred time zone for logs streamed to CloudWatch Logs. Valid values are <code>LOCAL</code> and <code>UTC</code>, for Coordinated Universal Time.</p>"]
#[doc="<p>Describes a command.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Command {
    #[doc="<p>Date and time when the command was acknowledged.</p>"]
    #[serde(rename="AcknowledgedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledged_at: Option<String>,
    #[doc="<p>The command ID.</p>"]
    #[serde(rename="CommandId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command_id: Option<String>,
    #[doc="<p>Date when the command completed.</p>"]
    #[serde(rename="CompletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,
    #[doc="<p>Date and time when the command was run.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>The command deployment ID.</p>"]
    #[serde(rename="DeploymentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc="<p>The command exit code.</p>"]
    #[serde(rename="ExitCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc="<p>The ID of the instance where the command was executed.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The URL of the command log.</p>"]
    #[serde(rename="LogUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,
    #[doc="<p>The command status:</p> <ul> <li> <p>failed</p> </li> <li> <p>successful</p> </li> <li> <p>skipped</p> </li> <li> <p>pending</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The command type:</p> <ul> <li> <p> <code>configure</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>execute_recipes</code> </p> </li> <li> <p> <code>install_dependencies</code> </p> </li> <li> <p> <code>restart</code> </p> </li> <li> <p> <code>rollback</code> </p> </li> <li> <p> <code>setup</code> </p> </li> <li> <p> <code>start</code> </p> </li> <li> <p> <code>stop</code> </p> </li> <li> <p> <code>undeploy</code> </p> </li> <li> <p> <code>update_custom_cookbooks</code> </p> </li> <li> <p> <code>update_dependencies</code> </p> </li> </ul>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateAppRequest {
    #[doc="<p>A <code>Source</code> object that specifies the app repository.</p>"]
    #[serde(rename="AppSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_source: Option<Source>,
    #[doc="<p>One or more user-defined key/value pairs to be added to the stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app's data source.</p>"]
    #[serde(rename="DataSources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[doc="<p>A description of the app.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The app virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>"]
    #[serde(rename="Domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[doc="<p>Whether to enable SSL for the app.</p>"]
    #[serde(rename="EnableSsl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_ssl: Option<bool>,
    #[doc="<p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment\"> Environment Variables</a>.</p> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 10 KB (10240 Bytes). This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, \"Environment: is too large (maximum is 10KB).\"</p> <note> <p>This parameter is supported only by Chef 11.10 stacks. If you have specified one or more environment variables, you cannot modify the stack's Chef version.</p> </note>"]
    #[serde(rename="Environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    #[doc="<p>The app name.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The app's short name.</p>"]
    #[serde(rename="Shortname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shortname: Option<String>,
    #[doc="<p>An <code>SslConfiguration</code> object with the SSL configuration.</p>"]
    #[serde(rename="SslConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[doc="<p>The app type. Each supported type is associated with a particular layer. For example, PHP applications are associated with a PHP layer. AWS OpsWorks Stacks deploys an application to those instances that are members of the corresponding layer. If your app isn't one of the standard types, or you prefer to implement your own Deploy recipes, specify <code>other</code>.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
}

#[doc="<p>Contains the response to a <code>CreateApp</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateAppResult {
    #[doc="<p>The app ID.</p>"]
    #[serde(rename="AppId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDeploymentRequest {
    #[doc="<p>The app ID. This parameter is required for app deployments, but not for other deployment commands.</p>"]
    #[serde(rename="AppId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<String>,
    #[doc="<p>A <code>DeploymentCommand</code> object that specifies the deployment command and any associated arguments.</p>"]
    #[serde(rename="Command")]
    pub command: DeploymentCommand,
    #[doc="<p>A user-defined comment.</p>"]
    #[serde(rename="Comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,
    #[doc="<p>A string that contains user-defined, custom JSON. It is used to override the corresponding default stack configuration JSON values. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The instance IDs for the deployment targets.</p>"]
    #[serde(rename="InstanceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    #[doc="<p>The layer IDs for the deployment targets.</p>"]
    #[serde(rename="LayerIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>CreateDeployment</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDeploymentResult {
    #[doc="<p>The deployment ID, which can be used with other requests to identify the deployment.</p>"]
    #[serde(rename="DeploymentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateInstanceRequest {
    #[doc="<p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p> <code>INHERIT</code> - Use the stack's default agent version setting.</p> </li> <li> <p> <i>version_number</i> - Use the specified agent version. This value overrides the stack's default setting. To update the agent version, edit the instance configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the instance.</p> </li> </ul> <p>The default setting is <code>INHERIT</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>A custom AMI ID to be used to create the instance. The AMI should be based on one of the supported operating systems. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Using Custom AMIs</a>.</p> <note> <p>If you specify a custom AMI, you must set <code>Os</code> to <code>Custom</code>.</p> </note>"]
    #[serde(rename="AmiId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ami_id: Option<String>,
    #[doc="<p>The instance architecture. The default option is <code>x86_64</code>. Instance types do not necessarily support both architectures. For a list of the architectures that are supported by the different instance types, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html\">Instance Families and Types</a>.</p>"]
    #[serde(rename="Architecture")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub architecture: Option<String>,
    #[doc="<p>For load-based or time-based instances, the type. Windows stacks can use only time-based instances.</p>"]
    #[serde(rename="AutoScalingType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_type: Option<String>,
    #[doc="<p>The instance Availability Zone. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>An array of <code>BlockDeviceMapping</code> objects that specify the instance's block devices. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html\">Block Device Mapping</a>. Note that block device mappings are not supported for custom AMIs.</p>"]
    #[serde(rename="BlockDeviceMappings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    #[doc="<p>Whether to create an Amazon EBS-optimized instance.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[doc="<p>The instance host name.</p>"]
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code> to ensure that your instances have the latest security updates.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>The instance type, such as <code>t2.micro</code>. For a list of supported instance types, open the stack in the console, choose <b>Instances</b>, and choose <b>+ Instance</b>. The <b>Size</b> list contains the currently supported types. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html\">Instance Families and Types</a>. The parameter values that you use to specify the various types are in the <b>API Name</b> column of the <b>Available Instance Types</b> table.</p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>An array that contains the instance's layer IDs.</p>"]
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
    #[doc="<p>The instance's operating system, which must be set to one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>.</p> </li> </ul> <p>For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">AWS OpsWorks Stacks Operating Systems</a>.</p> <p>The default option is the current Amazon Linux version. If you set this parameter to <code>Custom</code>, you must use the <a>CreateInstance</a> action's AmiId parameter to specify the custom AMI that you want to use. Block device mappings are not supported if the value is <code>Custom</code>. For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">Operating Systems</a>For more information on how to use custom AMIs with AWS OpsWorks Stacks, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Using Custom AMIs</a>.</p>"]
    #[serde(rename="Os")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub os: Option<String>,
    #[doc="<p>The instance root device type. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="RootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub root_device_type: Option<String>,
    #[doc="<p>The instance's Amazon EC2 key-pair name.</p>"]
    #[serde(rename="SshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key_name: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[doc="<p>The ID of the instance's subnet. If the stack is running in a VPC, you can use this parameter to override the stack's default subnet ID value and direct AWS OpsWorks Stacks to launch the instance in a different subnet.</p>"]
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc="<p>The instance's tenancy option. The default option is no tenancy, or if the instance is running in a VPC, inherit tenancy settings from the VPC. The following are valid values for this parameter: <code>dedicated</code>, <code>default</code>, or <code>host</code>. Because there are costs associated with changes in tenancy options, we recommend that you research tenancy options before choosing them for your instances. For more information about dedicated hosts, see <a href=\"http://aws.amazon.com/ec2/dedicated-hosts/\">Dedicated Hosts Overview</a> and <a href=\"http://aws.amazon.com/ec2/dedicated-hosts/\">Amazon EC2 Dedicated Hosts</a>. For more information about dedicated instances, see <a href=\"http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/dedicated-instance.html\">Dedicated Instances</a> and <a href=\"http://aws.amazon.com/ec2/purchasing-options/dedicated-instances/\">Amazon EC2 Dedicated Instances</a>.</p>"]
    #[serde(rename="Tenancy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenancy: Option<String>,
    #[doc="<p>The instance's virtualization type, <code>paravirtual</code> or <code>hvm</code>.</p>"]
    #[serde(rename="VirtualizationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtualization_type: Option<String>,
}

#[doc="<p>Contains the response to a <code>CreateInstance</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateInstanceResult {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateLayerRequest {
    #[doc="<p>One or more user-defined key-value pairs to be added to the stack attributes.</p> <p>To create a cluster layer, set the <code>EcsClusterArn</code> attribute to the cluster's ARN.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Whether to automatically assign an <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP address</a> to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignElasticIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    #[doc="<p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignPublicIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    #[doc="<p>Specifies CloudWatch Logs configuration options for the layer. For more information, see <a>CloudWatchLogsLogStream</a>.</p>"]
    #[serde(rename="CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    #[doc="<p>The ARN of an IAM profile to be used for the layer's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="CustomInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    #[doc="<p>A JSON-formatted string containing custom stack configuration and deployment attributes to be installed on the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html\"> Using Custom JSON</a>. This feature is supported as of version 1.7.42 of the AWS CLI. </p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>A <code>LayerCustomRecipes</code> object that specifies the layer custom recipes.</p>"]
    #[serde(rename="CustomRecipes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    #[doc="<p>An array containing the layer custom security group IDs.</p>"]
    #[serde(rename="CustomSecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    #[doc="<p>Whether to disable auto healing for the layer.</p>"]
    #[serde(rename="EnableAutoHealing")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>To ensure that your instances have the latest security updates, we strongly recommend using the default value of <code>true</code>.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>A <code>LifeCycleEventConfiguration</code> object that you can use to configure the Shutdown event to specify an execution timeout and enable or disable Elastic Load Balancer connection draining.</p>"]
    #[serde(rename="LifecycleEventConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    #[doc="<p>The layer name, which is used by the console.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>An array of <code>Package</code> objects that describes the layer packages.</p>"]
    #[serde(rename="Packages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[doc="<p>For custom layers only, use this parameter to specify the layer's short name, which is used internally by AWS OpsWorks Stacks and by Chef recipes. The short name is also used as the name for the directory where your app files are installed. It can have a maximum of 200 characters, which are limited to the alphanumeric characters, '-', '_', and '.'.</p> <p>The built-in layers' short names are defined by AWS OpsWorks Stacks. For more information, see the <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/layers.html\">Layer Reference</a>.</p>"]
    #[serde(rename="Shortname")]
    pub shortname: String,
    #[doc="<p>The layer stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[doc="<p>The layer type. A stack cannot have more than one built-in layer of the same type. It can have any number of custom layers. Built-in layers are not available in Chef 12 stacks.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
    #[doc="<p>Whether to use Amazon EBS-optimized instances.</p>"]
    #[serde(rename="UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    #[doc="<p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>"]
    #[serde(rename="VolumeConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

#[doc="<p>Contains the response to a <code>CreateLayer</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateLayerResult {
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateStackRequest {
    #[doc="<p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack's instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack's instances.</p> </li> </ul> <p>The default setting is the most recent release of the agent. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack's default setting.</p> </note>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>One or more user-defined key-value pairs to be added to the stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="ChefConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    #[doc="<p>The configuration manager. When you create a stack we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 11.4.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[serde(rename="CustomCookbooksSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    #[doc="<p>A string that contains user-defined, custom JSON. It can be used to override the corresponding default stack configuration attribute values or to pass data to recipes. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The stack's default Availability Zone, which must be in the specified region. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see the <code>VpcId</code> parameter description. </p>"]
    #[serde(rename="DefaultAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_availability_zone: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    #[doc="<p>The stack's default operating system, which is installed on every instance unless you specify a different operating system when you create the instance. You can specify one of the following.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\"> Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the current Amazon Linux version. For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">AWS OpsWorks Stacks Operating Systems</a>.</p>"]
    #[serde(rename="DefaultOs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_os: Option<String>,
    #[doc="<p>The default root device type. This value is the default for all instances in the stack, but you can override it when you create an instance. The default option is <code>instance-store</code>. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="DefaultRootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_root_device_type: Option<String>,
    #[doc="<p>A default Amazon EC2 key pair name. The default value is none. If you specify a key pair name, AWS OpsWorks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html\"> Using SSH to Communicate with an Instance</a> and <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html\"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html\"> create an instance</a>. </p>"]
    #[serde(rename="DefaultSshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    #[doc="<p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>"]
    #[serde(rename="DefaultSubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_subnet_id: Option<String>,
    #[doc="<p>The stack's host name theme, with spaces replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>"]
    #[serde(rename="HostnameTheme")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname_theme: Option<String>,
    #[doc="<p>The stack name.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The stack's AWS region, such as \"ap-south-1\". For more information about Amazon regions, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="Region")]
    pub region: String,
    #[doc="<p>The stack's AWS Identity and Access Management (IAM) role, which allows AWS OpsWorks Stacks to work with AWS resources on your behalf. You must set this parameter to the Amazon Resource Name (ARN) for an existing IAM role. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    #[doc="<p>Whether the stack uses custom cookbooks.</p>"]
    #[serde(rename="UseCustomCookbooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    #[doc="<p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. With <code>UseOpsworksSecurityGroups</code> you can instead provide your own custom security groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it, but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate EC2 security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on creation; custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    #[doc="<p>The ID of the VPC that the stack is to be launched into. The VPC must be in the stack's region. All instances are launched into this VPC. You cannot change the ID later.</p> <ul> <li> <p>If your account supports EC2-Classic, the default value is <code>no VPC</code>.</p> </li> <li> <p>If your account does not support EC2-Classic, the default value is the default VPC for the specified region.</p> </li> </ul> <p>If the VPC ID corresponds to a default VPC and you have specified either the <code>DefaultAvailabilityZone</code> or the <code>DefaultSubnetId</code> parameter only, AWS OpsWorks Stacks infers the value of the other parameter. If you specify neither parameter, AWS OpsWorks Stacks sets these parameters to the first valid Availability Zone for the specified region and the corresponding default VPC subnet ID, respectively.</p> <p>If you specify a nondefault VPC ID, note the following:</p> <ul> <li> <p>It must belong to a VPC in your account that is in the specified region.</p> </li> <li> <p>You must specify a value for <code>DefaultSubnetId</code>.</p> </li> </ul> <p>For more information on how to use AWS OpsWorks Stacks with a VPC, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-vpc.html\">Running a Stack in a VPC</a>. For more information on default VPC and EC2-Classic, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html\">Supported Platforms</a>. </p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>CreateStack</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateStackResult {
    #[doc="<p>The stack ID, which is an opaque string that you use to identify the stack when performing actions such as <code>DescribeStacks</code>.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserProfileRequest {
    #[doc="<p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html\">Setting an IAM User's Public SSH Key</a>.</p>"]
    #[serde(rename="AllowSelfManagement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_self_management: Option<bool>,
    #[doc="<p>The user's IAM ARN; this can also be a federated user's ARN.</p>"]
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    #[doc="<p>The user's public SSH key.</p>"]
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The user's SSH user name. The allowable characters are [a-z], [A-Z], [0-9], '-', and '_'. If the specified name includes other punctuation marks, AWS OpsWorks Stacks removes them. For example, <code>my.name</code> will be changed to <code>myname</code>. If you do not specify an SSH user name, AWS OpsWorks Stacks generates one from the IAM user name. </p>"]
    #[serde(rename="SshUsername")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_username: Option<String>,
}

#[doc="<p>Contains the response to a <code>CreateUserProfile</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserProfileResult {
    #[doc="<p>The user's IAM ARN.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
}

#[doc="<p>Describes an app's data source.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DataSource {
    #[doc="<p>The data source's ARN.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>The database name.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>The data source's type, <code>AutoSelectOpsworksMysqlInstance</code>, <code>OpsworksMysqlInstance</code>, or <code>RdsDbInstance</code>.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteAppRequest {
    #[doc="<p>The app ID.</p>"]
    #[serde(rename="AppId")]
    pub app_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteInstanceRequest {
    #[doc="<p>Whether to delete the instance Elastic IP address.</p>"]
    #[serde(rename="DeleteElasticIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_elastic_ip: Option<bool>,
    #[doc="<p>Whether to delete the instance's Amazon EBS volumes.</p>"]
    #[serde(rename="DeleteVolumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_volumes: Option<bool>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteLayerRequest {
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteStackRequest {
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserProfileRequest {
    #[doc="<p>The user's IAM ARN. This can also be a federated user's ARN.</p>"]
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
}

#[doc="<p>Describes a deployment of a stack or app.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Deployment {
    #[doc="<p>The app ID.</p>"]
    #[serde(rename="AppId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename="Command")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command: Option<DeploymentCommand>,
    #[doc="<p>A user-defined comment.</p>"]
    #[serde(rename="Comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,
    #[doc="<p>Date when the deployment completed.</p>"]
    #[serde(rename="CompletedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,
    #[doc="<p>Date when the deployment was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>A string that contains user-defined custom JSON. It can be used to override the corresponding default stack configuration attribute values for stack or to pass data to recipes. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The deployment ID.</p>"]
    #[serde(rename="DeploymentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc="<p>The deployment duration.</p>"]
    #[serde(rename="Duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<i64>,
    #[doc="<p>The user's IAM ARN.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[doc="<p>The IDs of the target instances.</p>"]
    #[serde(rename="InstanceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The deployment status:</p> <ul> <li> <p>running</p> </li> <li> <p>successful</p> </li> <li> <p>failed</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>Used to specify a stack or deployment command.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeploymentCommand {
    #[doc="<p>The arguments of those commands that take arguments. It should be set to a JSON object with the following format:</p> <p> <code>{\"arg_name1\" : [\"value1\", \"value2\", ...], \"arg_name2\" : [\"value1\", \"value2\", ...], ...}</code> </p> <p>The <code>update_dependencies</code> command takes two arguments:</p> <ul> <li> <p> <code>upgrade_os_to</code> - Specifies the desired Amazon Linux version for instances whose OS you want to upgrade, such as <code>Amazon Linux 2016.09</code>. You must also set the <code>allow_reboot</code> argument to true.</p> </li> <li> <p> <code>allow_reboot</code> - Specifies whether to allow AWS OpsWorks Stacks to reboot the instances if necessary, after installing the updates. This argument can be set to either <code>true</code> or <code>false</code>. The default value is <code>false</code>.</p> </li> </ul> <p>For example, to upgrade an instance to Amazon Linux 2016.09, set <code>Args</code> to the following.</p> <p> <code> { \"upgrade_os_to\":[\"Amazon Linux 2016.09\"], \"allow_reboot\":[\"true\"] } </code> </p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[doc="<p>Specifies the operation. You can specify only one command.</p> <p>For stacks, the following commands are available:</p> <ul> <li> <p> <code>execute_recipes</code>: Execute one or more recipes. To specify the recipes, set an <code>Args</code> parameter named <code>recipes</code> to the list of recipes to be executed. For example, to execute <code>phpapp::appsetup</code>, set <code>Args</code> to <code>{\"recipes\":[\"phpapp::appsetup\"]}</code>.</p> </li> <li> <p> <code>install_dependencies</code>: Install the stack's dependencies.</p> </li> <li> <p> <code>update_custom_cookbooks</code>: Update the stack's custom cookbooks.</p> </li> <li> <p> <code>update_dependencies</code>: Update the stack's dependencies.</p> </li> </ul> <note> <p>The update_dependencies and install_dependencies commands are supported only for Linux instances. You can run the commands successfully on Windows instances, but they do nothing.</p> </note> <p>For apps, the following commands are available:</p> <ul> <li> <p> <code>deploy</code>: Deploy an app. Ruby on Rails apps have an optional <code>Args</code> parameter named <code>migrate</code>. Set <code>Args</code> to {\"migrate\":[\"true\"]} to migrate the database. The default setting is {\"migrate\":[\"false\"]}.</p> </li> <li> <p> <code>rollback</code> Roll the app back to the previous version. When you update an app, AWS OpsWorks Stacks stores the previous version, up to a maximum of five versions. You can use this command to roll an app back as many as four versions.</p> </li> <li> <p> <code>start</code>: Start the app's web or application server.</p> </li> <li> <p> <code>stop</code>: Stop the app's web or application server.</p> </li> <li> <p> <code>restart</code>: Restart the app's web or application server.</p> </li> <li> <p> <code>undeploy</code>: Undeploy the app.</p> </li> </ul>"]
    #[serde(rename="Name")]
    pub name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterEcsClusterRequest {
    #[doc="<p>The cluster's ARN.</p>"]
    #[serde(rename="EcsClusterArn")]
    pub ecs_cluster_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterElasticIpRequest {
    #[doc="<p>The Elastic IP address.</p>"]
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterRdsDbInstanceRequest {
    #[doc="<p>The Amazon RDS instance's ARN.</p>"]
    #[serde(rename="RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterVolumeRequest {
    #[doc="<p>The AWS OpsWorks Stacks volume ID, which is the GUID that AWS OpsWorks Stacks assigned to the instance when you registered the volume with the stack, not the Amazon EC2 volume ID.</p>"]
    #[serde(rename="VolumeId")]
    pub volume_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAgentVersionsRequest {
    #[doc="<p>The configuration manager.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeAgentVersions</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAgentVersionsResult {
    #[doc="<p>The agent versions for the specified stack or configuration manager. Note that this value is the complete version number, not the abbreviated number used by the console.</p>"]
    #[serde(rename="AgentVersions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_versions: Option<Vec<AgentVersion>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAppsRequest {
    #[doc="<p>An array of app IDs for the apps to be described. If you use this parameter, <code>DescribeApps</code> returns a description of the specified apps. Otherwise, it returns a description of every app.</p>"]
    #[serde(rename="AppIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    #[doc="<p>The app stack ID. If you use this parameter, <code>DescribeApps</code> returns a description of the apps in the specified stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeApps</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAppsResult {
    #[doc="<p>An array of <code>App</code> objects that describe the specified apps. </p>"]
    #[serde(rename="Apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<App>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeCommandsRequest {
    #[doc="<p>An array of command IDs. If you include this parameter, <code>DescribeCommands</code> returns a description of the specified commands. Otherwise, it returns a description of every command.</p>"]
    #[serde(rename="CommandIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub command_ids: Option<Vec<String>>,
    #[doc="<p>The deployment ID. If you include this parameter, <code>DescribeCommands</code> returns a description of the commands associated with the specified deployment.</p>"]
    #[serde(rename="DeploymentId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc="<p>The instance ID. If you include this parameter, <code>DescribeCommands</code> returns a description of the commands associated with the specified instance.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeCommands</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeCommandsResult {
    #[doc="<p>An array of <code>Command</code> objects that describe each of the specified commands.</p>"]
    #[serde(rename="Commands")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commands: Option<Vec<Command>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDeploymentsRequest {
    #[doc="<p>The app ID. If you include this parameter, <code>DescribeDeployments</code> returns a description of the commands associated with the specified app.</p>"]
    #[serde(rename="AppId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<String>,
    #[doc="<p>An array of deployment IDs to be described. If you include this parameter, <code>DescribeDeployments</code> returns a description of the specified deployments. Otherwise, it returns a description of every deployment.</p>"]
    #[serde(rename="DeploymentIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID. If you include this parameter, <code>DescribeDeployments</code> returns a description of the commands associated with the specified stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeDeployments</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDeploymentsResult {
    #[doc="<p>An array of <code>Deployment</code> objects that describe the deployments.</p>"]
    #[serde(rename="Deployments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEcsClustersRequest {
    #[doc="<p>A list of ARNs, one for each cluster to be described.</p>"]
    #[serde(rename="EcsClusterArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_cluster_arns: Option<Vec<String>>,
    #[doc="<p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>If the previous paginated request did not return all of the remaining results, the response object's<code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>DescribeEcsClusters</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A stack ID. <code>DescribeEcsClusters</code> returns a description of the cluster that is registered with the stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeEcsClusters</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEcsClustersResult {
    #[doc="<p>A list of <code>EcsCluster</code> objects containing the cluster descriptions.</p>"]
    #[serde(rename="EcsClusters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_clusters: Option<Vec<EcsCluster>>,
    #[doc="<p>If a paginated request does not return all of the remaining results, this parameter is set to a token that you can assign to the request object's <code>NextToken</code> parameter to retrieve the next set of results. If the previous paginated request returned all of the remaining results, this parameter is set to <code>null</code>.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeElasticIpsRequest {
    #[doc="<p>The instance ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses associated with the specified instance.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>An array of Elastic IP addresses to be described. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the specified Elastic IP addresses. Otherwise, it returns a description of every Elastic IP address.</p>"]
    #[serde(rename="Ips")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ips: Option<Vec<String>>,
    #[doc="<p>A stack ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses that are registered with the specified stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeElasticIps</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeElasticIpsResult {
    #[doc="<p>An <code>ElasticIps</code> object that describes the specified Elastic IP addresses.</p>"]
    #[serde(rename="ElasticIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elastic_ips: Option<Vec<ElasticIp>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeElasticLoadBalancersRequest {
    #[doc="<p>A list of layer IDs. The action describes the Elastic Load Balancing instances for the specified layers.</p>"]
    #[serde(rename="LayerIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    #[doc="<p>A stack ID. The action describes the stack's Elastic Load Balancing instances.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeElasticLoadBalancers</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeElasticLoadBalancersResult {
    #[doc="<p>A list of <code>ElasticLoadBalancer</code> objects that describe the specified Elastic Load Balancing instances.</p>"]
    #[serde(rename="ElasticLoadBalancers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elastic_load_balancers: Option<Vec<ElasticLoadBalancer>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeInstancesRequest {
    #[doc="<p>An array of instance IDs to be described. If you use this parameter, <code>DescribeInstances</code> returns a description of the specified instances. Otherwise, it returns a description of every instance.</p>"]
    #[serde(rename="InstanceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    #[doc="<p>A layer ID. If you use this parameter, <code>DescribeInstances</code> returns descriptions of the instances associated with the specified layer.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
    #[doc="<p>A stack ID. If you use this parameter, <code>DescribeInstances</code> returns descriptions of the instances associated with the specified stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeInstances</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeInstancesResult {
    #[doc="<p>An array of <code>Instance</code> objects that describe the instances.</p>"]
    #[serde(rename="Instances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances: Option<Vec<Instance>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLayersRequest {
    #[doc="<p>An array of layer IDs that specify the layers to be described. If you omit this parameter, <code>DescribeLayers</code> returns a description of every layer in the specified stack.</p>"]
    #[serde(rename="LayerIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeLayers</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeLayersResult {
    #[doc="<p>An array of <code>Layer</code> objects that describe the layers.</p>"]
    #[serde(rename="Layers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLoadBasedAutoScalingRequest {
    #[doc="<p>An array of layer IDs.</p>"]
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
}

#[doc="<p>Contains the response to a <code>DescribeLoadBasedAutoScaling</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeLoadBasedAutoScalingResult {
    #[doc="<p>An array of <code>LoadBasedAutoScalingConfiguration</code> objects that describe each layer's configuration.</p>"]
    #[serde(rename="LoadBasedAutoScalingConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub load_based_auto_scaling_configurations: Option<Vec<LoadBasedAutoScalingConfiguration>>,
}

#[doc="<p>Contains the response to a <code>DescribeMyUserProfile</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeMyUserProfileResult {
    #[doc="<p>A <code>UserProfile</code> object that describes the user's SSH information.</p>"]
    #[serde(rename="UserProfile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_profile: Option<SelfUserProfile>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribePermissionsRequest {
    #[doc="<p>The user's IAM ARN. This can also be a federated user's ARN. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribePermissions</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribePermissionsResult {
    #[doc="<p>An array of <code>Permission</code> objects that describe the stack permissions.</p> <ul> <li> <p>If the request object contains only a stack ID, the array contains a <code>Permission</code> object with permissions for each of the stack IAM ARNs.</p> </li> <li> <p>If the request object contains only an IAM ARN, the array contains a <code>Permission</code> object with permissions for each of the user's stack IDs.</p> </li> <li> <p>If the request contains a stack ID and an IAM ARN, the array contains a single <code>Permission</code> object with permissions for the specified stack and IAM ARN.</p> </li> </ul>"]
    #[serde(rename="Permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRaidArraysRequest {
    #[doc="<p>The instance ID. If you use this parameter, <code>DescribeRaidArrays</code> returns descriptions of the RAID arrays associated with the specified instance. </p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>An array of RAID array IDs. If you use this parameter, <code>DescribeRaidArrays</code> returns descriptions of the specified arrays. Otherwise, it returns a description of every array.</p>"]
    #[serde(rename="RaidArrayIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_array_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeRaidArrays</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRaidArraysResult {
    #[doc="<p>A <code>RaidArrays</code> object that describes the specified RAID arrays.</p>"]
    #[serde(rename="RaidArrays")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_arrays: Option<Vec<RaidArray>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRdsDbInstancesRequest {
    #[doc="<p>An array containing the ARNs of the instances to be described.</p>"]
    #[serde(rename="RdsDbInstanceArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rds_db_instance_arns: Option<Vec<String>>,
    #[doc="<p>The stack ID that the instances are registered with. The operation returns descriptions of all registered Amazon RDS instances.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>DescribeRdsDbInstances</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRdsDbInstancesResult {
    #[doc="<p>An a array of <code>RdsDbInstance</code> objects that describe the instances.</p>"]
    #[serde(rename="RdsDbInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rds_db_instances: Option<Vec<RdsDbInstance>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeServiceErrorsRequest {
    #[doc="<p>The instance ID. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the errors associated with the specified instance.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>An array of service error IDs. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the specified errors. Otherwise, it returns a description of every error.</p>"]
    #[serde(rename="ServiceErrorIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_error_ids: Option<Vec<String>>,
    #[doc="<p>The stack ID. If you use this parameter, <code>DescribeServiceErrors</code> returns descriptions of the errors associated with the specified stack.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Contains the response to a <code>DescribeServiceErrors</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeServiceErrorsResult {
    #[doc="<p>An array of <code>ServiceError</code> objects that describe the specified service errors.</p>"]
    #[serde(rename="ServiceErrors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_errors: Option<Vec<ServiceError>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStackProvisioningParametersRequest {
    #[doc="<p>The stack ID</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>DescribeStackProvisioningParameters</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStackProvisioningParametersResult {
    #[doc="<p>The AWS OpsWorks Stacks agent installer's URL.</p>"]
    #[serde(rename="AgentInstallerUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_installer_url: Option<String>,
    #[doc="<p>An embedded object that contains the provisioning parameters.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStackSummaryRequest {
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>DescribeStackSummary</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStackSummaryResult {
    #[doc="<p>A <code>StackSummary</code> object that contains the results.</p>"]
    #[serde(rename="StackSummary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_summary: Option<StackSummary>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStacksRequest {
    #[doc="<p>An array of stack IDs that specify the stacks to be described. If you omit this parameter, <code>DescribeStacks</code> returns a description of every stack.</p>"]
    #[serde(rename="StackIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_ids: Option<Vec<String>>,
}

#[doc="<p>Contains the response to a <code>DescribeStacks</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStacksResult {
    #[doc="<p>An array of <code>Stack</code> objects that describe the stacks.</p>"]
    #[serde(rename="Stacks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stacks: Option<Vec<Stack>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTimeBasedAutoScalingRequest {
    #[doc="<p>An array of instance IDs.</p>"]
    #[serde(rename="InstanceIds")]
    pub instance_ids: Vec<String>,
}

#[doc="<p>Contains the response to a <code>DescribeTimeBasedAutoScaling</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTimeBasedAutoScalingResult {
    #[doc="<p>An array of <code>TimeBasedAutoScalingConfiguration</code> objects that describe the configuration for the specified instances.</p>"]
    #[serde(rename="TimeBasedAutoScalingConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time_based_auto_scaling_configurations: Option<Vec<TimeBasedAutoScalingConfiguration>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserProfilesRequest {
    #[doc="<p>An array of IAM or federated user ARNs that identify the users to be described.</p>"]
    #[serde(rename="IamUserArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arns: Option<Vec<String>>,
}

#[doc="<p>Contains the response to a <code>DescribeUserProfiles</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserProfilesResult {
    #[doc="<p>A <code>Users</code> object that describes the specified users.</p>"]
    #[serde(rename="UserProfiles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_profiles: Option<Vec<UserProfile>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeVolumesRequest {
    #[doc="<p>The instance ID. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the volumes associated with the specified instance.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The RAID array ID. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the volumes associated with the specified RAID array.</p>"]
    #[serde(rename="RaidArrayId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_array_id: Option<String>,
    #[doc="<p>A stack ID. The action describes the stack's registered Amazon EBS volumes.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>Am array of volume IDs. If you use this parameter, <code>DescribeVolumes</code> returns descriptions of the specified volumes. Otherwise, it returns a description of every volume.</p>"]
    #[serde(rename="VolumeIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_ids: Option<Vec<String>>,
}

#[doc="<p>Contains the response to a <code>DescribeVolumes</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeVolumesResult {
    #[doc="<p>An array of volume IDs.</p>"]
    #[serde(rename="Volumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachElasticLoadBalancerRequest {
    #[doc="<p>The Elastic Load Balancing instance's name.</p>"]
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    #[doc="<p>The ID of the layer that the Elastic Load Balancing instance is attached to.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateElasticIpRequest {
    #[doc="<p>The Elastic IP address.</p>"]
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
}

#[doc="<p>Describes an Amazon EBS volume. This data type maps directly to the Amazon EC2 <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html\">EbsBlockDevice</a> data type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EbsBlockDevice {
    #[doc="<p>Whether the volume is deleted on instance termination.</p>"]
    #[serde(rename="DeleteOnTermination")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[doc="<p>The number of I/O operations per second (IOPS) that the volume supports. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html\">EbsBlockDevice</a>.</p>"]
    #[serde(rename="Iops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iops: Option<i64>,
    #[doc="<p>The snapshot ID.</p>"]
    #[serde(rename="SnapshotId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc="<p>The volume size, in GiB. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EbsBlockDevice.html\">EbsBlockDevice</a>.</p>"]
    #[serde(rename="VolumeSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_size: Option<i64>,
    #[doc="<p>The volume type. <code>gp2</code> for General Purpose (SSD) volumes, <code>io1</code> for Provisioned IOPS (SSD) volumes, and <code>standard</code> for Magnetic volumes.</p>"]
    #[serde(rename="VolumeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_type: Option<String>,
}

#[doc="<p>Describes a registered Amazon ECS cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EcsCluster {
    #[doc="<p>The cluster's ARN.</p>"]
    #[serde(rename="EcsClusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    #[doc="<p>The cluster name.</p>"]
    #[serde(rename="EcsClusterName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_cluster_name: Option<String>,
    #[doc="<p>The time and date that the cluster was registered with the stack.</p>"]
    #[serde(rename="RegisteredAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered_at: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Describes an Elastic IP address.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ElasticIp {
    #[doc="<p>The domain.</p>"]
    #[serde(rename="Domain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain: Option<String>,
    #[doc="<p>The ID of the instance that the address is attached to.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The IP address.</p>"]
    #[serde(rename="Ip")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip: Option<String>,
    #[doc="<p>The name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The AWS region. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
}

#[doc="<p>Describes an Elastic Load Balancing instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ElasticLoadBalancer {
    #[doc="<p>A list of Availability Zones.</p>"]
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[doc="<p>The instance's public DNS name.</p>"]
    #[serde(rename="DnsName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns_name: Option<String>,
    #[doc="<p>A list of the EC2 instances that the Elastic Load Balancing instance is managing traffic for.</p>"]
    #[serde(rename="Ec2InstanceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_instance_ids: Option<Vec<String>>,
    #[doc="<p>The Elastic Load Balancing instance's name.</p>"]
    #[serde(rename="ElasticLoadBalancerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elastic_load_balancer_name: Option<String>,
    #[doc="<p>The ID of the layer that the instance is attached to.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
    #[doc="<p>The instance's AWS region.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[doc="<p>The ID of the stack that the instance is associated with.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>A list of subnet IDs, if the stack is running in a VPC.</p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[doc="<p>The VPC ID.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
}

#[doc="<p>Represents an app's environment variable.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EnvironmentVariable {
    #[doc="<p>(Required) The environment variable's name, which can consist of up to 64 characters and must be specified. The name can contain upper- and lowercase letters, numbers, and underscores (_), but it must start with a letter or underscore.</p>"]
    #[serde(rename="Key")]
    pub key: String,
    #[doc="<p>(Optional) Whether the variable's value will be returned by the <a>DescribeApps</a> action. To conceal an environment variable's value, set <code>Secure</code> to <code>true</code>. <code>DescribeApps</code> then returns <code>*****FILTERED*****</code> instead of the actual value. The default value for <code>Secure</code> is <code>false</code>. </p>"]
    #[serde(rename="Secure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secure: Option<bool>,
    #[doc="<p>(Optional) The environment variable's value, which can be left empty. If you specify a value, it can contain up to 256 characters, which must all be printable.</p>"]
    #[serde(rename="Value")]
    pub value: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetHostnameSuggestionRequest {
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

#[doc="<p>Contains the response to a <code>GetHostnameSuggestion</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetHostnameSuggestionResult {
    #[doc="<p>The generated host name.</p>"]
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GrantAccessRequest {
    #[doc="<p>The instance's AWS OpsWorks Stacks ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[doc="<p>The length of time (in minutes) that the grant is valid. When the grant expires at the end of this period, the user will no longer be able to use the credentials to log in. If the user is logged in at the time, he or she automatically will be logged out.</p>"]
    #[serde(rename="ValidForInMinutes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub valid_for_in_minutes: Option<i64>,
}

#[doc="<p>Contains the response to a <code>GrantAccess</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GrantAccessResult {
    #[doc="<p>A <code>TemporaryCredential</code> object that contains the data needed to log in to the instance by RDP clients, such as the Microsoft Remote Desktop Connection.</p>"]
    #[serde(rename="TemporaryCredential")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temporary_credential: Option<TemporaryCredential>,
}

#[doc="<p>Describes an instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Instance {
    #[doc="<p>The agent version. This parameter is set to <code>INHERIT</code> if the instance inherits the default stack setting or to a a version number for a fixed agent version.</p>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>A custom AMI ID to be used to create the instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Instances</a> </p>"]
    #[serde(rename="AmiId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ami_id: Option<String>,
    #[doc="<p>The instance architecture: \"i386\" or \"x86_64\".</p>"]
    #[serde(rename="Architecture")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>For load-based or time-based instances, the type.</p>"]
    #[serde(rename="AutoScalingType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_type: Option<String>,
    #[doc="<p>The instance Availability Zone. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>An array of <code>BlockDeviceMapping</code> objects that specify the instance's block device mappings.</p>"]
    #[serde(rename="BlockDeviceMappings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    #[doc="<p>The time that the instance was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>Whether this is an Amazon EBS-optimized instance.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[doc="<p>The ID of the associated Amazon EC2 instance.</p>"]
    #[serde(rename="Ec2InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    #[doc="<p>For container instances, the Amazon ECS cluster's ARN.</p>"]
    #[serde(rename="EcsClusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    #[doc="<p>For container instances, the instance's ARN.</p>"]
    #[serde(rename="EcsContainerInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_container_instance_arn: Option<String>,
    #[doc="<p>The instance <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP address </a>.</p>"]
    #[serde(rename="ElasticIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elastic_ip: Option<String>,
    #[doc="<p>The instance host name.</p>"]
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>For registered instances, the infrastructure class: <code>ec2</code> or <code>on-premises</code>.</p>"]
    #[serde(rename="InfrastructureClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub infrastructure_class: Option<String>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. If this value is set to <code>false</code>, you must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The ARN of the instance's IAM profile. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="InstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_profile_arn: Option<String>,
    #[doc="<p>The instance type, such as <code>t2.micro</code>.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The ID of the last service error. For more information, call <a>DescribeServiceErrors</a>.</p>"]
    #[serde(rename="LastServiceErrorId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_service_error_id: Option<String>,
    #[doc="<p>An array containing the instance layer IDs.</p>"]
    #[serde(rename="LayerIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    #[doc="<p>The instance's operating system.</p>"]
    #[serde(rename="Os")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub os: Option<String>,
    #[doc="<p>The instance's platform.</p>"]
    #[serde(rename="Platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,
    #[doc="<p>The instance's private DNS name.</p>"]
    #[serde(rename="PrivateDns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_dns: Option<String>,
    #[doc="<p>The instance's private IP address.</p>"]
    #[serde(rename="PrivateIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_ip: Option<String>,
    #[doc="<p>The instance public DNS name.</p>"]
    #[serde(rename="PublicDns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_dns: Option<String>,
    #[doc="<p>The instance public IP address.</p>"]
    #[serde(rename="PublicIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_ip: Option<String>,
    #[doc="<p>For registered instances, who performed the registration.</p>"]
    #[serde(rename="RegisteredBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered_by: Option<String>,
    #[doc="<p>The instance's reported AWS OpsWorks Stacks agent version.</p>"]
    #[serde(rename="ReportedAgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reported_agent_version: Option<String>,
    #[doc="<p>For registered instances, the reported operating system.</p>"]
    #[serde(rename="ReportedOs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reported_os: Option<ReportedOs>,
    #[doc="<p>The instance's root device type. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="RootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub root_device_type: Option<String>,
    #[doc="<p>The root device volume ID.</p>"]
    #[serde(rename="RootDeviceVolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub root_device_volume_id: Option<String>,
    #[doc="<p>An array containing the instance security group IDs.</p>"]
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[doc="<p>The SSH key's Deep Security Agent (DSA) fingerprint.</p>"]
    #[serde(rename="SshHostDsaKeyFingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_host_dsa_key_fingerprint: Option<String>,
    #[doc="<p>The SSH key's RSA fingerprint.</p>"]
    #[serde(rename="SshHostRsaKeyFingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_host_rsa_key_fingerprint: Option<String>,
    #[doc="<p>The instance's Amazon EC2 key-pair name.</p>"]
    #[serde(rename="SshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key_name: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The instance status:</p> <ul> <li> <p> <code>booting</code> </p> </li> <li> <p> <code>connection_lost</code> </p> </li> <li> <p> <code>online</code> </p> </li> <li> <p> <code>pending</code> </p> </li> <li> <p> <code>rebooting</code> </p> </li> <li> <p> <code>requested</code> </p> </li> <li> <p> <code>running_setup</code> </p> </li> <li> <p> <code>setup_failed</code> </p> </li> <li> <p> <code>shutting_down</code> </p> </li> <li> <p> <code>start_failed</code> </p> </li> <li> <p> <code>stop_failed</code> </p> </li> <li> <p> <code>stopped</code> </p> </li> <li> <p> <code>stopping</code> </p> </li> <li> <p> <code>terminated</code> </p> </li> <li> <p> <code>terminating</code> </p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The instance's subnet ID; applicable only if the stack is running in a VPC.</p>"]
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc="<p>The instance's tenancy option, such as <code>dedicated</code> or <code>host</code>.</p>"]
    #[serde(rename="Tenancy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tenancy: Option<String>,
    #[doc="<p>The instance's virtualization type: <code>paravirtual</code> or <code>hvm</code>.</p>"]
    #[serde(rename="VirtualizationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub virtualization_type: Option<String>,
}

#[doc="<p>Contains a description of an Amazon EC2 instance from the Amazon EC2 metadata service. For more information, see <a href=\"http://docs.aws.amazon.com/sdkfornet/latest/apidocs/Index.html\">Instance Metadata and User Data</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceIdentity {
    #[doc="<p>A JSON document that contains the metadata.</p>"]
    #[serde(rename="Document")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub document: Option<String>,
    #[doc="<p>A signature that can be used to verify the document's accuracy and authenticity.</p>"]
    #[serde(rename="Signature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,
}

#[doc="<p>Describes how many instances a stack has for each status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstancesCount {
    #[doc="<p>The number of instances in the Assigning state.</p>"]
    #[serde(rename="Assigning")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assigning: Option<i64>,
    #[doc="<p>The number of instances with <code>booting</code> status.</p>"]
    #[serde(rename="Booting")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub booting: Option<i64>,
    #[doc="<p>The number of instances with <code>connection_lost</code> status.</p>"]
    #[serde(rename="ConnectionLost")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection_lost: Option<i64>,
    #[doc="<p>The number of instances in the Deregistering state.</p>"]
    #[serde(rename="Deregistering")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deregistering: Option<i64>,
    #[doc="<p>The number of instances with <code>online</code> status.</p>"]
    #[serde(rename="Online")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub online: Option<i64>,
    #[doc="<p>The number of instances with <code>pending</code> status.</p>"]
    #[serde(rename="Pending")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending: Option<i64>,
    #[doc="<p>The number of instances with <code>rebooting</code> status.</p>"]
    #[serde(rename="Rebooting")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rebooting: Option<i64>,
    #[doc="<p>The number of instances in the Registered state.</p>"]
    #[serde(rename="Registered")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registered: Option<i64>,
    #[doc="<p>The number of instances in the Registering state.</p>"]
    #[serde(rename="Registering")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registering: Option<i64>,
    #[doc="<p>The number of instances with <code>requested</code> status.</p>"]
    #[serde(rename="Requested")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested: Option<i64>,
    #[doc="<p>The number of instances with <code>running_setup</code> status.</p>"]
    #[serde(rename="RunningSetup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_setup: Option<i64>,
    #[doc="<p>The number of instances with <code>setup_failed</code> status.</p>"]
    #[serde(rename="SetupFailed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub setup_failed: Option<i64>,
    #[doc="<p>The number of instances with <code>shutting_down</code> status.</p>"]
    #[serde(rename="ShuttingDown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutting_down: Option<i64>,
    #[doc="<p>The number of instances with <code>start_failed</code> status.</p>"]
    #[serde(rename="StartFailed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_failed: Option<i64>,
    #[doc="<p>The number of instances with <code>stopped</code> status.</p>"]
    #[serde(rename="Stopped")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped: Option<i64>,
    #[doc="<p>The number of instances with <code>stopping</code> status.</p>"]
    #[serde(rename="Stopping")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopping: Option<i64>,
    #[doc="<p>The number of instances with <code>terminated</code> status.</p>"]
    #[serde(rename="Terminated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub terminated: Option<i64>,
    #[doc="<p>The number of instances with <code>terminating</code> status.</p>"]
    #[serde(rename="Terminating")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub terminating: Option<i64>,
    #[doc="<p>The number of instances in the Unassigning state.</p>"]
    #[serde(rename="Unassigning")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unassigning: Option<i64>,
}

#[doc="<p>Describes a layer.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Layer {
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>The layer attributes.</p> <p>For the <code>HaproxyStatsPassword</code>, <code>MysqlRootPassword</code>, and <code>GangliaPassword</code> attributes, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value</p> <p>For an ECS Cluster layer, AWS OpsWorks Stacks the <code>EcsClusterArn</code> attribute is set to the cluster's ARN.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Whether to automatically assign an <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP address</a> to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignElasticIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    #[doc="<p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignPublicIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    #[doc="<p>The Amazon CloudWatch Logs configuration settings for the layer.</p>"]
    #[serde(rename="CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    #[doc="<p>Date when the layer was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>The ARN of the default IAM profile to be used for the layer's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="CustomInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    #[doc="<p>A JSON formatted string containing the layer's custom stack configuration and deployment attributes.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>A <code>LayerCustomRecipes</code> object that specifies the layer's custom recipes.</p>"]
    #[serde(rename="CustomRecipes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    #[doc="<p>An array containing the layer's custom security group IDs.</p>"]
    #[serde(rename="CustomSecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    #[serde(rename="DefaultRecipes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_recipes: Option<Recipes>,
    #[doc="<p>An array containing the layer's security group names.</p>"]
    #[serde(rename="DefaultSecurityGroupNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_security_group_names: Option<Vec<String>>,
    #[doc="<p>Whether auto healing is disabled for the layer.</p>"]
    #[serde(rename="EnableAutoHealing")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. If this value is set to <code>false</code>, you must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
    #[doc="<p>A <code>LifeCycleEventConfiguration</code> object that specifies the Shutdown event configuration.</p>"]
    #[serde(rename="LifecycleEventConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    #[doc="<p>The layer name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An array of <code>Package</code> objects that describe the layer's packages.</p>"]
    #[serde(rename="Packages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[doc="<p>The layer short name.</p>"]
    #[serde(rename="Shortname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shortname: Option<String>,
    #[doc="<p>The layer stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The layer type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
    #[doc="<p>Whether the layer uses Amazon EBS-optimized instances.</p>"]
    #[serde(rename="UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    #[doc="<p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>"]
    #[serde(rename="VolumeConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

#[doc="<p>Specifies the lifecycle event configuration</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct LifecycleEventConfiguration {
    #[doc="<p>A <code>ShutdownEventConfiguration</code> object that specifies the Shutdown event configuration.</p>"]
    #[serde(rename="Shutdown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutdown: Option<ShutdownEventConfiguration>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsRequest {
    #[doc="<p>Do not use. A validation exception occurs if you add a <code>MaxResults</code> parameter to a <code>ListTagsRequest</code> call. </p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>Do not use. A validation exception occurs if you add a <code>NextToken</code> parameter to a <code>ListTagsRequest</code> call. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The stack or layer's Amazon Resource Number (ARN).</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
}

#[doc="<p>Contains the response to a <code>ListTags</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsResult {
    #[doc="<p>If a paginated request does not return all of the remaining results, this parameter is set to a token that you can assign to the request object's <code>NextToken</code> parameter to get the next set of results. If the previous paginated request returned all of the remaining results, this parameter is set to <code>null</code>. </p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A set of key-value pairs that contain tag keys and tag values that are attached to a stack or layer.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>Describes a layer's load-based auto scaling configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LoadBasedAutoScalingConfiguration {
    #[doc="<p>An <code>AutoScalingThresholds</code> object that describes the downscaling configuration, which defines how and when AWS OpsWorks Stacks reduces the number of instances.</p>"]
    #[serde(rename="DownScaling")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub down_scaling: Option<AutoScalingThresholds>,
    #[doc="<p>Whether load-based auto scaling is enabled for the layer.</p>"]
    #[serde(rename="Enable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable: Option<bool>,
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_id: Option<String>,
    #[doc="<p>An <code>AutoScalingThresholds</code> object that describes the upscaling configuration, which defines how and when AWS OpsWorks Stacks increases the number of instances.</p>"]
    #[serde(rename="UpScaling")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub up_scaling: Option<AutoScalingThresholds>,
}

#[doc="<p>Describes stack or user permissions.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Permission {
    #[doc="<p>Whether the user can use SSH.</p>"]
    #[serde(rename="AllowSsh")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_ssh: Option<bool>,
    #[doc="<p>Whether the user can use <b>sudo</b>.</p>"]
    #[serde(rename="AllowSudo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_sudo: Option<bool>,
    #[doc="<p>The Amazon Resource Name (ARN) for an AWS Identity and Access Management (IAM) role. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[doc="<p>The user's permission level, which must be the following:</p> <ul> <li> <p> <code>deny</code> </p> </li> <li> <p> <code>show</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>manage</code> </p> </li> <li> <p> <code>iam_only</code> </p> </li> </ul> <p>For more information on the permissions associated with these levels, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a> </p>"]
    #[serde(rename="Level")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub level: Option<String>,
    #[doc="<p>A stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[doc="<p>Describes an instance's RAID array.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RaidArray {
    #[doc="<p>The array's Availability Zone. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>When the RAID array was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>The array's Linux device. For example /dev/mdadm0.</p>"]
    #[serde(rename="Device")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device: Option<String>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>For PIOPS volumes, the IOPS per disk.</p>"]
    #[serde(rename="Iops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iops: Option<i64>,
    #[doc="<p>The array's mount point.</p>"]
    #[serde(rename="MountPoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_point: Option<String>,
    #[doc="<p>The array name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The number of disks in the array.</p>"]
    #[serde(rename="NumberOfDisks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_of_disks: Option<i64>,
    #[doc="<p>The array ID.</p>"]
    #[serde(rename="RaidArrayId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_array_id: Option<String>,
    #[doc="<p>The <a href=\"http://en.wikipedia.org/wiki/Standard_RAID_levels\">RAID level</a>.</p>"]
    #[serde(rename="RaidLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_level: Option<i64>,
    #[doc="<p>The array's size.</p>"]
    #[serde(rename="Size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The volume type, standard or PIOPS.</p>"]
    #[serde(rename="VolumeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_type: Option<String>,
}

#[doc="<p>Describes an Amazon RDS instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RdsDbInstance {
    #[doc="<p>The instance's address.</p>"]
    #[serde(rename="Address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[doc="<p>The DB instance identifier.</p>"]
    #[serde(rename="DbInstanceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db_instance_identifier: Option<String>,
    #[doc="<p>AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>"]
    #[serde(rename="DbPassword")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db_password: Option<String>,
    #[doc="<p>The master user name.</p>"]
    #[serde(rename="DbUser")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db_user: Option<String>,
    #[doc="<p>The instance's database engine.</p>"]
    #[serde(rename="Engine")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine: Option<String>,
    #[doc="<p>Set to <code>true</code> if AWS OpsWorks Stacks is unable to discover the Amazon RDS instance. AWS OpsWorks Stacks attempts to discover the instance only once. If this value is set to <code>true</code>, you must deregister the instance, and then register it again.</p>"]
    #[serde(rename="MissingOnRds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub missing_on_rds: Option<bool>,
    #[doc="<p>The instance's ARN.</p>"]
    #[serde(rename="RdsDbInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rds_db_instance_arn: Option<String>,
    #[doc="<p>The instance's AWS region.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[doc="<p>The ID of the stack with which the instance is registered.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RebootInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[doc="<p>AWS OpsWorks Stacks supports five lifecycle events: <b>setup</b>, <b>configuration</b>, <b>deploy</b>, <b>undeploy</b>, and <b>shutdown</b>. For each layer, AWS OpsWorks Stacks runs a set of standard recipes for each event. In addition, you can provide custom recipes for any or all layers and events. AWS OpsWorks Stacks runs custom event recipes after the standard recipes. <code>LayerCustomRecipes</code> specifies the custom recipes for a particular layer to be run in response to each of the five events. </p> <p>To specify a recipe, use the cookbook's directory name in the repository followed by two colons and the recipe name, which is the recipe's file name without the .rb extension. For example: phpapp2::dbsetup specifies the dbsetup.rb recipe in the repository's phpapp2 folder.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Recipes {
    #[doc="<p>An array of custom recipe names to be run following a <code>configure</code> event.</p>"]
    #[serde(rename="Configure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configure: Option<Vec<String>>,
    #[doc="<p>An array of custom recipe names to be run following a <code>deploy</code> event.</p>"]
    #[serde(rename="Deploy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deploy: Option<Vec<String>>,
    #[doc="<p>An array of custom recipe names to be run following a <code>setup</code> event.</p>"]
    #[serde(rename="Setup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub setup: Option<Vec<String>>,
    #[doc="<p>An array of custom recipe names to be run following a <code>shutdown</code> event.</p>"]
    #[serde(rename="Shutdown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shutdown: Option<Vec<String>>,
    #[doc="<p>An array of custom recipe names to be run following a <code>undeploy</code> event.</p>"]
    #[serde(rename="Undeploy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub undeploy: Option<Vec<String>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterEcsClusterRequest {
    #[doc="<p>The cluster's ARN.</p>"]
    #[serde(rename="EcsClusterArn")]
    pub ecs_cluster_arn: String,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>RegisterEcsCluster</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterEcsClusterResult {
    #[doc="<p>The cluster's ARN.</p>"]
    #[serde(rename="EcsClusterArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterElasticIpRequest {
    #[doc="<p>The Elastic IP address.</p>"]
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>RegisterElasticIp</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterElasticIpResult {
    #[doc="<p>The Elastic IP address.</p>"]
    #[serde(rename="ElasticIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elastic_ip: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterInstanceRequest {
    #[doc="<p>The instance's hostname.</p>"]
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>An InstanceIdentity object that contains the instance's identity.</p>"]
    #[serde(rename="InstanceIdentity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_identity: Option<InstanceIdentity>,
    #[doc="<p>The instance's private IP address.</p>"]
    #[serde(rename="PrivateIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_ip: Option<String>,
    #[doc="<p>The instance's public IP address.</p>"]
    #[serde(rename="PublicIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_ip: Option<String>,
    #[doc="<p>The instances public RSA key. This key is used to encrypt communication between the instance and the service.</p>"]
    #[serde(rename="RsaPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rsa_public_key: Option<String>,
    #[doc="<p>The instances public RSA key fingerprint.</p>"]
    #[serde(rename="RsaPublicKeyFingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rsa_public_key_fingerprint: Option<String>,
    #[doc="<p>The ID of the stack that the instance is to be registered with.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>RegisterInstanceResult</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterInstanceResult {
    #[doc="<p>The registered instance's AWS OpsWorks Stacks ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterRdsDbInstanceRequest {
    #[doc="<p>The database password.</p>"]
    #[serde(rename="DbPassword")]
    pub db_password: String,
    #[doc="<p>The database's master user name.</p>"]
    #[serde(rename="DbUser")]
    pub db_user: String,
    #[doc="<p>The Amazon RDS instance's ARN.</p>"]
    #[serde(rename="RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterVolumeRequest {
    #[doc="<p>The Amazon EBS volume ID.</p>"]
    #[serde(rename="Ec2VolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_volume_id: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[doc="<p>Contains the response to a <code>RegisterVolume</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterVolumeResult {
    #[doc="<p>The volume ID.</p>"]
    #[serde(rename="VolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_id: Option<String>,
}

#[doc="<p>A registered instance's reported operating system.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReportedOs {
    #[doc="<p>The operating system family.</p>"]
    #[serde(rename="Family")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family: Option<String>,
    #[doc="<p>The operating system name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The operating system version.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>Describes a user's SSH information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SelfUserProfile {
    #[doc="<p>The user's IAM ARN.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[doc="<p>The user's name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The user's SSH public key.</p>"]
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The user's SSH user name.</p>"]
    #[serde(rename="SshUsername")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_username: Option<String>,
}

#[doc="<p>Describes an AWS OpsWorks Stacks service error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ServiceError {
    #[doc="<p>When the error occurred.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>A message that describes the error.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>The error ID.</p>"]
    #[serde(rename="ServiceErrorId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_error_id: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>The error type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SetLoadBasedAutoScalingRequest {
    #[doc="<p>An <code>AutoScalingThresholds</code> object with the downscaling threshold configuration. If the load falls below these thresholds for a specified amount of time, AWS OpsWorks Stacks stops a specified number of instances.</p>"]
    #[serde(rename="DownScaling")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub down_scaling: Option<AutoScalingThresholds>,
    #[doc="<p>Enables load-based auto scaling for the layer.</p>"]
    #[serde(rename="Enable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable: Option<bool>,
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
    #[doc="<p>An <code>AutoScalingThresholds</code> object with the upscaling threshold configuration. If the load exceeds these thresholds for a specified amount of time, AWS OpsWorks Stacks starts a specified number of instances.</p>"]
    #[serde(rename="UpScaling")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub up_scaling: Option<AutoScalingThresholds>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SetPermissionRequest {
    #[doc="<p>The user is allowed to use SSH to communicate with the instance.</p>"]
    #[serde(rename="AllowSsh")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_ssh: Option<bool>,
    #[doc="<p>The user is allowed to use <b>sudo</b> to elevate privileges.</p>"]
    #[serde(rename="AllowSudo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_sudo: Option<bool>,
    #[doc="<p>The user's IAM ARN. This can also be a federated user's ARN.</p>"]
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    #[doc="<p>The user's permission level, which must be set to one of the following strings. You cannot set your own permissions level.</p> <ul> <li> <p> <code>deny</code> </p> </li> <li> <p> <code>show</code> </p> </li> <li> <p> <code>deploy</code> </p> </li> <li> <p> <code>manage</code> </p> </li> <li> <p> <code>iam_only</code> </p> </li> </ul> <p>For more information on the permissions associated with these levels, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    #[serde(rename="Level")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub level: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SetTimeBasedAutoScalingRequest {
    #[doc="<p>An <code>AutoScalingSchedule</code> with the instance schedule.</p>"]
    #[serde(rename="AutoScalingSchedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_schedule: Option<WeeklyAutoScalingSchedule>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[doc="<p>The Shutdown event configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ShutdownEventConfiguration {
    #[doc="<p>Whether to enable Elastic Load Balancing connection draining. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/TerminologyandKeyConcepts.html#conn-drain\">Connection Draining</a> </p>"]
    #[serde(rename="DelayUntilElbConnectionsDrained")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delay_until_elb_connections_drained: Option<bool>,
    #[doc="<p>The time, in seconds, that AWS OpsWorks Stacks will wait after triggering a Shutdown event before shutting down an instance.</p>"]
    #[serde(rename="ExecutionTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub execution_timeout: Option<i64>,
}

#[doc="<p>Contains the information required to retrieve an app or cookbook from a repository. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html\">Creating Apps</a> or <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook.html\">Custom Recipes and Cookbooks</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Source {
    #[doc="<p>When included in a request, the parameter depends on the repository type.</p> <ul> <li> <p>For Amazon S3 bundles, set <code>Password</code> to the appropriate IAM secret access key.</p> </li> <li> <p>For HTTP bundles and Subversion repositories, set <code>Password</code> to the password.</p> </li> </ul> <p>For more information on how to safely handle IAM credentials, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html\">http://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html</a>.</p> <p>In responses, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,
    #[doc="<p>The application's version. AWS OpsWorks Stacks enables you to easily deploy new versions of an application. One of the simplest approaches is to have branches or revisions in your repository that represent different versions that can potentially be deployed.</p>"]
    #[serde(rename="Revision")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub revision: Option<String>,
    #[doc="<p>In requests, the repository's SSH key.</p> <p>In responses, AWS OpsWorks Stacks returns <code>*****FILTERED*****</code> instead of the actual value.</p>"]
    #[serde(rename="SshKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key: Option<String>,
    #[doc="<p>The repository type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
    #[doc="<p>The source URL. The following is an example of an Amazon S3 source URL: <code>https://s3.amazonaws.com/opsworks-demo-bucket/opsworks_cookbook_demo.tar.gz</code>.</p>"]
    #[serde(rename="Url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[doc="<p>This parameter depends on the repository type.</p> <ul> <li> <p>For Amazon S3 bundles, set <code>Username</code> to the appropriate IAM access key ID.</p> </li> <li> <p>For HTTP bundles, Git repositories, and Subversion repositories, set <code>Username</code> to the user name.</p> </li> </ul>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

#[doc="<p>Describes an app's SSL configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SslConfiguration {
    #[doc="<p>The contents of the certificate's domain.crt file.</p>"]
    #[serde(rename="Certificate")]
    pub certificate: String,
    #[doc="<p>Optional. Can be used to specify an intermediate certificate authority key or client authentication.</p>"]
    #[serde(rename="Chain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chain: Option<String>,
    #[doc="<p>The private key; the contents of the certificate's domain.kex file.</p>"]
    #[serde(rename="PrivateKey")]
    pub private_key: String,
}

#[doc="<p>Describes a stack.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Stack {
    #[doc="<p>The agent version. This parameter is set to <code>LATEST</code> for auto-update. or a version number for a fixed agent version.</p>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>The stack's ARN.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>The stack's attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="ChefConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    #[doc="<p>The configuration manager.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[doc="<p>The date when the stack was created.</p>"]
    #[serde(rename="CreatedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename="CustomCookbooksSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    #[doc="<p>A JSON object that contains user-defined attributes to be added to the stack configuration and deployment attributes. You can use custom JSON to override the corresponding default stack configuration attribute values or to pass data to recipes. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The stack's default Availability Zone. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="DefaultAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_availability_zone: Option<String>,
    #[doc="<p>The ARN of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    #[doc="<p>The stack's default operating system.</p>"]
    #[serde(rename="DefaultOs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_os: Option<String>,
    #[doc="<p>The default root device type. This value is used by default for all instances in the stack, but you can override it when you create an instance. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="DefaultRootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_root_device_type: Option<String>,
    #[doc="<p>A default Amazon EC2 key pair for the stack's instances. You can override this value when you create or update an instance.</p>"]
    #[serde(rename="DefaultSshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    #[doc="<p>The default subnet ID; applicable only if the stack is running in a VPC.</p>"]
    #[serde(rename="DefaultSubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_subnet_id: Option<String>,
    #[doc="<p>The stack host name theme, with spaces replaced by underscores.</p>"]
    #[serde(rename="HostnameTheme")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname_theme: Option<String>,
    #[doc="<p>The stack name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The stack AWS region, such as \"ap-northeast-2\". For more information about AWS regions, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[doc="<p>The stack AWS Identity and Access Management (IAM) role.</p>"]
    #[serde(rename="ServiceRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role_arn: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
    #[doc="<p>Whether the stack uses custom cookbooks.</p>"]
    #[serde(rename="UseCustomCookbooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    #[doc="<p>Whether the stack automatically associates the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p>"]
    #[serde(rename="UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    #[doc="<p>The VPC ID; applicable only if the stack is running in a VPC.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
}

#[doc="<p>Describes the configuration manager.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StackConfigurationManager {
    #[doc="<p>The name. This parameter must be set to \"Chef\".</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The Chef version. This parameter must be set to 12, 11.10, or 11.4 for Linux stacks, and to 12.2 for Windows stacks. The default value for Linux stacks is 11.4.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}

#[doc="<p>Summarizes the number of layers, instances, and apps in a stack.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StackSummary {
    #[doc="<p>The number of apps.</p>"]
    #[serde(rename="AppsCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps_count: Option<i64>,
    #[doc="<p>The stack's ARN.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<String>,
    #[doc="<p>An <code>InstancesCount</code> object with the number of instances in each status.</p>"]
    #[serde(rename="InstancesCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances_count: Option<InstancesCount>,
    #[doc="<p>The number of layers.</p>"]
    #[serde(rename="LayersCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layers_count: Option<i64>,
    #[doc="<p>The stack name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StartStackRequest {
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct StopStackRequest {
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct TagResourceRequest {
    #[doc="<p>The stack or layer's Amazon Resource Number (ARN).</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>A map that contains tag keys and tag values that are attached to a stack or layer.</p> <ul> <li> <p>The key cannot be empty.</p> </li> <li> <p>The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>Leading and trailing white spaces are trimmed from both the key and value.</p> </li> <li> <p>A maximum of 40 tags is allowed for any resource.</p> </li> </ul>"]
    #[serde(rename="Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[doc="<p>Contains the data needed by RDP clients such as the Microsoft Remote Desktop Connection to log in to the instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TemporaryCredential {
    #[doc="<p>The instance's AWS OpsWorks Stacks ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>The password.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
    #[doc="<p>The length of time (in minutes) that the grant is valid. When the grant expires, at the end of this period, the user will no longer be able to use the credentials to log in. If they are logged in at the time, they will be automatically logged out.</p>"]
    #[serde(rename="ValidForInMinutes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub valid_for_in_minutes: Option<i64>,
}

#[doc="<p>Describes an instance's time-based auto scaling configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TimeBasedAutoScalingConfiguration {
    #[doc="<p>A <code>WeeklyAutoScalingSchedule</code> object with the instance schedule.</p>"]
    #[serde(rename="AutoScalingSchedule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_schedule: Option<WeeklyAutoScalingSchedule>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UnassignInstanceRequest {
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UnassignVolumeRequest {
    #[doc="<p>The volume ID.</p>"]
    #[serde(rename="VolumeId")]
    pub volume_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UntagResourceRequest {
    #[doc="<p>The stack or layer's Amazon Resource Number (ARN).</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>A list of the keys of tags to be removed from a stack or layer.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateAppRequest {
    #[doc="<p>The app ID.</p>"]
    #[serde(rename="AppId")]
    pub app_id: String,
    #[doc="<p>A <code>Source</code> object that specifies the app repository.</p>"]
    #[serde(rename="AppSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_source: Option<Source>,
    #[doc="<p>One or more user-defined key/value pairs to be added to the stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app's data sources.</p>"]
    #[serde(rename="DataSources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[doc="<p>A description of the app.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The app's virtual host settings, with multiple domains separated by commas. For example: <code>'www.example.com, example.com'</code> </p>"]
    #[serde(rename="Domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[doc="<p>Whether SSL is enabled for the app.</p>"]
    #[serde(rename="EnableSsl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_ssl: Option<bool>,
    #[doc="<p>An array of <code>EnvironmentVariable</code> objects that specify environment variables to be associated with the app. After you deploy the app, these variables are defined on the associated app server instances.For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html#workingapps-creating-environment\"> Environment Variables</a>.</p> <p>There is no specific limit on the number of environment variables. However, the size of the associated data structure - which includes the variables' names, values, and protected flag values - cannot exceed 10 KB (10240 Bytes). This limit should accommodate most if not all use cases. Exceeding it will cause an exception with the message, \"Environment: is too large (maximum is 10KB).\"</p> <note> <p>This parameter is supported only by Chef 11.10 stacks. If you have specified one or more environment variables, you cannot modify the stack's Chef version.</p> </note>"]
    #[serde(rename="Environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    #[doc="<p>The app name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An <code>SslConfiguration</code> object with the SSL configuration.</p>"]
    #[serde(rename="SslConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
    #[doc="<p>The app type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateElasticIpRequest {
    #[doc="<p>The address.</p>"]
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
    #[doc="<p>The new name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateInstanceRequest {
    #[doc="<p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p> <code>INHERIT</code> - Use the stack's default agent version setting.</p> </li> <li> <p> <i>version_number</i> - Use the specified agent version. This value overrides the stack's default setting. To update the agent version, you must edit the instance configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the instance.</p> </li> </ul> <p>The default setting is <code>INHERIT</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>.</p> <p>AgentVersion cannot be set to Chef 12.2.</p>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>The ID of the AMI that was used to create the instance. The value of this parameter must be the same AMI ID that the instance is already using. You cannot apply a new AMI to an instance by running UpdateInstance. UpdateInstance does not work on instances that are using custom AMIs. </p>"]
    #[serde(rename="AmiId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ami_id: Option<String>,
    #[doc="<p>The instance architecture. Instance types do not necessarily support both architectures. For a list of the architectures that are supported by the different instance types, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html\">Instance Families and Types</a>.</p>"]
    #[serde(rename="Architecture")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub architecture: Option<String>,
    #[doc="<p>For load-based or time-based instances, the type. Windows stacks can use only time-based instances.</p>"]
    #[serde(rename="AutoScalingType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_type: Option<String>,
    #[doc="<p>This property cannot be updated.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[doc="<p>The instance host name.</p>"]
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or by manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    pub instance_id: String,
    #[doc="<p>The instance type, such as <code>t2.micro</code>. For a list of supported instance types, open the stack in the console, choose <b>Instances</b>, and choose <b>+ Instance</b>. The <b>Size</b> list contains the currently supported types. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html\">Instance Families and Types</a>. The parameter values that you use to specify the various types are in the <b>API Name</b> column of the <b>Available Instance Types</b> table.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The instance's layer IDs.</p>"]
    #[serde(rename="LayerIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub layer_ids: Option<Vec<String>>,
    #[doc="<p>The instance's operating system, which must be set to one of the following. You cannot update an instance that is using a custom AMI.</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> </ul> <p>For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">AWS OpsWorks Stacks Operating Systems</a>.</p> <p>The default option is the current Amazon Linux version. If you set this parameter to <code>Custom</code>, you must use the AmiId parameter to specify the custom AMI that you want to use. For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">Operating Systems</a>. For more information on how to use custom AMIs with OpsWorks, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Using Custom AMIs</a>.</p> <note> <p>You can specify a different Linux operating system for the updated stack, but you cannot change from Linux to Windows or Windows to Linux.</p> </note>"]
    #[serde(rename="Os")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub os: Option<String>,
    #[doc="<p>The instance's Amazon EC2 key name.</p>"]
    #[serde(rename="SshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key_name: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateLayerRequest {
    #[doc="<p>One or more user-defined key/value pairs to be added to the stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Whether to automatically assign an <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP address</a> to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignElasticIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_elastic_ips: Option<bool>,
    #[doc="<p>For stacks that are running in a VPC, whether to automatically assign a public IP address to the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-edit.html\">How to Edit a Layer</a>.</p>"]
    #[serde(rename="AutoAssignPublicIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_assign_public_ips: Option<bool>,
    #[doc="<p>Specifies CloudWatch Logs configuration options for the layer. For more information, see <a>CloudWatchLogsLogStream</a>.</p>"]
    #[serde(rename="CloudWatchLogsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,
    #[doc="<p>The ARN of an IAM profile to be used for all of the layer's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="CustomInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    #[doc="<p>A JSON-formatted string containing custom stack configuration and deployment attributes to be installed on the layer's instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-json-override.html\"> Using Custom JSON</a>. </p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>A <code>LayerCustomRecipes</code> object that specifies the layer's custom recipes.</p>"]
    #[serde(rename="CustomRecipes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_recipes: Option<Recipes>,
    #[doc="<p>An array containing the layer's custom security group IDs.</p>"]
    #[serde(rename="CustomSecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    #[doc="<p>Whether to disable auto healing for the layer.</p>"]
    #[serde(rename="EnableAutoHealing")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enable_auto_healing: Option<bool>,
    #[doc="<p>Whether to install operating system and package updates when the instance boots. The default value is <code>true</code>. To control when updates are installed, set this value to <code>false</code>. You must then update your instances manually by using <a>CreateDeployment</a> to run the <code>update_dependencies</code> stack command or manually running <code>yum</code> (Amazon Linux) or <code>apt-get</code> (Ubuntu) on the instances. </p> <note> <p>We strongly recommend using the default value of <code>true</code>, to ensure that your instances have the latest security updates.</p> </note>"]
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    #[doc="<p>The layer ID.</p>"]
    #[serde(rename="LayerId")]
    pub layer_id: String,
    #[doc="<p/>"]
    #[serde(rename="LifecycleEventConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    #[doc="<p>The layer name, which is used by the console.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An array of <code>Package</code> objects that describe the layer's packages.</p>"]
    #[serde(rename="Packages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[doc="<p>For custom layers only, use this parameter to specify the layer's short name, which is used internally by AWS OpsWorks Stacks and by Chef. The short name is also used as the name for the directory where your app files are installed. It can have a maximum of 200 characters and must be in the following format: /\\A[a-z0-9\\-\\_\\.]+\\Z/.</p> <p>The built-in layers' short names are defined by AWS OpsWorks Stacks. For more information, see the <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/layers.html\">Layer Reference</a> </p>"]
    #[serde(rename="Shortname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shortname: Option<String>,
    #[doc="<p>Whether to use Amazon EBS-optimized instances.</p>"]
    #[serde(rename="UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    #[doc="<p>A <code>VolumeConfigurations</code> object that describes the layer's Amazon EBS volumes.</p>"]
    #[serde(rename="VolumeConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateMyUserProfileRequest {
    #[doc="<p>The user's SSH public key.</p>"]
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRdsDbInstanceRequest {
    #[doc="<p>The database password.</p>"]
    #[serde(rename="DbPassword")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db_password: Option<String>,
    #[doc="<p>The master user name.</p>"]
    #[serde(rename="DbUser")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db_user: Option<String>,
    #[doc="<p>The Amazon RDS instance's ARN.</p>"]
    #[serde(rename="RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateStackRequest {
    #[doc="<p>The default AWS OpsWorks Stacks agent version. You have the following options:</p> <ul> <li> <p>Auto-update - Set this parameter to <code>LATEST</code>. AWS OpsWorks Stacks automatically installs new agent versions on the stack's instances as soon as they are available.</p> </li> <li> <p>Fixed version - Set this parameter to your preferred agent version. To update the agent version, you must edit the stack configuration and specify a new version. AWS OpsWorks Stacks then automatically installs that version on the stack's instances.</p> </li> </ul> <p>The default setting is <code>LATEST</code>. To specify an agent version, you must use the complete version number, not the abbreviated number shown on the console. For a list of available agent version numbers, call <a>DescribeAgentVersions</a>. AgentVersion cannot be set to Chef 12.2.</p> <note> <p>You can also specify an agent version when you create or update an instance, which overrides the stack's default setting.</p> </note>"]
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agent_version: Option<String>,
    #[doc="<p>One or more user-defined key-value pairs to be added to the stack attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A <code>ChefConfiguration</code> object that specifies whether to enable Berkshelf and the Berkshelf version on Chef 11.10 stacks. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="ChefConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chef_configuration: Option<ChefConfiguration>,
    #[doc="<p>The configuration manager. When you update a stack, we recommend that you use the configuration manager to specify the Chef version: 12, 11.10, or 11.4 for Linux stacks, or 12.2 for Windows stacks. The default value for Linux stacks is currently 11.4.</p>"]
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configuration_manager: Option<StackConfigurationManager>,
    #[serde(rename="CustomCookbooksSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_cookbooks_source: Option<Source>,
    #[doc="<p>A string that contains user-defined, custom JSON. It can be used to override the corresponding default stack configuration JSON values or to pass data to recipes. The string should be in the following format:</p> <p> <code>\"{\\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\",...}\"</code> </p> <p>For more information on custom JSON, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-json.html\">Use Custom JSON to Modify the Stack Configuration Attributes</a>.</p>"]
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_json: Option<String>,
    #[doc="<p>The stack's default Availability Zone, which must be in the stack's region. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>. If you also specify a value for <code>DefaultSubnetId</code>, the subnet must be in the same zone. For more information, see <a>CreateStack</a>. </p>"]
    #[serde(rename="DefaultAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_availability_zone: Option<String>,
    #[doc="<p>The ARN of an IAM profile that is the default profile for all of the stack's EC2 instances. For more information about IAM ARNs, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html\">Using Identifiers</a>.</p>"]
    #[serde(rename="DefaultInstanceProfileArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_instance_profile_arn: Option<String>,
    #[doc="<p>The stack's operating system, which must be set to one of the following:</p> <ul> <li> <p>A supported Linux operating system: An Amazon Linux version, such as <code>Amazon Linux 2017.03</code>, <code>Amazon Linux 2016.09</code>, <code>Amazon Linux 2016.03</code>, <code>Amazon Linux 2015.09</code>, or <code>Amazon Linux 2015.03</code>.</p> </li> <li> <p>A supported Ubuntu operating system, such as <code>Ubuntu 16.04 LTS</code>, <code>Ubuntu 14.04 LTS</code>, or <code>Ubuntu 12.04 LTS</code>.</p> </li> <li> <p> <code>CentOS Linux 7</code> </p> </li> <li> <p> <code>Red Hat Enterprise Linux 7</code> </p> </li> <li> <p>A supported Windows operating system, such as <code>Microsoft Windows Server 2012 R2 Base</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Express</code>, <code>Microsoft Windows Server 2012 R2 with SQL Server Standard</code>, or <code>Microsoft Windows Server 2012 R2 with SQL Server Web</code>.</p> </li> <li> <p>A custom AMI: <code>Custom</code>. You specify the custom AMI you want to use when you create instances. For more information on how to use custom AMIs with OpsWorks, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-custom-ami.html\">Using Custom AMIs</a>.</p> </li> </ul> <p>The default option is the stack's current operating system. For more information on the supported operating systems, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-os.html\">AWS OpsWorks Stacks Operating Systems</a>.</p>"]
    #[serde(rename="DefaultOs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_os: Option<String>,
    #[doc="<p>The default root device type. This value is used by default for all instances in the stack, but you can override it when you create an instance. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ComponentsAMIs.html#storage-for-the-root-device\">Storage for the Root Device</a>.</p>"]
    #[serde(rename="DefaultRootDeviceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_root_device_type: Option<String>,
    #[doc="<p>A default Amazon EC2 key-pair name. The default value is <code>none</code>. If you specify a key-pair name, AWS OpsWorks Stacks installs the public key on the instance and you can use the private key with an SSH client to log in to the instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-ssh.html\"> Using SSH to Communicate with an Instance</a> and <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-ssh-access.html\"> Managing SSH Access</a>. You can override this setting by specifying a different key pair, or no key pair, when you <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html\"> create an instance</a>. </p>"]
    #[serde(rename="DefaultSshKeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    #[doc="<p>The stack's default VPC subnet ID. This parameter is required if you specify a value for the <code>VpcId</code> parameter. All instances are launched into this subnet unless you specify otherwise when you create the instance. If you also specify a value for <code>DefaultAvailabilityZone</code>, the subnet must be in that zone. For information on default values and when this parameter is required, see the <code>VpcId</code> parameter description. </p>"]
    #[serde(rename="DefaultSubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_subnet_id: Option<String>,
    #[doc="<p>The stack's new host name theme, with spaces replaced by underscores. The theme is used to generate host names for the stack's instances. By default, <code>HostnameTheme</code> is set to <code>Layer_Dependent</code>, which creates host names by appending integers to the layer's short name. The other themes are:</p> <ul> <li> <p> <code>Baked_Goods</code> </p> </li> <li> <p> <code>Clouds</code> </p> </li> <li> <p> <code>Europe_Cities</code> </p> </li> <li> <p> <code>Fruits</code> </p> </li> <li> <p> <code>Greek_Deities</code> </p> </li> <li> <p> <code>Legendary_creatures_from_Japan</code> </p> </li> <li> <p> <code>Planets_and_Moons</code> </p> </li> <li> <p> <code>Roman_Deities</code> </p> </li> <li> <p> <code>Scottish_Islands</code> </p> </li> <li> <p> <code>US_Cities</code> </p> </li> <li> <p> <code>Wild_Cats</code> </p> </li> </ul> <p>To obtain a generated host name, call <code>GetHostNameSuggestion</code>, which returns a host name based on the current theme.</p>"]
    #[serde(rename="HostnameTheme")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname_theme: Option<String>,
    #[doc="<p>The stack's new name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Do not use this parameter. You cannot update a stack's service role.</p>"]
    #[serde(rename="ServiceRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role_arn: Option<String>,
    #[doc="<p>The stack ID.</p>"]
    #[serde(rename="StackId")]
    pub stack_id: String,
    #[doc="<p>Whether the stack uses custom cookbooks.</p>"]
    #[serde(rename="UseCustomCookbooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    #[doc="<p>Whether to associate the AWS OpsWorks Stacks built-in security groups with the stack's layers.</p> <p>AWS OpsWorks Stacks provides a standard set of built-in security groups, one for each layer, which are associated with layers by default. <code>UseOpsworksSecurityGroups</code> allows you to provide your own custom security groups instead of using the built-in groups. <code>UseOpsworksSecurityGroups</code> has the following settings: </p> <ul> <li> <p>True - AWS OpsWorks Stacks automatically associates the appropriate built-in security group with each layer (default setting). You can associate additional security groups with a layer after you create it, but you cannot delete the built-in security group.</p> </li> <li> <p>False - AWS OpsWorks Stacks does not associate built-in security groups with layers. You must create appropriate EC2 security groups and associate a security group with each layer that you create. However, you can still manually associate a built-in security group with a layer on. Custom security groups are required only for those layers that need custom settings.</p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-creating.html\">Create a New Stack</a>.</p>"]
    #[serde(rename="UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserProfileRequest {
    #[doc="<p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html\">Managing User Permissions</a>.</p>"]
    #[serde(rename="AllowSelfManagement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_self_management: Option<bool>,
    #[doc="<p>The user IAM ARN. This can also be a federated user's ARN.</p>"]
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    #[doc="<p>The user's new SSH public key.</p>"]
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The user's SSH user name. The allowable characters are [a-z], [A-Z], [0-9], '-', and '_'. If the specified name includes other punctuation marks, AWS OpsWorks Stacks removes them. For example, <code>my.name</code> will be changed to <code>myname</code>. If you do not specify an SSH user name, AWS OpsWorks Stacks generates one from the IAM user name. </p>"]
    #[serde(rename="SshUsername")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_username: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateVolumeRequest {
    #[doc="<p>The new mount point.</p>"]
    #[serde(rename="MountPoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_point: Option<String>,
    #[doc="<p>The new name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The volume ID.</p>"]
    #[serde(rename="VolumeId")]
    pub volume_id: String,
}

#[doc="<p>Describes a user's SSH information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserProfile {
    #[doc="<p>Whether users can specify their own SSH public key through the My Settings page. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/security-settingsshkey.html\">Managing User Permissions</a>.</p>"]
    #[serde(rename="AllowSelfManagement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_self_management: Option<bool>,
    #[doc="<p>The user's IAM ARN.</p>"]
    #[serde(rename="IamUserArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[doc="<p>The user's name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The user's SSH public key.</p>"]
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_public_key: Option<String>,
    #[doc="<p>The user's SSH user name.</p>"]
    #[serde(rename="SshUsername")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_username: Option<String>,
}

#[doc="<p>Describes an instance's Amazon EBS volume.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Volume {
    #[doc="<p>The volume Availability Zone. For more information, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The device name.</p>"]
    #[serde(rename="Device")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device: Option<String>,
    #[doc="<p>The Amazon EC2 volume ID.</p>"]
    #[serde(rename="Ec2VolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_volume_id: Option<String>,
    #[doc="<p>The instance ID.</p>"]
    #[serde(rename="InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_id: Option<String>,
    #[doc="<p>For PIOPS volumes, the IOPS per disk.</p>"]
    #[serde(rename="Iops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iops: Option<i64>,
    #[doc="<p>The volume mount point. For example, \"/mnt/disk1\".</p>"]
    #[serde(rename="MountPoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mount_point: Option<String>,
    #[doc="<p>The volume name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The RAID array ID.</p>"]
    #[serde(rename="RaidArrayId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_array_id: Option<String>,
    #[doc="<p>The AWS region. For more information about AWS regions, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html\">Regions and Endpoints</a>.</p>"]
    #[serde(rename="Region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<String>,
    #[doc="<p>The volume size.</p>"]
    #[serde(rename="Size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[doc="<p>The value returned by <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeVolumes.html\">DescribeVolumes</a>.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The volume ID.</p>"]
    #[serde(rename="VolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_id: Option<String>,
    #[doc="<p>The volume type, standard or PIOPS.</p>"]
    #[serde(rename="VolumeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_type: Option<String>,
}

#[doc="<p>Describes an Amazon EBS volume configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct VolumeConfiguration {
    #[doc="<p>For PIOPS volumes, the IOPS per disk.</p>"]
    #[serde(rename="Iops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iops: Option<i64>,
    #[doc="<p>The volume mount point. For example \"/dev/sdh\".</p>"]
    #[serde(rename="MountPoint")]
    pub mount_point: String,
    #[doc="<p>The number of disks in the volume.</p>"]
    #[serde(rename="NumberOfDisks")]
    pub number_of_disks: i64,
    #[doc="<p>The volume <a href=\"http://en.wikipedia.org/wiki/Standard_RAID_levels\">RAID level</a>.</p>"]
    #[serde(rename="RaidLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raid_level: Option<i64>,
    #[doc="<p>The volume size.</p>"]
    #[serde(rename="Size")]
    pub size: i64,
    #[doc="<p>The volume type:</p> <ul> <li> <p> <code>standard</code> - Magnetic</p> </li> <li> <p> <code>io1</code> - Provisioned IOPS (SSD)</p> </li> <li> <p> <code>gp2</code> - General Purpose (SSD)</p> </li> </ul>"]
    #[serde(rename="VolumeType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_type: Option<String>,
}

#[doc="<p>Describes a time-based instance's auto scaling schedule. The schedule consists of a set of key-value pairs.</p> <ul> <li> <p>The key is the time period (a UTC hour) and must be an integer from 0 - 23.</p> </li> <li> <p>The value indicates whether the instance should be online or offline for the specified period, and must be set to \"on\" or \"off\"</p> </li> </ul> <p>The default setting for all time periods is off, so you use the following parameters primarily to specify the online periods. You don't have to explicitly specify offline periods unless you want to change an online period to an offline period.</p> <p>The following example specifies that the instance should be online for four hours, from UTC 1200 - 1600. It will be off for the remainder of the day.</p> <p> <code> { \"12\":\"on\", \"13\":\"on\", \"14\":\"on\", \"15\":\"on\" } </code> </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct WeeklyAutoScalingSchedule {
    #[doc="<p>The schedule for Friday.</p>"]
    #[serde(rename="Friday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub friday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Monday.</p>"]
    #[serde(rename="Monday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub monday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Saturday.</p>"]
    #[serde(rename="Saturday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub saturday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Sunday.</p>"]
    #[serde(rename="Sunday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sunday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Thursday.</p>"]
    #[serde(rename="Thursday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thursday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Tuesday.</p>"]
    #[serde(rename="Tuesday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tuesday: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The schedule for Wednesday.</p>"]
    #[serde(rename="Wednesday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wednesday: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by AssignInstance
#[derive(Debug, PartialEq)]
pub enum AssignInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssignInstanceError {
    pub fn from_body(body: &str) -> AssignInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        AssignInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssignInstanceError::Validation(error_message.to_string())
                    }
                    _ => AssignInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssignInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssignInstanceError {
    fn from(err: serde_json::error::Error) -> AssignInstanceError {
        AssignInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssignInstanceError {
    fn from(err: CredentialsError) -> AssignInstanceError {
        AssignInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssignInstanceError {
    fn from(err: HttpDispatchError) -> AssignInstanceError {
        AssignInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for AssignInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssignInstanceError {
    fn description(&self) -> &str {
        match *self {
            AssignInstanceError::ResourceNotFound(ref cause) => cause,
            AssignInstanceError::Validation(ref cause) => cause,
            AssignInstanceError::Credentials(ref err) => err.description(),
            AssignInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssignInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssignVolume
#[derive(Debug, PartialEq)]
pub enum AssignVolumeError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssignVolumeError {
    pub fn from_body(body: &str) -> AssignVolumeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        AssignVolumeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssignVolumeError::Validation(error_message.to_string())
                    }
                    _ => AssignVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssignVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssignVolumeError {
    fn from(err: serde_json::error::Error) -> AssignVolumeError {
        AssignVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssignVolumeError {
    fn from(err: CredentialsError) -> AssignVolumeError {
        AssignVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssignVolumeError {
    fn from(err: HttpDispatchError) -> AssignVolumeError {
        AssignVolumeError::HttpDispatch(err)
    }
}
impl fmt::Display for AssignVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssignVolumeError {
    fn description(&self) -> &str {
        match *self {
            AssignVolumeError::ResourceNotFound(ref cause) => cause,
            AssignVolumeError::Validation(ref cause) => cause,
            AssignVolumeError::Credentials(ref err) => err.description(),
            AssignVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssignVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateElasticIp
#[derive(Debug, PartialEq)]
pub enum AssociateElasticIpError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateElasticIpError {
    pub fn from_body(body: &str) -> AssociateElasticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        AssociateElasticIpError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateElasticIpError::Validation(error_message.to_string())
                    }
                    _ => AssociateElasticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateElasticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateElasticIpError {
    fn from(err: serde_json::error::Error) -> AssociateElasticIpError {
        AssociateElasticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateElasticIpError {
    fn from(err: CredentialsError) -> AssociateElasticIpError {
        AssociateElasticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateElasticIpError {
    fn from(err: HttpDispatchError) -> AssociateElasticIpError {
        AssociateElasticIpError::HttpDispatch(err)
    }
}
impl fmt::Display for AssociateElasticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateElasticIpError {
    fn description(&self) -> &str {
        match *self {
            AssociateElasticIpError::ResourceNotFound(ref cause) => cause,
            AssociateElasticIpError::Validation(ref cause) => cause,
            AssociateElasticIpError::Credentials(ref err) => err.description(),
            AssociateElasticIpError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateElasticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachElasticLoadBalancer
#[derive(Debug, PartialEq)]
pub enum AttachElasticLoadBalancerError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachElasticLoadBalancerError {
    pub fn from_body(body: &str) -> AttachElasticLoadBalancerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => AttachElasticLoadBalancerError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        AttachElasticLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => AttachElasticLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachElasticLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachElasticLoadBalancerError {
    fn from(err: serde_json::error::Error) -> AttachElasticLoadBalancerError {
        AttachElasticLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachElasticLoadBalancerError {
    fn from(err: CredentialsError) -> AttachElasticLoadBalancerError {
        AttachElasticLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachElasticLoadBalancerError {
    fn from(err: HttpDispatchError) -> AttachElasticLoadBalancerError {
        AttachElasticLoadBalancerError::HttpDispatch(err)
    }
}
impl fmt::Display for AttachElasticLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachElasticLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            AttachElasticLoadBalancerError::ResourceNotFound(ref cause) => cause,
            AttachElasticLoadBalancerError::Validation(ref cause) => cause,
            AttachElasticLoadBalancerError::Credentials(ref err) => err.description(),
            AttachElasticLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachElasticLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CloneStack
#[derive(Debug, PartialEq)]
pub enum CloneStackError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CloneStackError {
    pub fn from_body(body: &str) -> CloneStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        CloneStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => CloneStackError::Validation(error_message.to_string()),
                    _ => CloneStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => CloneStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CloneStackError {
    fn from(err: serde_json::error::Error) -> CloneStackError {
        CloneStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CloneStackError {
    fn from(err: CredentialsError) -> CloneStackError {
        CloneStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CloneStackError {
    fn from(err: HttpDispatchError) -> CloneStackError {
        CloneStackError::HttpDispatch(err)
    }
}
impl fmt::Display for CloneStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CloneStackError {
    fn description(&self) -> &str {
        match *self {
            CloneStackError::ResourceNotFound(ref cause) => cause,
            CloneStackError::Validation(ref cause) => cause,
            CloneStackError::Credentials(ref err) => err.description(),
            CloneStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CloneStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateAppError {
    pub fn from_body(body: &str) -> CreateAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        CreateAppError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => CreateAppError::Validation(error_message.to_string()),
                    _ => CreateAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAppError {
    fn from(err: serde_json::error::Error) -> CreateAppError {
        CreateAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAppError {
    fn from(err: CredentialsError) -> CreateAppError {
        CreateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppError {
    fn from(err: HttpDispatchError) -> CreateAppError {
        CreateAppError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::ResourceNotFound(ref cause) => cause,
            CreateAppError::Validation(ref cause) => cause,
            CreateAppError::Credentials(ref err) => err.description(),
            CreateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDeploymentError {
    pub fn from_body(body: &str) -> CreateDeploymentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        CreateDeploymentError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeploymentError::Validation(error_message.to_string())
                    }
                    _ => CreateDeploymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeploymentError::Unknown(String::from(body)),
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
            CreateDeploymentError::ResourceNotFound(ref cause) => cause,
            CreateDeploymentError::Validation(ref cause) => cause,
            CreateDeploymentError::Credentials(ref err) => err.description(),
            CreateDeploymentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDeploymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstance
#[derive(Debug, PartialEq)]
pub enum CreateInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateInstanceError {
    pub fn from_body(body: &str) -> CreateInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        CreateInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateInstanceError::Validation(error_message.to_string())
                    }
                    _ => CreateInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInstanceError {
    fn from(err: serde_json::error::Error) -> CreateInstanceError {
        CreateInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInstanceError {
    fn from(err: CredentialsError) -> CreateInstanceError {
        CreateInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInstanceError {
    fn from(err: HttpDispatchError) -> CreateInstanceError {
        CreateInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateInstanceError::ResourceNotFound(ref cause) => cause,
            CreateInstanceError::Validation(ref cause) => cause,
            CreateInstanceError::Credentials(ref err) => err.description(),
            CreateInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLayer
#[derive(Debug, PartialEq)]
pub enum CreateLayerError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateLayerError {
    pub fn from_body(body: &str) -> CreateLayerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        CreateLayerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLayerError::Validation(error_message.to_string())
                    }
                    _ => CreateLayerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLayerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLayerError {
    fn from(err: serde_json::error::Error) -> CreateLayerError {
        CreateLayerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLayerError {
    fn from(err: CredentialsError) -> CreateLayerError {
        CreateLayerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLayerError {
    fn from(err: HttpDispatchError) -> CreateLayerError {
        CreateLayerError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateLayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLayerError {
    fn description(&self) -> &str {
        match *self {
            CreateLayerError::ResourceNotFound(ref cause) => cause,
            CreateLayerError::Validation(ref cause) => cause,
            CreateLayerError::Credentials(ref err) => err.description(),
            CreateLayerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLayerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStack
#[derive(Debug, PartialEq)]
pub enum CreateStackError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateStackError {
    pub fn from_body(body: &str) -> CreateStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        CreateStackError::Validation(error_message.to_string())
                    }
                    _ => CreateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStackError {
    fn from(err: serde_json::error::Error) -> CreateStackError {
        CreateStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStackError {
    fn from(err: CredentialsError) -> CreateStackError {
        CreateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStackError {
    fn from(err: HttpDispatchError) -> CreateStackError {
        CreateStackError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackError {
    fn description(&self) -> &str {
        match *self {
            CreateStackError::Validation(ref cause) => cause,
            CreateStackError::Credentials(ref err) => err.description(),
            CreateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserProfileError {
    pub fn from_body(body: &str) -> CreateUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        CreateUserProfileError::Validation(error_message.to_string())
                    }
                    _ => CreateUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserProfileError {
    fn from(err: serde_json::error::Error) -> CreateUserProfileError {
        CreateUserProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserProfileError {
    fn from(err: CredentialsError) -> CreateUserProfileError {
        CreateUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserProfileError {
    fn from(err: HttpDispatchError) -> CreateUserProfileError {
        CreateUserProfileError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateUserProfileError::Validation(ref cause) => cause,
            CreateUserProfileError::Credentials(ref err) => err.description(),
            CreateUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteAppError {
    pub fn from_body(body: &str) -> DeleteAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeleteAppError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => DeleteAppError::Validation(error_message.to_string()),
                    _ => DeleteAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAppError {
    fn from(err: serde_json::error::Error) -> DeleteAppError {
        DeleteAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppError {
    fn from(err: CredentialsError) -> DeleteAppError {
        DeleteAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppError {
    fn from(err: HttpDispatchError) -> DeleteAppError {
        DeleteAppError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::ResourceNotFound(ref cause) => cause,
            DeleteAppError::Validation(ref cause) => cause,
            DeleteAppError::Credentials(ref err) => err.description(),
            DeleteAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteInstanceError {
    pub fn from_body(body: &str) -> DeleteInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeleteInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeleteInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteInstanceError {
    fn from(err: serde_json::error::Error) -> DeleteInstanceError {
        DeleteInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInstanceError {
    fn from(err: CredentialsError) -> DeleteInstanceError {
        DeleteInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInstanceError {
    fn from(err: HttpDispatchError) -> DeleteInstanceError {
        DeleteInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceError::ResourceNotFound(ref cause) => cause,
            DeleteInstanceError::Validation(ref cause) => cause,
            DeleteInstanceError::Credentials(ref err) => err.description(),
            DeleteInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLayer
#[derive(Debug, PartialEq)]
pub enum DeleteLayerError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteLayerError {
    pub fn from_body(body: &str) -> DeleteLayerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeleteLayerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLayerError::Validation(error_message.to_string())
                    }
                    _ => DeleteLayerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLayerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLayerError {
    fn from(err: serde_json::error::Error) -> DeleteLayerError {
        DeleteLayerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLayerError {
    fn from(err: CredentialsError) -> DeleteLayerError {
        DeleteLayerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLayerError {
    fn from(err: HttpDispatchError) -> DeleteLayerError {
        DeleteLayerError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteLayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLayerError {
    fn description(&self) -> &str {
        match *self {
            DeleteLayerError::ResourceNotFound(ref cause) => cause,
            DeleteLayerError::Validation(ref cause) => cause,
            DeleteLayerError::Credentials(ref err) => err.description(),
            DeleteLayerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLayerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteStackError {
    pub fn from_body(body: &str) -> DeleteStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeleteStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStackError::Validation(error_message.to_string())
                    }
                    _ => DeleteStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStackError {
    fn from(err: serde_json::error::Error) -> DeleteStackError {
        DeleteStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStackError {
    fn from(err: CredentialsError) -> DeleteStackError {
        DeleteStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStackError {
    fn from(err: HttpDispatchError) -> DeleteStackError {
        DeleteStackError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStackError {
    fn description(&self) -> &str {
        match *self {
            DeleteStackError::ResourceNotFound(ref cause) => cause,
            DeleteStackError::Validation(ref cause) => cause,
            DeleteStackError::Credentials(ref err) => err.description(),
            DeleteStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserProfile
#[derive(Debug, PartialEq)]
pub enum DeleteUserProfileError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserProfileError {
    pub fn from_body(body: &str) -> DeleteUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeleteUserProfileError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUserProfileError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserProfileError {
    fn from(err: serde_json::error::Error) -> DeleteUserProfileError {
        DeleteUserProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserProfileError {
    fn from(err: CredentialsError) -> DeleteUserProfileError {
        DeleteUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserProfileError {
    fn from(err: HttpDispatchError) -> DeleteUserProfileError {
        DeleteUserProfileError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserProfileError::ResourceNotFound(ref cause) => cause,
            DeleteUserProfileError::Validation(ref cause) => cause,
            DeleteUserProfileError::Credentials(ref err) => err.description(),
            DeleteUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterEcsCluster
#[derive(Debug, PartialEq)]
pub enum DeregisterEcsClusterError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterEcsClusterError {
    pub fn from_body(body: &str) -> DeregisterEcsClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeregisterEcsClusterError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterEcsClusterError::Validation(error_message.to_string())
                    }
                    _ => DeregisterEcsClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterEcsClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterEcsClusterError {
    fn from(err: serde_json::error::Error) -> DeregisterEcsClusterError {
        DeregisterEcsClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterEcsClusterError {
    fn from(err: CredentialsError) -> DeregisterEcsClusterError {
        DeregisterEcsClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterEcsClusterError {
    fn from(err: HttpDispatchError) -> DeregisterEcsClusterError {
        DeregisterEcsClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterEcsClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterEcsClusterError {
    fn description(&self) -> &str {
        match *self {
            DeregisterEcsClusterError::ResourceNotFound(ref cause) => cause,
            DeregisterEcsClusterError::Validation(ref cause) => cause,
            DeregisterEcsClusterError::Credentials(ref err) => err.description(),
            DeregisterEcsClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterEcsClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterElasticIp
#[derive(Debug, PartialEq)]
pub enum DeregisterElasticIpError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterElasticIpError {
    pub fn from_body(body: &str) -> DeregisterElasticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeregisterElasticIpError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterElasticIpError::Validation(error_message.to_string())
                    }
                    _ => DeregisterElasticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterElasticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterElasticIpError {
    fn from(err: serde_json::error::Error) -> DeregisterElasticIpError {
        DeregisterElasticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterElasticIpError {
    fn from(err: CredentialsError) -> DeregisterElasticIpError {
        DeregisterElasticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterElasticIpError {
    fn from(err: HttpDispatchError) -> DeregisterElasticIpError {
        DeregisterElasticIpError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterElasticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterElasticIpError {
    fn description(&self) -> &str {
        match *self {
            DeregisterElasticIpError::ResourceNotFound(ref cause) => cause,
            DeregisterElasticIpError::Validation(ref cause) => cause,
            DeregisterElasticIpError::Credentials(ref err) => err.description(),
            DeregisterElasticIpError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterElasticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterInstanceError {
    pub fn from_body(body: &str) -> DeregisterInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeregisterInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeregisterInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterInstanceError {
        DeregisterInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterInstanceError {
    fn from(err: CredentialsError) -> DeregisterInstanceError {
        DeregisterInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterInstanceError {
        DeregisterInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterInstanceError::ResourceNotFound(ref cause) => cause,
            DeregisterInstanceError::Validation(ref cause) => cause,
            DeregisterInstanceError::Credentials(ref err) => err.description(),
            DeregisterInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterRdsDbInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterRdsDbInstanceError {
    pub fn from_body(body: &str) -> DeregisterRdsDbInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeregisterRdsDbInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterRdsDbInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeregisterRdsDbInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterRdsDbInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterRdsDbInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterRdsDbInstanceError {
        DeregisterRdsDbInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterRdsDbInstanceError {
    fn from(err: CredentialsError) -> DeregisterRdsDbInstanceError {
        DeregisterRdsDbInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterRdsDbInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterRdsDbInstanceError {
        DeregisterRdsDbInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterRdsDbInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterRdsDbInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterRdsDbInstanceError::ResourceNotFound(ref cause) => cause,
            DeregisterRdsDbInstanceError::Validation(ref cause) => cause,
            DeregisterRdsDbInstanceError::Credentials(ref err) => err.description(),
            DeregisterRdsDbInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterRdsDbInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterVolume
#[derive(Debug, PartialEq)]
pub enum DeregisterVolumeError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterVolumeError {
    pub fn from_body(body: &str) -> DeregisterVolumeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DeregisterVolumeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterVolumeError::Validation(error_message.to_string())
                    }
                    _ => DeregisterVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterVolumeError {
    fn from(err: serde_json::error::Error) -> DeregisterVolumeError {
        DeregisterVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterVolumeError {
    fn from(err: CredentialsError) -> DeregisterVolumeError {
        DeregisterVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterVolumeError {
    fn from(err: HttpDispatchError) -> DeregisterVolumeError {
        DeregisterVolumeError::HttpDispatch(err)
    }
}
impl fmt::Display for DeregisterVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterVolumeError {
    fn description(&self) -> &str {
        match *self {
            DeregisterVolumeError::ResourceNotFound(ref cause) => cause,
            DeregisterVolumeError::Validation(ref cause) => cause,
            DeregisterVolumeError::Credentials(ref err) => err.description(),
            DeregisterVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeregisterVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAgentVersions
#[derive(Debug, PartialEq)]
pub enum DescribeAgentVersionsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAgentVersionsError {
    pub fn from_body(body: &str) -> DescribeAgentVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeAgentVersionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAgentVersionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAgentVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAgentVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAgentVersionsError {
    fn from(err: serde_json::error::Error) -> DescribeAgentVersionsError {
        DescribeAgentVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAgentVersionsError {
    fn from(err: CredentialsError) -> DescribeAgentVersionsError {
        DescribeAgentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAgentVersionsError {
    fn from(err: HttpDispatchError) -> DescribeAgentVersionsError {
        DescribeAgentVersionsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeAgentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAgentVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAgentVersionsError::ResourceNotFound(ref cause) => cause,
            DescribeAgentVersionsError::Validation(ref cause) => cause,
            DescribeAgentVersionsError::Credentials(ref err) => err.description(),
            DescribeAgentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAgentVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApps
#[derive(Debug, PartialEq)]
pub enum DescribeAppsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAppsError {
    pub fn from_body(body: &str) -> DescribeAppsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeAppsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAppsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAppsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAppsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAppsError {
    fn from(err: serde_json::error::Error) -> DescribeAppsError {
        DescribeAppsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAppsError {
    fn from(err: CredentialsError) -> DescribeAppsError {
        DescribeAppsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAppsError {
    fn from(err: HttpDispatchError) -> DescribeAppsError {
        DescribeAppsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAppsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAppsError::ResourceNotFound(ref cause) => cause,
            DescribeAppsError::Validation(ref cause) => cause,
            DescribeAppsError::Credentials(ref err) => err.description(),
            DescribeAppsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAppsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCommands
#[derive(Debug, PartialEq)]
pub enum DescribeCommandsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCommandsError {
    pub fn from_body(body: &str) -> DescribeCommandsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeCommandsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCommandsError::Validation(error_message.to_string())
                    }
                    _ => DescribeCommandsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCommandsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCommandsError {
    fn from(err: serde_json::error::Error) -> DescribeCommandsError {
        DescribeCommandsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCommandsError {
    fn from(err: CredentialsError) -> DescribeCommandsError {
        DescribeCommandsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCommandsError {
    fn from(err: HttpDispatchError) -> DescribeCommandsError {
        DescribeCommandsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCommandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCommandsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCommandsError::ResourceNotFound(ref cause) => cause,
            DescribeCommandsError::Validation(ref cause) => cause,
            DescribeCommandsError::Credentials(ref err) => err.description(),
            DescribeCommandsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeCommandsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeployments
#[derive(Debug, PartialEq)]
pub enum DescribeDeploymentsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDeploymentsError {
    pub fn from_body(body: &str) -> DescribeDeploymentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeDeploymentsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDeploymentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeDeploymentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDeploymentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDeploymentsError {
    fn from(err: serde_json::error::Error) -> DescribeDeploymentsError {
        DescribeDeploymentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDeploymentsError {
    fn from(err: CredentialsError) -> DescribeDeploymentsError {
        DescribeDeploymentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDeploymentsError {
    fn from(err: HttpDispatchError) -> DescribeDeploymentsError {
        DescribeDeploymentsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeploymentsError::ResourceNotFound(ref cause) => cause,
            DescribeDeploymentsError::Validation(ref cause) => cause,
            DescribeDeploymentsError::Credentials(ref err) => err.description(),
            DescribeDeploymentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDeploymentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEcsClusters
#[derive(Debug, PartialEq)]
pub enum DescribeEcsClustersError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEcsClustersError {
    pub fn from_body(body: &str) -> DescribeEcsClustersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeEcsClustersError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEcsClustersError::Validation(error_message.to_string())
                    }
                    _ => DescribeEcsClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEcsClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEcsClustersError {
    fn from(err: serde_json::error::Error) -> DescribeEcsClustersError {
        DescribeEcsClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEcsClustersError {
    fn from(err: CredentialsError) -> DescribeEcsClustersError {
        DescribeEcsClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEcsClustersError {
    fn from(err: HttpDispatchError) -> DescribeEcsClustersError {
        DescribeEcsClustersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEcsClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEcsClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeEcsClustersError::ResourceNotFound(ref cause) => cause,
            DescribeEcsClustersError::Validation(ref cause) => cause,
            DescribeEcsClustersError::Credentials(ref err) => err.description(),
            DescribeEcsClustersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEcsClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeElasticIps
#[derive(Debug, PartialEq)]
pub enum DescribeElasticIpsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeElasticIpsError {
    pub fn from_body(body: &str) -> DescribeElasticIpsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeElasticIpsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeElasticIpsError::Validation(error_message.to_string())
                    }
                    _ => DescribeElasticIpsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeElasticIpsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeElasticIpsError {
    fn from(err: serde_json::error::Error) -> DescribeElasticIpsError {
        DescribeElasticIpsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeElasticIpsError {
    fn from(err: CredentialsError) -> DescribeElasticIpsError {
        DescribeElasticIpsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeElasticIpsError {
    fn from(err: HttpDispatchError) -> DescribeElasticIpsError {
        DescribeElasticIpsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeElasticIpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeElasticIpsError {
    fn description(&self) -> &str {
        match *self {
            DescribeElasticIpsError::ResourceNotFound(ref cause) => cause,
            DescribeElasticIpsError::Validation(ref cause) => cause,
            DescribeElasticIpsError::Credentials(ref err) => err.description(),
            DescribeElasticIpsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeElasticIpsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeElasticLoadBalancers
#[derive(Debug, PartialEq)]
pub enum DescribeElasticLoadBalancersError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeElasticLoadBalancersError {
    pub fn from_body(body: &str) -> DescribeElasticLoadBalancersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeElasticLoadBalancersError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeElasticLoadBalancersError::Validation(error_message.to_string())
                    }
                    _ => DescribeElasticLoadBalancersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeElasticLoadBalancersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeElasticLoadBalancersError {
    fn from(err: serde_json::error::Error) -> DescribeElasticLoadBalancersError {
        DescribeElasticLoadBalancersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeElasticLoadBalancersError {
    fn from(err: CredentialsError) -> DescribeElasticLoadBalancersError {
        DescribeElasticLoadBalancersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeElasticLoadBalancersError {
    fn from(err: HttpDispatchError) -> DescribeElasticLoadBalancersError {
        DescribeElasticLoadBalancersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeElasticLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeElasticLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            DescribeElasticLoadBalancersError::ResourceNotFound(ref cause) => cause,
            DescribeElasticLoadBalancersError::Validation(ref cause) => cause,
            DescribeElasticLoadBalancersError::Credentials(ref err) => err.description(),
            DescribeElasticLoadBalancersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeElasticLoadBalancersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstances
#[derive(Debug, PartialEq)]
pub enum DescribeInstancesError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeInstancesError {
    pub fn from_body(body: &str) -> DescribeInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeInstancesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeInstancesError::Validation(error_message.to_string())
                    }
                    _ => DescribeInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeInstancesError {
        DescribeInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstancesError {
    fn from(err: CredentialsError) -> DescribeInstancesError {
        DescribeInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancesError {
    fn from(err: HttpDispatchError) -> DescribeInstancesError {
        DescribeInstancesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancesError::ResourceNotFound(ref cause) => cause,
            DescribeInstancesError::Validation(ref cause) => cause,
            DescribeInstancesError::Credentials(ref err) => err.description(),
            DescribeInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLayers
#[derive(Debug, PartialEq)]
pub enum DescribeLayersError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeLayersError {
    pub fn from_body(body: &str) -> DescribeLayersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeLayersError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLayersError::Validation(error_message.to_string())
                    }
                    _ => DescribeLayersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLayersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLayersError {
    fn from(err: serde_json::error::Error) -> DescribeLayersError {
        DescribeLayersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLayersError {
    fn from(err: CredentialsError) -> DescribeLayersError {
        DescribeLayersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLayersError {
    fn from(err: HttpDispatchError) -> DescribeLayersError {
        DescribeLayersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeLayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLayersError {
    fn description(&self) -> &str {
        match *self {
            DescribeLayersError::ResourceNotFound(ref cause) => cause,
            DescribeLayersError::Validation(ref cause) => cause,
            DescribeLayersError::Credentials(ref err) => err.description(),
            DescribeLayersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLayersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoadBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum DescribeLoadBasedAutoScalingError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeLoadBasedAutoScalingError {
    pub fn from_body(body: &str) -> DescribeLoadBasedAutoScalingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeLoadBasedAutoScalingError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeLoadBasedAutoScalingError::Validation(error_message.to_string())
                    }
                    _ => DescribeLoadBasedAutoScalingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLoadBasedAutoScalingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLoadBasedAutoScalingError {
    fn from(err: serde_json::error::Error) -> DescribeLoadBasedAutoScalingError {
        DescribeLoadBasedAutoScalingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLoadBasedAutoScalingError {
    fn from(err: CredentialsError) -> DescribeLoadBasedAutoScalingError {
        DescribeLoadBasedAutoScalingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoadBasedAutoScalingError {
    fn from(err: HttpDispatchError) -> DescribeLoadBasedAutoScalingError {
        DescribeLoadBasedAutoScalingError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeLoadBasedAutoScalingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoadBasedAutoScalingError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoadBasedAutoScalingError::ResourceNotFound(ref cause) => cause,
            DescribeLoadBasedAutoScalingError::Validation(ref cause) => cause,
            DescribeLoadBasedAutoScalingError::Credentials(ref err) => err.description(),
            DescribeLoadBasedAutoScalingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoadBasedAutoScalingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMyUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeMyUserProfileError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeMyUserProfileError {
    pub fn from_body(body: &str) -> DescribeMyUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeMyUserProfileError::Validation(error_message.to_string())
                    }
                    _ => DescribeMyUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMyUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMyUserProfileError {
    fn from(err: serde_json::error::Error) -> DescribeMyUserProfileError {
        DescribeMyUserProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMyUserProfileError {
    fn from(err: CredentialsError) -> DescribeMyUserProfileError {
        DescribeMyUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMyUserProfileError {
    fn from(err: HttpDispatchError) -> DescribeMyUserProfileError {
        DescribeMyUserProfileError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeMyUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMyUserProfileError {
    fn description(&self) -> &str {
        match *self {
            DescribeMyUserProfileError::Validation(ref cause) => cause,
            DescribeMyUserProfileError::Credentials(ref err) => err.description(),
            DescribeMyUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMyUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePermissions
#[derive(Debug, PartialEq)]
pub enum DescribePermissionsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribePermissionsError {
    pub fn from_body(body: &str) -> DescribePermissionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribePermissionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePermissionsError::Validation(error_message.to_string())
                    }
                    _ => DescribePermissionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePermissionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePermissionsError {
    fn from(err: serde_json::error::Error) -> DescribePermissionsError {
        DescribePermissionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePermissionsError {
    fn from(err: CredentialsError) -> DescribePermissionsError {
        DescribePermissionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePermissionsError {
    fn from(err: HttpDispatchError) -> DescribePermissionsError {
        DescribePermissionsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribePermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePermissionsError {
    fn description(&self) -> &str {
        match *self {
            DescribePermissionsError::ResourceNotFound(ref cause) => cause,
            DescribePermissionsError::Validation(ref cause) => cause,
            DescribePermissionsError::Credentials(ref err) => err.description(),
            DescribePermissionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePermissionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRaidArrays
#[derive(Debug, PartialEq)]
pub enum DescribeRaidArraysError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeRaidArraysError {
    pub fn from_body(body: &str) -> DescribeRaidArraysError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeRaidArraysError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRaidArraysError::Validation(error_message.to_string())
                    }
                    _ => DescribeRaidArraysError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRaidArraysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRaidArraysError {
    fn from(err: serde_json::error::Error) -> DescribeRaidArraysError {
        DescribeRaidArraysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRaidArraysError {
    fn from(err: CredentialsError) -> DescribeRaidArraysError {
        DescribeRaidArraysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRaidArraysError {
    fn from(err: HttpDispatchError) -> DescribeRaidArraysError {
        DescribeRaidArraysError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeRaidArraysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRaidArraysError {
    fn description(&self) -> &str {
        match *self {
            DescribeRaidArraysError::ResourceNotFound(ref cause) => cause,
            DescribeRaidArraysError::Validation(ref cause) => cause,
            DescribeRaidArraysError::Credentials(ref err) => err.description(),
            DescribeRaidArraysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRaidArraysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRdsDbInstances
#[derive(Debug, PartialEq)]
pub enum DescribeRdsDbInstancesError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeRdsDbInstancesError {
    pub fn from_body(body: &str) -> DescribeRdsDbInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeRdsDbInstancesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRdsDbInstancesError::Validation(error_message.to_string())
                    }
                    _ => DescribeRdsDbInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRdsDbInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRdsDbInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeRdsDbInstancesError {
        DescribeRdsDbInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRdsDbInstancesError {
    fn from(err: CredentialsError) -> DescribeRdsDbInstancesError {
        DescribeRdsDbInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRdsDbInstancesError {
    fn from(err: HttpDispatchError) -> DescribeRdsDbInstancesError {
        DescribeRdsDbInstancesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeRdsDbInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRdsDbInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeRdsDbInstancesError::ResourceNotFound(ref cause) => cause,
            DescribeRdsDbInstancesError::Validation(ref cause) => cause,
            DescribeRdsDbInstancesError::Credentials(ref err) => err.description(),
            DescribeRdsDbInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRdsDbInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServiceErrors
#[derive(Debug, PartialEq)]
pub enum DescribeServiceErrorsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeServiceErrorsError {
    pub fn from_body(body: &str) -> DescribeServiceErrorsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeServiceErrorsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeServiceErrorsError::Validation(error_message.to_string())
                    }
                    _ => DescribeServiceErrorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeServiceErrorsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeServiceErrorsError {
    fn from(err: serde_json::error::Error) -> DescribeServiceErrorsError {
        DescribeServiceErrorsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeServiceErrorsError {
    fn from(err: CredentialsError) -> DescribeServiceErrorsError {
        DescribeServiceErrorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeServiceErrorsError {
    fn from(err: HttpDispatchError) -> DescribeServiceErrorsError {
        DescribeServiceErrorsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeServiceErrorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServiceErrorsError {
    fn description(&self) -> &str {
        match *self {
            DescribeServiceErrorsError::ResourceNotFound(ref cause) => cause,
            DescribeServiceErrorsError::Validation(ref cause) => cause,
            DescribeServiceErrorsError::Credentials(ref err) => err.description(),
            DescribeServiceErrorsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeServiceErrorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackProvisioningParameters
#[derive(Debug, PartialEq)]
pub enum DescribeStackProvisioningParametersError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStackProvisioningParametersError {
    pub fn from_body(body: &str) -> DescribeStackProvisioningParametersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeStackProvisioningParametersError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeStackProvisioningParametersError::Validation(error_message
                                                                                 .to_string())
                    }
                    _ => DescribeStackProvisioningParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStackProvisioningParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStackProvisioningParametersError {
    fn from(err: serde_json::error::Error) -> DescribeStackProvisioningParametersError {
        DescribeStackProvisioningParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStackProvisioningParametersError {
    fn from(err: CredentialsError) -> DescribeStackProvisioningParametersError {
        DescribeStackProvisioningParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackProvisioningParametersError {
    fn from(err: HttpDispatchError) -> DescribeStackProvisioningParametersError {
        DescribeStackProvisioningParametersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeStackProvisioningParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackProvisioningParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackProvisioningParametersError::ResourceNotFound(ref cause) => cause,
            DescribeStackProvisioningParametersError::Validation(ref cause) => cause,
            DescribeStackProvisioningParametersError::Credentials(ref err) => err.description(),
            DescribeStackProvisioningParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackProvisioningParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackSummary
#[derive(Debug, PartialEq)]
pub enum DescribeStackSummaryError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStackSummaryError {
    pub fn from_body(body: &str) -> DescribeStackSummaryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeStackSummaryError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStackSummaryError::Validation(error_message.to_string())
                    }
                    _ => DescribeStackSummaryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStackSummaryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStackSummaryError {
    fn from(err: serde_json::error::Error) -> DescribeStackSummaryError {
        DescribeStackSummaryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStackSummaryError {
    fn from(err: CredentialsError) -> DescribeStackSummaryError {
        DescribeStackSummaryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackSummaryError {
    fn from(err: HttpDispatchError) -> DescribeStackSummaryError {
        DescribeStackSummaryError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeStackSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackSummaryError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackSummaryError::ResourceNotFound(ref cause) => cause,
            DescribeStackSummaryError::Validation(ref cause) => cause,
            DescribeStackSummaryError::Credentials(ref err) => err.description(),
            DescribeStackSummaryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackSummaryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStacksError {
    pub fn from_body(body: &str) -> DescribeStacksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeStacksError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStacksError::Validation(error_message.to_string())
                    }
                    _ => DescribeStacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStacksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStacksError {
    fn from(err: serde_json::error::Error) -> DescribeStacksError {
        DescribeStacksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStacksError {
    fn from(err: CredentialsError) -> DescribeStacksError {
        DescribeStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStacksError {
    fn from(err: HttpDispatchError) -> DescribeStacksError {
        DescribeStacksError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStacksError {
    fn description(&self) -> &str {
        match *self {
            DescribeStacksError::ResourceNotFound(ref cause) => cause,
            DescribeStacksError::Validation(ref cause) => cause,
            DescribeStacksError::Credentials(ref err) => err.description(),
            DescribeStacksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTimeBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum DescribeTimeBasedAutoScalingError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeTimeBasedAutoScalingError {
    pub fn from_body(body: &str) -> DescribeTimeBasedAutoScalingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DescribeTimeBasedAutoScalingError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeTimeBasedAutoScalingError::Validation(error_message.to_string())
                    }
                    _ => DescribeTimeBasedAutoScalingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTimeBasedAutoScalingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTimeBasedAutoScalingError {
    fn from(err: serde_json::error::Error) -> DescribeTimeBasedAutoScalingError {
        DescribeTimeBasedAutoScalingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTimeBasedAutoScalingError {
    fn from(err: CredentialsError) -> DescribeTimeBasedAutoScalingError {
        DescribeTimeBasedAutoScalingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTimeBasedAutoScalingError {
    fn from(err: HttpDispatchError) -> DescribeTimeBasedAutoScalingError {
        DescribeTimeBasedAutoScalingError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeTimeBasedAutoScalingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTimeBasedAutoScalingError {
    fn description(&self) -> &str {
        match *self {
            DescribeTimeBasedAutoScalingError::ResourceNotFound(ref cause) => cause,
            DescribeTimeBasedAutoScalingError::Validation(ref cause) => cause,
            DescribeTimeBasedAutoScalingError::Credentials(ref err) => err.description(),
            DescribeTimeBasedAutoScalingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTimeBasedAutoScalingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserProfiles
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfilesError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserProfilesError {
    pub fn from_body(body: &str) -> DescribeUserProfilesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeUserProfilesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserProfilesError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserProfilesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserProfilesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserProfilesError {
    fn from(err: serde_json::error::Error) -> DescribeUserProfilesError {
        DescribeUserProfilesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserProfilesError {
    fn from(err: CredentialsError) -> DescribeUserProfilesError {
        DescribeUserProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserProfilesError {
    fn from(err: HttpDispatchError) -> DescribeUserProfilesError {
        DescribeUserProfilesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeUserProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserProfilesError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserProfilesError::ResourceNotFound(ref cause) => cause,
            DescribeUserProfilesError::Validation(ref cause) => cause,
            DescribeUserProfilesError::Credentials(ref err) => err.description(),
            DescribeUserProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserProfilesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeVolumesError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeVolumesError {
    pub fn from_body(body: &str) -> DescribeVolumesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeVolumesError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeVolumesError::Validation(error_message.to_string())
                    }
                    _ => DescribeVolumesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeVolumesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeVolumesError {
    fn from(err: serde_json::error::Error) -> DescribeVolumesError {
        DescribeVolumesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVolumesError {
    fn from(err: CredentialsError) -> DescribeVolumesError {
        DescribeVolumesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVolumesError {
    fn from(err: HttpDispatchError) -> DescribeVolumesError {
        DescribeVolumesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeVolumesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVolumesError {
    fn description(&self) -> &str {
        match *self {
            DescribeVolumesError::ResourceNotFound(ref cause) => cause,
            DescribeVolumesError::Validation(ref cause) => cause,
            DescribeVolumesError::Credentials(ref err) => err.description(),
            DescribeVolumesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeVolumesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachElasticLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DetachElasticLoadBalancerError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachElasticLoadBalancerError {
    pub fn from_body(body: &str) -> DetachElasticLoadBalancerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => DetachElasticLoadBalancerError::ResourceNotFound(String::from(error_message)),
                    "ValidationException" => {
                        DetachElasticLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => DetachElasticLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachElasticLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachElasticLoadBalancerError {
    fn from(err: serde_json::error::Error) -> DetachElasticLoadBalancerError {
        DetachElasticLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachElasticLoadBalancerError {
    fn from(err: CredentialsError) -> DetachElasticLoadBalancerError {
        DetachElasticLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachElasticLoadBalancerError {
    fn from(err: HttpDispatchError) -> DetachElasticLoadBalancerError {
        DetachElasticLoadBalancerError::HttpDispatch(err)
    }
}
impl fmt::Display for DetachElasticLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachElasticLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DetachElasticLoadBalancerError::ResourceNotFound(ref cause) => cause,
            DetachElasticLoadBalancerError::Validation(ref cause) => cause,
            DetachElasticLoadBalancerError::Credentials(ref err) => err.description(),
            DetachElasticLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachElasticLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateElasticIp
#[derive(Debug, PartialEq)]
pub enum DisassociateElasticIpError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateElasticIpError {
    pub fn from_body(body: &str) -> DisassociateElasticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DisassociateElasticIpError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateElasticIpError::Validation(error_message.to_string())
                    }
                    _ => DisassociateElasticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateElasticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateElasticIpError {
    fn from(err: serde_json::error::Error) -> DisassociateElasticIpError {
        DisassociateElasticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateElasticIpError {
    fn from(err: CredentialsError) -> DisassociateElasticIpError {
        DisassociateElasticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateElasticIpError {
    fn from(err: HttpDispatchError) -> DisassociateElasticIpError {
        DisassociateElasticIpError::HttpDispatch(err)
    }
}
impl fmt::Display for DisassociateElasticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateElasticIpError {
    fn description(&self) -> &str {
        match *self {
            DisassociateElasticIpError::ResourceNotFound(ref cause) => cause,
            DisassociateElasticIpError::Validation(ref cause) => cause,
            DisassociateElasticIpError::Credentials(ref err) => err.description(),
            DisassociateElasticIpError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateElasticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetHostnameSuggestion
#[derive(Debug, PartialEq)]
pub enum GetHostnameSuggestionError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetHostnameSuggestionError {
    pub fn from_body(body: &str) -> GetHostnameSuggestionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        GetHostnameSuggestionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetHostnameSuggestionError::Validation(error_message.to_string())
                    }
                    _ => GetHostnameSuggestionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetHostnameSuggestionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetHostnameSuggestionError {
    fn from(err: serde_json::error::Error) -> GetHostnameSuggestionError {
        GetHostnameSuggestionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetHostnameSuggestionError {
    fn from(err: CredentialsError) -> GetHostnameSuggestionError {
        GetHostnameSuggestionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHostnameSuggestionError {
    fn from(err: HttpDispatchError) -> GetHostnameSuggestionError {
        GetHostnameSuggestionError::HttpDispatch(err)
    }
}
impl fmt::Display for GetHostnameSuggestionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHostnameSuggestionError {
    fn description(&self) -> &str {
        match *self {
            GetHostnameSuggestionError::ResourceNotFound(ref cause) => cause,
            GetHostnameSuggestionError::Validation(ref cause) => cause,
            GetHostnameSuggestionError::Credentials(ref err) => err.description(),
            GetHostnameSuggestionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHostnameSuggestionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GrantAccess
#[derive(Debug, PartialEq)]
pub enum GrantAccessError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GrantAccessError {
    pub fn from_body(body: &str) -> GrantAccessError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        GrantAccessError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GrantAccessError::Validation(error_message.to_string())
                    }
                    _ => GrantAccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => GrantAccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GrantAccessError {
    fn from(err: serde_json::error::Error) -> GrantAccessError {
        GrantAccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GrantAccessError {
    fn from(err: CredentialsError) -> GrantAccessError {
        GrantAccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GrantAccessError {
    fn from(err: HttpDispatchError) -> GrantAccessError {
        GrantAccessError::HttpDispatch(err)
    }
}
impl fmt::Display for GrantAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GrantAccessError {
    fn description(&self) -> &str {
        match *self {
            GrantAccessError::ResourceNotFound(ref cause) => cause,
            GrantAccessError::Validation(ref cause) => cause,
            GrantAccessError::Credentials(ref err) => err.description(),
            GrantAccessError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GrantAccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTagsError {
    pub fn from_body(body: &str) -> ListTagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        ListTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => ListTagsError::Validation(error_message.to_string()),
                    _ => ListTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::ResourceNotFound(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootInstance
#[derive(Debug, PartialEq)]
pub enum RebootInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RebootInstanceError {
    pub fn from_body(body: &str) -> RebootInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RebootInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RebootInstanceError::Validation(error_message.to_string())
                    }
                    _ => RebootInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebootInstanceError {
    fn from(err: serde_json::error::Error) -> RebootInstanceError {
        RebootInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebootInstanceError {
    fn from(err: CredentialsError) -> RebootInstanceError {
        RebootInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootInstanceError {
    fn from(err: HttpDispatchError) -> RebootInstanceError {
        RebootInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for RebootInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootInstanceError {
    fn description(&self) -> &str {
        match *self {
            RebootInstanceError::ResourceNotFound(ref cause) => cause,
            RebootInstanceError::Validation(ref cause) => cause,
            RebootInstanceError::Credentials(ref err) => err.description(),
            RebootInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterEcsCluster
#[derive(Debug, PartialEq)]
pub enum RegisterEcsClusterError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterEcsClusterError {
    pub fn from_body(body: &str) -> RegisterEcsClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RegisterEcsClusterError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterEcsClusterError::Validation(error_message.to_string())
                    }
                    _ => RegisterEcsClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterEcsClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterEcsClusterError {
    fn from(err: serde_json::error::Error) -> RegisterEcsClusterError {
        RegisterEcsClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterEcsClusterError {
    fn from(err: CredentialsError) -> RegisterEcsClusterError {
        RegisterEcsClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterEcsClusterError {
    fn from(err: HttpDispatchError) -> RegisterEcsClusterError {
        RegisterEcsClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterEcsClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterEcsClusterError {
    fn description(&self) -> &str {
        match *self {
            RegisterEcsClusterError::ResourceNotFound(ref cause) => cause,
            RegisterEcsClusterError::Validation(ref cause) => cause,
            RegisterEcsClusterError::Credentials(ref err) => err.description(),
            RegisterEcsClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterEcsClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterElasticIp
#[derive(Debug, PartialEq)]
pub enum RegisterElasticIpError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterElasticIpError {
    pub fn from_body(body: &str) -> RegisterElasticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RegisterElasticIpError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterElasticIpError::Validation(error_message.to_string())
                    }
                    _ => RegisterElasticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterElasticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterElasticIpError {
    fn from(err: serde_json::error::Error) -> RegisterElasticIpError {
        RegisterElasticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterElasticIpError {
    fn from(err: CredentialsError) -> RegisterElasticIpError {
        RegisterElasticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterElasticIpError {
    fn from(err: HttpDispatchError) -> RegisterElasticIpError {
        RegisterElasticIpError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterElasticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterElasticIpError {
    fn description(&self) -> &str {
        match *self {
            RegisterElasticIpError::ResourceNotFound(ref cause) => cause,
            RegisterElasticIpError::Validation(ref cause) => cause,
            RegisterElasticIpError::Credentials(ref err) => err.description(),
            RegisterElasticIpError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterElasticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterInstance
#[derive(Debug, PartialEq)]
pub enum RegisterInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterInstanceError {
    pub fn from_body(body: &str) -> RegisterInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RegisterInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterInstanceError::Validation(error_message.to_string())
                    }
                    _ => RegisterInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterInstanceError {
    fn from(err: serde_json::error::Error) -> RegisterInstanceError {
        RegisterInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterInstanceError {
    fn from(err: CredentialsError) -> RegisterInstanceError {
        RegisterInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterInstanceError {
    fn from(err: HttpDispatchError) -> RegisterInstanceError {
        RegisterInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterInstanceError::ResourceNotFound(ref cause) => cause,
            RegisterInstanceError::Validation(ref cause) => cause,
            RegisterInstanceError::Credentials(ref err) => err.description(),
            RegisterInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum RegisterRdsDbInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterRdsDbInstanceError {
    pub fn from_body(body: &str) -> RegisterRdsDbInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RegisterRdsDbInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterRdsDbInstanceError::Validation(error_message.to_string())
                    }
                    _ => RegisterRdsDbInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterRdsDbInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterRdsDbInstanceError {
    fn from(err: serde_json::error::Error) -> RegisterRdsDbInstanceError {
        RegisterRdsDbInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterRdsDbInstanceError {
    fn from(err: CredentialsError) -> RegisterRdsDbInstanceError {
        RegisterRdsDbInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterRdsDbInstanceError {
    fn from(err: HttpDispatchError) -> RegisterRdsDbInstanceError {
        RegisterRdsDbInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterRdsDbInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterRdsDbInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterRdsDbInstanceError::ResourceNotFound(ref cause) => cause,
            RegisterRdsDbInstanceError::Validation(ref cause) => cause,
            RegisterRdsDbInstanceError::Credentials(ref err) => err.description(),
            RegisterRdsDbInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterRdsDbInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterVolume
#[derive(Debug, PartialEq)]
pub enum RegisterVolumeError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterVolumeError {
    pub fn from_body(body: &str) -> RegisterVolumeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        RegisterVolumeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterVolumeError::Validation(error_message.to_string())
                    }
                    _ => RegisterVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterVolumeError {
    fn from(err: serde_json::error::Error) -> RegisterVolumeError {
        RegisterVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterVolumeError {
    fn from(err: CredentialsError) -> RegisterVolumeError {
        RegisterVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterVolumeError {
    fn from(err: HttpDispatchError) -> RegisterVolumeError {
        RegisterVolumeError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterVolumeError {
    fn description(&self) -> &str {
        match *self {
            RegisterVolumeError::ResourceNotFound(ref cause) => cause,
            RegisterVolumeError::Validation(ref cause) => cause,
            RegisterVolumeError::Credentials(ref err) => err.description(),
            RegisterVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoadBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum SetLoadBasedAutoScalingError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetLoadBasedAutoScalingError {
    pub fn from_body(body: &str) -> SetLoadBasedAutoScalingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        SetLoadBasedAutoScalingError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetLoadBasedAutoScalingError::Validation(error_message.to_string())
                    }
                    _ => SetLoadBasedAutoScalingError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetLoadBasedAutoScalingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetLoadBasedAutoScalingError {
    fn from(err: serde_json::error::Error) -> SetLoadBasedAutoScalingError {
        SetLoadBasedAutoScalingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetLoadBasedAutoScalingError {
    fn from(err: CredentialsError) -> SetLoadBasedAutoScalingError {
        SetLoadBasedAutoScalingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoadBasedAutoScalingError {
    fn from(err: HttpDispatchError) -> SetLoadBasedAutoScalingError {
        SetLoadBasedAutoScalingError::HttpDispatch(err)
    }
}
impl fmt::Display for SetLoadBasedAutoScalingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoadBasedAutoScalingError {
    fn description(&self) -> &str {
        match *self {
            SetLoadBasedAutoScalingError::ResourceNotFound(ref cause) => cause,
            SetLoadBasedAutoScalingError::Validation(ref cause) => cause,
            SetLoadBasedAutoScalingError::Credentials(ref err) => err.description(),
            SetLoadBasedAutoScalingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoadBasedAutoScalingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetPermission
#[derive(Debug, PartialEq)]
pub enum SetPermissionError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetPermissionError {
    pub fn from_body(body: &str) -> SetPermissionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        SetPermissionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetPermissionError::Validation(error_message.to_string())
                    }
                    _ => SetPermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetPermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetPermissionError {
    fn from(err: serde_json::error::Error) -> SetPermissionError {
        SetPermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetPermissionError {
    fn from(err: CredentialsError) -> SetPermissionError {
        SetPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetPermissionError {
    fn from(err: HttpDispatchError) -> SetPermissionError {
        SetPermissionError::HttpDispatch(err)
    }
}
impl fmt::Display for SetPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetPermissionError {
    fn description(&self) -> &str {
        match *self {
            SetPermissionError::ResourceNotFound(ref cause) => cause,
            SetPermissionError::Validation(ref cause) => cause,
            SetPermissionError::Credentials(ref err) => err.description(),
            SetPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTimeBasedAutoScaling
#[derive(Debug, PartialEq)]
pub enum SetTimeBasedAutoScalingError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetTimeBasedAutoScalingError {
    pub fn from_body(body: &str) -> SetTimeBasedAutoScalingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        SetTimeBasedAutoScalingError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetTimeBasedAutoScalingError::Validation(error_message.to_string())
                    }
                    _ => SetTimeBasedAutoScalingError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetTimeBasedAutoScalingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetTimeBasedAutoScalingError {
    fn from(err: serde_json::error::Error) -> SetTimeBasedAutoScalingError {
        SetTimeBasedAutoScalingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetTimeBasedAutoScalingError {
    fn from(err: CredentialsError) -> SetTimeBasedAutoScalingError {
        SetTimeBasedAutoScalingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetTimeBasedAutoScalingError {
    fn from(err: HttpDispatchError) -> SetTimeBasedAutoScalingError {
        SetTimeBasedAutoScalingError::HttpDispatch(err)
    }
}
impl fmt::Display for SetTimeBasedAutoScalingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTimeBasedAutoScalingError {
    fn description(&self) -> &str {
        match *self {
            SetTimeBasedAutoScalingError::ResourceNotFound(ref cause) => cause,
            SetTimeBasedAutoScalingError::Validation(ref cause) => cause,
            SetTimeBasedAutoScalingError::Credentials(ref err) => err.description(),
            SetTimeBasedAutoScalingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetTimeBasedAutoScalingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartInstance
#[derive(Debug, PartialEq)]
pub enum StartInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartInstanceError {
    pub fn from_body(body: &str) -> StartInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        StartInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartInstanceError::Validation(error_message.to_string())
                    }
                    _ => StartInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartInstanceError {
    fn from(err: serde_json::error::Error) -> StartInstanceError {
        StartInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartInstanceError {
    fn from(err: CredentialsError) -> StartInstanceError {
        StartInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartInstanceError {
    fn from(err: HttpDispatchError) -> StartInstanceError {
        StartInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for StartInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartInstanceError {
    fn description(&self) -> &str {
        match *self {
            StartInstanceError::ResourceNotFound(ref cause) => cause,
            StartInstanceError::Validation(ref cause) => cause,
            StartInstanceError::Credentials(ref err) => err.description(),
            StartInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartStack
#[derive(Debug, PartialEq)]
pub enum StartStackError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartStackError {
    pub fn from_body(body: &str) -> StartStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        StartStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StartStackError::Validation(error_message.to_string()),
                    _ => StartStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartStackError {
    fn from(err: serde_json::error::Error) -> StartStackError {
        StartStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartStackError {
    fn from(err: CredentialsError) -> StartStackError {
        StartStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartStackError {
    fn from(err: HttpDispatchError) -> StartStackError {
        StartStackError::HttpDispatch(err)
    }
}
impl fmt::Display for StartStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartStackError {
    fn description(&self) -> &str {
        match *self {
            StartStackError::ResourceNotFound(ref cause) => cause,
            StartStackError::Validation(ref cause) => cause,
            StartStackError::Credentials(ref err) => err.description(),
            StartStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopInstance
#[derive(Debug, PartialEq)]
pub enum StopInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopInstanceError {
    pub fn from_body(body: &str) -> StopInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        StopInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopInstanceError::Validation(error_message.to_string())
                    }
                    _ => StopInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopInstanceError {
    fn from(err: serde_json::error::Error) -> StopInstanceError {
        StopInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopInstanceError {
    fn from(err: CredentialsError) -> StopInstanceError {
        StopInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopInstanceError {
    fn from(err: HttpDispatchError) -> StopInstanceError {
        StopInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for StopInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopInstanceError {
    fn description(&self) -> &str {
        match *self {
            StopInstanceError::ResourceNotFound(ref cause) => cause,
            StopInstanceError::Validation(ref cause) => cause,
            StopInstanceError::Credentials(ref err) => err.description(),
            StopInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopStack
#[derive(Debug, PartialEq)]
pub enum StopStackError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopStackError {
    pub fn from_body(body: &str) -> StopStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        StopStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StopStackError::Validation(error_message.to_string()),
                    _ => StopStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopStackError {
    fn from(err: serde_json::error::Error) -> StopStackError {
        StopStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopStackError {
    fn from(err: CredentialsError) -> StopStackError {
        StopStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopStackError {
    fn from(err: HttpDispatchError) -> StopStackError {
        StopStackError::HttpDispatch(err)
    }
}
impl fmt::Display for StopStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopStackError {
    fn description(&self) -> &str {
        match *self {
            StopStackError::ResourceNotFound(ref cause) => cause,
            StopStackError::Validation(ref cause) => cause,
            StopStackError::Credentials(ref err) => err.description(),
            StopStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UnassignInstance
#[derive(Debug, PartialEq)]
pub enum UnassignInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UnassignInstanceError {
    pub fn from_body(body: &str) -> UnassignInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UnassignInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UnassignInstanceError::Validation(error_message.to_string())
                    }
                    _ => UnassignInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UnassignInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UnassignInstanceError {
    fn from(err: serde_json::error::Error) -> UnassignInstanceError {
        UnassignInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UnassignInstanceError {
    fn from(err: CredentialsError) -> UnassignInstanceError {
        UnassignInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnassignInstanceError {
    fn from(err: HttpDispatchError) -> UnassignInstanceError {
        UnassignInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for UnassignInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnassignInstanceError {
    fn description(&self) -> &str {
        match *self {
            UnassignInstanceError::ResourceNotFound(ref cause) => cause,
            UnassignInstanceError::Validation(ref cause) => cause,
            UnassignInstanceError::Credentials(ref err) => err.description(),
            UnassignInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UnassignInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UnassignVolume
#[derive(Debug, PartialEq)]
pub enum UnassignVolumeError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UnassignVolumeError {
    pub fn from_body(body: &str) -> UnassignVolumeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UnassignVolumeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UnassignVolumeError::Validation(error_message.to_string())
                    }
                    _ => UnassignVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UnassignVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UnassignVolumeError {
    fn from(err: serde_json::error::Error) -> UnassignVolumeError {
        UnassignVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UnassignVolumeError {
    fn from(err: CredentialsError) -> UnassignVolumeError {
        UnassignVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnassignVolumeError {
    fn from(err: HttpDispatchError) -> UnassignVolumeError {
        UnassignVolumeError::HttpDispatch(err)
    }
}
impl fmt::Display for UnassignVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnassignVolumeError {
    fn description(&self) -> &str {
        match *self {
            UnassignVolumeError::ResourceNotFound(ref cause) => cause,
            UnassignVolumeError::Validation(ref cause) => cause,
            UnassignVolumeError::Credentials(ref err) => err.description(),
            UnassignVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UnassignVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateAppError {
    pub fn from_body(body: &str) -> UpdateAppError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateAppError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => UpdateAppError::Validation(error_message.to_string()),
                    _ => UpdateAppError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAppError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAppError {
    fn from(err: serde_json::error::Error) -> UpdateAppError {
        UpdateAppError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAppError {
    fn from(err: CredentialsError) -> UpdateAppError {
        UpdateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAppError {
    fn from(err: HttpDispatchError) -> UpdateAppError {
        UpdateAppError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAppError {
    fn description(&self) -> &str {
        match *self {
            UpdateAppError::ResourceNotFound(ref cause) => cause,
            UpdateAppError::Validation(ref cause) => cause,
            UpdateAppError::Credentials(ref err) => err.description(),
            UpdateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAppError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateElasticIp
#[derive(Debug, PartialEq)]
pub enum UpdateElasticIpError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateElasticIpError {
    pub fn from_body(body: &str) -> UpdateElasticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateElasticIpError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateElasticIpError::Validation(error_message.to_string())
                    }
                    _ => UpdateElasticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateElasticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateElasticIpError {
    fn from(err: serde_json::error::Error) -> UpdateElasticIpError {
        UpdateElasticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateElasticIpError {
    fn from(err: CredentialsError) -> UpdateElasticIpError {
        UpdateElasticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateElasticIpError {
    fn from(err: HttpDispatchError) -> UpdateElasticIpError {
        UpdateElasticIpError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateElasticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateElasticIpError {
    fn description(&self) -> &str {
        match *self {
            UpdateElasticIpError::ResourceNotFound(ref cause) => cause,
            UpdateElasticIpError::Validation(ref cause) => cause,
            UpdateElasticIpError::Credentials(ref err) => err.description(),
            UpdateElasticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateElasticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateInstance
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateInstanceError {
    pub fn from_body(body: &str) -> UpdateInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateInstanceError::Validation(error_message.to_string())
                    }
                    _ => UpdateInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateInstanceError {
    fn from(err: serde_json::error::Error) -> UpdateInstanceError {
        UpdateInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateInstanceError {
    fn from(err: CredentialsError) -> UpdateInstanceError {
        UpdateInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateInstanceError {
    fn from(err: HttpDispatchError) -> UpdateInstanceError {
        UpdateInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateInstanceError {
    fn description(&self) -> &str {
        match *self {
            UpdateInstanceError::ResourceNotFound(ref cause) => cause,
            UpdateInstanceError::Validation(ref cause) => cause,
            UpdateInstanceError::Credentials(ref err) => err.description(),
            UpdateInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLayer
#[derive(Debug, PartialEq)]
pub enum UpdateLayerError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateLayerError {
    pub fn from_body(body: &str) -> UpdateLayerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateLayerError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateLayerError::Validation(error_message.to_string())
                    }
                    _ => UpdateLayerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateLayerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateLayerError {
    fn from(err: serde_json::error::Error) -> UpdateLayerError {
        UpdateLayerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLayerError {
    fn from(err: CredentialsError) -> UpdateLayerError {
        UpdateLayerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLayerError {
    fn from(err: HttpDispatchError) -> UpdateLayerError {
        UpdateLayerError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateLayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLayerError {
    fn description(&self) -> &str {
        match *self {
            UpdateLayerError::ResourceNotFound(ref cause) => cause,
            UpdateLayerError::Validation(ref cause) => cause,
            UpdateLayerError::Credentials(ref err) => err.description(),
            UpdateLayerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateLayerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMyUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateMyUserProfileError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateMyUserProfileError {
    pub fn from_body(body: &str) -> UpdateMyUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        UpdateMyUserProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateMyUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMyUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMyUserProfileError {
    fn from(err: serde_json::error::Error) -> UpdateMyUserProfileError {
        UpdateMyUserProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMyUserProfileError {
    fn from(err: CredentialsError) -> UpdateMyUserProfileError {
        UpdateMyUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMyUserProfileError {
    fn from(err: HttpDispatchError) -> UpdateMyUserProfileError {
        UpdateMyUserProfileError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateMyUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMyUserProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateMyUserProfileError::Validation(ref cause) => cause,
            UpdateMyUserProfileError::Credentials(ref err) => err.description(),
            UpdateMyUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMyUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRdsDbInstance
#[derive(Debug, PartialEq)]
pub enum UpdateRdsDbInstanceError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRdsDbInstanceError {
    pub fn from_body(body: &str) -> UpdateRdsDbInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateRdsDbInstanceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRdsDbInstanceError::Validation(error_message.to_string())
                    }
                    _ => UpdateRdsDbInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRdsDbInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRdsDbInstanceError {
    fn from(err: serde_json::error::Error) -> UpdateRdsDbInstanceError {
        UpdateRdsDbInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRdsDbInstanceError {
    fn from(err: CredentialsError) -> UpdateRdsDbInstanceError {
        UpdateRdsDbInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRdsDbInstanceError {
    fn from(err: HttpDispatchError) -> UpdateRdsDbInstanceError {
        UpdateRdsDbInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateRdsDbInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRdsDbInstanceError {
    fn description(&self) -> &str {
        match *self {
            UpdateRdsDbInstanceError::ResourceNotFound(ref cause) => cause,
            UpdateRdsDbInstanceError::Validation(ref cause) => cause,
            UpdateRdsDbInstanceError::Credentials(ref err) => err.description(),
            UpdateRdsDbInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRdsDbInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateStackError {
    pub fn from_body(body: &str) -> UpdateStackError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateStackError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateStackError::Validation(error_message.to_string())
                    }
                    _ => UpdateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateStackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateStackError {
    fn from(err: serde_json::error::Error) -> UpdateStackError {
        UpdateStackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateStackError {
    fn from(err: CredentialsError) -> UpdateStackError {
        UpdateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStackError {
    fn from(err: HttpDispatchError) -> UpdateStackError {
        UpdateStackError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackError::ResourceNotFound(ref cause) => cause,
            UpdateStackError::Validation(ref cause) => cause,
            UpdateStackError::Credentials(ref err) => err.description(),
            UpdateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserProfileError {
    pub fn from_body(body: &str) -> UpdateUserProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateUserProfileError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUserProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserProfileError {
    fn from(err: serde_json::error::Error) -> UpdateUserProfileError {
        UpdateUserProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserProfileError {
    fn from(err: CredentialsError) -> UpdateUserProfileError {
        UpdateUserProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserProfileError {
    fn from(err: HttpDispatchError) -> UpdateUserProfileError {
        UpdateUserProfileError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateUserProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserProfileError::ResourceNotFound(ref cause) => cause,
            UpdateUserProfileError::Validation(ref cause) => cause,
            UpdateUserProfileError::Credentials(ref err) => err.description(),
            UpdateUserProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVolume
#[derive(Debug, PartialEq)]
pub enum UpdateVolumeError {
    ///<p>Indicates that a resource was not found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateVolumeError {
    pub fn from_body(body: &str) -> UpdateVolumeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UpdateVolumeError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateVolumeError::Validation(error_message.to_string())
                    }
                    _ => UpdateVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateVolumeError {
    fn from(err: serde_json::error::Error) -> UpdateVolumeError {
        UpdateVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVolumeError {
    fn from(err: CredentialsError) -> UpdateVolumeError {
        UpdateVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVolumeError {
    fn from(err: HttpDispatchError) -> UpdateVolumeError {
        UpdateVolumeError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVolumeError {
    fn description(&self) -> &str {
        match *self {
            UpdateVolumeError::ResourceNotFound(ref cause) => cause,
            UpdateVolumeError::Validation(ref cause) => cause,
            UpdateVolumeError::Credentials(ref err) => err.description(),
            UpdateVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS OpsWorks API. AWS OpsWorks clients implement this trait.
pub trait OpsWorks {
    #[doc="<p>Assign a registered instance to a layer.</p> <ul> <li> <p>You can assign registered on-premises instances to any layer type.</p> </li> <li> <p>You can assign registered Amazon EC2 instances only to custom layers.</p> </li> <li> <p>You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> </li> </ul> <p> <b>Required Permissions</b>: To use this action, an AWS Identity and Access Management (IAM) user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn assign_instance(&self, input: &AssignInstanceRequest) -> Result<(), AssignInstanceError>;


    #[doc="<p>Assigns one of the stack's registered Amazon EBS volumes to a specified instance. The volume must first be registered with the stack by calling <a>RegisterVolume</a>. After you register the volume, you must call <a>UpdateVolume</a> to specify a mount point before calling <code>AssignVolume</code>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn assign_volume(&self, input: &AssignVolumeRequest) -> Result<(), AssignVolumeError>;


    #[doc="<p>Associates one of the stack's registered Elastic IP addresses with a specified instance. The address must first be registered with the stack by calling <a>RegisterElasticIp</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn associate_elastic_ip(&self,
                            input: &AssociateElasticIpRequest)
                            -> Result<(), AssociateElasticIpError>;


    #[doc="<p>Attaches an Elastic Load Balancing load balancer to a specified layer. AWS OpsWorks Stacks does not support Application Load Balancer. You can only use Classic Load Balancer with AWS OpsWorks Stacks. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/layers-elb.html\">Elastic Load Balancing</a>.</p> <note> <p>You must create the Elastic Load Balancing instance separately, by using the Elastic Load Balancing console, API, or CLI. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/Welcome.html\"> Elastic Load Balancing Developer Guide</a>.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn attach_elastic_load_balancer(&self,
                                    input: &AttachElasticLoadBalancerRequest)
                                    -> Result<(), AttachElasticLoadBalancerError>;


    #[doc="<p>Creates a clone of a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-cloning.html\">Clone a Stack</a>. By default, all parameters are set to the values used by the parent stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn clone_stack(&self, input: &CloneStackRequest) -> Result<CloneStackResult, CloneStackError>;


    #[doc="<p>Creates an app for a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html\">Creating Apps</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_app(&self, input: &CreateAppRequest) -> Result<CreateAppResult, CreateAppError>;


    #[doc="<p>Runs deployment or stack commands. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html\">Deploying Apps</a> and <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html\">Run Stack Commands</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_deployment(&self,
                         input: &CreateDeploymentRequest)
                         -> Result<CreateDeploymentResult, CreateDeploymentError>;


    #[doc="<p>Creates an instance in a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html\">Adding an Instance to a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_instance(&self,
                       input: &CreateInstanceRequest)
                       -> Result<CreateInstanceResult, CreateInstanceError>;


    #[doc="<p>Creates a layer. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-create.html\">How to Create a Layer</a>.</p> <note> <p>You should use <b>CreateLayer</b> for noncustom layer types such as PHP App Server only if the stack does not have an existing layer of that type. A stack can have at most one instance of each noncustom layer; if you attempt to create a second instance, <b>CreateLayer</b> fails. A stack can have an arbitrary number of custom layers, so you can call <b>CreateLayer</b> as many times as you like for that layer type.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_layer(&self,
                    input: &CreateLayerRequest)
                    -> Result<CreateLayerResult, CreateLayerError>;


    #[doc="<p>Creates a new stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-edit.html\">Create a New Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_stack(&self,
                    input: &CreateStackRequest)
                    -> Result<CreateStackResult, CreateStackError>;


    #[doc="<p>Creates a new user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_user_profile(&self,
                           input: &CreateUserProfileRequest)
                           -> Result<CreateUserProfileResult, CreateUserProfileError>;


    #[doc="<p>Deletes a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_app(&self, input: &DeleteAppRequest) -> Result<(), DeleteAppError>;


    #[doc="<p>Deletes a specified instance, which terminates the associated Amazon EC2 instance. You must stop an instance before you can delete it.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-delete.html\">Deleting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_instance(&self, input: &DeleteInstanceRequest) -> Result<(), DeleteInstanceError>;


    #[doc="<p>Deletes a specified layer. You must first stop and then delete all associated instances or unassign registered instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-delete.html\">How to Delete a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_layer(&self, input: &DeleteLayerRequest) -> Result<(), DeleteLayerError>;


    #[doc="<p>Deletes a specified stack. You must first delete all instances, layers, and apps or deregister registered instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-shutting.html\">Shut Down a Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_stack(&self, input: &DeleteStackRequest) -> Result<(), DeleteStackError>;


    #[doc="<p>Deletes a user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_user_profile(&self,
                           input: &DeleteUserProfileRequest)
                           -> Result<(), DeleteUserProfileError>;


    #[doc="<p>Deregisters a specified Amazon ECS cluster from a stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html#workinglayers-ecscluster-delete\"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html</a>.</p>"]
    fn deregister_ecs_cluster(&self,
                              input: &DeregisterEcsClusterRequest)
                              -> Result<(), DeregisterEcsClusterError>;


    #[doc="<p>Deregisters a specified Elastic IP address. The address can then be registered by another stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_elastic_ip(&self,
                             input: &DeregisterElasticIpRequest)
                             -> Result<(), DeregisterElasticIpError>;


    #[doc="<p>Deregister a registered Amazon EC2 or on-premises instance. This action removes the instance from the stack and returns it to your control. This action can not be used with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_instance(&self,
                           input: &DeregisterInstanceRequest)
                           -> Result<(), DeregisterInstanceError>;


    #[doc="<p>Deregisters an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_rds_db_instance(&self,
                                  input: &DeregisterRdsDbInstanceRequest)
                                  -> Result<(), DeregisterRdsDbInstanceError>;


    #[doc="<p>Deregisters an Amazon EBS volume. The volume can then be registered by another stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_volume(&self,
                         input: &DeregisterVolumeRequest)
                         -> Result<(), DeregisterVolumeError>;


    #[doc="<p>Describes the available AWS OpsWorks Stacks agent versions. You must specify a stack ID or a configuration manager. <code>DescribeAgentVersions</code> returns a list of available agent versions for the specified stack or configuration manager.</p>"]
    fn describe_agent_versions
        (&self,
         input: &DescribeAgentVersionsRequest)
         -> Result<DescribeAgentVersionsResult, DescribeAgentVersionsError>;


    #[doc="<p>Requests a description of a specified set of apps.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_apps(&self,
                     input: &DescribeAppsRequest)
                     -> Result<DescribeAppsResult, DescribeAppsError>;


    #[doc="<p>Describes the results of specified commands.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_commands(&self,
                         input: &DescribeCommandsRequest)
                         -> Result<DescribeCommandsResult, DescribeCommandsError>;


    #[doc="<p>Requests a description of a specified set of deployments.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_deployments(&self,
                            input: &DescribeDeploymentsRequest)
                            -> Result<DescribeDeploymentsResult, DescribeDeploymentsError>;


    #[doc="<p>Describes Amazon ECS clusters that are registered with a stack. If you specify only a stack ID, you can use the <code>MaxResults</code> and <code>NextToken</code> parameters to paginate the response. However, AWS OpsWorks Stacks currently supports only one cluster per layer, so the result set has a maximum of one element.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permission. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_ecs_clusters(&self,
                             input: &DescribeEcsClustersRequest)
                             -> Result<DescribeEcsClustersResult, DescribeEcsClustersError>;


    #[doc="<p>Describes <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP addresses</a>.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_elastic_ips(&self,
                            input: &DescribeElasticIpsRequest)
                            -> Result<DescribeElasticIpsResult, DescribeElasticIpsError>;


    #[doc="<p>Describes a stack's Elastic Load Balancing instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_elastic_load_balancers
        (&self,
         input: &DescribeElasticLoadBalancersRequest)
         -> Result<DescribeElasticLoadBalancersResult, DescribeElasticLoadBalancersError>;


    #[doc="<p>Requests a description of a set of instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_instances(&self,
                          input: &DescribeInstancesRequest)
                          -> Result<DescribeInstancesResult, DescribeInstancesError>;


    #[doc="<p>Requests a description of one or more layers in a specified stack.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_layers(&self,
                       input: &DescribeLayersRequest)
                       -> Result<DescribeLayersResult, DescribeLayersError>;


    #[doc="<p>Describes load-based auto scaling configurations for specified layers.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_load_based_auto_scaling
        (&self,
         input: &DescribeLoadBasedAutoScalingRequest)
         -> Result<DescribeLoadBasedAutoScalingResult, DescribeLoadBasedAutoScalingError>;


    #[doc="<p>Describes a user's SSH information.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_my_user_profile
        (&self)
         -> Result<DescribeMyUserProfileResult, DescribeMyUserProfileError>;


    #[doc="<p>Describes the permissions for a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_permissions(&self,
                            input: &DescribePermissionsRequest)
                            -> Result<DescribePermissionsResult, DescribePermissionsError>;


    #[doc="<p>Describe an instance's RAID arrays.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_raid_arrays(&self,
                            input: &DescribeRaidArraysRequest)
                            -> Result<DescribeRaidArraysResult, DescribeRaidArraysError>;


    #[doc="<p>Describes Amazon RDS instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_rds_db_instances
        (&self,
         input: &DescribeRdsDbInstancesRequest)
         -> Result<DescribeRdsDbInstancesResult, DescribeRdsDbInstancesError>;


    #[doc="<p>Describes AWS OpsWorks Stacks service errors.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_service_errors
        (&self,
         input: &DescribeServiceErrorsRequest)
         -> Result<DescribeServiceErrorsResult, DescribeServiceErrorsError>;


    #[doc="<p>Requests a description of a stack's provisioning parameters.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stack_provisioning_parameters
        (&self,
         input: &DescribeStackProvisioningParametersRequest)
         -> Result<DescribeStackProvisioningParametersResult,
                   DescribeStackProvisioningParametersError>;


    #[doc="<p>Describes the number of layers and apps in a specified stack, and the number of instances in each state, such as <code>running_setup</code> or <code>online</code>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stack_summary(&self,
                              input: &DescribeStackSummaryRequest)
                              -> Result<DescribeStackSummaryResult, DescribeStackSummaryError>;


    #[doc="<p>Requests a description of one or more stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksRequest)
                       -> Result<DescribeStacksResult, DescribeStacksError>;


    #[doc="<p>Describes time-based auto scaling configurations for specified instances.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_time_based_auto_scaling
        (&self,
         input: &DescribeTimeBasedAutoScalingRequest)
         -> Result<DescribeTimeBasedAutoScalingResult, DescribeTimeBasedAutoScalingError>;


    #[doc="<p>Describe specified users.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_user_profiles(&self,
                              input: &DescribeUserProfilesRequest)
                              -> Result<DescribeUserProfilesResult, DescribeUserProfilesError>;


    #[doc="<p>Describes an instance's Amazon EBS volumes.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_volumes(&self,
                        input: &DescribeVolumesRequest)
                        -> Result<DescribeVolumesResult, DescribeVolumesError>;


    #[doc="<p>Detaches a specified Elastic Load Balancing instance from its layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn detach_elastic_load_balancer(&self,
                                    input: &DetachElasticLoadBalancerRequest)
                                    -> Result<(), DetachElasticLoadBalancerError>;


    #[doc="<p>Disassociates an Elastic IP address from its instance. The address remains registered with the stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn disassociate_elastic_ip(&self,
                               input: &DisassociateElasticIpRequest)
                               -> Result<(), DisassociateElasticIpError>;


    #[doc="<p>Gets a generated host name for the specified layer, based on the current host name theme.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn get_hostname_suggestion
        (&self,
         input: &GetHostnameSuggestionRequest)
         -> Result<GetHostnameSuggestionResult, GetHostnameSuggestionError>;


    #[doc="<note> <p>This action can be used only with Windows stacks.</p> </note> <p>Grants RDP access to a Windows instance for a specified time period.</p>"]
    fn grant_access(&self,
                    input: &GrantAccessRequest)
                    -> Result<GrantAccessResult, GrantAccessError>;


    #[doc="<p>Returns a list of tags that are applied to the specified stack or layer.</p>"]
    fn list_tags(&self, input: &ListTagsRequest) -> Result<ListTagsResult, ListTagsError>;


    #[doc="<p>Reboots a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn reboot_instance(&self, input: &RebootInstanceRequest) -> Result<(), RebootInstanceError>;


    #[doc="<p>Registers a specified Amazon ECS cluster with a stack. You can register only one cluster with a stack. A cluster can be registered with only one stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html\"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\"> Managing User Permissions</a>.</p>"]
    fn register_ecs_cluster(&self,
                            input: &RegisterEcsClusterRequest)
                            -> Result<RegisterEcsClusterResult, RegisterEcsClusterError>;


    #[doc="<p>Registers an Elastic IP address with a specified stack. An address can be registered with only one stack at a time. If the address is already registered, you must first deregister it by calling <a>DeregisterElasticIp</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_elastic_ip(&self,
                           input: &RegisterElasticIpRequest)
                           -> Result<RegisterElasticIpResult, RegisterElasticIpError>;


    #[doc="<p>Registers instances that were created outside of AWS OpsWorks Stacks with a specified stack.</p> <note> <p>We do not recommend using this action to register instances. The complete registration operation includes two tasks: installing the AWS OpsWorks Stacks agent on the instance, and registering the instance with the stack. <code>RegisterInstance</code> handles only the second step. You should instead use the AWS CLI <code>register</code> command, which performs the entire registration operation. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register.html\"> Registering an Instance with an AWS OpsWorks Stacks Stack</a>.</p> </note> <p>Registered instances have the same requirements as instances that are created by using the <a>CreateInstance</a> API. For example, registered instances must be running a supported Linux-based operating system, and they must have a supported instance type. For more information about requirements for instances that you want to register, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register-registering-preparer.html\"> Preparing the Instance</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_instance(&self,
                         input: &RegisterInstanceRequest)
                         -> Result<RegisterInstanceResult, RegisterInstanceError>;


    #[doc="<p>Registers an Amazon RDS instance with a stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_rds_db_instance(&self,
                                input: &RegisterRdsDbInstanceRequest)
                                -> Result<(), RegisterRdsDbInstanceError>;


    #[doc="<p>Registers an Amazon EBS volume with a specified stack. A volume can be registered with only one stack at a time. If the volume is already registered, you must first deregister it by calling <a>DeregisterVolume</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_volume(&self,
                       input: &RegisterVolumeRequest)
                       -> Result<RegisterVolumeResult, RegisterVolumeError>;


    #[doc="<p>Specify the load-based auto scaling configuration for a specified layer. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html\">Managing Load with Time-based and Load-based Instances</a>.</p> <note> <p>To use load-based auto scaling, you must create a set of load-based auto scaling instances. Load-based auto scaling operates only on the instances from that set, so you must ensure that you have created enough instances to handle the maximum anticipated load.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_load_based_auto_scaling(&self,
                                   input: &SetLoadBasedAutoScalingRequest)
                                   -> Result<(), SetLoadBasedAutoScalingError>;


    #[doc="<p>Specifies a user's permissions. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingsecurity.html\">Security and Permissions</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_permission(&self, input: &SetPermissionRequest) -> Result<(), SetPermissionError>;


    #[doc="<p>Specify the time-based auto scaling configuration for a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html\">Managing Load with Time-based and Load-based Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_time_based_auto_scaling(&self,
                                   input: &SetTimeBasedAutoScalingRequest)
                                   -> Result<(), SetTimeBasedAutoScalingError>;


    #[doc="<p>Starts a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn start_instance(&self, input: &StartInstanceRequest) -> Result<(), StartInstanceError>;


    #[doc="<p>Starts a stack's instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn start_stack(&self, input: &StartStackRequest) -> Result<(), StartStackError>;


    #[doc="<p>Stops a specified instance. When you stop a standard instance, the data disappears and must be reinstalled when you restart the instance. You can stop an Amazon EBS-backed instance without losing data. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn stop_instance(&self, input: &StopInstanceRequest) -> Result<(), StopInstanceError>;


    #[doc="<p>Stops a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn stop_stack(&self, input: &StopStackRequest) -> Result<(), StopStackError>;


    #[doc="<p>Apply cost-allocation tags to a specified stack or layer in AWS OpsWorks Stacks. For more information about how tagging works, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/tagging.html\">Tags</a> in the AWS OpsWorks User Guide.</p>"]
    fn tag_resource(&self, input: &TagResourceRequest) -> Result<(), TagResourceError>;


    #[doc="<p>Unassigns a registered instance from all of it's layers. The instance remains in the stack as an unassigned instance and can be assigned to another layer, as needed. You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn unassign_instance(&self,
                         input: &UnassignInstanceRequest)
                         -> Result<(), UnassignInstanceError>;


    #[doc="<p>Unassigns an assigned Amazon EBS volume. The volume remains registered with the stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn unassign_volume(&self, input: &UnassignVolumeRequest) -> Result<(), UnassignVolumeError>;


    #[doc="<p>Removes tags from a specified stack or layer.</p>"]
    fn untag_resource(&self, input: &UntagResourceRequest) -> Result<(), UntagResourceError>;


    #[doc="<p>Updates a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_app(&self, input: &UpdateAppRequest) -> Result<(), UpdateAppError>;


    #[doc="<p>Updates a registered Elastic IP address's name. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_elastic_ip(&self,
                         input: &UpdateElasticIpRequest)
                         -> Result<(), UpdateElasticIpError>;


    #[doc="<p>Updates a specified instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_instance(&self, input: &UpdateInstanceRequest) -> Result<(), UpdateInstanceError>;


    #[doc="<p>Updates a specified layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_layer(&self, input: &UpdateLayerRequest) -> Result<(), UpdateLayerError>;


    #[doc="<p>Updates a user's SSH public key.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_my_user_profile(&self,
                              input: &UpdateMyUserProfileRequest)
                              -> Result<(), UpdateMyUserProfileError>;


    #[doc="<p>Updates an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_rds_db_instance(&self,
                              input: &UpdateRdsDbInstanceRequest)
                              -> Result<(), UpdateRdsDbInstanceError>;


    #[doc="<p>Updates a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_stack(&self, input: &UpdateStackRequest) -> Result<(), UpdateStackError>;


    #[doc="<p>Updates a specified user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_user_profile(&self,
                           input: &UpdateUserProfileRequest)
                           -> Result<(), UpdateUserProfileError>;


    #[doc="<p>Updates an Amazon EBS volume's name or mount point. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_volume(&self, input: &UpdateVolumeRequest) -> Result<(), UpdateVolumeError>;
}
/// A client for the AWS OpsWorks API.
pub struct OpsWorksClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> OpsWorksClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        OpsWorksClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> OpsWorks for OpsWorksClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Assign a registered instance to a layer.</p> <ul> <li> <p>You can assign registered on-premises instances to any layer type.</p> </li> <li> <p>You can assign registered Amazon EC2 instances only to custom layers.</p> </li> <li> <p>You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> </li> </ul> <p> <b>Required Permissions</b>: To use this action, an AWS Identity and Access Management (IAM) user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn assign_instance(&self, input: &AssignInstanceRequest) -> Result<(), AssignInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssignInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(AssignInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Assigns one of the stack's registered Amazon EBS volumes to a specified instance. The volume must first be registered with the stack by calling <a>RegisterVolume</a>. After you register the volume, you must call <a>UpdateVolume</a> to specify a mount point before calling <code>AssignVolume</code>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn assign_volume(&self, input: &AssignVolumeRequest) -> Result<(), AssignVolumeError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssignVolume");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(AssignVolumeError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Associates one of the stack's registered Elastic IP addresses with a specified instance. The address must first be registered with the stack by calling <a>RegisterElasticIp</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn associate_elastic_ip(&self,
                            input: &AssociateElasticIpRequest)
                            -> Result<(), AssociateElasticIpError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.AssociateElasticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(AssociateElasticIpError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Attaches an Elastic Load Balancing load balancer to a specified layer. AWS OpsWorks Stacks does not support Application Load Balancer. You can only use Classic Load Balancer with AWS OpsWorks Stacks. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/layers-elb.html\">Elastic Load Balancing</a>.</p> <note> <p>You must create the Elastic Load Balancing instance separately, by using the Elastic Load Balancing console, API, or CLI. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/Welcome.html\"> Elastic Load Balancing Developer Guide</a>.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn attach_elastic_load_balancer(&self,
                                    input: &AttachElasticLoadBalancerRequest)
                                    -> Result<(), AttachElasticLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.AttachElasticLoadBalancer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(AttachElasticLoadBalancerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a clone of a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-cloning.html\">Clone a Stack</a>. By default, all parameters are set to the values used by the parent stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn clone_stack(&self, input: &CloneStackRequest) -> Result<CloneStackResult, CloneStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CloneStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CloneStackResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CloneStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an app for a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-creating.html\">Creating Apps</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_app(&self, input: &CreateAppRequest) -> Result<CreateAppResult, CreateAppError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateApp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<CreateAppResult>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(CreateAppError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Runs deployment or stack commands. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingapps-deploying.html\">Deploying Apps</a> and <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-commands.html\">Run Stack Commands</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_deployment(&self,
                         input: &CreateDeploymentRequest)
                         -> Result<CreateDeploymentResult, CreateDeploymentError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateDeployment");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDeploymentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateDeploymentError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an instance in a specified stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-add.html\">Adding an Instance to a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_instance(&self,
                       input: &CreateInstanceRequest)
                       -> Result<CreateInstanceResult, CreateInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateInstanceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a layer. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-create.html\">How to Create a Layer</a>.</p> <note> <p>You should use <b>CreateLayer</b> for noncustom layer types such as PHP App Server only if the stack does not have an existing layer of that type. A stack can have at most one instance of each noncustom layer; if you attempt to create a second instance, <b>CreateLayer</b> fails. A stack can have an arbitrary number of custom layers, so you can call <b>CreateLayer</b> as many times as you like for that layer type.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_layer(&self,
                    input: &CreateLayerRequest)
                    -> Result<CreateLayerResult, CreateLayerError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateLayer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateLayerResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateLayerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-edit.html\">Create a New Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_stack(&self,
                    input: &CreateStackRequest)
                    -> Result<CreateStackResult, CreateStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateStackResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn create_user_profile(&self,
                           input: &CreateUserProfileRequest)
                           -> Result<CreateUserProfileResult, CreateUserProfileError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.CreateUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateUserProfileResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateUserProfileError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_app(&self, input: &DeleteAppRequest) -> Result<(), DeleteAppError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteApp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeleteAppError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a specified instance, which terminates the associated Amazon EC2 instance. You must stop an instance before you can delete it.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-delete.html\">Deleting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_instance(&self, input: &DeleteInstanceRequest) -> Result<(), DeleteInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified layer. You must first stop and then delete all associated instances or unassign registered instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-basics-delete.html\">How to Delete a Layer</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_layer(&self, input: &DeleteLayerRequest) -> Result<(), DeleteLayerError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteLayer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeleteLayerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a specified stack. You must first delete all instances, layers, and apps or deregister registered instances. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingstacks-shutting.html\">Shut Down a Stack</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_stack(&self, input: &DeleteStackRequest) -> Result<(), DeleteStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeleteStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn delete_user_profile(&self,
                           input: &DeleteUserProfileRequest)
                           -> Result<(), DeleteUserProfileError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeleteUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteUserProfileError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters a specified Amazon ECS cluster from a stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html#workinglayers-ecscluster-delete\"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html</a>.</p>"]
    fn deregister_ecs_cluster(&self,
                              input: &DeregisterEcsClusterRequest)
                              -> Result<(), DeregisterEcsClusterError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterEcsCluster");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeregisterEcsClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters a specified Elastic IP address. The address can then be registered by another stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_elastic_ip(&self,
                             input: &DeregisterElasticIpRequest)
                             -> Result<(), DeregisterElasticIpError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterElasticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeregisterElasticIpError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Deregister a registered Amazon EC2 or on-premises instance. This action removes the instance from the stack and returns it to your control. This action can not be used with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_instance(&self,
                           input: &DeregisterInstanceRequest)
                           -> Result<(), DeregisterInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeregisterInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Deregisters an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_rds_db_instance(&self,
                                  input: &DeregisterRdsDbInstanceRequest)
                                  -> Result<(), DeregisterRdsDbInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterRdsDbInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeregisterRdsDbInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deregisters an Amazon EBS volume. The volume can then be registered by another stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn deregister_volume(&self,
                         input: &DeregisterVolumeRequest)
                         -> Result<(), DeregisterVolumeError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DeregisterVolume");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeregisterVolumeError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the available AWS OpsWorks Stacks agent versions. You must specify a stack ID or a configuration manager. <code>DescribeAgentVersions</code> returns a list of available agent versions for the specified stack or configuration manager.</p>"]
    fn describe_agent_versions
        (&self,
         input: &DescribeAgentVersionsRequest)
         -> Result<DescribeAgentVersionsResult, DescribeAgentVersionsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeAgentVersions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAgentVersionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeAgentVersionsError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a description of a specified set of apps.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_apps(&self,
                     input: &DescribeAppsRequest)
                     -> Result<DescribeAppsResult, DescribeAppsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeApps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAppsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeAppsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes the results of specified commands.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_commands(&self,
                         input: &DescribeCommandsRequest)
                         -> Result<DescribeCommandsResult, DescribeCommandsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeCommands");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeCommandsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeCommandsError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a description of a specified set of deployments.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_deployments(&self,
                            input: &DescribeDeploymentsRequest)
                            -> Result<DescribeDeploymentsResult, DescribeDeploymentsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeDeployments");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDeploymentsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeDeploymentsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Describes Amazon ECS clusters that are registered with a stack. If you specify only a stack ID, you can use the <code>MaxResults</code> and <code>NextToken</code> parameters to paginate the response. However, AWS OpsWorks Stacks currently supports only one cluster per layer, so the result set has a maximum of one element.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permission. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_ecs_clusters(&self,
                             input: &DescribeEcsClustersRequest)
                             -> Result<DescribeEcsClustersResult, DescribeEcsClustersError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeEcsClusters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEcsClustersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEcsClustersError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Describes <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP addresses</a>.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_elastic_ips(&self,
                            input: &DescribeElasticIpsRequest)
                            -> Result<DescribeElasticIpsResult, DescribeElasticIpsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeElasticIps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeElasticIpsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeElasticIpsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Describes a stack's Elastic Load Balancing instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_elastic_load_balancers
        (&self,
         input: &DescribeElasticLoadBalancersRequest)
         -> Result<DescribeElasticLoadBalancersResult, DescribeElasticLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.DescribeElasticLoadBalancers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeElasticLoadBalancersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeElasticLoadBalancersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Requests a description of a set of instances.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_instances(&self,
                          input: &DescribeInstancesRequest)
                          -> Result<DescribeInstancesResult, DescribeInstancesError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstancesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeInstancesError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a description of one or more layers in a specified stack.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_layers(&self,
                       input: &DescribeLayersRequest)
                       -> Result<DescribeLayersResult, DescribeLayersError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeLayers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeLayersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeLayersError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Describes load-based auto scaling configurations for specified layers.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_load_based_auto_scaling
        (&self,
         input: &DescribeLoadBasedAutoScalingRequest)
         -> Result<DescribeLoadBasedAutoScalingResult, DescribeLoadBasedAutoScalingError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.DescribeLoadBasedAutoScaling");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeLoadBasedAutoScalingResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeLoadBasedAutoScalingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes a user's SSH information.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_my_user_profile
        (&self)
         -> Result<DescribeMyUserProfileResult, DescribeMyUserProfileError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeMyUserProfile");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMyUserProfileResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeMyUserProfileError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the permissions for a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_permissions(&self,
                            input: &DescribePermissionsRequest)
                            -> Result<DescribePermissionsResult, DescribePermissionsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribePermissions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribePermissionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribePermissionsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Describe an instance's RAID arrays.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_raid_arrays(&self,
                            input: &DescribeRaidArraysRequest)
                            -> Result<DescribeRaidArraysResult, DescribeRaidArraysError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeRaidArrays");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRaidArraysResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeRaidArraysError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Describes Amazon RDS instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_rds_db_instances
        (&self,
         input: &DescribeRdsDbInstancesRequest)
         -> Result<DescribeRdsDbInstancesResult, DescribeRdsDbInstancesError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeRdsDbInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRdsDbInstancesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeRdsDbInstancesError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Describes AWS OpsWorks Stacks service errors.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p> <p>This call accepts only one resource-identifying parameter.</p>"]
    fn describe_service_errors
        (&self,
         input: &DescribeServiceErrorsRequest)
         -> Result<DescribeServiceErrorsResult, DescribeServiceErrorsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeServiceErrors");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeServiceErrorsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeServiceErrorsError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a description of a stack's provisioning parameters.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stack_provisioning_parameters
        (&self,
         input: &DescribeStackProvisioningParametersRequest)
         -> Result<DescribeStackProvisioningParametersResult,
                   DescribeStackProvisioningParametersError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.DescribeStackProvisioningParameters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStackProvisioningParametersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeStackProvisioningParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes the number of layers and apps in a specified stack, and the number of instances in each state, such as <code>running_setup</code> or <code>online</code>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stack_summary(&self,
                              input: &DescribeStackSummaryRequest)
                              -> Result<DescribeStackSummaryResult, DescribeStackSummaryError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeStackSummary");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStackSummaryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeStackSummaryError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Requests a description of one or more stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksRequest)
                       -> Result<DescribeStacksResult, DescribeStacksError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeStacks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStacksResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeStacksError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Describes time-based auto scaling configurations for specified instances.</p> <note> <p>You must specify at least one of the parameters.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_time_based_auto_scaling
        (&self,
         input: &DescribeTimeBasedAutoScalingRequest)
         -> Result<DescribeTimeBasedAutoScalingResult, DescribeTimeBasedAutoScalingError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.DescribeTimeBasedAutoScaling");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTimeBasedAutoScalingResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeTimeBasedAutoScalingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describe specified users.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_user_profiles(&self,
                              input: &DescribeUserProfilesRequest)
                              -> Result<DescribeUserProfilesResult, DescribeUserProfilesError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeUserProfiles");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeUserProfilesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeUserProfilesError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Describes an instance's Amazon EBS volumes.</p> <note> <p>This call accepts only one resource-identifying parameter.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Show, Deploy, or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn describe_volumes(&self,
                        input: &DescribeVolumesRequest)
                        -> Result<DescribeVolumesResult, DescribeVolumesError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DescribeVolumes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeVolumesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeVolumesError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Detaches a specified Elastic Load Balancing instance from its layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn detach_elastic_load_balancer(&self,
                                    input: &DetachElasticLoadBalancerRequest)
                                    -> Result<(), DetachElasticLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "OpsWorks_20130218.DetachElasticLoadBalancer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DetachElasticLoadBalancerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Disassociates an Elastic IP address from its instance. The address remains registered with the stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn disassociate_elastic_ip(&self,
                               input: &DisassociateElasticIpRequest)
                               -> Result<(), DisassociateElasticIpError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.DisassociateElasticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DisassociateElasticIpError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Gets a generated host name for the specified layer, based on the current host name theme.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn get_hostname_suggestion
        (&self,
         input: &GetHostnameSuggestionRequest)
         -> Result<GetHostnameSuggestionResult, GetHostnameSuggestionError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.GetHostnameSuggestion");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetHostnameSuggestionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetHostnameSuggestionError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<note> <p>This action can be used only with Windows stacks.</p> </note> <p>Grants RDP access to a Windows instance for a specified time period.</p>"]
    fn grant_access(&self,
                    input: &GrantAccessRequest)
                    -> Result<GrantAccessResult, GrantAccessError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.GrantAccess");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GrantAccessResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GrantAccessError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a list of tags that are applied to the specified stack or layer.</p>"]
    fn list_tags(&self, input: &ListTagsRequest) -> Result<ListTagsResult, ListTagsError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.ListTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<ListTagsResult>(String::from_utf8_lossy(&response.body)
                                                              .as_ref())
                           .unwrap())
            }
            _ => Err(ListTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Reboots a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn reboot_instance(&self, input: &RebootInstanceRequest) -> Result<(), RebootInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RebootInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(RebootInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Registers a specified Amazon ECS cluster with a stack. You can register only one cluster with a stack. A cluster can be registered with only one stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinglayers-ecscluster.html\"> Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\"> Managing User Permissions</a>.</p>"]
    fn register_ecs_cluster(&self,
                            input: &RegisterEcsClusterRequest)
                            -> Result<RegisterEcsClusterResult, RegisterEcsClusterError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterEcsCluster");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterEcsClusterResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RegisterEcsClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Registers an Elastic IP address with a specified stack. An address can be registered with only one stack at a time. If the address is already registered, you must first deregister it by calling <a>DeregisterElasticIp</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_elastic_ip(&self,
                           input: &RegisterElasticIpRequest)
                           -> Result<RegisterElasticIpResult, RegisterElasticIpError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterElasticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterElasticIpResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RegisterElasticIpError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Registers instances that were created outside of AWS OpsWorks Stacks with a specified stack.</p> <note> <p>We do not recommend using this action to register instances. The complete registration operation includes two tasks: installing the AWS OpsWorks Stacks agent on the instance, and registering the instance with the stack. <code>RegisterInstance</code> handles only the second step. You should instead use the AWS CLI <code>register</code> command, which performs the entire registration operation. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register.html\"> Registering an Instance with an AWS OpsWorks Stacks Stack</a>.</p> </note> <p>Registered instances have the same requirements as instances that are created by using the <a>CreateInstance</a> API. For example, registered instances must be running a supported Linux-based operating system, and they must have a supported instance type. For more information about requirements for instances that you want to register, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/registered-instances-register-registering-preparer.html\"> Preparing the Instance</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_instance(&self,
                         input: &RegisterInstanceRequest)
                         -> Result<RegisterInstanceResult, RegisterInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterInstanceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RegisterInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Registers an Amazon RDS instance with a stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_rds_db_instance(&self,
                                input: &RegisterRdsDbInstanceRequest)
                                -> Result<(), RegisterRdsDbInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterRdsDbInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(RegisterRdsDbInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Registers an Amazon EBS volume with a specified stack. A volume can be registered with only one stack at a time. If the volume is already registered, you must first deregister it by calling <a>DeregisterVolume</a>. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn register_volume(&self,
                       input: &RegisterVolumeRequest)
                       -> Result<RegisterVolumeResult, RegisterVolumeError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.RegisterVolume");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterVolumeResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RegisterVolumeError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Specify the load-based auto scaling configuration for a specified layer. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html\">Managing Load with Time-based and Load-based Instances</a>.</p> <note> <p>To use load-based auto scaling, you must create a set of load-based auto scaling instances. Load-based auto scaling operates only on the instances from that set, so you must ensure that you have created enough instances to handle the maximum anticipated load.</p> </note> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_load_based_auto_scaling(&self,
                                   input: &SetLoadBasedAutoScalingRequest)
                                   -> Result<(), SetLoadBasedAutoScalingError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetLoadBasedAutoScaling");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(SetLoadBasedAutoScalingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Specifies a user's permissions. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workingsecurity.html\">Security and Permissions</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_permission(&self, input: &SetPermissionRequest) -> Result<(), SetPermissionError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetPermission");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(SetPermissionError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Specify the time-based auto scaling configuration for a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-autoscaling.html\">Managing Load with Time-based and Load-based Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn set_time_based_auto_scaling(&self,
                                   input: &SetTimeBasedAutoScalingRequest)
                                   -> Result<(), SetTimeBasedAutoScalingError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.SetTimeBasedAutoScaling");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(SetTimeBasedAutoScalingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Starts a specified instance. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn start_instance(&self, input: &StartInstanceRequest) -> Result<(), StartInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StartInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(StartInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts a stack's instances.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn start_stack(&self, input: &StartStackRequest) -> Result<(), StartStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StartStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(StartStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Stops a specified instance. When you stop a standard instance, the data disappears and must be reinstalled when you restart the instance. You can stop an Amazon EBS-backed instance without losing data. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/workinginstances-starting.html\">Starting, Stopping, and Rebooting Instances</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn stop_instance(&self, input: &StopInstanceRequest) -> Result<(), StopInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StopInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(StopInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Stops a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn stop_stack(&self, input: &StopStackRequest) -> Result<(), StopStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.StopStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(StopStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Apply cost-allocation tags to a specified stack or layer in AWS OpsWorks Stacks. For more information about how tagging works, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/tagging.html\">Tags</a> in the AWS OpsWorks User Guide.</p>"]
    fn tag_resource(&self, input: &TagResourceRequest) -> Result<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.TagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(TagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Unassigns a registered instance from all of it's layers. The instance remains in the stack as an unassigned instance and can be assigned to another layer, as needed. You cannot use this action with instances that were created with AWS OpsWorks Stacks.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn unassign_instance(&self,
                         input: &UnassignInstanceRequest)
                         -> Result<(), UnassignInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UnassignInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UnassignInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Unassigns an assigned Amazon EBS volume. The volume remains registered with the stack. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn unassign_volume(&self, input: &UnassignVolumeRequest) -> Result<(), UnassignVolumeError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UnassignVolume");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UnassignVolumeError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Removes tags from a specified stack or layer.</p>"]
    fn untag_resource(&self, input: &UntagResourceRequest) -> Result<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UntagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UntagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a specified app.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Deploy or Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_app(&self, input: &UpdateAppRequest) -> Result<(), UpdateAppError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateApp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(UpdateAppError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates a registered Elastic IP address's name. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_elastic_ip(&self,
                         input: &UpdateElasticIpRequest)
                         -> Result<(), UpdateElasticIpError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateElasticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateElasticIpError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Updates a specified instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_instance(&self, input: &UpdateInstanceRequest) -> Result<(), UpdateInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Updates a specified layer.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_layer(&self, input: &UpdateLayerRequest) -> Result<(), UpdateLayerError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateLayer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(UpdateLayerError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates a user's SSH public key.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have self-management enabled or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_my_user_profile(&self,
                              input: &UpdateMyUserProfileRequest)
                              -> Result<(), UpdateMyUserProfileError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateMyUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateMyUserProfileError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Updates an Amazon RDS instance.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_rds_db_instance(&self,
                              input: &UpdateRdsDbInstanceRequest)
                              -> Result<(), UpdateRdsDbInstanceError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateRdsDbInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateRdsDbInstanceError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Updates a specified stack.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_stack(&self, input: &UpdateStackRequest) -> Result<(), UpdateStackError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateStack");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(UpdateStackError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates a specified user profile.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_user_profile(&self,
                           input: &UpdateUserProfileRequest)
                           -> Result<(), UpdateUserProfileError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateUserProfile");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateUserProfileError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Updates an Amazon EBS volume's name or mount point. For more information, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/resources.html\">Resource Management</a>.</p> <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href=\"http://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html\">Managing User Permissions</a>.</p>"]
    fn update_volume(&self, input: &UpdateVolumeRequest) -> Result<(), UpdateVolumeError> {
        let mut request = SignedRequest::new("POST", "opsworks", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorks_20130218.UpdateVolume");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(UpdateVolumeError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
