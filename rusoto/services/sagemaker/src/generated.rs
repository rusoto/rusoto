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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of <code>Tag</code> objects. Each tag is a key-value pair. Only the <code>key</code> parameter is required. If you don't specify a value, Amazon SageMaker sets the value to an empty string. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddTagsOutput {
    /// <p>A list of tags associated with the Amazon SageMaker resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Specifies the training algorithm to use in a <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateTrainingJob.html">CreateTrainingJob</a> request.</p> <p>For more information about algorithms provided by Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about using your own algorithms, see <a>your-algorithms</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmSpecification {
    /// <p>The registry path of the Docker image that contains the training algorithm. For information about docker registry paths for built-in algorithms, see <a>sagemaker-algo-docker-registry-paths</a>.</p>
    #[serde(rename = "TrainingImage")]
    pub training_image: String,
    /// <p>The input mode that the algorithm supports. For the input modes that Amazon SageMaker algorithms support, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. If an algorithm supports the <code>File</code> input mode, Amazon SageMaker downloads the training data from S3 to the provisioned ML storage Volume, and mounts the directory to docker volume for training container. If an algorithm supports the <code>Pipe</code> input mode, Amazon SageMaker streams data directly from S3 to the container. </p> <p> In File mode, make sure you provision ML storage volume with sufficient capacity to accommodate the data download from S3. In addition to the training data, the ML storage volume also stores the output model. The algorithm container use ML storage volume to also store intermediate information, if any. </p> <p> For distributed algorithms using File mode, training data is distributed uniformly, and your training duration is predictable if the input data objects size is approximately same. Amazon SageMaker does not split the files any further for model training. If the object sizes are skewed, training won't be optimal as the data distribution is also skewed where one host in a training cluster is overloaded, thus becoming bottleneck in training. </p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>A list of categorical hyperparameters to tune.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoricalParameterRange {
    /// <p>The name of the categorical hyperparameter to tune.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of the categories for the hyperparameter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A channel is a named input source that training algorithms can consume. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// <p>The name of the channel. </p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>If training data is compressed, the compression type. The default value is <code>None</code>. <code>CompressionType</code> is used only in Pipe input mode. In File mode, leave this field unset or set it to None.</p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p>The MIME type of the data.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The location of the channel data.</p>
    #[serde(rename = "DataSource")]
    pub data_source: DataSource,
    /// <p><p/> <p>Specify RecordIO as the value when input data is in raw format but the training algorithm requires the RecordIO format, in which case, Amazon SageMaker wraps each individual S3 object in a RecordIO record. If the input data is already in RecordIO format, you don&#39;t need to set this attribute. For more information, see <a href="https://mxnet.incubator.apache.org/how_to/recordio.html?highlight=im2rec">Create a Dataset Using RecordIO</a>. </p> <p>In FILE mode, leave this field unset or set it to None.</p> <p/></p>
    #[serde(rename = "RecordWrapperType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_wrapper_type: Option<String>,
}

/// <p>Describes the container, as part of model definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerDefinition {
    /// <p>The DNS host name for the container after Amazon SageMaker deploys it.</p>
    #[serde(rename = "ContainerHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_hostname: Option<String>,
    /// <p>The environment variables to set in the Docker container. Each key and value in the <code>Environment</code> string to string map can have length of up to 1024. We support up to 16 entries in the map. </p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon EC2 Container Registry (Amazon ECR) path where inference code is stored. If you are using your own custom algorithm instead of an algorithm provided by Amazon SageMaker, the inference code must meet Amazon SageMaker requirements. Amazon SageMaker supports both <code>registry/repository[:tag]</code> and <code>registry/repository[@digest]</code> image path formats. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a> </p>
    #[serde(rename = "Image")]
    pub image: String,
    /// <p>The S3 path where the model artifacts, which result from model training, are stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). </p> <p>If you provide a value for this parameter, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provide. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS i an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<String>,
}

/// <p>A list of continuous hyperparameters to tune.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuousParameterRange {
    /// <p>The maximum value for the hyperparameter. The tuning job uses floating-point values between <code>MinValue</code> value and this value for tuning.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: String,
    /// <p>The minimum value for the hyperparameter. The tuning job uses floating-point values between this value and <code>MaxValue</code>for tuning.</p>
    #[serde(rename = "MinValue")]
    pub min_value: String,
    /// <p>The name of the continuous hyperparameter to tune.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEndpointConfigInput {
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,
    /// <p>An array of key-value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateEndpointConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint configuration. </p>
    #[serde(rename = "EndpointConfigArn")]
    pub endpoint_config_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEndpointInput {
    /// <p>The name of an endpoint configuration. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a>. </p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The name of the endpoint. The name must be unique within an AWS Region in your AWS account.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>An array of key-value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a>in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateEndpointOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHyperParameterTuningJobRequest {
    /// <p>The <a>HyperParameterTuningJobConfig</a> object that describes the tuning job, including the search strategy, metric used to evaluate training jobs, ranges of parameters to search, and resource limits for the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobConfig")]
    pub hyper_parameter_tuning_job_config: HyperParameterTuningJobConfig,
    /// <p>The name of the tuning job. This name is the prefix for the names of all training jobs that this tuning job launches. The name must be unique within the same AWS account and AWS Region. Names are not case sensitive, and must be between 1-32 characters.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
    /// <p>An array of key-value pairs. You can use tags to categorize your AWS resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="http://docs.aws.amazon.com//awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <a>HyperParameterTrainingJobDefinition</a> object that describes the training jobs that this tuning job launches, including static hyperparameters, input data configuration, output data configuration, resource configuration, and stopping condition.</p>
    #[serde(rename = "TrainingJobDefinition")]
    pub training_job_definition: HyperParameterTrainingJobDefinition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateHyperParameterTuningJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobArn")]
    pub hyper_parameter_tuning_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelInput {
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute instances. Deploying on ML compute instances is part of model hosting. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The name of the new model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>The location of the primary docker image containing inference code, associated artifacts, and custom environment map that the inference code uses when the model is deployed into production. </p>
    #[serde(rename = "PrimaryContainer")]
    pub primary_container: ContainerDefinition,
    /// <p>An array of key-value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that you want your model to connect to. Control access to and from your model container by configuring the VPC. For more information, see <a>host-vpc</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateModelOutput {
    /// <p>The ARN of the model created in Amazon SageMaker.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNotebookInstanceInput {
    /// <p>Sets whether Amazon SageMaker provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance will be able to access resources only in your VPC, and will not be able to connect to Amazon SageMaker training and endpoint services unless your configure a NAT Gateway in your VPC.</p> <p>For more information, see <a>appendix-notebook-and-internet-access</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p>
    #[serde(rename = "DirectInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_internet_access: Option<String>,
    /// <p>The type of ML compute instance to launch for the notebook instance.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p> If you provide a AWS KMS key ID, Amazon SageMaker uses it to encrypt data at rest on the ML storage volume that is attached to your notebook instance. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    #[serde(rename = "LifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_config_name: Option<String>,
    /// <p>The name of the new notebook instance.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p><p> When you send any requests to AWS resources from the notebook instance, Amazon SageMaker assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so Amazon SageMaker can perform these tasks. The policy must allow the Amazon SageMaker service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be for the same VPC as specified in the subnet. </p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The ID of the subnet in a VPC to which you would like to have a connectivity from your ML compute instance. </p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>A list of tags to associate with the notebook instance. You can add tags later by using the <code>CreateTags</code> API.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
    /// <p>A shell script that runs only once, when you create a notebook instance.</p>
    #[serde(rename = "OnCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// <p>A shell script that runs every time you start a notebook instance, including when you create the notebook instance.</p>
    #[serde(rename = "OnStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNotebookInstanceLifecycleConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNotebookInstanceOutput {
    /// <p>The Amazon Resource Name (ARN) of the notebook instance. </p>
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePresignedNotebookInstanceUrlInput {
    /// <p>The name of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p>The duration of the session, in seconds. The default is 12 hours.</p>
    #[serde(rename = "SessionExpirationDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expiration_duration_in_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePresignedNotebookInstanceUrlOutput {
    /// <p>A JSON object that contains the URL string. </p>
    #[serde(rename = "AuthorizedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTrainingJobRequest {
    /// <p>The registry path of the Docker image that contains the training algorithm and algorithm-specific metadata, including the input mode. For more information about algorithms provided by Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about providing your own algorithms, see <a>your-algorithms</a>. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p>Algorithm-specific parameters that influence the quality of the model. You set hyperparameters before you start the learning process. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> <p>You can specify a maximum of 100 hyperparameters. Each hyperparameter is a key-value pair. Each key and value is limited to 256 characters, as specified by the <code>Length Constraint</code>. </p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects. Each channel is a named input source. <code>InputDataConfig</code> describes the input data and its location. </p> <p>Algorithms can accept input data from one or more channels. For example, an algorithm might have two channels of input data, <code>training_data</code> and <code>validation_data</code>. The configuration for each channel provides the S3 location where the input data is stored. It also provides information about the stored data: the MIME type, compression method, and whether the data is wrapped in RecordIO format. </p> <p>Depending on the input mode that the algorithm supports, Amazon SageMaker either copies input data files from an S3 bucket to a local directory in the Docker container, or makes it available as input streams. </p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<Channel>,
    /// <p>Specifies the path to the S3 bucket where you want to store model artifacts. Amazon SageMaker creates subfolders for the artifacts. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The resources, including the ML compute instances and ML storage volumes, to use for model training. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use ML storage volumes for scratch space. If you want Amazon SageMaker to use the ML storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p><p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf. </p> <p>During model training, Amazon SageMaker needs your permission to read input data from an S3 bucket, download a Docker image that contains training code, write model artifacts to an S3 bucket, write logs to Amazon CloudWatch Logs, and publish metrics to Amazon CloudWatch. You grant permissions for all of these tasks to an IAM role. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Sets a duration for training. Use this parameter to cap model training costs. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts. </p> <p>When Amazon SageMaker terminates a job because the stopping condition has been met, training algorithms provided by Amazon SageMaker save the intermediate results of the job. This intermediate data is a valid model artifact. You can use it to create a model using the <code>CreateModel</code> API. </p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>An array of key-value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the training job. The name must be unique within an AWS Region in an AWS account. </p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that you want your training job to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a>train-vpc</a> </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTrainingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTransformJobRequest {
    /// <p>Determines the number of records included in a single mini-batch. <code>SingleRecord</code> means only one record is used per mini-batch. <code>MultiRecord</code> means a mini-batch is set to contain as many records that can fit within the <code>MaxPayloadInMB</code> limit.</p> <p>Batch transform will automatically split your input data into whatever payload size is specified if you set <code>SplitType</code> to <code>Line</code> and <code>BatchStrategy</code> to <code>MultiRecord</code>. There's no need to split the dataset into smaller files or to use larger payload sizes unless the records in your dataset are very large.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The maximum number of parallel requests that can be sent to each instance in a transform job. This is good for algorithms that implement multiple workers on larger instances . The default value is <code>1</code>. To allow Amazon SageMaker to determine the appropriate number for <code>MaxConcurrentTransforms</code>, set the value to <code>0</code>.</p>
    #[serde(rename = "MaxConcurrentTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transforms: Option<i64>,
    /// <p>The maximum payload size allowed, in MB. A payload is the data portion of a record (without metadata). The value in <code>MaxPayloadInMB</code> must be greater or equal to the size of a single record. You can approximate the size of a record by dividing the size of your dataset by the number of records. Then multiply this value by the number of records you want in a mini-batch. It is recommended to enter a value slightly larger than this to ensure the records fit within the maximum payload size. The default value is <code>6</code> MB. For an unlimited payload size, set the value to <code>0</code>.</p>
    #[serde(rename = "MaxPayloadInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_in_mb: Option<i64>,
    /// <p>The name of the model that you want to use for the transform job. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an AWS Region in an AWS account.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>An array of key-value pairs. Adding tags is optional. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Describes the input source and the way the transform job consumes it.</p>
    #[serde(rename = "TransformInput")]
    pub transform_input: TransformInput,
    /// <p>The name of the transform job. The name must be unique within an AWS Region in an AWS account. </p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
    /// <p>Describes the results of the transform job.</p>
    #[serde(rename = "TransformOutput")]
    pub transform_output: TransformOutput,
    /// <p>Describes the resources, including ML instance types and ML instance count, to use for the transform job.</p>
    #[serde(rename = "TransformResources")]
    pub transform_resources: TransformResources,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTransformJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the transform job.</p>
    #[serde(rename = "TransformJobArn")]
    pub transform_job_arn: String,
}

/// <p>Describes the location of the channel data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    /// <p>The S3 location of the data source that is associated with a channel.</p>
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: S3DataSource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEndpointConfigInput {
    /// <p>The name of the endpoint configuration that you want to delete.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEndpointInput {
    /// <p>The name of the endpoint that you want to delete.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteModelInput {
    /// <p>The name of the model to delete.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNotebookInstanceInput {
    /// <p>The name of the Amazon SageMaker notebook instance to delete.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration to delete.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags you want to delete.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array or one or more tag keys to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTagsOutput {}

/// <p>Gets the Amazon EC2 Container Registry path of the docker image of the model that is hosted in this <a>ProductionVariant</a>.</p> <p>If you used the <code>registry/repository[:tag]</code> form to to specify the image path of the primary container when you created the model hosted in this <code>ProductionVariant</code>, the path resolves to a path of the form <code>registry/repository[@digest]</code>. A digest is a hash value that identifies a specific version of an image. For information about Amazon ECR paths, see <a href="http://docs.aws.amazon.com//AmazonECR/latest/userguide/docker-pull-ecr-image.html">Pulling an Image</a> in the <i>Amazon ECR User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeployedImage {
    /// <p>The date and time when the image path for the model resolved to the <code>ResolvedImage</code> </p>
    #[serde(rename = "ResolutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_time: Option<f64>,
    /// <p>The specific digest path of the image hosted in this <code>ProductionVariant</code>.</p>
    #[serde(rename = "ResolvedImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_image: Option<String>,
    /// <p>The image path you specified when you created the model.</p>
    #[serde(rename = "SpecifiedImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specified_image: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointConfigInput {
    /// <p>The name of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEndpointConfigOutput {
    /// <p>A timestamp that shows when the endpoint configuration was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigArn")]
    pub endpoint_config_arn: String,
    /// <p>Name of the Amazon SageMaker endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>AWS KMS key ID Amazon SageMaker uses to encrypt data when storing it on the ML storage volume attached to the instance.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointInput {
    /// <p>The name of the endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEndpointOutput {
    /// <p>A timestamp that shows when the endpoint was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The name of the endpoint configuration associated with this endpoint.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>Name of the endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The status of the endpoint.</p>
    #[serde(rename = "EndpointStatus")]
    pub endpoint_status: String,
    /// <p>If the status of the endpoint is <code>Failed</code>, the reason why it failed. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A timestamp that shows when the endpoint was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p> An array of <a>ProductionVariantSummary</a> objects, one for each model hosted behind this endpoint. </p>
    #[serde(rename = "ProductionVariants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_variants: Option<Vec<ProductionVariantSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job to describe.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeHyperParameterTuningJobResponse {
    /// <p>A <a>TrainingJobSummary</a> object that describes the training job that completed with the best current <a>HyperParameterTuningJobObjective</a>.</p>
    #[serde(rename = "BestTrainingJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_training_job: Option<HyperParameterTrainingJobSummary>,
    /// <p>The date and time that the tuning job started.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If the tuning job failed, the reason it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time that the tuning job ended.</p>
    #[serde(rename = "HyperParameterTuningEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_tuning_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobArn")]
    pub hyper_parameter_tuning_job_arn: String,
    /// <p>The <a>HyperParameterTuningJobConfig</a> object that specifies the configuration of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobConfig")]
    pub hyper_parameter_tuning_job_config: HyperParameterTuningJobConfig,
    /// <p>The name of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
    /// <p>The status of the tuning job: InProgress, Completed, Failed, Stopping, or Stopped.</p>
    #[serde(rename = "HyperParameterTuningJobStatus")]
    pub hyper_parameter_tuning_job_status: String,
    /// <p>The date and time that the status of the tuning job was modified. </p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The <a>ObjectiveStatusCounters</a> object that specifies the number of training jobs, categorized by the status of their final objective metric, that this tuning job launched.</p>
    #[serde(rename = "ObjectiveStatusCounters")]
    pub objective_status_counters: ObjectiveStatusCounters,
    /// <p>The <a>HyperParameterTrainingJobDefinition</a> object that specifies the definition of the training jobs that this tuning job launches.</p>
    #[serde(rename = "TrainingJobDefinition")]
    pub training_job_definition: HyperParameterTrainingJobDefinition,
    /// <p>The <a>TrainingJobStatusCounters</a> object that specifies the number of training jobs, categorized by status, that this tuning job launched.</p>
    #[serde(rename = "TrainingJobStatusCounters")]
    pub training_job_status_counters: TrainingJobStatusCounters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeModelInput {
    /// <p>The name of the model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeModelOutput {
    /// <p>A timestamp that shows when the model was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p>
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
    /// <p>Name of the Amazon SageMaker model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>The location of the primary inference code, associated artifacts, and custom environment map that the inference code uses when it is deployed in production. </p>
    #[serde(rename = "PrimaryContainer")]
    pub primary_container: ContainerDefinition,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that this model has access to. For more information, see <a>host-vpc</a> </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeNotebookInstanceInput {
    /// <p>The name of the notebook instance that you want information about.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration to describe.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeNotebookInstanceLifecycleConfigOutput {
    /// <p>A timestamp that tells when the lifecycle configuration was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A timestamp that tells when the lifecycle configuration was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_arn: Option<String>,
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name: Option<String>,
    /// <p>The shell script that runs only once, when you create a notebook instance.</p>
    #[serde(rename = "OnCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// <p>The shell script that runs every time you start a notebook instance, including when you create the notebook instance.</p>
    #[serde(rename = "OnStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeNotebookInstanceOutput {
    /// <p>A timestamp. Use this parameter to return the time when the notebook instance was created</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Describes whether Amazon SageMaker provides internet access to the notebook instance. If this value is set to <i>Disabled, he notebook instance does not have internet access, and cannot connect to Amazon SageMaker training and endpoint services</i>.</p> <p>For more information, see <a>appendix-notebook-and-internet-access</a>.</p>
    #[serde(rename = "DirectInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_internet_access: Option<String>,
    /// <p>If status is failed, the reason it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The type of ML compute instance running on the notebook instance.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p> AWS KMS key ID Amazon SageMaker uses to encrypt data when storing it on the ML storage volume attached to the instance. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>A timestamp. Use this parameter to retrieve the time when the notebook instance was last modified. </p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p> Network interface IDs that Amazon SageMaker created at the time of creating the instance. </p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
    /// <p>Returns the name of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name: Option<String>,
    /// <p> Name of the Amazon SageMaker notebook instance. </p>
    #[serde(rename = "NotebookInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_name: Option<String>,
    /// <p>The status of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_status: Option<String>,
    /// <p> Amazon Resource Name (ARN) of the IAM role associated with the instance. </p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The IDs of the VPC security groups.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The ID of the VPC subnet.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The URL that you use to connect to the Jupyter notebook that is running in your notebook instance. </p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrainingJobRequest {
    /// <p>The name of the training job.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTrainingJobResponse {
    /// <p>Information about the algorithm used for training, and algorithm metadata. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p>A timestamp that indicates when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If the training job failed, the reason it failed. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Algorithm-specific parameters. </p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects that describes each data input channel. </p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<Channel>,
    /// <p>A timestamp that indicates when the status of the training job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>Information about the Amazon S3 location that is configured for storing model artifacts. </p>
    #[serde(rename = "ModelArtifacts")]
    pub model_artifacts: ModelArtifacts,
    /// <p>The S3 path where model artifacts that you configured when creating the job are stored. Amazon SageMaker creates subfolders for model artifacts. </p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>Resources, including ML compute instances and ML storage volumes, that are configured for model training. </p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p>The AWS Identity and Access Management (IAM) role configured for the training job. </p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p> Provides granular information about the system state. For more information, see <code>TrainingJobStatus</code>. </p> <ul> <li> <p> <code>Starting</code> - starting the training job.</p> </li> <li> <p> <code>LaunchingMLInstances</code> - launching ML instances for the training job.</p> </li> <li> <p> <code>PreparingTrainingStack</code> - preparing the ML instances for the training job.</p> </li> <li> <p> <code>Downloading</code> - downloading the input data.</p> </li> <li> <p> <code>DownloadingTrainingImage</code> - downloading the training algorithm image.</p> </li> <li> <p> <code>Training</code> - model training is in progress.</p> </li> <li> <p> <code>Uploading</code> - uploading the trained model.</p> </li> <li> <p> <code>Stopping</code> - stopping the training job.</p> </li> <li> <p> <code>Stopped</code> - the training job has stopped.</p> </li> <li> <p> <code>MaxRuntimeExceeded</code> - the training job exceeded the specified max run time and has been stopped.</p> </li> <li> <p> <code>Completed</code> - the training job has completed.</p> </li> <li> <p> <code>Failed</code> - the training job has failed. The failure reason is provided in the <code>StatusMessage</code>.</p> </li> </ul> <important> <p>The valid values for <code>SecondaryStatus</code> are subject to change. They primarily provide information on the progress of the training job.</p> </important></p>
    #[serde(rename = "SecondaryStatus")]
    pub secondary_status: String,
    /// <p>To give an overview of the training job lifecycle, <code>SecondaryStatusTransitions</code> is a log of time-ordered secondary statuses that a training job has transitioned.</p>
    #[serde(rename = "SecondaryStatusTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_status_transitions: Option<Vec<SecondaryStatusTransition>>,
    /// <p>The condition under which to stop the training job. </p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>Indicates the time when the training job ends on training instances. You are billed for the time interval between the value of <code>TrainingStartTime</code> and this time. For successful jobs and stopped jobs, this is the time after model artifacts are uploaded. For failed jobs, this is the time when Amazon SageMaker detects a job failure.</p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
    /// <p> Name of the model training job. </p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p><p>The status of the training job. </p> <p>For the <code>InProgress</code> status, Amazon SageMaker can return these secondary statuses:</p> <ul> <li> <p>Starting - Preparing for training.</p> </li> <li> <p>Downloading - Optional stage for algorithms that support File training input mode. It indicates data is being downloaded to ML storage volumes.</p> </li> <li> <p>Training - Training is in progress.</p> </li> <li> <p>Uploading - Training is complete and model upload is in progress.</p> </li> </ul> <p>For the <code>Stopped</code> training status, Amazon SageMaker can return these secondary statuses:</p> <ul> <li> <p>MaxRuntimeExceeded - Job stopped as a result of maximum allowed runtime exceeded.</p> </li> </ul></p>
    #[serde(rename = "TrainingJobStatus")]
    pub training_job_status: String,
    /// <p>Indicates the time when the training job starts on training instances. You are billed for the time interval between this time and the value of <code>TrainingEndTime</code>. The start time in CloudWatch Logs might be later than this time. The difference is due to the time it takes to download the training data and to the size of the training container.</p>
    #[serde(rename = "TrainingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the associated hyperparameter tuning job if the training job was launched by a hyperparameter tuning job.</p>
    #[serde(rename = "TuningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_job_arn: Option<String>,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that this training job has access to. For more information, see <a>train-vpc</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTransformJobRequest {
    /// <p>The name of the transform job that you want to view details of.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTransformJobResponse {
    /// <p>SingleRecord means only one record was used per a batch. <code>MultiRecord</code> means batches contained as many records that could possibly fit within the <code>MaxPayloadInMB</code> limit.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>A timestamp that shows when the transform Job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p><p/></p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>If the transform job failed, the reason that it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The maximum number of parallel requests on each instance node that can be launched in a transform job. The default value is 1.</p>
    #[serde(rename = "MaxConcurrentTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transforms: Option<i64>,
    /// <p>The maximum payload size , in MB used in the transform job.</p>
    #[serde(rename = "MaxPayloadInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_in_mb: Option<i64>,
    /// <p>The name of the model used in the transform job.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>Indicates when the transform job is <code>Completed</code>, <code>Stopped</code>, or <code>Failed</code>. You are billed for the time interval between this time and the value of <code>TransformStartTime</code>.</p>
    #[serde(rename = "TransformEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_end_time: Option<f64>,
    /// <p>Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p>
    #[serde(rename = "TransformInput")]
    pub transform_input: TransformInput,
    /// <p>The Amazon Resource Name (ARN) of the transform job.</p>
    #[serde(rename = "TransformJobArn")]
    pub transform_job_arn: String,
    /// <p>The name of the transform job.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
    /// <p>The status of the transform job. If the transform job failed, the reason is returned in the <code>FailureReason</code> field.</p>
    #[serde(rename = "TransformJobStatus")]
    pub transform_job_status: String,
    /// <p>Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p>
    #[serde(rename = "TransformOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_output: Option<TransformOutput>,
    /// <p>Describes the resources, including ML instance types and ML instance count, to use for the transform job.</p>
    #[serde(rename = "TransformResources")]
    pub transform_resources: TransformResources,
    /// <p>Indicates when the transform job starts on ML instances. You are billed for the time interval between this time and the value of <code>TransformEndTime</code>.</p>
    #[serde(rename = "TransformStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_start_time: Option<f64>,
}

/// <p>Specifies weight and capacity values for a production variant.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DesiredWeightAndCapacity {
    /// <p>The variant's capacity.</p>
    #[serde(rename = "DesiredInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_instance_count: Option<i64>,
    /// <p>The variant's weight.</p>
    #[serde(rename = "DesiredWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_weight: Option<f32>,
    /// <p>The name of the variant to update.</p>
    #[serde(rename = "VariantName")]
    pub variant_name: String,
}

/// <p>Provides summary information for an endpoint configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EndpointConfigSummary {
    /// <p>A timestamp that shows when the endpoint configuration was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigArn")]
    pub endpoint_config_arn: String,
    /// <p>The name of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

/// <p>Provides summary information for an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EndpointSummary {
    /// <p>A timestamp that shows when the endpoint was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The name of the endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The status of the endpoint.</p>
    #[serde(rename = "EndpointStatus")]
    pub endpoint_status: String,
    /// <p>A timestamp that shows when the endpoint was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
}

/// <p>Shows the final value for the objective metric for a training job that was launched by a hyperparameter tuning job. You define the objective metric in the <code>HyperParameterTuningJobObjective</code> parameter of <a>HyperParameterTuningJobConfig</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FinalHyperParameterTuningJobObjectiveMetric {
    /// <p>The name of the objective metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>Whether to minimize or maximize the objective metric. Valid values are Minimize and Maximize.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the objective metric.</p>
    #[serde(rename = "Value")]
    pub value: f32,
}

/// <p>Specifies which training algorithm to use for training jobs that a hyperparameter tuning job launches and the metrics to monitor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterAlgorithmSpecification {
    /// <p>An array of <a>MetricDefinition</a> objects that specify the metrics that the algorithm emits.</p>
    #[serde(rename = "MetricDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,
    /// <p> The registry path of the Docker image that contains the training algorithm. For information about Docker registry paths for built-in algorithms, see <a>sagemaker-algo-docker-registry-paths</a>.</p>
    #[serde(rename = "TrainingImage")]
    pub training_image: String,
    /// <p>The input mode that the algorithm supports: File or Pipe. In File input mode, Amazon SageMaker downloads the training data from Amazon S3 to the storage volume that is attached to the training instance and mounts the directory to the Docker volume for the training container. In Pipe input mode, Amazon SageMaker streams data directly from Amazon S3 to the container. </p> <p>If you specify File mode, make sure that you provision the storage volume that is attached to the training instance with enough capacity to accommodate the training data downloaded from Amazon S3, the model artifacts, and intermediate information.</p> <p/> <p>For more information about input modes, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>Defines the training jobs launched by a hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTrainingJobDefinition {
    /// <p>The <a>HyperParameterAlgorithmSpecification</a> object that specifies the algorithm to use for the training jobs that the tuning job launches.</p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: HyperParameterAlgorithmSpecification,
    /// <p>An array of <a>Channel</a> objects that specify the input for the training jobs that the tuning job launches.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<Channel>,
    /// <p>Specifies the path to the Amazon S3 bucket where you store model artifacts from the training jobs that the tuning job launches.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The resources, including the compute instances and storage volumes, to use for the training jobs that the tuning job launches.</p> <p>Storage volumes store model artifacts and incremental states. Training algorithms might also use storage volumes for scratch space. If you want Amazon SageMaker to use the storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the training jobs that the tuning job launches.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Specifies the values of hyperparameters that do not change for the tuning job.</p>
    #[serde(rename = "StaticHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Sets a maximum duration for the training jobs that the tuning job launches. Use this parameter to limit model training costs. </p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal. This delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts.</p> <p>When Amazon SageMaker terminates a job because the stopping condition has been met, training algorithms provided by Amazon SageMaker save the intermediate results of the job.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>The <a>VpcConfig</a> object that specifies the VPC that you want the training jobs that this hyperparameter tuning job launches to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a>train-vpc</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Specifies summary information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HyperParameterTrainingJobSummary {
    /// <p>The date and time that the training job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The reason that the training job failed. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The <a>FinalHyperParameterTuningJobObjectiveMetric</a> object that specifies the value of the objective metric of the tuning job that launched this training job.</p>
    #[serde(rename = "FinalHyperParameterTuningJobObjectiveMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_hyper_parameter_tuning_job_objective_metric:
        Option<FinalHyperParameterTuningJobObjectiveMetric>,
    /// <p><p>The status of the objective metric for the training job:</p> <ul> <li> <p>Succeeded: The final objective metric for the training job was evaluated by the hyperparameter tuning job and used in the hyperparameter tuning process.</p> </li> </ul> <ul> <li> <p>Pending: The training job is in progress and evaluation of its final objective metric is pending.</p> </li> </ul> <ul> <li> <p>Failed: The final objective metric for the training job was not evaluated, and was not used in the hyperparameter tuning process. This typically occurs when the training job failed or did not emit an objective metric.</p> </li> </ul></p>
    #[serde(rename = "ObjectiveStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_status: Option<String>,
    /// <p>The date and time that the training job ended.</p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
    /// <p>The name of the training job.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p>The status of the training job.</p>
    #[serde(rename = "TrainingJobStatus")]
    pub training_job_status: String,
    /// <p>The date and time that the training job started.</p>
    #[serde(rename = "TrainingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
    /// <p>A list of the hyperparameters for which you specified ranges to search.</p>
    #[serde(rename = "TunedHyperParameters")]
    pub tuned_hyper_parameters: ::std::collections::HashMap<String, String>,
}

/// <p>Configures a hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTuningJobConfig {
    /// <p>The <a>HyperParameterTuningJobObjective</a> object that specifies the objective metric for this tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobObjective")]
    pub hyper_parameter_tuning_job_objective: HyperParameterTuningJobObjective,
    /// <p>The <a>ParameterRanges</a> object that specifies the ranges of hyperparameters that this tuning job searches.</p>
    #[serde(rename = "ParameterRanges")]
    pub parameter_ranges: ParameterRanges,
    /// <p>The <a>ResourceLimits</a> object that specifies the maximum number of training jobs and parallel training jobs for this tuning job.</p>
    #[serde(rename = "ResourceLimits")]
    pub resource_limits: ResourceLimits,
    /// <p>Specifies the search strategy for hyperparameters. Currently, the only valid value is <code>Bayesian</code>.</p>
    #[serde(rename = "Strategy")]
    pub strategy: String,
}

/// <p>Defines the objective metric for a hyperparameter tuning job. Hyperparameter tuning uses the value of this metric to evaluate the training jobs it launches, and returns the training job that results in either the highest or lowest value for this metric, depending on the value you specify for the <code>Type</code> parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTuningJobObjective {
    /// <p>The name of the metric to use for the objective metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>Whether to minimize or maximize the objective metric.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Provides summary information about a hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HyperParameterTuningJobSummary {
    /// <p>The date and time that the tuning job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The date and time that the tuning job ended.</p>
    #[serde(rename = "HyperParameterTuningEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_tuning_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobArn")]
    pub hyper_parameter_tuning_job_arn: String,
    /// <p>The name of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
    /// <p>The status of the tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobStatus")]
    pub hyper_parameter_tuning_job_status: String,
    /// <p>The date and time that the tuning job was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The <a>ObjectiveStatusCounters</a> object that specifies the numbers of training jobs, categorized by objective metric status, that this tuning job launched.</p>
    #[serde(rename = "ObjectiveStatusCounters")]
    pub objective_status_counters: ObjectiveStatusCounters,
    /// <p>The <a>ResourceLimits</a> object that specifies the maximum number of training jobs and parallel training jobs allowed for this tuning job.</p>
    #[serde(rename = "ResourceLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<ResourceLimits>,
    /// <p>Specifies the search strategy hyperparameter tuning uses to choose which hyperparameters to use for each iteration. Currently, the only valid value is Bayesian.</p>
    #[serde(rename = "Strategy")]
    pub strategy: String,
    /// <p>The <a>TrainingJobStatusCounters</a> object that specifies the numbers of training jobs, categorized by status, that this tuning job launched.</p>
    #[serde(rename = "TrainingJobStatusCounters")]
    pub training_job_status_counters: TrainingJobStatusCounters,
}

/// <p>For a hyperparameter of the integer type, specifies the range that a hyperparameter tuning job searches.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerParameterRange {
    /// <p>The maximum value of the hyperparameter to search.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: String,
    /// <p>The minimum value of the hyperparameter to search.</p>
    #[serde(rename = "MinValue")]
    pub min_value: String,
    /// <p>The name of the hyperparameter to search.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEndpointConfigsInput {
    /// <p>A filter that returns only endpoint configurations created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only endpoint configurations created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The maximum number of training jobs to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the endpoint configuration name. This filter returns only endpoint configurations whose name contains the specified string. </p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListEndpointConfig</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of endpoint configurations, use the token in the next request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field to sort results by. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEndpointConfigsOutput {
    /// <p>An array of endpoint configurations.</p>
    #[serde(rename = "EndpointConfigs")]
    pub endpoint_configs: Vec<EndpointConfigSummary>,
    /// <p> If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of endpoint configurations, use it in the subsequent request </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEndpointsInput {
    /// <p>A filter that returns only endpoints that were created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only endpoints that were created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p> A filter that returns only endpoints that were modified after the specified timestamp. </p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p> A filter that returns only endpoints that were modified before the specified timestamp. </p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of endpoints to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in endpoint names. This filter returns only endpoints whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of a <code>ListEndpoints</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of endpoints, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sorts the list of results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p> A filter that returns only endpoints with the specified status. </p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEndpointsOutput {
    /// <p> An array or endpoint objects. </p>
    #[serde(rename = "Endpoints")]
    pub endpoints: Vec<EndpointSummary>,
    /// <p> If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of training jobs, use it in the subsequent request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHyperParameterTuningJobsRequest {
    /// <p>A filter that returns only tuning jobs that were created after the specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only tuning jobs that were created before the specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only tuning jobs that were modified after the specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only tuning jobs that were modified before the specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of tuning jobs to return. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the tuning job name. This filter returns only tuning jobs whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListHyperParameterTuningJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of tuning jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field to sort results by. The default is <code>Name</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that returns only tuning jobs with the specified status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListHyperParameterTuningJobsResponse {
    /// <p>A list of <a>HyperParameterTuningJobSummary</a> objects that describe the tuning jobs that the <code>ListHyperParameterTuningJobs</code> request returned.</p>
    #[serde(rename = "HyperParameterTuningJobSummaries")]
    pub hyper_parameter_tuning_job_summaries: Vec<HyperParameterTuningJobSummary>,
    /// <p>If the result of this <code>ListHyperParameterTuningJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of tuning jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListModelsInput {
    /// <p>A filter that returns only models created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only models created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The maximum number of models to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the training job name. This filter returns only models in the training job whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the response to a previous <code>ListModels</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of models, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sorts the list of results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListModelsOutput {
    /// <p>An array of <code>ModelSummary</code> objects, each of which lists a model.</p>
    #[serde(rename = "Models")]
    pub models: Vec<ModelSummary>,
    /// <p> If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of models, use it in the subsequent request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNotebookInstanceLifecycleConfigsInput {
    /// <p>A filter that returns only lifecycle configurations that were created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only lifecycle configurations that were created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only lifecycle configurations that were modified after the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only lifecycle configurations that were modified before the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of lifecycle configurations to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the lifecycle configuration name. This filter returns only lifecycle configurations whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of a <code>ListNotebookInstanceLifecycleConfigs</code> request was truncated, the response includes a <code>NextToken</code>. To get the next set of lifecycle configurations, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sorts the list of results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListNotebookInstanceLifecycleConfigsOutput {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To get the next set of lifecycle configurations, use it in the next request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>NotebookInstanceLifecycleConfiguration</code> objects, each listing a lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_configs: Option<Vec<NotebookInstanceLifecycleConfigSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNotebookInstancesInput {
    /// <p>A filter that returns only notebook instances that were created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only notebook instances that were created before the specified time (timestamp). </p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only notebook instances that were modified after the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only notebook instances that were modified before the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of notebook instances to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the notebook instances' name. This filter returns only notebook instances whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p><p> If the previous call to the <code>ListNotebookInstances</code> is truncated, the response includes a <code>NextToken</code>. You can use this token in your subsequent <code>ListNotebookInstances</code> request to fetch the next set of notebook instances. </p> <note> <p> You might specify a filter or a sort order in your request. When response is truncated, you must use the same values for the filer and sort order in the next request. </p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A string in the name of a notebook instances lifecycle configuration associated with this notebook instance. This filter returns only notebook instances associated with a lifecycle configuration with a name that contains the specified string.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigNameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name_contains: Option<String>,
    /// <p>The field to sort results by. The default is <code>Name</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. </p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that returns only notebook instances with the specified status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListNotebookInstancesOutput {
    /// <p>If the response to the previous <code>ListNotebookInstances</code> request was truncated, Amazon SageMaker returns this token. To retrieve the next set of notebook instances, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>NotebookInstanceSummary</code> objects, one for each notebook instance.</p>
    #[serde(rename = "NotebookInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instances: Option<Vec<NotebookInstanceSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsInput {
    /// <p>Maximum number of tags to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> If the response to the previous <code>ListTags</code> request is truncated, Amazon SageMaker returns this token. To retrieve the next set of tags, use it in the subsequent request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags you want to retrieve.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsOutput {
    /// <p> If response is truncated, Amazon SageMaker includes a token in the response. You can use this token in your subsequent request to fetch next set of tokens. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>Tag</code> objects, each with a tag key and a value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTrainingJobsForHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job whose training jobs you want to list.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
    /// <p>The maximum number of training jobs to return. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous <code>ListTrainingJobsForHyperParameterTuningJob</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of training jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field to sort results by. The default is <code>Name</code>.</p> <p>If the value of this field is <code>FinalObjectiveMetricValue</code>, any training jobs that did not return an objective metric are not listed.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that returns only training jobs with the specified status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTrainingJobsForHyperParameterTuningJobResponse {
    /// <p>If the result of this <code>ListTrainingJobsForHyperParameterTuningJob</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of training jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <a>TrainingJobSummary</a> objects that describe the training jobs that the <code>ListTrainingJobsForHyperParameterTuningJob</code> request returned.</p>
    #[serde(rename = "TrainingJobSummaries")]
    pub training_job_summaries: Vec<HyperParameterTrainingJobSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTrainingJobsRequest {
    /// <p>A filter that returns only training jobs created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only training jobs created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only training jobs modified after the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only training jobs modified before the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of training jobs to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the training job name. This filter returns only training jobs whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListTrainingJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of training jobs, use the token in the next request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field to sort results by. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that retrieves only training jobs with a specific status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTrainingJobsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of training jobs, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>TrainingJobSummary</code> objects, each listing a training job.</p>
    #[serde(rename = "TrainingJobSummaries")]
    pub training_job_summaries: Vec<TrainingJobSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTransformJobsRequest {
    /// <p>A filter that returns only transform jobs created after the specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only transform jobs created before the specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only transform jobs modified after the specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only transform jobs modified before the specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of transform jobs to return in the response. The default value is <code>10</code>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the transform job name. This filter returns only transform jobs whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListTransformJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of transform jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field to sort results by. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that retrieves only transform jobs with a specific status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTransformJobsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of transform jobs, use it in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>TransformJobSummary</code> objects.</p>
    #[serde(rename = "TransformJobSummaries")]
    pub transform_job_summaries: Vec<TransformJobSummary>,
}

/// <p>Specifies a metric that the training algorithm writes to <code>stderr</code> or <code>stdout</code>. Amazon SageMakerHyperparamter tuning captures all defined metrics. You specify one metric that a hyperparameter tuning job uses as its objective metric to choose the best training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A regular expression that searches the output of a training job and gets the value of the metric. For more information about using regular expressions to define metrics, see <a>automatic-model-tuning-define-metrics</a>.</p>
    #[serde(rename = "Regex")]
    pub regex: String,
}

/// <p>Provides information about the location that is configured for storing model artifacts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelArtifacts {
    /// <p>The path of the S3 object that contains the model artifacts. For example, <code>s3://bucket-name/keynameprefix/model.tar.gz</code>.</p>
    #[serde(rename = "S3ModelArtifacts")]
    pub s3_model_artifacts: String,
}

/// <p>Provides summary information about a model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelSummary {
    /// <p>A timestamp that indicates when the model was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
    /// <p>The name of the model that you want a summary for.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

/// <p>Provides a summary of a notebook instance lifecycle configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NotebookInstanceLifecycleConfigSummary {
    /// <p>A timestamp that tells when the lifecycle configuration was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A timestamp that tells when the lifecycle configuration was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigArn")]
    pub notebook_instance_lifecycle_config_arn: String,
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
}

/// <p>Contains the notebook instance lifecycle configuration script.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookInstanceLifecycleHook {
    /// <p>A base64-encoded string that contains a shell script for a notebook instance lifecycle configuration.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// <p>Provides summary information for an Amazon SageMaker notebook instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NotebookInstanceSummary {
    /// <p>A timestamp that shows when the notebook instance was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The type of ML compute instance that the notebook instance is running on.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>A timestamp that shows when the notebook instance was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceArn")]
    pub notebook_instance_arn: String,
    /// <p>The name of a notebook instance lifecycle configuration associated with this notebook instance.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name: Option<String>,
    /// <p>The name of the notebook instance that you want a summary for.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p>The status of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_status: Option<String>,
    /// <p>The URL that you use to connect to the Jupyter instance running in your notebook instance. </p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies the number of training jobs that this hyperparameter tuning job launched, categorized by the status of their objective metric. The objective metric status shows whether the final objective metric for the training job has been evaluated by the tuning job and used in the hyperparameter tuning process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ObjectiveStatusCounters {
    /// <p>The number of training jobs whose final objective metric was not evaluated and used in the hyperparameter tuning process. This typically occurs when the training job failed or did not emit an objective metric.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    /// <p>The number of training jobs that are in progress and pending evaluation of their final objective metric.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// <p>The number of training jobs whose final objective metric was evaluated by the hyperparameter tuning job and used in the hyperparameter tuning process.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i64>,
}

/// <p>Provides information about how to store model training results (model artifacts).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDataConfig {
    /// <p><p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption. </p> <note> <p>If you don&#39;t provide the KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role&#39;s account. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in Amazon Simple Storage Service developer guide.</p> </note> <note> <p> The KMS key policy must grant permission to the IAM role you specify in your <code>CreateTrainingJob</code> request. <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the AWS Key Management Service Developer Guide. </p> </note></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Identifies the S3 path where you want Amazon SageMaker to store the model artifacts. For example, <code>s3://bucket-name/key-name-prefix</code>. </p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Specifies ranges of integer, continuous, and categorical hyperparameters that a hyperparameter tuning job searches.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterRanges {
    /// <p>The array of <a>CategoricalParameterRange</a> objects that specify ranges of categorical hyperparameters that a hyperparameter tuning job searches.</p>
    #[serde(rename = "CategoricalParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_parameter_ranges: Option<Vec<CategoricalParameterRange>>,
    /// <p>The array of <a>ContinuousParameterRange</a> objects that specify ranges of continuous hyperparameters that a hyperparameter tuning job searches.</p>
    #[serde(rename = "ContinuousParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_parameter_ranges: Option<Vec<ContinuousParameterRange>>,
    /// <p>The array of <a>IntegerParameterRange</a> objects that specify ranges of integer hyperparameters that a hyperparameter tuning job searches.</p>
    #[serde(rename = "IntegerParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_ranges: Option<Vec<IntegerParameterRange>>,
}

/// <p>Identifies a model that you want to host and the resources to deploy for hosting it. If you are deploying multiple models, tell Amazon SageMaker how to distribute traffic among the models by specifying variant weights. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionVariant {
    /// <p>Number of instances to launch initially.</p>
    #[serde(rename = "InitialInstanceCount")]
    pub initial_instance_count: i64,
    /// <p>Determines initial traffic distribution among all of the models that you specify in the endpoint configuration. The traffic to a production variant is determined by the ratio of the <code>VariantWeight</code> to the sum of all <code>VariantWeight</code> values across all ProductionVariants. If unspecified, it defaults to 1.0. </p>
    #[serde(rename = "InitialVariantWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_variant_weight: Option<f32>,
    /// <p>The ML compute instance type.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The name of the model that you want to host. This is the name that you specified when creating the model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>The name of the production variant.</p>
    #[serde(rename = "VariantName")]
    pub variant_name: String,
}

/// <p>Describes weight and capacities for a production variant associated with an endpoint. If you sent a request to the <code>UpdateEndpointWeightsAndCapacities</code> API and the endpoint status is <code>Updating</code>, you get different desired and current values. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProductionVariantSummary {
    /// <p>The number of instances associated with the variant.</p>
    #[serde(rename = "CurrentInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instance_count: Option<i64>,
    /// <p>The weight associated with the variant.</p>
    #[serde(rename = "CurrentWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_weight: Option<f32>,
    /// <p>An array of <code>DeployedImage</code> objects that specify the Amazon EC2 Container Registry paths of the inference images deployed on instances of this <code>ProductionVariant</code>.</p>
    #[serde(rename = "DeployedImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_images: Option<Vec<DeployedImage>>,
    /// <p>The number of instances requested in the <code>UpdateEndpointWeightsAndCapacities</code> request. </p>
    #[serde(rename = "DesiredInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_instance_count: Option<i64>,
    /// <p>The requested weight, as specified in the <code>UpdateEndpointWeightsAndCapacities</code> request. </p>
    #[serde(rename = "DesiredWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_weight: Option<f32>,
    /// <p>The name of the variant.</p>
    #[serde(rename = "VariantName")]
    pub variant_name: String,
}

/// <p>Describes the resources, including ML compute instances and ML storage volumes, to use for model training. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// <p>The number of ML compute instances to use. For distributed training, provide a value greater than 1. </p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The ML compute instance type. </p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the training job.</p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p><p>The size of the ML storage volume that you want to provision. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use the ML storage volume for scratch space. If you want to store the training data in the ML storage volume, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. </p> <p>You must specify sufficient ML storage for your scenario. </p> <note> <p> Amazon SageMaker supports only the General Purpose SSD (gp2) ML storage volume type. </p> </note></p>
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,
}

/// <p>Specifies the maximum number of training jobs and parallel training jobs that a hyperparameter tuning job can launch.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// <p>The maximum number of training jobs that a hyperparameter tuning job can launch.</p>
    #[serde(rename = "MaxNumberOfTrainingJobs")]
    pub max_number_of_training_jobs: i64,
    /// <p>The maximum number of concurrent training jobs that a hyperparameter tuning job can launch.</p>
    #[serde(rename = "MaxParallelTrainingJobs")]
    pub max_parallel_training_jobs: i64,
}

/// <p>Describes the S3 data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3DataSource {
    /// <p>If you want Amazon SageMaker to replicate the entire dataset on each ML compute instance that is launched for model training, specify <code>FullyReplicated</code>. </p> <p>If you want Amazon SageMaker to replicate a subset of data on each ML compute instance that is launched for model training, specify <code>ShardedByS3Key</code>. If there are <i>n</i> ML compute instances launched for a training job, each instance gets approximately 1/<i>n</i> of the number of S3 objects. In this case, model training on each machine uses only the subset of training data. </p> <p>Don't choose more ML compute instances for training than available S3 objects. If you do, some nodes won't get any data and you will pay for nodes that aren't getting any training data. This applies in both FILE and PIPE modes. Keep this in mind when developing algorithms. </p> <p>In distributed training, where you use multiple ML compute EC2 instances, you might choose <code>ShardedByS3Key</code>. If the algorithm requires copying training data to the ML storage volume (when <code>TrainingInputMode</code> is set to <code>File</code>), this copies 1/<i>n</i> of the number of objects. </p>
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<String>,
    /// <p>If you choose <code>S3Prefix</code>, <code>S3Uri</code> identifies a key name prefix. Amazon SageMaker uses all objects with the specified key name prefix for model training. </p> <p>If you choose <code>ManifestFile</code>, <code>S3Uri</code> identifies an object that is a manifest file containing a list of object keys that you want Amazon SageMaker to use for model training. </p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example: </p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> <code>[</code> </p> <p> <code> {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code> &quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code> &quot;relative/path/custdata-2&quot;,</code> </p> <p> <code> ...</code> </p> <p> <code> ]</code> </p> <p> The preceding JSON matches the following <code>s3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer_bucket/some/prefix/relative/path/custdata-1</code> </p> <p> <code>...</code> </p> <p> The complete set of <code>s3uris</code> in this manifest constitutes the input data for the channel for this datasource. The object that each <code>s3uris</code> points to must readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf. </p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Specifies a secondary status the job has transitioned into. It includes a start timestamp and later an end timestamp. The end timestamp is added either after the job transitions to a different secondary status or after the job has ended.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SecondaryStatusTransition {
    /// <p>A timestamp that shows when the secondary status has ended and the job has transitioned into another secondary status. The <code>EndTime</code> timestamp is also set after the training job has ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>A timestamp that shows when the training job has entered this secondary status.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    /// <p>Provides granular information about the system state. For more information, see <code>SecondaryStatus</code> under the <a>DescribeTrainingJob</a> response elements.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>Shows a brief description and other information about the secondary status. For example, the <code>LaunchingMLInstances</code> secondary status could show a status message of "Insufficent capacity error while launching instances".</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartNotebookInstanceInput {
    /// <p>The name of the notebook instance to start.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job to stop.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopNotebookInstanceInput {
    /// <p>The name of the notebook instance to terminate.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTrainingJobRequest {
    /// <p>The name of the training job to stop.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTransformJobRequest {
    /// <p>The name of the transform job to stop.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
}

/// <p>Specifies how long model training can run. When model training reaches the limit, Amazon SageMaker ends the training job. Use this API to cap model training cost.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of training is not lost. </p> <p>Training algorithms provided by Amazon SageMaker automatically saves the intermediate results of a model training job (it is best effort case, as model might not be ready to save as some stages, for example training just started). This intermediate data is a valid model artifact. You can use it to create a model (<code>CreateModel</code>). </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoppingCondition {
    /// <p>The maximum length of time, in seconds, that the training job can run. If model training does not complete during this time, Amazon SageMaker ends the job. If value is not specified, default value is 1 day. Maximum value is 5 days.</p>
    #[serde(rename = "MaxRuntimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_in_seconds: Option<i64>,
}

/// <p>Describes a tag. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The tag value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>The numbers of training jobs launched by a hyperparameter tuning job, categorized by status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrainingJobStatusCounters {
    /// <p>The number of completed training jobs launched by a hyperparameter tuning job.</p>
    #[serde(rename = "Completed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    /// <p>The number of in-progress training jobs launched by a hyperparameter tuning job.</p>
    #[serde(rename = "InProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// <p>The number of training jobs that failed and can't be retried. A failed training job can't be retried if it failed because a client error occurred.</p>
    #[serde(rename = "NonRetryableError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_retryable_error: Option<i64>,
    /// <p>The number of training jobs that failed, but can be retried. A failed training job can be retried only if it failed because an internal service error occurred.</p>
    #[serde(rename = "RetryableError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable_error: Option<i64>,
    /// <p>The number of training jobs launched by a hyperparameter tuning job that were manually stopped.</p>
    #[serde(rename = "Stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
}

/// <p>Provides summary information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrainingJobSummary {
    /// <p>A timestamp that shows when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p> Timestamp when the training job was last modified. </p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>A timestamp that shows when the training job ended. This field is set only if the training job has one of the terminal statuses (<code>Completed</code>, <code>Failed</code>, or <code>Stopped</code>). </p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
    /// <p>The name of the training job that you want a summary for.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p>The status of the training job.</p>
    #[serde(rename = "TrainingJobStatus")]
    pub training_job_status: String,
}

/// <p>Describes the location of the channel data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformDataSource {
    /// <p>The S3 location of the data source that is associated with a channel.</p>
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: TransformS3DataSource,
}

/// <p>Describes the input source of a transform job and the way the transform job consumes it.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformInput {
    /// <p>Compressing data helps save on storage space. If your transform data is compressed, specify the compression type.and Amazon SageMaker will automatically decompress the data for the transform job accordingly. The default value is <code>None</code>.</p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p>The multipurpose internet mail extension (MIME) type of the data. Amazon SageMaker uses the MIME type with each http call to transfer data to the transform job.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Describes the location of the channel data, meaning the S3 location of the input data that the model can consume.</p>
    #[serde(rename = "DataSource")]
    pub data_source: TransformDataSource,
    /// <p><p>The method to use to split the transform job&#39;s data into smaller batches. The default value is <code>None</code>. If you don&#39;t want to split the data, specify <code>None</code>. If you want to split records on a newline character boundary, specify <code>Line</code>. To split records according to the RecordIO format, specify <code>RecordIO</code>.</p> <p>Amazon SageMaker will send maximum number of records per batch in each request up to the MaxPayloadInMB limit. For more information, see <a href="http://mxnet.io/architecture/note_data_loading.html#data-format">RecordIO data format</a>.</p> <note> <p>For information about the <code>RecordIO</code> format, see <a href="http://mxnet.io/architecture/note_data_loading.html#data-format">Data Format</a>.</p> </note></p>
    #[serde(rename = "SplitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_type: Option<String>,
}

/// <p>Provides a summary information for a transform job. Multiple TransformJobSummary objects are returned as a list after calling <a>ListTransformJobs</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TransformJobSummary {
    /// <p>A timestamp that shows when the transform Job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If the transform job failed, the reason it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Indicates when the transform job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>Indicates when the transform job ends on compute instances. For successful jobs and stopped jobs, this is the exact time recorded after the results are uploaded. For failed jobs, this is when Amazon SageMaker detected that the job failed.</p>
    #[serde(rename = "TransformEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the transform job.</p>
    #[serde(rename = "TransformJobArn")]
    pub transform_job_arn: String,
    /// <p>The name of the transform job.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
    /// <p>The status of the transform job.</p>
    #[serde(rename = "TransformJobStatus")]
    pub transform_job_status: String,
}

/// <p>Describes the results of a transform job output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformOutput {
    /// <p>The MIME type used to specify the output data. Amazon SageMaker uses the MIME type with each http call to transfer data from the transform job.</p>
    #[serde(rename = "Accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>Defines how to assemble the results of the transform job as a single S3 object. You should select a format that is most convenient to you. To concatenate the results in binary format, specify <code>None</code>. To add a newline character at the end of every transformed record, specify <code>Line</code>. To assemble the output in RecordIO format, specify <code>RecordIO</code>. The default value is <code>None</code>.</p> <p>For information about the <code>RecordIO</code> format, see <a href="http://mxnet.io/architecture/note_data_loading.html#data-format">Data Format</a>.</p>
    #[serde(rename = "AssembleWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assemble_with: Option<String>,
    /// <p>The AWS Key Management Service (AWS KMS) key for Amazon S3 server-side encryption that Amazon SageMaker uses to encrypt the transformed data.</p> <p>If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateTramsformJob</code> request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The Amazon S3 path where you want Amazon SageMaker to store the results of the transform job. For example, <code>s3://bucket-name/key-name-prefix</code>.</p> <p>For every S3 object used as input for the transform job, the transformed data is stored in a corresponding subfolder in the location under the output prefix. For example, the input data <code>s3://bucket-name/input-name-prefix/dataset01/data.csv</code> will have the transformed data stored at <code>s3://bucket-name/key-name-prefix/dataset01/</code>, based on the original name, as a series of .part files (.part0001, part0002, etc).</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Describes the resources, including ML instance types and ML instance count, to use for transform job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformResources {
    /// <p>The number of ML compute instances to use in the transform job. For distributed transform, provide a value greater than 1. The default value is <code>1</code>.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The ML compute instance type for the transform job. For using built-in algorithms to transform moderately sized datasets, ml.m4.xlarge or <code>ml.m5.large</code> should suffice. There is no default value for <code>InstanceType</code>.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
}

/// <p>Describes the S3 data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformS3DataSource {
    /// <p>If you choose <code>S3Prefix</code>, <code>S3Uri</code> identifies a key name prefix. Amazon SageMaker uses all objects with the specified key name prefix for batch transform. </p> <p>If you choose <code>ManifestFile</code>, <code>S3Uri</code> identifies an object that is a manifest file containing a list of object keys that you want Amazon SageMaker to use for batch transform. </p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example:</p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> <code>[</code> </p> <p> <code> {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code> &quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code> &quot;relative/path/custdata-2&quot;,</code> </p> <p> <code> ...</code> </p> <p> <code> ]</code> </p> <p> The preceding JSON matches the following <code>S3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer_bucket/some/prefix/relative/path/custdata-1</code> </p> <p> <code>...</code> </p> <p> The complete set of <code>S3Uris</code> in this manifest constitutes the input data for the channel for this datasource. The object that each <code>S3Uris</code> points to must be readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf.</p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointInput {
    /// <p>The name of the new endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The name of the endpoint whose configuration you want to update.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateEndpointOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointWeightsAndCapacitiesInput {
    /// <p>An object that provides new capacity and weight values for a variant.</p>
    #[serde(rename = "DesiredWeightsAndCapacities")]
    pub desired_weights_and_capacities: Vec<DesiredWeightAndCapacity>,
    /// <p>The name of an existing Amazon SageMaker endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateEndpointWeightsAndCapacitiesOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNotebookInstanceInput {
    /// <p>The Amazon ML compute instance type.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The name of the notebook instance to update.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker can assume to access the notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
    /// <p>The shell script that runs only once, when you create a notebook instance</p>
    #[serde(rename = "OnCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// <p>The shell script that runs every time you start a notebook instance, including when you create the notebook instance.</p>
    #[serde(rename = "OnStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateNotebookInstanceLifecycleConfigOutput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateNotebookInstanceOutput {}

/// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a>host-vpc</a> and <a>train-vpc</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfig {
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The ID of the subnets in the VPC to which you want to connect your training job or model.</p>
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
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

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> AddTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => return AddTagsError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return AddTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AddTagsError {
    fn from(err: serde_json::error::Error) -> AddTagsError {
        AddTagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsError {
    fn from(err: CredentialsError) -> AddTagsError {
        AddTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsError {
    fn from(err: HttpDispatchError) -> AddTagsError {
        AddTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsError {
    fn from(err: io::Error) -> AddTagsError {
        AddTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsError {
    fn description(&self) -> &str {
        match *self {
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::ParseError(ref cause) => cause,
            AddTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return CreateEndpointError::ResourceLimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEndpointError {
    fn from(err: serde_json::error::Error) -> CreateEndpointError {
        CreateEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEndpointError {
    fn from(err: CredentialsError) -> CreateEndpointError {
        CreateEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEndpointError {
    fn from(err: HttpDispatchError) -> CreateEndpointError {
        CreateEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEndpointError {
    fn from(err: io::Error) -> CreateEndpointError {
        CreateEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateEndpointError::ResourceLimitExceeded(ref cause) => cause,
            CreateEndpointError::Validation(ref cause) => cause,
            CreateEndpointError::Credentials(ref err) => err.description(),
            CreateEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateEndpointError::ParseError(ref cause) => cause,
            CreateEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateEndpointConfig
#[derive(Debug, PartialEq)]
pub enum CreateEndpointConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEndpointConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return CreateEndpointConfigError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateEndpointConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateEndpointConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEndpointConfigError {
    fn from(err: serde_json::error::Error) -> CreateEndpointConfigError {
        CreateEndpointConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEndpointConfigError {
    fn from(err: CredentialsError) -> CreateEndpointConfigError {
        CreateEndpointConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEndpointConfigError {
    fn from(err: HttpDispatchError) -> CreateEndpointConfigError {
        CreateEndpointConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEndpointConfigError {
    fn from(err: io::Error) -> CreateEndpointConfigError {
        CreateEndpointConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEndpointConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEndpointConfigError {
    fn description(&self) -> &str {
        match *self {
            CreateEndpointConfigError::ResourceLimitExceeded(ref cause) => cause,
            CreateEndpointConfigError::Validation(ref cause) => cause,
            CreateEndpointConfigError::Credentials(ref err) => err.description(),
            CreateEndpointConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEndpointConfigError::ParseError(ref cause) => cause,
            CreateEndpointConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum CreateHyperParameterTuningJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateHyperParameterTuningJobError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateHyperParameterTuningJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceInUse" => {
                    return CreateHyperParameterTuningJobError::ResourceInUse(String::from(
                        error_message,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return CreateHyperParameterTuningJobError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateHyperParameterTuningJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateHyperParameterTuningJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateHyperParameterTuningJobError {
    fn from(err: serde_json::error::Error) -> CreateHyperParameterTuningJobError {
        CreateHyperParameterTuningJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateHyperParameterTuningJobError {
    fn from(err: CredentialsError) -> CreateHyperParameterTuningJobError {
        CreateHyperParameterTuningJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHyperParameterTuningJobError {
    fn from(err: HttpDispatchError) -> CreateHyperParameterTuningJobError {
        CreateHyperParameterTuningJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHyperParameterTuningJobError {
    fn from(err: io::Error) -> CreateHyperParameterTuningJobError {
        CreateHyperParameterTuningJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHyperParameterTuningJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHyperParameterTuningJobError {
    fn description(&self) -> &str {
        match *self {
            CreateHyperParameterTuningJobError::ResourceInUse(ref cause) => cause,
            CreateHyperParameterTuningJobError::ResourceLimitExceeded(ref cause) => cause,
            CreateHyperParameterTuningJobError::Validation(ref cause) => cause,
            CreateHyperParameterTuningJobError::Credentials(ref err) => err.description(),
            CreateHyperParameterTuningJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateHyperParameterTuningJobError::ParseError(ref cause) => cause,
            CreateHyperParameterTuningJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateModelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return CreateModelError::ResourceLimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateModelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateModelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateModelError {
    fn from(err: serde_json::error::Error) -> CreateModelError {
        CreateModelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateModelError {
    fn from(err: CredentialsError) -> CreateModelError {
        CreateModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateModelError {
    fn from(err: HttpDispatchError) -> CreateModelError {
        CreateModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateModelError {
    fn from(err: io::Error) -> CreateModelError {
        CreateModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateModelError {
    fn description(&self) -> &str {
        match *self {
            CreateModelError::ResourceLimitExceeded(ref cause) => cause,
            CreateModelError::Validation(ref cause) => cause,
            CreateModelError::Credentials(ref err) => err.description(),
            CreateModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateModelError::ParseError(ref cause) => cause,
            CreateModelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateNotebookInstance
#[derive(Debug, PartialEq)]
pub enum CreateNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return CreateNotebookInstanceError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> CreateNotebookInstanceError {
        CreateNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNotebookInstanceError {
    fn from(err: CredentialsError) -> CreateNotebookInstanceError {
        CreateNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNotebookInstanceError {
    fn from(err: HttpDispatchError) -> CreateNotebookInstanceError {
        CreateNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNotebookInstanceError {
    fn from(err: io::Error) -> CreateNotebookInstanceError {
        CreateNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateNotebookInstanceError::ResourceLimitExceeded(ref cause) => cause,
            CreateNotebookInstanceError::Validation(ref cause) => cause,
            CreateNotebookInstanceError::Credentials(ref err) => err.description(),
            CreateNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNotebookInstanceError::ParseError(ref cause) => cause,
            CreateNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum CreateNotebookInstanceLifecycleConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateNotebookInstanceLifecycleConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateNotebookInstanceLifecycleConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return CreateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return CreateNotebookInstanceLifecycleConfigError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return CreateNotebookInstanceLifecycleConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateNotebookInstanceLifecycleConfigError {
    fn from(err: serde_json::error::Error) -> CreateNotebookInstanceLifecycleConfigError {
        CreateNotebookInstanceLifecycleConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNotebookInstanceLifecycleConfigError {
    fn from(err: CredentialsError) -> CreateNotebookInstanceLifecycleConfigError {
        CreateNotebookInstanceLifecycleConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNotebookInstanceLifecycleConfigError {
    fn from(err: HttpDispatchError) -> CreateNotebookInstanceLifecycleConfigError {
        CreateNotebookInstanceLifecycleConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNotebookInstanceLifecycleConfigError {
    fn from(err: io::Error) -> CreateNotebookInstanceLifecycleConfigError {
        CreateNotebookInstanceLifecycleConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {
            CreateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(ref cause) => cause,
            CreateNotebookInstanceLifecycleConfigError::Validation(ref cause) => cause,
            CreateNotebookInstanceLifecycleConfigError::Credentials(ref err) => err.description(),
            CreateNotebookInstanceLifecycleConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNotebookInstanceLifecycleConfigError::ParseError(ref cause) => cause,
            CreateNotebookInstanceLifecycleConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePresignedNotebookInstanceUrl
#[derive(Debug, PartialEq)]
pub enum CreatePresignedNotebookInstanceUrlError {
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

impl CreatePresignedNotebookInstanceUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePresignedNotebookInstanceUrlError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return CreatePresignedNotebookInstanceUrlError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return CreatePresignedNotebookInstanceUrlError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePresignedNotebookInstanceUrlError {
    fn from(err: serde_json::error::Error) -> CreatePresignedNotebookInstanceUrlError {
        CreatePresignedNotebookInstanceUrlError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePresignedNotebookInstanceUrlError {
    fn from(err: CredentialsError) -> CreatePresignedNotebookInstanceUrlError {
        CreatePresignedNotebookInstanceUrlError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePresignedNotebookInstanceUrlError {
    fn from(err: HttpDispatchError) -> CreatePresignedNotebookInstanceUrlError {
        CreatePresignedNotebookInstanceUrlError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePresignedNotebookInstanceUrlError {
    fn from(err: io::Error) -> CreatePresignedNotebookInstanceUrlError {
        CreatePresignedNotebookInstanceUrlError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePresignedNotebookInstanceUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePresignedNotebookInstanceUrlError {
    fn description(&self) -> &str {
        match *self {
            CreatePresignedNotebookInstanceUrlError::Validation(ref cause) => cause,
            CreatePresignedNotebookInstanceUrlError::Credentials(ref err) => err.description(),
            CreatePresignedNotebookInstanceUrlError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePresignedNotebookInstanceUrlError::ParseError(ref cause) => cause,
            CreatePresignedNotebookInstanceUrlError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateTrainingJob
#[derive(Debug, PartialEq)]
pub enum CreateTrainingJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateTrainingJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceInUse" => {
                    return CreateTrainingJobError::ResourceInUse(String::from(error_message))
                }
                "ResourceLimitExceeded" => {
                    return CreateTrainingJobError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateTrainingJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateTrainingJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateTrainingJobError {
    fn from(err: serde_json::error::Error) -> CreateTrainingJobError {
        CreateTrainingJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTrainingJobError {
    fn from(err: CredentialsError) -> CreateTrainingJobError {
        CreateTrainingJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTrainingJobError {
    fn from(err: HttpDispatchError) -> CreateTrainingJobError {
        CreateTrainingJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTrainingJobError {
    fn from(err: io::Error) -> CreateTrainingJobError {
        CreateTrainingJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTrainingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTrainingJobError {
    fn description(&self) -> &str {
        match *self {
            CreateTrainingJobError::ResourceInUse(ref cause) => cause,
            CreateTrainingJobError::ResourceLimitExceeded(ref cause) => cause,
            CreateTrainingJobError::Validation(ref cause) => cause,
            CreateTrainingJobError::Credentials(ref err) => err.description(),
            CreateTrainingJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTrainingJobError::ParseError(ref cause) => cause,
            CreateTrainingJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateTransformJob
#[derive(Debug, PartialEq)]
pub enum CreateTransformJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl CreateTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateTransformJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceInUse" => {
                    return CreateTransformJobError::ResourceInUse(String::from(error_message))
                }
                "ResourceLimitExceeded" => {
                    return CreateTransformJobError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateTransformJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateTransformJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateTransformJobError {
    fn from(err: serde_json::error::Error) -> CreateTransformJobError {
        CreateTransformJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTransformJobError {
    fn from(err: CredentialsError) -> CreateTransformJobError {
        CreateTransformJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTransformJobError {
    fn from(err: HttpDispatchError) -> CreateTransformJobError {
        CreateTransformJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTransformJobError {
    fn from(err: io::Error) -> CreateTransformJobError {
        CreateTransformJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTransformJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTransformJobError {
    fn description(&self) -> &str {
        match *self {
            CreateTransformJobError::ResourceInUse(ref cause) => cause,
            CreateTransformJobError::ResourceLimitExceeded(ref cause) => cause,
            CreateTransformJobError::Validation(ref cause) => cause,
            CreateTransformJobError::Credentials(ref err) => err.description(),
            CreateTransformJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTransformJobError::ParseError(ref cause) => cause,
            CreateTransformJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
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

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteEndpointError {
        DeleteEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEndpointError {
    fn from(err: CredentialsError) -> DeleteEndpointError {
        DeleteEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEndpointError {
    fn from(err: HttpDispatchError) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEndpointError {
    fn from(err: io::Error) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::Validation(ref cause) => cause,
            DeleteEndpointError::Credentials(ref err) => err.description(),
            DeleteEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteEndpointError::ParseError(ref cause) => cause,
            DeleteEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEndpointConfig
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointConfigError {
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

impl DeleteEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEndpointConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteEndpointConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEndpointConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEndpointConfigError {
    fn from(err: serde_json::error::Error) -> DeleteEndpointConfigError {
        DeleteEndpointConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEndpointConfigError {
    fn from(err: CredentialsError) -> DeleteEndpointConfigError {
        DeleteEndpointConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEndpointConfigError {
    fn from(err: HttpDispatchError) -> DeleteEndpointConfigError {
        DeleteEndpointConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEndpointConfigError {
    fn from(err: io::Error) -> DeleteEndpointConfigError {
        DeleteEndpointConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEndpointConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointConfigError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointConfigError::Validation(ref cause) => cause,
            DeleteEndpointConfigError::Credentials(ref err) => err.description(),
            DeleteEndpointConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEndpointConfigError::ParseError(ref cause) => cause,
            DeleteEndpointConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
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

impl DeleteModelError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteModelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteModelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteModelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteModelError {
    fn from(err: serde_json::error::Error) -> DeleteModelError {
        DeleteModelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteModelError {
    fn from(err: CredentialsError) -> DeleteModelError {
        DeleteModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteModelError {
    fn from(err: HttpDispatchError) -> DeleteModelError {
        DeleteModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteModelError {
    fn from(err: io::Error) -> DeleteModelError {
        DeleteModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteModelError {
    fn description(&self) -> &str {
        match *self {
            DeleteModelError::Validation(ref cause) => cause,
            DeleteModelError::Credentials(ref err) => err.description(),
            DeleteModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteModelError::ParseError(ref cause) => cause,
            DeleteModelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteNotebookInstance
#[derive(Debug, PartialEq)]
pub enum DeleteNotebookInstanceError {
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

impl DeleteNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> DeleteNotebookInstanceError {
        DeleteNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotebookInstanceError {
    fn from(err: CredentialsError) -> DeleteNotebookInstanceError {
        DeleteNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotebookInstanceError {
    fn from(err: HttpDispatchError) -> DeleteNotebookInstanceError {
        DeleteNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotebookInstanceError {
    fn from(err: io::Error) -> DeleteNotebookInstanceError {
        DeleteNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotebookInstanceError::Validation(ref cause) => cause,
            DeleteNotebookInstanceError::Credentials(ref err) => err.description(),
            DeleteNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotebookInstanceError::ParseError(ref cause) => cause,
            DeleteNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum DeleteNotebookInstanceLifecycleConfigError {
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

impl DeleteNotebookInstanceLifecycleConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteNotebookInstanceLifecycleConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteNotebookInstanceLifecycleConfigError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DeleteNotebookInstanceLifecycleConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteNotebookInstanceLifecycleConfigError {
    fn from(err: serde_json::error::Error) -> DeleteNotebookInstanceLifecycleConfigError {
        DeleteNotebookInstanceLifecycleConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotebookInstanceLifecycleConfigError {
    fn from(err: CredentialsError) -> DeleteNotebookInstanceLifecycleConfigError {
        DeleteNotebookInstanceLifecycleConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotebookInstanceLifecycleConfigError {
    fn from(err: HttpDispatchError) -> DeleteNotebookInstanceLifecycleConfigError {
        DeleteNotebookInstanceLifecycleConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotebookInstanceLifecycleConfigError {
    fn from(err: io::Error) -> DeleteNotebookInstanceLifecycleConfigError {
        DeleteNotebookInstanceLifecycleConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotebookInstanceLifecycleConfigError::Validation(ref cause) => cause,
            DeleteNotebookInstanceLifecycleConfigError::Credentials(ref err) => err.description(),
            DeleteNotebookInstanceLifecycleConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotebookInstanceLifecycleConfigError::ParseError(ref cause) => cause,
            DeleteNotebookInstanceLifecycleConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
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

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DeleteTagsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteTagsError {
    fn from(err: serde_json::error::Error) -> DeleteTagsError {
        DeleteTagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::ParseError(ref cause) => cause,
            DeleteTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointError {
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

impl DescribeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DescribeEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEndpointError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointError {
        DescribeEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointError {
    fn from(err: CredentialsError) -> DescribeEndpointError {
        DescribeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointError {
    fn from(err: HttpDispatchError) -> DescribeEndpointError {
        DescribeEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEndpointError {
    fn from(err: io::Error) -> DescribeEndpointError {
        DescribeEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointError::Validation(ref cause) => cause,
            DescribeEndpointError::Credentials(ref err) => err.description(),
            DescribeEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEndpointError::ParseError(ref cause) => cause,
            DescribeEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEndpointConfig
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointConfigError {
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

impl DescribeEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEndpointConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DescribeEndpointConfigError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeEndpointConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEndpointConfigError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointConfigError {
        DescribeEndpointConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointConfigError {
    fn from(err: CredentialsError) -> DescribeEndpointConfigError {
        DescribeEndpointConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointConfigError {
    fn from(err: HttpDispatchError) -> DescribeEndpointConfigError {
        DescribeEndpointConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEndpointConfigError {
    fn from(err: io::Error) -> DescribeEndpointConfigError {
        DescribeEndpointConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEndpointConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointConfigError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointConfigError::Validation(ref cause) => cause,
            DescribeEndpointConfigError::Credentials(ref err) => err.description(),
            DescribeEndpointConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEndpointConfigError::ParseError(ref cause) => cause,
            DescribeEndpointConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum DescribeHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl DescribeHyperParameterTuningJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeHyperParameterTuningJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return DescribeHyperParameterTuningJobError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeHyperParameterTuningJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeHyperParameterTuningJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeHyperParameterTuningJobError {
    fn from(err: serde_json::error::Error) -> DescribeHyperParameterTuningJobError {
        DescribeHyperParameterTuningJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeHyperParameterTuningJobError {
    fn from(err: CredentialsError) -> DescribeHyperParameterTuningJobError {
        DescribeHyperParameterTuningJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHyperParameterTuningJobError {
    fn from(err: HttpDispatchError) -> DescribeHyperParameterTuningJobError {
        DescribeHyperParameterTuningJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHyperParameterTuningJobError {
    fn from(err: io::Error) -> DescribeHyperParameterTuningJobError {
        DescribeHyperParameterTuningJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHyperParameterTuningJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHyperParameterTuningJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeHyperParameterTuningJobError::ResourceNotFound(ref cause) => cause,
            DescribeHyperParameterTuningJobError::Validation(ref cause) => cause,
            DescribeHyperParameterTuningJobError::Credentials(ref err) => err.description(),
            DescribeHyperParameterTuningJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHyperParameterTuningJobError::ParseError(ref cause) => cause,
            DescribeHyperParameterTuningJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeModel
#[derive(Debug, PartialEq)]
pub enum DescribeModelError {
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

impl DescribeModelError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeModelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DescribeModelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeModelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeModelError {
    fn from(err: serde_json::error::Error) -> DescribeModelError {
        DescribeModelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeModelError {
    fn from(err: CredentialsError) -> DescribeModelError {
        DescribeModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeModelError {
    fn from(err: HttpDispatchError) -> DescribeModelError {
        DescribeModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeModelError {
    fn from(err: io::Error) -> DescribeModelError {
        DescribeModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeModelError {
    fn description(&self) -> &str {
        match *self {
            DescribeModelError::Validation(ref cause) => cause,
            DescribeModelError::Credentials(ref err) => err.description(),
            DescribeModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeModelError::ParseError(ref cause) => cause,
            DescribeModelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeNotebookInstance
#[derive(Debug, PartialEq)]
pub enum DescribeNotebookInstanceError {
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

impl DescribeNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DescribeNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> DescribeNotebookInstanceError {
        DescribeNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNotebookInstanceError {
    fn from(err: CredentialsError) -> DescribeNotebookInstanceError {
        DescribeNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotebookInstanceError {
    fn from(err: HttpDispatchError) -> DescribeNotebookInstanceError {
        DescribeNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotebookInstanceError {
    fn from(err: io::Error) -> DescribeNotebookInstanceError {
        DescribeNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotebookInstanceError::Validation(ref cause) => cause,
            DescribeNotebookInstanceError::Credentials(ref err) => err.description(),
            DescribeNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotebookInstanceError::ParseError(ref cause) => cause,
            DescribeNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum DescribeNotebookInstanceLifecycleConfigError {
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

impl DescribeNotebookInstanceLifecycleConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeNotebookInstanceLifecycleConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return DescribeNotebookInstanceLifecycleConfigError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeNotebookInstanceLifecycleConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeNotebookInstanceLifecycleConfigError {
    fn from(err: serde_json::error::Error) -> DescribeNotebookInstanceLifecycleConfigError {
        DescribeNotebookInstanceLifecycleConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNotebookInstanceLifecycleConfigError {
    fn from(err: CredentialsError) -> DescribeNotebookInstanceLifecycleConfigError {
        DescribeNotebookInstanceLifecycleConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotebookInstanceLifecycleConfigError {
    fn from(err: HttpDispatchError) -> DescribeNotebookInstanceLifecycleConfigError {
        DescribeNotebookInstanceLifecycleConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotebookInstanceLifecycleConfigError {
    fn from(err: io::Error) -> DescribeNotebookInstanceLifecycleConfigError {
        DescribeNotebookInstanceLifecycleConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotebookInstanceLifecycleConfigError::Validation(ref cause) => cause,
            DescribeNotebookInstanceLifecycleConfigError::Credentials(ref err) => err.description(),
            DescribeNotebookInstanceLifecycleConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotebookInstanceLifecycleConfigError::ParseError(ref cause) => cause,
            DescribeNotebookInstanceLifecycleConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeTrainingJob
#[derive(Debug, PartialEq)]
pub enum DescribeTrainingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl DescribeTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeTrainingJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return DescribeTrainingJobError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeTrainingJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeTrainingJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeTrainingJobError {
    fn from(err: serde_json::error::Error) -> DescribeTrainingJobError {
        DescribeTrainingJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrainingJobError {
    fn from(err: CredentialsError) -> DescribeTrainingJobError {
        DescribeTrainingJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrainingJobError {
    fn from(err: HttpDispatchError) -> DescribeTrainingJobError {
        DescribeTrainingJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTrainingJobError {
    fn from(err: io::Error) -> DescribeTrainingJobError {
        DescribeTrainingJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTrainingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrainingJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrainingJobError::ResourceNotFound(ref cause) => cause,
            DescribeTrainingJobError::Validation(ref cause) => cause,
            DescribeTrainingJobError::Credentials(ref err) => err.description(),
            DescribeTrainingJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTrainingJobError::ParseError(ref cause) => cause,
            DescribeTrainingJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeTransformJob
#[derive(Debug, PartialEq)]
pub enum DescribeTransformJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl DescribeTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeTransformJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return DescribeTransformJobError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeTransformJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeTransformJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeTransformJobError {
    fn from(err: serde_json::error::Error) -> DescribeTransformJobError {
        DescribeTransformJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTransformJobError {
    fn from(err: CredentialsError) -> DescribeTransformJobError {
        DescribeTransformJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTransformJobError {
    fn from(err: HttpDispatchError) -> DescribeTransformJobError {
        DescribeTransformJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTransformJobError {
    fn from(err: io::Error) -> DescribeTransformJobError {
        DescribeTransformJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTransformJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTransformJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeTransformJobError::ResourceNotFound(ref cause) => cause,
            DescribeTransformJobError::Validation(ref cause) => cause,
            DescribeTransformJobError::Credentials(ref err) => err.description(),
            DescribeTransformJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTransformJobError::ParseError(ref cause) => cause,
            DescribeTransformJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListEndpointConfigs
#[derive(Debug, PartialEq)]
pub enum ListEndpointConfigsError {
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

impl ListEndpointConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListEndpointConfigsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListEndpointConfigsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListEndpointConfigsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEndpointConfigsError {
    fn from(err: serde_json::error::Error) -> ListEndpointConfigsError {
        ListEndpointConfigsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEndpointConfigsError {
    fn from(err: CredentialsError) -> ListEndpointConfigsError {
        ListEndpointConfigsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEndpointConfigsError {
    fn from(err: HttpDispatchError) -> ListEndpointConfigsError {
        ListEndpointConfigsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEndpointConfigsError {
    fn from(err: io::Error) -> ListEndpointConfigsError {
        ListEndpointConfigsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEndpointConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEndpointConfigsError {
    fn description(&self) -> &str {
        match *self {
            ListEndpointConfigsError::Validation(ref cause) => cause,
            ListEndpointConfigsError::Credentials(ref err) => err.description(),
            ListEndpointConfigsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEndpointConfigsError::ParseError(ref cause) => cause,
            ListEndpointConfigsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListEndpoints
#[derive(Debug, PartialEq)]
pub enum ListEndpointsError {
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

impl ListEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListEndpointsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListEndpointsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListEndpointsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEndpointsError {
    fn from(err: serde_json::error::Error) -> ListEndpointsError {
        ListEndpointsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEndpointsError {
    fn from(err: CredentialsError) -> ListEndpointsError {
        ListEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEndpointsError {
    fn from(err: HttpDispatchError) -> ListEndpointsError {
        ListEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEndpointsError {
    fn from(err: io::Error) -> ListEndpointsError {
        ListEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEndpointsError {
    fn description(&self) -> &str {
        match *self {
            ListEndpointsError::Validation(ref cause) => cause,
            ListEndpointsError::Credentials(ref err) => err.description(),
            ListEndpointsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListEndpointsError::ParseError(ref cause) => cause,
            ListEndpointsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListHyperParameterTuningJobs
#[derive(Debug, PartialEq)]
pub enum ListHyperParameterTuningJobsError {
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

impl ListHyperParameterTuningJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListHyperParameterTuningJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListHyperParameterTuningJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListHyperParameterTuningJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListHyperParameterTuningJobsError {
    fn from(err: serde_json::error::Error) -> ListHyperParameterTuningJobsError {
        ListHyperParameterTuningJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListHyperParameterTuningJobsError {
    fn from(err: CredentialsError) -> ListHyperParameterTuningJobsError {
        ListHyperParameterTuningJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListHyperParameterTuningJobsError {
    fn from(err: HttpDispatchError) -> ListHyperParameterTuningJobsError {
        ListHyperParameterTuningJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListHyperParameterTuningJobsError {
    fn from(err: io::Error) -> ListHyperParameterTuningJobsError {
        ListHyperParameterTuningJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListHyperParameterTuningJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHyperParameterTuningJobsError {
    fn description(&self) -> &str {
        match *self {
            ListHyperParameterTuningJobsError::Validation(ref cause) => cause,
            ListHyperParameterTuningJobsError::Credentials(ref err) => err.description(),
            ListHyperParameterTuningJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListHyperParameterTuningJobsError::ParseError(ref cause) => cause,
            ListHyperParameterTuningJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListModels
#[derive(Debug, PartialEq)]
pub enum ListModelsError {
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

impl ListModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListModelsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListModelsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListModelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListModelsError {
    fn from(err: serde_json::error::Error) -> ListModelsError {
        ListModelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListModelsError {
    fn from(err: CredentialsError) -> ListModelsError {
        ListModelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListModelsError {
    fn from(err: HttpDispatchError) -> ListModelsError {
        ListModelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListModelsError {
    fn from(err: io::Error) -> ListModelsError {
        ListModelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListModelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListModelsError {
    fn description(&self) -> &str {
        match *self {
            ListModelsError::Validation(ref cause) => cause,
            ListModelsError::Credentials(ref err) => err.description(),
            ListModelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListModelsError::ParseError(ref cause) => cause,
            ListModelsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListNotebookInstanceLifecycleConfigs
#[derive(Debug, PartialEq)]
pub enum ListNotebookInstanceLifecycleConfigsError {
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

impl ListNotebookInstanceLifecycleConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListNotebookInstanceLifecycleConfigsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListNotebookInstanceLifecycleConfigsError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return ListNotebookInstanceLifecycleConfigsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListNotebookInstanceLifecycleConfigsError {
    fn from(err: serde_json::error::Error) -> ListNotebookInstanceLifecycleConfigsError {
        ListNotebookInstanceLifecycleConfigsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListNotebookInstanceLifecycleConfigsError {
    fn from(err: CredentialsError) -> ListNotebookInstanceLifecycleConfigsError {
        ListNotebookInstanceLifecycleConfigsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListNotebookInstanceLifecycleConfigsError {
    fn from(err: HttpDispatchError) -> ListNotebookInstanceLifecycleConfigsError {
        ListNotebookInstanceLifecycleConfigsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListNotebookInstanceLifecycleConfigsError {
    fn from(err: io::Error) -> ListNotebookInstanceLifecycleConfigsError {
        ListNotebookInstanceLifecycleConfigsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListNotebookInstanceLifecycleConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNotebookInstanceLifecycleConfigsError {
    fn description(&self) -> &str {
        match *self {
            ListNotebookInstanceLifecycleConfigsError::Validation(ref cause) => cause,
            ListNotebookInstanceLifecycleConfigsError::Credentials(ref err) => err.description(),
            ListNotebookInstanceLifecycleConfigsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListNotebookInstanceLifecycleConfigsError::ParseError(ref cause) => cause,
            ListNotebookInstanceLifecycleConfigsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListNotebookInstances
#[derive(Debug, PartialEq)]
pub enum ListNotebookInstancesError {
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

impl ListNotebookInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListNotebookInstancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListNotebookInstancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListNotebookInstancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListNotebookInstancesError {
    fn from(err: serde_json::error::Error) -> ListNotebookInstancesError {
        ListNotebookInstancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListNotebookInstancesError {
    fn from(err: CredentialsError) -> ListNotebookInstancesError {
        ListNotebookInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListNotebookInstancesError {
    fn from(err: HttpDispatchError) -> ListNotebookInstancesError {
        ListNotebookInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListNotebookInstancesError {
    fn from(err: io::Error) -> ListNotebookInstancesError {
        ListNotebookInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListNotebookInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNotebookInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListNotebookInstancesError::Validation(ref cause) => cause,
            ListNotebookInstancesError::Credentials(ref err) => err.description(),
            ListNotebookInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListNotebookInstancesError::ParseError(ref cause) => cause,
            ListNotebookInstancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
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

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListTagsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::ParseError(err.description().to_string())
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
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::ParseError(ref cause) => cause,
            ListTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTrainingJobs
#[derive(Debug, PartialEq)]
pub enum ListTrainingJobsError {
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

impl ListTrainingJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTrainingJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListTrainingJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTrainingJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTrainingJobsError {
    fn from(err: serde_json::error::Error) -> ListTrainingJobsError {
        ListTrainingJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTrainingJobsError {
    fn from(err: CredentialsError) -> ListTrainingJobsError {
        ListTrainingJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrainingJobsError {
    fn from(err: HttpDispatchError) -> ListTrainingJobsError {
        ListTrainingJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrainingJobsError {
    fn from(err: io::Error) -> ListTrainingJobsError {
        ListTrainingJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrainingJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrainingJobsError {
    fn description(&self) -> &str {
        match *self {
            ListTrainingJobsError::Validation(ref cause) => cause,
            ListTrainingJobsError::Credentials(ref err) => err.description(),
            ListTrainingJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTrainingJobsError::ParseError(ref cause) => cause,
            ListTrainingJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTrainingJobsForHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum ListTrainingJobsForHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl ListTrainingJobsForHyperParameterTuningJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> ListTrainingJobsForHyperParameterTuningJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return ListTrainingJobsForHyperParameterTuningJobError::ResourceNotFound(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return ListTrainingJobsForHyperParameterTuningJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return ListTrainingJobsForHyperParameterTuningJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTrainingJobsForHyperParameterTuningJobError {
    fn from(err: serde_json::error::Error) -> ListTrainingJobsForHyperParameterTuningJobError {
        ListTrainingJobsForHyperParameterTuningJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTrainingJobsForHyperParameterTuningJobError {
    fn from(err: CredentialsError) -> ListTrainingJobsForHyperParameterTuningJobError {
        ListTrainingJobsForHyperParameterTuningJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTrainingJobsForHyperParameterTuningJobError {
    fn from(err: HttpDispatchError) -> ListTrainingJobsForHyperParameterTuningJobError {
        ListTrainingJobsForHyperParameterTuningJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTrainingJobsForHyperParameterTuningJobError {
    fn from(err: io::Error) -> ListTrainingJobsForHyperParameterTuningJobError {
        ListTrainingJobsForHyperParameterTuningJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTrainingJobsForHyperParameterTuningJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrainingJobsForHyperParameterTuningJobError {
    fn description(&self) -> &str {
        match *self {
            ListTrainingJobsForHyperParameterTuningJobError::ResourceNotFound(ref cause) => cause,
            ListTrainingJobsForHyperParameterTuningJobError::Validation(ref cause) => cause,
            ListTrainingJobsForHyperParameterTuningJobError::Credentials(ref err) => {
                err.description()
            }
            ListTrainingJobsForHyperParameterTuningJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTrainingJobsForHyperParameterTuningJobError::ParseError(ref cause) => cause,
            ListTrainingJobsForHyperParameterTuningJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTransformJobs
#[derive(Debug, PartialEq)]
pub enum ListTransformJobsError {
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

impl ListTransformJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTransformJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return ListTransformJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTransformJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTransformJobsError {
    fn from(err: serde_json::error::Error) -> ListTransformJobsError {
        ListTransformJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTransformJobsError {
    fn from(err: CredentialsError) -> ListTransformJobsError {
        ListTransformJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTransformJobsError {
    fn from(err: HttpDispatchError) -> ListTransformJobsError {
        ListTransformJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTransformJobsError {
    fn from(err: io::Error) -> ListTransformJobsError {
        ListTransformJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTransformJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTransformJobsError {
    fn description(&self) -> &str {
        match *self {
            ListTransformJobsError::Validation(ref cause) => cause,
            ListTransformJobsError::Credentials(ref err) => err.description(),
            ListTransformJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTransformJobsError::ParseError(ref cause) => cause,
            ListTransformJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartNotebookInstance
#[derive(Debug, PartialEq)]
pub enum StartNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl StartNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> StartNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return StartNotebookInstanceError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> StartNotebookInstanceError {
        StartNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartNotebookInstanceError {
    fn from(err: CredentialsError) -> StartNotebookInstanceError {
        StartNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartNotebookInstanceError {
    fn from(err: HttpDispatchError) -> StartNotebookInstanceError {
        StartNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartNotebookInstanceError {
    fn from(err: io::Error) -> StartNotebookInstanceError {
        StartNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            StartNotebookInstanceError::ResourceLimitExceeded(ref cause) => cause,
            StartNotebookInstanceError::Validation(ref cause) => cause,
            StartNotebookInstanceError::Credentials(ref err) => err.description(),
            StartNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartNotebookInstanceError::ParseError(ref cause) => cause,
            StartNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum StopHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl StopHyperParameterTuningJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopHyperParameterTuningJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return StopHyperParameterTuningJobError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StopHyperParameterTuningJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopHyperParameterTuningJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopHyperParameterTuningJobError {
    fn from(err: serde_json::error::Error) -> StopHyperParameterTuningJobError {
        StopHyperParameterTuningJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopHyperParameterTuningJobError {
    fn from(err: CredentialsError) -> StopHyperParameterTuningJobError {
        StopHyperParameterTuningJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopHyperParameterTuningJobError {
    fn from(err: HttpDispatchError) -> StopHyperParameterTuningJobError {
        StopHyperParameterTuningJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopHyperParameterTuningJobError {
    fn from(err: io::Error) -> StopHyperParameterTuningJobError {
        StopHyperParameterTuningJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopHyperParameterTuningJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopHyperParameterTuningJobError {
    fn description(&self) -> &str {
        match *self {
            StopHyperParameterTuningJobError::ResourceNotFound(ref cause) => cause,
            StopHyperParameterTuningJobError::Validation(ref cause) => cause,
            StopHyperParameterTuningJobError::Credentials(ref err) => err.description(),
            StopHyperParameterTuningJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopHyperParameterTuningJobError::ParseError(ref cause) => cause,
            StopHyperParameterTuningJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopNotebookInstance
#[derive(Debug, PartialEq)]
pub enum StopNotebookInstanceError {
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

impl StopNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> StopNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ValidationException" => {
                    return StopNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> StopNotebookInstanceError {
        StopNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopNotebookInstanceError {
    fn from(err: CredentialsError) -> StopNotebookInstanceError {
        StopNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopNotebookInstanceError {
    fn from(err: HttpDispatchError) -> StopNotebookInstanceError {
        StopNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopNotebookInstanceError {
    fn from(err: io::Error) -> StopNotebookInstanceError {
        StopNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            StopNotebookInstanceError::Validation(ref cause) => cause,
            StopNotebookInstanceError::Credentials(ref err) => err.description(),
            StopNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopNotebookInstanceError::ParseError(ref cause) => cause,
            StopNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopTrainingJob
#[derive(Debug, PartialEq)]
pub enum StopTrainingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl StopTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopTrainingJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return StopTrainingJobError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopTrainingJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopTrainingJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopTrainingJobError {
    fn from(err: serde_json::error::Error) -> StopTrainingJobError {
        StopTrainingJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTrainingJobError {
    fn from(err: CredentialsError) -> StopTrainingJobError {
        StopTrainingJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTrainingJobError {
    fn from(err: HttpDispatchError) -> StopTrainingJobError {
        StopTrainingJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTrainingJobError {
    fn from(err: io::Error) -> StopTrainingJobError {
        StopTrainingJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTrainingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTrainingJobError {
    fn description(&self) -> &str {
        match *self {
            StopTrainingJobError::ResourceNotFound(ref cause) => cause,
            StopTrainingJobError::Validation(ref cause) => cause,
            StopTrainingJobError::Credentials(ref err) => err.description(),
            StopTrainingJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopTrainingJobError::ParseError(ref cause) => cause,
            StopTrainingJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopTransformJob
#[derive(Debug, PartialEq)]
pub enum StopTransformJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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

impl StopTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopTransformJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceNotFound" => {
                    return StopTransformJobError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopTransformJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopTransformJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopTransformJobError {
    fn from(err: serde_json::error::Error) -> StopTransformJobError {
        StopTransformJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTransformJobError {
    fn from(err: CredentialsError) -> StopTransformJobError {
        StopTransformJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTransformJobError {
    fn from(err: HttpDispatchError) -> StopTransformJobError {
        StopTransformJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTransformJobError {
    fn from(err: io::Error) -> StopTransformJobError {
        StopTransformJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTransformJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTransformJobError {
    fn description(&self) -> &str {
        match *self {
            StopTransformJobError::ResourceNotFound(ref cause) => cause,
            StopTransformJobError::Validation(ref cause) => cause,
            StopTransformJobError::Credentials(ref err) => err.description(),
            StopTransformJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopTransformJobError::ParseError(ref cause) => cause,
            StopTransformJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl UpdateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return UpdateEndpointError::ResourceLimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEndpointError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointError {
        UpdateEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointError {
    fn from(err: CredentialsError) -> UpdateEndpointError {
        UpdateEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointError {
    fn from(err: HttpDispatchError) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointError {
    fn from(err: io::Error) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointError::ResourceLimitExceeded(ref cause) => cause,
            UpdateEndpointError::Validation(ref cause) => cause,
            UpdateEndpointError::Credentials(ref err) => err.description(),
            UpdateEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateEndpointError::ParseError(ref cause) => cause,
            UpdateEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEndpointWeightsAndCapacities
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointWeightsAndCapacitiesError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl UpdateEndpointWeightsAndCapacitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEndpointWeightsAndCapacitiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return UpdateEndpointWeightsAndCapacitiesError::ResourceLimitExceeded(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return UpdateEndpointWeightsAndCapacitiesError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return UpdateEndpointWeightsAndCapacitiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEndpointWeightsAndCapacitiesError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointWeightsAndCapacitiesError {
        UpdateEndpointWeightsAndCapacitiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointWeightsAndCapacitiesError {
    fn from(err: CredentialsError) -> UpdateEndpointWeightsAndCapacitiesError {
        UpdateEndpointWeightsAndCapacitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointWeightsAndCapacitiesError {
    fn from(err: HttpDispatchError) -> UpdateEndpointWeightsAndCapacitiesError {
        UpdateEndpointWeightsAndCapacitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointWeightsAndCapacitiesError {
    fn from(err: io::Error) -> UpdateEndpointWeightsAndCapacitiesError {
        UpdateEndpointWeightsAndCapacitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointWeightsAndCapacitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointWeightsAndCapacitiesError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointWeightsAndCapacitiesError::ResourceLimitExceeded(ref cause) => cause,
            UpdateEndpointWeightsAndCapacitiesError::Validation(ref cause) => cause,
            UpdateEndpointWeightsAndCapacitiesError::Credentials(ref err) => err.description(),
            UpdateEndpointWeightsAndCapacitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEndpointWeightsAndCapacitiesError::ParseError(ref cause) => cause,
            UpdateEndpointWeightsAndCapacitiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateNotebookInstance
#[derive(Debug, PartialEq)]
pub enum UpdateNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl UpdateNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateNotebookInstanceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return UpdateNotebookInstanceError::ResourceLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateNotebookInstanceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateNotebookInstanceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateNotebookInstanceError {
    fn from(err: serde_json::error::Error) -> UpdateNotebookInstanceError {
        UpdateNotebookInstanceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNotebookInstanceError {
    fn from(err: CredentialsError) -> UpdateNotebookInstanceError {
        UpdateNotebookInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNotebookInstanceError {
    fn from(err: HttpDispatchError) -> UpdateNotebookInstanceError {
        UpdateNotebookInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNotebookInstanceError {
    fn from(err: io::Error) -> UpdateNotebookInstanceError {
        UpdateNotebookInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {
            UpdateNotebookInstanceError::ResourceLimitExceeded(ref cause) => cause,
            UpdateNotebookInstanceError::Validation(ref cause) => cause,
            UpdateNotebookInstanceError::Credentials(ref err) => err.description(),
            UpdateNotebookInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNotebookInstanceError::ParseError(ref cause) => cause,
            UpdateNotebookInstanceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum UpdateNotebookInstanceLifecycleConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
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

impl UpdateNotebookInstanceLifecycleConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateNotebookInstanceLifecycleConfigError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ResourceLimitExceeded" => {
                    return UpdateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return UpdateNotebookInstanceLifecycleConfigError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return UpdateNotebookInstanceLifecycleConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateNotebookInstanceLifecycleConfigError {
    fn from(err: serde_json::error::Error) -> UpdateNotebookInstanceLifecycleConfigError {
        UpdateNotebookInstanceLifecycleConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNotebookInstanceLifecycleConfigError {
    fn from(err: CredentialsError) -> UpdateNotebookInstanceLifecycleConfigError {
        UpdateNotebookInstanceLifecycleConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNotebookInstanceLifecycleConfigError {
    fn from(err: HttpDispatchError) -> UpdateNotebookInstanceLifecycleConfigError {
        UpdateNotebookInstanceLifecycleConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNotebookInstanceLifecycleConfigError {
    fn from(err: io::Error) -> UpdateNotebookInstanceLifecycleConfigError {
        UpdateNotebookInstanceLifecycleConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {
            UpdateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(ref cause) => cause,
            UpdateNotebookInstanceLifecycleConfigError::Validation(ref cause) => cause,
            UpdateNotebookInstanceLifecycleConfigError::Credentials(ref err) => err.description(),
            UpdateNotebookInstanceLifecycleConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNotebookInstanceLifecycleConfigError::ParseError(ref cause) => cause,
            UpdateNotebookInstanceLifecycleConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the SageMaker API. SageMaker clients implement this trait.
pub trait SageMaker {
    /// <p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, models, endpoint configurations, and endpoints. </p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS i an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> RusotoFuture<CreateEndpointOutput, CreateEndpointError>;

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> RusotoFuture<CreateEndpointConfigOutput, CreateEndpointConfigError>;

    /// <p>Starts a hyperparameter tuning job.</p>
    fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> RusotoFuture<CreateHyperParameterTuningJobResponse, CreateHyperParameterTuningJobError>;

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe one or more containers. For each container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model into production. </p> <p>Use this API to create a model only if you want to use Amazon SageMaker hosting services. To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. </p> <p>Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter. </p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    fn create_model(
        &self,
        input: CreateModelInput,
    ) -> RusotoFuture<CreateModelOutput, CreateModelError>;

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN).</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> RusotoFuture<CreateNotebookInstanceOutput, CreateNotebookInstanceError>;

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        CreateNotebookInstanceLifecycleConfigOutput,
        CreateNotebookInstanceLifecycleConfigError,
    >;

    /// <p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page. </p>
    fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> RusotoFuture<
        CreatePresignedNotebookInstanceUrlOutput,
        CreatePresignedNotebookInstanceUrlError,
    >;

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a deep learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to influence the quality of the final model. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - Sets a duration for training. Use this parameter to cap model training costs. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> RusotoFuture<CreateTrainingJobResponse, CreateTrainingJobError>;

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an AWS Region in an AWS account.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p> For more information about how batch transformation works Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">How It Works</a>. </p>
    fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> RusotoFuture<CreateTransformJobResponse, CreateTransformJobError>;

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError>;

    /// <p>Deletes an endpoint configuration. The <code>DeleteEndpointConfig</code> API deletes only the specified configuration. It does not delete endpoints created using the configuration. </p>
    fn delete_endpoint_config(
        &self,
        input: DeleteEndpointConfigInput,
    ) -> RusotoFuture<(), DeleteEndpointConfigError>;

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    fn delete_model(&self, input: DeleteModelInput) -> RusotoFuture<(), DeleteModelError>;

    /// <p><p> Deletes an Amazon SageMaker notebook instance. Before you can delete a notebook instance, you must call the <code>StopNotebookInstance</code> API. </p> <important> <p>When you delete a notebook instance, you lose all of your data. Amazon SageMaker removes the ML compute instance, and deletes the ML storage volume and the network interface associated with the notebook instance. </p> </important></p>
    fn delete_notebook_instance(
        &self,
        input: DeleteNotebookInstanceInput,
    ) -> RusotoFuture<(), DeleteNotebookInstanceError>;

    /// <p>Deletes a notebook instance lifecycle configuration.</p>
    fn delete_notebook_instance_lifecycle_config(
        &self,
        input: DeleteNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<(), DeleteNotebookInstanceLifecycleConfigError>;

    /// <p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource's tags, use the <code>ListTags</code> API. </p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError>;

    /// <p>Returns the description of an endpoint.</p>
    fn describe_endpoint(
        &self,
        input: DescribeEndpointInput,
    ) -> RusotoFuture<DescribeEndpointOutput, DescribeEndpointError>;

    /// <p>Returns the description of an endpoint configuration created using the <code>CreateEndpointConfig</code> API.</p>
    fn describe_endpoint_config(
        &self,
        input: DescribeEndpointConfigInput,
    ) -> RusotoFuture<DescribeEndpointConfigOutput, DescribeEndpointConfigError>;

    /// <p>Gets a description of a hyperparameter tuning job.</p>
    fn describe_hyper_parameter_tuning_job(
        &self,
        input: DescribeHyperParameterTuningJobRequest,
    ) -> RusotoFuture<DescribeHyperParameterTuningJobResponse, DescribeHyperParameterTuningJobError>;

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> RusotoFuture<DescribeModelOutput, DescribeModelError>;

    /// <p>Returns information about a notebook instance.</p>
    fn describe_notebook_instance(
        &self,
        input: DescribeNotebookInstanceInput,
    ) -> RusotoFuture<DescribeNotebookInstanceOutput, DescribeNotebookInstanceError>;

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        DescribeNotebookInstanceLifecycleConfigOutput,
        DescribeNotebookInstanceLifecycleConfigError,
    >;

    /// <p>Returns information about a training job.</p>
    fn describe_training_job(
        &self,
        input: DescribeTrainingJobRequest,
    ) -> RusotoFuture<DescribeTrainingJobResponse, DescribeTrainingJobError>;

    /// <p>Returns information about a transform job.</p>
    fn describe_transform_job(
        &self,
        input: DescribeTransformJobRequest,
    ) -> RusotoFuture<DescribeTransformJobResponse, DescribeTransformJobError>;

    /// <p>Lists endpoint configurations.</p>
    fn list_endpoint_configs(
        &self,
        input: ListEndpointConfigsInput,
    ) -> RusotoFuture<ListEndpointConfigsOutput, ListEndpointConfigsError>;

    /// <p>Lists endpoints.</p>
    fn list_endpoints(
        &self,
        input: ListEndpointsInput,
    ) -> RusotoFuture<ListEndpointsOutput, ListEndpointsError>;

    /// <p>Gets a list of <a>HyperParameterTuningJobSummary</a> objects that describe the hyperparameter tuning jobs launched in your account.</p>
    fn list_hyper_parameter_tuning_jobs(
        &self,
        input: ListHyperParameterTuningJobsRequest,
    ) -> RusotoFuture<ListHyperParameterTuningJobsResponse, ListHyperParameterTuningJobsError>;

    /// <p>Lists models created with the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
    fn list_models(
        &self,
        input: ListModelsInput,
    ) -> RusotoFuture<ListModelsOutput, ListModelsError>;

    /// <p>Lists notebook instance lifestyle configurations created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    fn list_notebook_instance_lifecycle_configs(
        &self,
        input: ListNotebookInstanceLifecycleConfigsInput,
    ) -> RusotoFuture<
        ListNotebookInstanceLifecycleConfigsOutput,
        ListNotebookInstanceLifecycleConfigsError,
    >;

    /// <p>Returns a list of the Amazon SageMaker notebook instances in the requester's account in an AWS Region. </p>
    fn list_notebook_instances(
        &self,
        input: ListNotebookInstancesInput,
    ) -> RusotoFuture<ListNotebookInstancesOutput, ListNotebookInstancesError>;

    /// <p>Returns the tags for the specified Amazon SageMaker resource.</p>
    fn list_tags(&self, input: ListTagsInput) -> RusotoFuture<ListTagsOutput, ListTagsError>;

    /// <p>Lists training jobs.</p>
    fn list_training_jobs(
        &self,
        input: ListTrainingJobsRequest,
    ) -> RusotoFuture<ListTrainingJobsResponse, ListTrainingJobsError>;

    /// <p>Gets a list of <a>TrainingJobSummary</a> objects that describe the training jobs that a hyperparameter tuning job launched.</p>
    fn list_training_jobs_for_hyper_parameter_tuning_job(
        &self,
        input: ListTrainingJobsForHyperParameterTuningJobRequest,
    ) -> RusotoFuture<
        ListTrainingJobsForHyperParameterTuningJobResponse,
        ListTrainingJobsForHyperParameterTuningJobError,
    >;

    /// <p>Lists transform jobs.</p>
    fn list_transform_jobs(
        &self,
        input: ListTransformJobsRequest,
    ) -> RusotoFuture<ListTransformJobsResponse, ListTransformJobsError>;

    /// <p>Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, Amazon SageMaker sets the notebook instance status to <code>InService</code>. A notebook instance's status must be <code>InService</code> before you can connect to your Jupyter notebook. </p>
    fn start_notebook_instance(
        &self,
        input: StartNotebookInstanceInput,
    ) -> RusotoFuture<(), StartNotebookInstanceError>;

    /// <p>Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched.</p> <p>All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). All data that the training jobs write to Amazon CloudWatch Logs are still available in CloudWatch. After the tuning job moves to the <code>Stopped</code> state, it releases all reserved resources for the tuning job.</p>
    fn stop_hyper_parameter_tuning_job(
        &self,
        input: StopHyperParameterTuningJobRequest,
    ) -> RusotoFuture<(), StopHyperParameterTuningJobError>;

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. </p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> RusotoFuture<(), StopNotebookInstanceError>;

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>Training algorithms provided by Amazon SageMaker save the intermediate results of a model training job. This intermediate data is a valid model artifact. You can use the model artifacts that are saved when Amazon SageMaker stops a training job to create a model. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> RusotoFuture<(), StopTrainingJobError>;

    /// <p>Stops a transform job.</p> <p>When Amazon SageMaker receives a <code>StopTransformJob</code> request, the status of the job changes to <code>Stopping</code>. After Amazon SageMaker stops the job, the status is set to <code>Stopped</code>. When you stop a transform job before it is completed, Amazon SageMaker doesn't store the job's output in Amazon S3.</p>
    fn stop_transform_job(
        &self,
        input: StopTransformJobRequest,
    ) -> RusotoFuture<(), StopTransformJobError>;

    /// <p><p> Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You cannot update an endpoint with the current <code>EndpointConfig</code>. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> RusotoFuture<UpdateEndpointOutput, UpdateEndpointError>;

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> RusotoFuture<
        UpdateEndpointWeightsAndCapacitiesOutput,
        UpdateEndpointWeightsAndCapacitiesError,
    >;

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements. You can also update the VPC security groups.</p>
    fn update_notebook_instance(
        &self,
        input: UpdateNotebookInstanceInput,
    ) -> RusotoFuture<UpdateNotebookInstanceOutput, UpdateNotebookInstanceError>;

    /// <p>Updates a notebook instance lifecycle configuration created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    fn update_notebook_instance_lifecycle_config(
        &self,
        input: UpdateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        UpdateNotebookInstanceLifecycleConfigOutput,
        UpdateNotebookInstanceLifecycleConfigError,
    >;
}
/// A client for the SageMaker API.
pub struct SageMakerClient {
    client: Client,
    region: region::Region,
}

impl SageMakerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SageMakerClient {
        SageMakerClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SageMakerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        SageMakerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl SageMaker for SageMakerClient {
    /// <p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, models, endpoint configurations, and endpoints. </p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.AddTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS i an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> RusotoFuture<CreateEndpointOutput, CreateEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEndpointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> RusotoFuture<CreateEndpointConfigOutput, CreateEndpointConfigError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEndpointConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateEndpointConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts a hyperparameter tuning job.</p>
    fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> RusotoFuture<CreateHyperParameterTuningJobResponse, CreateHyperParameterTuningJobError>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateHyperParameterTuningJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateHyperParameterTuningJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe one or more containers. For each container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model into production. </p> <p>Use this API to create a model only if you want to use Amazon SageMaker hosting services. To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. </p> <p>Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter. </p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    fn create_model(
        &self,
        input: CreateModelInput,
    ) -> RusotoFuture<CreateModelOutput, CreateModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN).</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> RusotoFuture<CreateNotebookInstanceOutput, CreateNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateNotebookInstanceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateNotebookInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        CreateNotebookInstanceLifecycleConfigOutput,
        CreateNotebookInstanceLifecycleConfigError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.CreateNotebookInstanceLifecycleConfig",
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

                    serde_json::from_str::<CreateNotebookInstanceLifecycleConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateNotebookInstanceLifecycleConfigError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page. </p>
    fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> RusotoFuture<
        CreatePresignedNotebookInstanceUrlOutput,
        CreatePresignedNotebookInstanceUrlError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.CreatePresignedNotebookInstanceUrl",
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

                    serde_json::from_str::<CreatePresignedNotebookInstanceUrlOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePresignedNotebookInstanceUrlError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a deep learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to influence the quality of the final model. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - Sets a duration for training. Use this parameter to cap model training costs. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> RusotoFuture<CreateTrainingJobResponse, CreateTrainingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTrainingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTrainingJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTrainingJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an AWS Region in an AWS account.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p> For more information about how batch transformation works Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">How It Works</a>. </p>
    fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> RusotoFuture<CreateTransformJobResponse, CreateTransformJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTransformJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTransformJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTransformJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpoint");
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
                        .and_then(|response| Err(DeleteEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an endpoint configuration. The <code>DeleteEndpointConfig</code> API deletes only the specified configuration. It does not delete endpoints created using the configuration. </p>
    fn delete_endpoint_config(
        &self,
        input: DeleteEndpointConfigInput,
    ) -> RusotoFuture<(), DeleteEndpointConfigError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteEndpointConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    fn delete_model(&self, input: DeleteModelInput) -> RusotoFuture<(), DeleteModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteModel");
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
                        .and_then(|response| Err(DeleteModelError::from_response(response))),
                )
            }
        })
    }

    /// <p><p> Deletes an Amazon SageMaker notebook instance. Before you can delete a notebook instance, you must call the <code>StopNotebookInstance</code> API. </p> <important> <p>When you delete a notebook instance, you lose all of your data. Amazon SageMaker removes the ML compute instance, and deletes the ML storage volume and the network interface associated with the notebook instance. </p> </important></p>
    fn delete_notebook_instance(
        &self,
        input: DeleteNotebookInstanceInput,
    ) -> RusotoFuture<(), DeleteNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteNotebookInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a notebook instance lifecycle configuration.</p>
    fn delete_notebook_instance_lifecycle_config(
        &self,
        input: DeleteNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<(), DeleteNotebookInstanceLifecycleConfigError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.DeleteNotebookInstanceLifecycleConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotebookInstanceLifecycleConfigError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource's tags, use the <code>ListTags</code> API. </p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the description of an endpoint.</p>
    fn describe_endpoint(
        &self,
        input: DescribeEndpointInput,
    ) -> RusotoFuture<DescribeEndpointOutput, DescribeEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEndpointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the description of an endpoint configuration created using the <code>CreateEndpointConfig</code> API.</p>
    fn describe_endpoint_config(
        &self,
        input: DescribeEndpointConfigInput,
    ) -> RusotoFuture<DescribeEndpointConfigOutput, DescribeEndpointConfigError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEndpointConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeEndpointConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a description of a hyperparameter tuning job.</p>
    fn describe_hyper_parameter_tuning_job(
        &self,
        input: DescribeHyperParameterTuningJobRequest,
    ) -> RusotoFuture<DescribeHyperParameterTuningJobResponse, DescribeHyperParameterTuningJobError>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeHyperParameterTuningJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHyperParameterTuningJobError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> RusotoFuture<DescribeModelOutput, DescribeModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a notebook instance.</p>
    fn describe_notebook_instance(
        &self,
        input: DescribeNotebookInstanceInput,
    ) -> RusotoFuture<DescribeNotebookInstanceOutput, DescribeNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeNotebookInstanceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotebookInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a>notebook-lifecycle-config</a>.</p>
    fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        DescribeNotebookInstanceLifecycleConfigOutput,
        DescribeNotebookInstanceLifecycleConfigError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.DescribeNotebookInstanceLifecycleConfig",
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

                    serde_json::from_str::<DescribeNotebookInstanceLifecycleConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotebookInstanceLifecycleConfigError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about a training job.</p>
    fn describe_training_job(
        &self,
        input: DescribeTrainingJobRequest,
    ) -> RusotoFuture<DescribeTrainingJobResponse, DescribeTrainingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTrainingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTrainingJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeTrainingJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a transform job.</p>
    fn describe_transform_job(
        &self,
        input: DescribeTransformJobRequest,
    ) -> RusotoFuture<DescribeTransformJobResponse, DescribeTransformJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTransformJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTransformJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeTransformJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists endpoint configurations.</p>
    fn list_endpoint_configs(
        &self,
        input: ListEndpointConfigsInput,
    ) -> RusotoFuture<ListEndpointConfigsOutput, ListEndpointConfigsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpointConfigs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEndpointConfigsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListEndpointConfigsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists endpoints.</p>
    fn list_endpoints(
        &self,
        input: ListEndpointsInput,
    ) -> RusotoFuture<ListEndpointsOutput, ListEndpointsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEndpointsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of <a>HyperParameterTuningJobSummary</a> objects that describe the hyperparameter tuning jobs launched in your account.</p>
    fn list_hyper_parameter_tuning_jobs(
        &self,
        input: ListHyperParameterTuningJobsRequest,
    ) -> RusotoFuture<ListHyperParameterTuningJobsResponse, ListHyperParameterTuningJobsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListHyperParameterTuningJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListHyperParameterTuningJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListHyperParameterTuningJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists models created with the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
    fn list_models(
        &self,
        input: ListModelsInput,
    ) -> RusotoFuture<ListModelsOutput, ListModelsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListModelsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListModelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists notebook instance lifestyle configurations created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    fn list_notebook_instance_lifecycle_configs(
        &self,
        input: ListNotebookInstanceLifecycleConfigsInput,
    ) -> RusotoFuture<
        ListNotebookInstanceLifecycleConfigsOutput,
        ListNotebookInstanceLifecycleConfigsError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.ListNotebookInstanceLifecycleConfigs",
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

                    serde_json::from_str::<ListNotebookInstanceLifecycleConfigsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListNotebookInstanceLifecycleConfigsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of the Amazon SageMaker notebook instances in the requester's account in an AWS Region. </p>
    fn list_notebook_instances(
        &self,
        input: ListNotebookInstancesInput,
    ) -> RusotoFuture<ListNotebookInstancesOutput, ListNotebookInstancesError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListNotebookInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListNotebookInstancesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListNotebookInstancesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the tags for the specified Amazon SageMaker resource.</p>
    fn list_tags(&self, input: ListTagsInput) -> RusotoFuture<ListTagsOutput, ListTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists training jobs.</p>
    fn list_training_jobs(
        &self,
        input: ListTrainingJobsRequest,
    ) -> RusotoFuture<ListTrainingJobsResponse, ListTrainingJobsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTrainingJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTrainingJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTrainingJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of <a>TrainingJobSummary</a> objects that describe the training jobs that a hyperparameter tuning job launched.</p>
    fn list_training_jobs_for_hyper_parameter_tuning_job(
        &self,
        input: ListTrainingJobsForHyperParameterTuningJobRequest,
    ) -> RusotoFuture<
        ListTrainingJobsForHyperParameterTuningJobResponse,
        ListTrainingJobsForHyperParameterTuningJobError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.ListTrainingJobsForHyperParameterTuningJob",
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

                    serde_json::from_str::<ListTrainingJobsForHyperParameterTuningJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTrainingJobsForHyperParameterTuningJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists transform jobs.</p>
    fn list_transform_jobs(
        &self,
        input: ListTransformJobsRequest,
    ) -> RusotoFuture<ListTransformJobsResponse, ListTransformJobsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTransformJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTransformJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTransformJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, Amazon SageMaker sets the notebook instance status to <code>InService</code>. A notebook instance's status must be <code>InService</code> before you can connect to your Jupyter notebook. </p>
    fn start_notebook_instance(
        &self,
        input: StartNotebookInstanceInput,
    ) -> RusotoFuture<(), StartNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StartNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartNotebookInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched.</p> <p>All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). All data that the training jobs write to Amazon CloudWatch Logs are still available in CloudWatch. After the tuning job moves to the <code>Stopped</code> state, it releases all reserved resources for the tuning job.</p>
    fn stop_hyper_parameter_tuning_job(
        &self,
        input: StopHyperParameterTuningJobRequest,
    ) -> RusotoFuture<(), StopHyperParameterTuningJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopHyperParameterTuningJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. </p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> RusotoFuture<(), StopNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopNotebookInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>Training algorithms provided by Amazon SageMaker save the intermediate results of a model training job. This intermediate data is a valid model artifact. You can use the model artifacts that are saved when Amazon SageMaker stops a training job to create a model. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> RusotoFuture<(), StopTrainingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTrainingJob");
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
                        .and_then(|response| Err(StopTrainingJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops a transform job.</p> <p>When Amazon SageMaker receives a <code>StopTransformJob</code> request, the status of the job changes to <code>Stopping</code>. After Amazon SageMaker stops the job, the status is set to <code>Stopped</code>. When you stop a transform job before it is completed, Amazon SageMaker doesn't store the job's output in Amazon S3.</p>
    fn stop_transform_job(
        &self,
        input: StopTransformJobRequest,
    ) -> RusotoFuture<(), StopTransformJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTransformJob");
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
                        .and_then(|response| Err(StopTransformJobError::from_response(response))),
                )
            }
        })
    }

    /// <p><p> Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You cannot update an endpoint with the current <code>EndpointConfig</code>. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> RusotoFuture<UpdateEndpointOutput, UpdateEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateEndpointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> RusotoFuture<
        UpdateEndpointWeightsAndCapacitiesOutput,
        UpdateEndpointWeightsAndCapacitiesError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.UpdateEndpointWeightsAndCapacities",
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

                    serde_json::from_str::<UpdateEndpointWeightsAndCapacitiesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEndpointWeightsAndCapacitiesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements. You can also update the VPC security groups.</p>
    fn update_notebook_instance(
        &self,
        input: UpdateNotebookInstanceInput,
    ) -> RusotoFuture<UpdateNotebookInstanceOutput, UpdateNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateNotebookInstanceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateNotebookInstanceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates a notebook instance lifecycle configuration created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    fn update_notebook_instance_lifecycle_config(
        &self,
        input: UpdateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        UpdateNotebookInstanceLifecycleConfigOutput,
        UpdateNotebookInstanceLifecycleConfigError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.UpdateNotebookInstanceLifecycleConfig",
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

                    serde_json::from_str::<UpdateNotebookInstanceLifecycleConfigOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNotebookInstanceLifecycleConfigError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
