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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
#[cfg_attr(test, derive(Serialize))]
pub struct AddTagsOutput {
    /// <p>A list of tags associated with the Amazon SageMaker resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Specifies the training algorithm to use in a <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateTrainingJob.html">CreateTrainingJob</a> request.</p> <p>For more information about algorithms provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about using your own algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmSpecification {
    /// <p>The name of the algorithm resource to use for the training job. This must be an algorithm resource that you created or subscribe to on AWS Marketplace. If you specify a value for this parameter, you can't specify a value for <code>TrainingImage</code>.</p>
    #[serde(rename = "AlgorithmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_name: Option<String>,
    /// <p>A list of metric definition objects. Each object specifies the metric name and regular expressions used to parse algorithm logs. Amazon SageMaker publishes each metric to Amazon CloudWatch.</p>
    #[serde(rename = "MetricDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,
    /// <p>The registry path of the Docker image that contains the training algorithm. For information about docker registry paths for built-in algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-algo-docker-registry-paths.html">Algorithms Provided by Amazon SageMaker: Common Parameters</a>. Amazon SageMaker supports both <code>registry/repository[:tag]</code> and <code>registry/repository[@digest]</code> image path formats. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>.</p>
    #[serde(rename = "TrainingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_image: Option<String>,
    /// <p>The input mode that the algorithm supports. For the input modes that Amazon SageMaker algorithms support, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. If an algorithm supports the <code>File</code> input mode, Amazon SageMaker downloads the training data from S3 to the provisioned ML storage Volume, and mounts the directory to docker volume for training container. If an algorithm supports the <code>Pipe</code> input mode, Amazon SageMaker streams data directly from S3 to the container. </p> <p> In File mode, make sure you provision ML storage volume with sufficient capacity to accommodate the data download from S3. In addition to the training data, the ML storage volume also stores the output model. The algorithm container use ML storage volume to also store intermediate information, if any. </p> <p> For distributed algorithms using File mode, training data is distributed uniformly, and your training duration is predictable if the input data objects size is approximately same. Amazon SageMaker does not split the files any further for model training. If the object sizes are skewed, training won't be optimal as the data distribution is also skewed where one host in a training cluster is overloaded, thus becoming bottleneck in training. </p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>Specifies the validation and image scan statuses of the algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AlgorithmStatusDetails {
    /// <p>The status of the scan of the algorithm's Docker image container.</p>
    #[serde(rename = "ImageScanStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_statuses: Option<Vec<AlgorithmStatusItem>>,
    /// <p>The status of algorithm validation.</p>
    #[serde(rename = "ValidationStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_statuses: Option<Vec<AlgorithmStatusItem>>,
}

/// <p>Represents the overall status of an algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AlgorithmStatusItem {
    /// <p>if the overall status is <code>Failed</code>, the reason for the failure.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the algorithm for which the overall status is being reported.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The current status.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Provides summary information about an algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AlgorithmSummary {
    /// <p>The Amazon Resource Name (ARN) of the algorithm.</p>
    #[serde(rename = "AlgorithmArn")]
    pub algorithm_arn: String,
    /// <p>A brief description of the algorithm.</p>
    #[serde(rename = "AlgorithmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_description: Option<String>,
    /// <p>The name of the algorithm that is described by the summary.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    /// <p>The overall status of the algorithm.</p>
    #[serde(rename = "AlgorithmStatus")]
    pub algorithm_status: String,
    /// <p>A timestamp that shows when the algorithm was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
}

/// <p>Defines a training job and a batch transform job that Amazon SageMaker runs to validate your algorithm.</p> <p>The data provided in the validation profile is made available to your buyers on AWS Marketplace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmValidationProfile {
    /// <p>The name of the profile for the algorithm. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "ProfileName")]
    pub profile_name: String,
    /// <p>The <code>TrainingJobDefinition</code> object that describes the training job that Amazon SageMaker runs to validate your algorithm.</p>
    #[serde(rename = "TrainingJobDefinition")]
    pub training_job_definition: TrainingJobDefinition,
    /// <p>The <code>TransformJobDefinition</code> object that describes the transform job that Amazon SageMaker runs to validate your algorithm.</p>
    #[serde(rename = "TransformJobDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_job_definition: Option<TransformJobDefinition>,
}

/// <p>Specifies configurations for one or more training jobs that Amazon SageMaker runs to test the algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmValidationSpecification {
    /// <p>An array of <code>AlgorithmValidationProfile</code> objects, each of which specifies a training job and batch transform job that Amazon SageMaker runs to validate your algorithm.</p>
    #[serde(rename = "ValidationProfiles")]
    pub validation_profiles: Vec<AlgorithmValidationProfile>,
    /// <p>The IAM roles that Amazon SageMaker uses to run the training jobs.</p>
    #[serde(rename = "ValidationRole")]
    pub validation_role: String,
}

/// <p>Configures how labels are consolidated across human workers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnotationConsolidationConfig {
    /// <p>The Amazon Resource Name (ARN) of a Lambda function implements the logic for annotation consolidation.</p> <p>For the built-in bounding box, image classification, semantic segmentation, and text classification task types, Amazon SageMaker Ground Truth provides the following Lambda functions:</p> <ul> <li> <p> <i>Bounding box</i> - Finds the most similar boxes from different workers based on the Jaccard index of the boxes.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-BoundingBox</code> </p> </li> <li> <p> <i>Image classification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of an image based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-ImageMultiClass</code> </p> </li> <li> <p> <i>Semantic segmentation</i> - Treats each pixel in an image as a multi-class classification and treats pixel annotations from workers as "votes" for the correct label.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-SemanticSegmentation</code> </p> </li> <li> <p> <i>Text classification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of text based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-TextMultiClass</code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-annotation-consolidation.html">Annotation Consolidation</a>.</p>
    #[serde(rename = "AnnotationConsolidationLambdaArn")]
    pub annotation_consolidation_lambda_arn: String,
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

/// <p>Defines the possible values for a categorical hyperparameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoricalParameterRangeSpecification {
    /// <p>The allowed categories for the hyperparameter.</p>
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
    /// <p>(Optional) The input mode to use for the data channel in a training job. If you don't set a value for <code>InputMode</code>, Amazon SageMaker uses the value set for <code>TrainingInputMode</code>. Use this parameter to override the <code>TrainingInputMode</code> setting in a <a>AlgorithmSpecification</a> request when you have a channel that needs a different input mode from the training job's general setting. To download the data from Amazon Simple Storage Service (Amazon S3) to the provisioned ML storage volume, and mount the directory to a Docker volume, use <code>File</code> input mode. To stream data directly from Amazon S3 to the container, choose <code>Pipe</code> input mode.</p> <p>To use a model for incremental training, choose <code>File</code> input model.</p>
    #[serde(rename = "InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_mode: Option<String>,
    /// <p><p/> <p>Specify RecordIO as the value when input data is in raw format but the training algorithm requires the RecordIO format. In this case, Amazon SageMaker wraps each individual S3 object in a RecordIO record. If the input data is already in RecordIO format, you don&#39;t need to set this attribute. For more information, see <a href="https://mxnet.incubator.apache.org/architecture/note_data_loading.html#data-format">Create a Dataset Using RecordIO</a>. </p> <p>In File mode, leave this field unset or set it to None.</p></p>
    #[serde(rename = "RecordWrapperType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_wrapper_type: Option<String>,
    /// <p>A configuration for a shuffle option for input data in a channel. If you use <code>S3Prefix</code> for <code>S3DataType</code>, this shuffles the results of the S3 key prefix matches. If you use <code>ManifestFile</code>, the order of the S3 object references in the <code>ManifestFile</code> is shuffled. If you use <code>AugmentedManifestFile</code>, the order of the JSON lines in the <code>AugmentedManifestFile</code> is shuffled. The shuffling order is determined using the <code>Seed</code> value.</p> <p>For Pipe input mode, shuffling is done at the start of every epoch. With large datasets this ensures that the order of the training data is different for each epoch, it helps reduce bias and possible overfitting. In a multi-node training job when ShuffleConfig is combined with <code>S3DataDistributionType</code> of <code>ShardedByS3Key</code>, the data is shuffled across nodes so that the content sent to a particular node on the first epoch might be sent to a different node on the second epoch.</p>
    #[serde(rename = "ShuffleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shuffle_config: Option<ShuffleConfig>,
}

/// <p>Defines a named input source, called a channel, to be used by an algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelSpecification {
    /// <p>A brief description of the channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether the channel is required by the algorithm.</p>
    #[serde(rename = "IsRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The allowed compression types, if data compression is used.</p>
    #[serde(rename = "SupportedCompressionTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_compression_types: Option<Vec<String>>,
    /// <p>The supported MIME types for the data.</p>
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Vec<String>,
    /// <p>The allowed input mode, either FILE or PIPE.</p> <p>In FILE mode, Amazon SageMaker copies the data from the input source onto the local Amazon Elastic Block Store (Amazon EBS) volumes before starting your training algorithm. This is the most commonly used input mode.</p> <p>In PIPE mode, Amazon SageMaker streams input data from the source directly to your algorithm without using the EBS volume.</p>
    #[serde(rename = "SupportedInputModes")]
    pub supported_input_modes: Vec<String>,
}

/// <p>Specifies summary information about a Git repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CodeRepositorySummary {
    /// <p>The Amazon Resource Name (ARN) of the Git repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
    /// <p>The name of the Git repository.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
    /// <p>The date and time that the Git repository was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Configuration details for the Git repository, including the URL where it is located and the ARN of the AWS Secrets Manager secret that contains the credentials used to access the repository.</p>
    #[serde(rename = "GitConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_config: Option<GitConfig>,
    /// <p>The date and time that the Git repository was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
}

/// <p>Identifies a Amazon Cognito user group. A user group can be used in on or more work teams.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitoMemberDefinition {
    /// <p>An identifier for an application client. You must create the app client ID using Amazon Cognito.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>An identifier for a user group.</p>
    #[serde(rename = "UserGroup")]
    pub user_group: String,
    /// <p>An identifier for a user pool. The user pool must be in the same region as the service that you are calling.</p>
    #[serde(rename = "UserPool")]
    pub user_pool: String,
}

/// <p>A summary of a model compilation job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CompilationJobSummary {
    /// <p>The time when the model compilation job completed.</p>
    #[serde(rename = "CompilationEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the model compilation job.</p>
    #[serde(rename = "CompilationJobArn")]
    pub compilation_job_arn: String,
    /// <p>The name of the model compilation job that you want a summary for.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
    /// <p>The status of the model compilation job.</p>
    #[serde(rename = "CompilationJobStatus")]
    pub compilation_job_status: String,
    /// <p>The time when the model compilation job started.</p>
    #[serde(rename = "CompilationStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation_start_time: Option<f64>,
    /// <p>The type of device that the model will run on after compilation has completed.</p>
    #[serde(rename = "CompilationTargetDevice")]
    pub compilation_target_device: String,
    /// <p>The time when the model compilation job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The time when the model compilation job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

/// <p>Describes the container, as part of model definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerDefinition {
    /// <p>This parameter is ignored for models that contain only a <code>PrimaryContainer</code>.</p> <p>When a <code>ContainerDefinition</code> is part of an inference pipeline, the value of ths parameter uniquely identifies the container for the purposes of logging and metrics. For information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/inference-pipeline-logs-metrics.html">Use Logs and Metrics to Monitor an Inference Pipeline</a>. If you don't specify a value for this parameter for a <code>ContainerDefinition</code> that is part of an inference pipeline, a unique name is automatically assigned based on the position of the <code>ContainerDefinition</code> in the pipeline. If you specify a value for the <code>ContainerHostName</code> for any <code>ContainerDefinition</code> that is part of an inference pipeline, you must specify a value for the <code>ContainerHostName</code> parameter of every <code>ContainerDefinition</code> in that pipeline.</p>
    #[serde(rename = "ContainerHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_hostname: Option<String>,
    /// <p>The environment variables to set in the Docker container. Each key and value in the <code>Environment</code> string to string map can have length of up to 1024. We support up to 16 entries in the map. </p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon EC2 Container Registry (Amazon ECR) path where inference code is stored. If you are using your own custom algorithm instead of an algorithm provided by Amazon SageMaker, the inference code must meet Amazon SageMaker requirements. Amazon SageMaker supports both <code>registry/repository[:tag]</code> and <code>registry/repository[@digest]</code> image path formats. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a> </p>
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p><p>The S3 path where the model artifacts, which result from model training, are stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). The S3 path is required for Amazon SageMaker built-in algorithms, but not if you use your own algorithms. For more information on built-in algorithms, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-algo-docker-registry-paths.html">Common Parameters</a>. </p> <p>If you provide a value for this parameter, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provide. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <important> <p>If you use a built-in algorithm to create a model, Amazon SageMaker requires that you provide a S3 path to the model artifacts in <code>ModelDataUrl</code>.</p> </important></p>
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<String>,
    /// <p>The name of the model package to use to create the model.</p>
    #[serde(rename = "ModelPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_name: Option<String>,
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
    /// <p><p>The scale that hyperparameter tuning uses to search the hyperparameter range. For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com//sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon SageMaker hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparemeter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have only values greater than 0.</p> </dd> <dt>ReverseLogarithmic</dt> <dd> <p>Hyperparemeter tuning searches the values in the hyperparameter range by using a reverse logarithmic scale.</p> <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0&lt;=x&lt;1.0.</p> </dd> </dl></p>
    #[serde(rename = "ScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

/// <p>Defines the possible values for a continuous hyperparameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuousParameterRangeSpecification {
    /// <p>The maximum floating-point value allowed.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: String,
    /// <p>The minimum floating-point value allowed.</p>
    #[serde(rename = "MinValue")]
    pub min_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAlgorithmInput {
    /// <p>A description of the algorithm.</p>
    #[serde(rename = "AlgorithmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_description: Option<String>,
    /// <p>The name of the algorithm.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    /// <p>Whether to certify the algorithm so that it can be listed in AWS Marketplace.</p>
    #[serde(rename = "CertifyForMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certify_for_marketplace: Option<bool>,
    /// <p><p>Specifies details about inference jobs that the algorithm runs, including the following:</p> <ul> <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li> <li> <p>The instance types that the algorithm supports for transform jobs and real-time endpoints used for inference.</p> </li> <li> <p>The input and output content formats that the algorithm supports for inference.</p> </li> </ul></p>
    #[serde(rename = "InferenceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_specification: Option<InferenceSpecification>,
    /// <p><p>Specifies details about training jobs run by this algorithm, including the following:</p> <ul> <li> <p>The Amazon ECR path of the container and the version digest of the algorithm.</p> </li> <li> <p>The hyperparameters that the algorithm supports.</p> </li> <li> <p>The instance types that the algorithm supports for training.</p> </li> <li> <p>Whether the algorithm supports distributed training.</p> </li> <li> <p>The metrics that the algorithm emits to Amazon CloudWatch.</p> </li> <li> <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p> </li> <li> <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p> </li> </ul></p>
    #[serde(rename = "TrainingSpecification")]
    pub training_specification: TrainingSpecification,
    /// <p>Specifies configurations for one or more training jobs and that Amazon SageMaker runs to test the algorithm's training code and, optionally, one or more batch transform jobs that Amazon SageMaker runs to test the algorithm's inference code.</p>
    #[serde(rename = "ValidationSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_specification: Option<AlgorithmValidationSpecification>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAlgorithmOutput {
    /// <p>The Amazon Resource Name (ARN) of the new algorithm.</p>
    #[serde(rename = "AlgorithmArn")]
    pub algorithm_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCodeRepositoryInput {
    /// <p>The name of the Git repository. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
    /// <p>Specifies details about the repository, including the URL where the repository is located, the default branch, and credentials to use to access the repository.</p>
    #[serde(rename = "GitConfig")]
    pub git_config: GitConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCodeRepositoryOutput {
    /// <p>The Amazon Resource Name (ARN) of the new repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCompilationJobRequest {
    /// <p>A name for the model compilation job. The name must be unique within the AWS Region and within your AWS account. </p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
    /// <p>Provides information about the location of input model artifacts, the name and shape of the expected data inputs, and the framework in which the model was trained.</p>
    #[serde(rename = "InputConfig")]
    pub input_config: InputConfig,
    /// <p>Provides information about the output location for the compiled model and the target device the model runs on.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: OutputConfig,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf. </p> <p>During model compilation, Amazon SageMaker needs your permission to:</p> <ul> <li> <p>Read input data from an S3 bucket</p> </li> <li> <p>Write model artifacts to an S3 bucket</p> </li> <li> <p>Write logs to Amazon CloudWatch Logs</p> </li> <li> <p>Publish metrics to Amazon CloudWatch</p> </li> </ul> <p>You grant permissions for all of these tasks to an IAM role. To pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles.</a> </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The duration allowed for model compilation.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCompilationJobResponse {
    /// <p><p>If the action is successful, the service sends back an HTTP 200 response. Amazon SageMaker returns the following data in JSON format:</p> <ul> <li> <p> <code>CompilationJobArn</code>: The Amazon Resource Name (ARN) of the compiled job.</p> </li> </ul></p>
    #[serde(rename = "CompilationJobArn")]
    pub compilation_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEndpointConfigInput {
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>An list of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,
    /// <p>A list of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i> AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateEndpointConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint configuration. </p>
    #[serde(rename = "EndpointConfigArn")]
    pub endpoint_config_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEndpointInput {
    /// <p>The name of an endpoint configuration. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a>. </p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The name of the endpoint. The name must be unique within an AWS Region in your AWS account.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a>in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateEndpointOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHyperParameterTuningJobRequest {
    /// <p>The <a>HyperParameterTuningJobConfig</a> object that describes the tuning job, including the search strategy, the objective metric used to evaluate training jobs, ranges of parameters to search, and resource limits for the tuning job. For more information, see <a>automatic-model-tuning</a> </p>
    #[serde(rename = "HyperParameterTuningJobConfig")]
    pub hyper_parameter_tuning_job_config: HyperParameterTuningJobConfig,
    /// <p>The name of the tuning job. This name is the prefix for the names of all training jobs that this tuning job launches. The name must be unique within the same AWS account and AWS Region. The name must have { } to { } characters. Valid characters are a-z, A-Z, 0-9, and : + = @ _ % - (hyphen). The name is not case sensitive.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
    /// <p>An array of key-value pairs. You can use tags to categorize your AWS resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <p>Tags that you specify for the tuning job are also added to all training jobs that the tuning job launches.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <a>HyperParameterTrainingJobDefinition</a> object that describes the training jobs that this tuning job launches, including static hyperparameters, input data configuration, output data configuration, resource configuration, and stopping condition.</p>
    #[serde(rename = "TrainingJobDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_definition: Option<HyperParameterTrainingJobDefinition>,
    /// <p><p>Specifies the configuration for starting the hyperparameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p> <p>All training jobs launched by the new hyperparameter tuning job are evaluated by using the objective metric. If you specify <code>IDENTICAL<em>DATA</em>AND_ALGORITHM</code> as the <code>WarmStartType</code> value for the warm start configuration, the training job that performs the best in the new tuning job is compared to the best training jobs from the parent tuning jobs. From these, the training job that performs the best as measured by the objective metric is returned as the overall best training job.</p> <note> <p>All training jobs launched by parent hyperparameter tuning jobs and the new hyperparameter tuning jobs count against the limit of training jobs for the tuning job.</p> </note></p>
    #[serde(rename = "WarmStartConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_start_config: Option<HyperParameterTuningJobWarmStartConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateHyperParameterTuningJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the tuning job. Amazon SageMaker assigns an ARN to a hyperparameter tuning job when you create it.</p>
    #[serde(rename = "HyperParameterTuningJobArn")]
    pub hyper_parameter_tuning_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLabelingJobRequest {
    /// <p>Configures the information required for human workers to complete a labeling task.</p>
    #[serde(rename = "HumanTaskConfig")]
    pub human_task_config: HumanTaskConfig,
    /// <p>Input data for the labeling job, such as the Amazon S3 location of the data objects and the location of the manifest file that describes the data objects.</p>
    #[serde(rename = "InputConfig")]
    pub input_config: LabelingJobInputConfig,
    /// <p>The attribute name to use for the label in the output manifest file. This is the key for the key/value pair formed with the label that a worker assigns to the object. The name can't end with "-metadata". If you are running a semantic segmentation labeling job, the attribute name must end with "-ref". If you are running any other kind of labeling job, the attribute name must not end with "-ref".</p>
    #[serde(rename = "LabelAttributeName")]
    pub label_attribute_name: String,
    /// <p>The S3 URL of the file that defines the categories used to label the data objects.</p> <p>The file is a JSON structure in the following format:</p> <p> <code>{</code> </p> <p> <code> "document-version": "2018-11-28"</code> </p> <p> <code> "labels": [</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 1</i>"</code> </p> <p> <code> },</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 2</i>"</code> </p> <p> <code> },</code> </p> <p> <code> ...</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label n</i>"</code> </p> <p> <code> }</code> </p> <p> <code> ]</code> </p> <p> <code>}</code> </p>
    #[serde(rename = "LabelCategoryConfigS3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_category_config_s3_uri: Option<String>,
    /// <p>Configures the information required to perform automated data labeling.</p>
    #[serde(rename = "LabelingJobAlgorithmsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_algorithms_config: Option<LabelingJobAlgorithmsConfig>,
    /// <p>The name of the labeling job. This name is used to identify the job in a list of labeling jobs.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
    /// <p>The location of the output data and the AWS Key Management Service key ID for the key used to encrypt the output data, if any.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: LabelingJobOutputConfig,
    /// <p>The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during data labeling. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete data labeling.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>A set of conditions for stopping the labeling job. If any of the conditions are met, the job is automatically stopped. You can use these conditions to control the cost of data labeling.</p>
    #[serde(rename = "StoppingConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_conditions: Option<LabelingJobStoppingConditions>,
    /// <p>An array of key/value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLabelingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the labeling job. You use this ARN to identify the labeling job.</p>
    #[serde(rename = "LabelingJobArn")]
    pub labeling_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelInput {
    /// <p>Specifies the containers in the inference pipeline.</p>
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerDefinition>>,
    /// <p><p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p> <note> <p>The Semantic Segmentation built-in algorithm does not support network isolation.</p> </note></p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute instances or for batch transform jobs. Deploying on ML compute instances is part of model hosting. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The name of the new model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>The location of the primary docker image containing inference code, associated artifacts, and custom environment map that the inference code uses when the model is deployed for predictions. </p>
    #[serde(rename = "PrimaryContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_container: Option<ContainerDefinition>,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that you want your model to connect to. Control access to and from your model container by configuring the VPC. <code>VpcConfig</code> is used in hosting services and in batch transform. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-vpc.html">Protect Data in Batch Transform Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateModelOutput {
    /// <p>The ARN of the model created in Amazon SageMaker.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelPackageInput {
    /// <p>Whether to certify the model package for listing on AWS Marketplace.</p>
    #[serde(rename = "CertifyForMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certify_for_marketplace: Option<bool>,
    /// <p><p>Specifies details about inference jobs that can be run with models based on this model package, including the following:</p> <ul> <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li> <li> <p>The instance types that the model package supports for transform jobs and real-time endpoints used for inference.</p> </li> <li> <p>The input and output content formats that the model package supports for inference.</p> </li> </ul></p>
    #[serde(rename = "InferenceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_specification: Option<InferenceSpecification>,
    /// <p>A description of the model package.</p>
    #[serde(rename = "ModelPackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_description: Option<String>,
    /// <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
    /// <p>Details about the algorithm that was used to create the model package.</p>
    #[serde(rename = "SourceAlgorithmSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_algorithm_specification: Option<SourceAlgorithmSpecification>,
    /// <p>Specifies configurations for one or more transform jobs that Amazon SageMaker runs to test the model package.</p>
    #[serde(rename = "ValidationSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_specification: Option<ModelPackageValidationSpecification>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateModelPackageOutput {
    /// <p>The Amazon Resource Name (ARN) of the new model package.</p>
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNotebookInstanceInput {
    /// <p>A list of Elastic Inference (EI) instance types to associate with this notebook instance. Currently, only one instance type can be associated with a notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository: Option<String>,
    /// <p>Sets whether Amazon SageMaker provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance will be able to access resources only in your VPC, and will not be able to connect to Amazon SageMaker training and endpoint services unless your configure a NAT Gateway in your VPC.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p>
    #[serde(rename = "DirectInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_internet_access: Option<String>,
    /// <p>The type of ML compute instance to launch for the notebook instance.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    #[serde(rename = "LifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_config_name: Option<String>,
    /// <p>The name of the new notebook instance.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p><p> When you send any requests to AWS resources from the notebook instance, Amazon SageMaker assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so Amazon SageMaker can perform these tasks. The policy must allow the Amazon SageMaker service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p><p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p> <note> <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p> </note></p>
    #[serde(rename = "RootAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_access: Option<String>,
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
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p>
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
    /// <p>A shell script that runs only once, when you create a notebook instance. The shell script must be a base64-encoded string.</p>
    #[serde(rename = "OnCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// <p>A shell script that runs every time you start a notebook instance, including when you create the notebook instance. The shell script must be a base64-encoded string.</p>
    #[serde(rename = "OnStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateNotebookInstanceLifecycleConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePresignedNotebookInstanceUrlOutput {
    /// <p>A JSON object that contains the URL string. </p>
    #[serde(rename = "AuthorizedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTrainingJobRequest {
    /// <p>The registry path of the Docker image that contains the training algorithm and algorithm-specific metadata, including the input mode. For more information about algorithms provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about providing your own algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-encrypt.html">Protect Communications Between ML Compute Instances in a Distributed Training Job</a>.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p><p>Isolates the training container. No inbound or outbound network calls can be made, except for calls between peers within a training cluster for distributed training. If you enable network isolation for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p> <note> <p>The Semantic Segmentation built-in algorithm does not support network isolation.</p> </note></p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    /// <p>Algorithm-specific parameters that influence the quality of the model. You set hyperparameters before you start the learning process. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> <p>You can specify a maximum of 100 hyperparameters. Each hyperparameter is a key-value pair. Each key and value is limited to 256 characters, as specified by the <code>Length Constraint</code>. </p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects. Each channel is a named input source. <code>InputDataConfig</code> describes the input data and its location. </p> <p>Algorithms can accept input data from one or more channels. For example, an algorithm might have two channels of input data, <code>training_data</code> and <code>validation_data</code>. The configuration for each channel provides the S3 location where the input data is stored. It also provides information about the stored data: the MIME type, compression method, and whether the data is wrapped in RecordIO format. </p> <p>Depending on the input mode that the algorithm supports, Amazon SageMaker either copies input data files from an S3 bucket to a local directory in the Docker container, or makes it available as input streams. </p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<Channel>>,
    /// <p>Specifies the path to the S3 bucket where you want to store model artifacts. Amazon SageMaker creates subfolders for the artifacts. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The resources, including the ML compute instances and ML storage volumes, to use for model training. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use ML storage volumes for scratch space. If you want Amazon SageMaker to use the ML storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p><p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf. </p> <p>During model training, Amazon SageMaker needs your permission to read input data from an S3 bucket, download a Docker image that contains training code, write model artifacts to an S3 bucket, write logs to Amazon CloudWatch Logs, and publish metrics to Amazon CloudWatch. You grant permissions for all of these tasks to an IAM role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Sets a duration for training. Use this parameter to cap model training costs. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts. </p> <p>When Amazon SageMaker terminates a job because the stopping condition has been met, training algorithms provided by Amazon SageMaker save the intermediate results of the job. This intermediate data is a valid model artifact. You can use it to create a model using the <code>CreateModel</code> API. </p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the training job. The name must be unique within an AWS Region in an AWS account. </p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that you want your training job to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTrainingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTransformJobRequest {
    /// <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set <code>SplitType</code> to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p> <p>To use only one record when making an HTTP invocation request to a container, set <code>BatchStrategy</code> to <code>SingleRecord</code> and <code>SplitType</code> to <code>Line</code>.</p> <p>To fit as many records in a mini-batch as can fit within the <code>MaxPayloadInMB</code> limit, set <code>BatchStrategy</code> to <code>MultiRecord</code> and <code>SplitType</code> to <code>Line</code>.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The maximum number of parallel requests that can be sent to each instance in a transform job. If <code>MaxConcurrentTransforms</code> is set to <code>0</code> or left unset, Amazon SageMaker checks the optional execution-parameters to determine the optimal settings for your chosen algorithm. If the execution-parameters endpoint is not enabled, the default value is <code>1</code>. For more information on execution-parameters, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-batch-code.html#your-algorithms-batch-code-how-containe-serves-requests">How Containers Serve Requests</a>. For built-in algorithms, you don't need to set a value for <code>MaxConcurrentTransforms</code>.</p>
    #[serde(rename = "MaxConcurrentTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transforms: Option<i64>,
    /// <p>The maximum allowed size of the payload, in MB. A <i>payload</i> is the data portion of a record (without metadata). The value in <code>MaxPayloadInMB</code> must be greater than, or equal to, the size of a single record. To estimate the size of a record in MB, divide the size of your dataset by the number of records. To ensure that the records fit within the maximum payload size, we recommend using a slightly larger value. The default value is <code>6</code> MB. </p> <p>For cases where the payload might be arbitrarily large and is transmitted using HTTP chunked encoding, set the value to <code>0</code>. This feature works only in supported algorithms. Currently, Amazon SageMaker built-in algorithms do not support HTTP chunked encoding.</p>
    #[serde(rename = "MaxPayloadInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_in_mb: Option<i64>,
    /// <p>The name of the model that you want to use for the transform job. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an AWS Region in an AWS account.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTransformJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the transform job.</p>
    #[serde(rename = "TransformJobArn")]
    pub transform_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkteamRequest {
    /// <p>A description of the work team.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A list of <code>MemberDefinition</code> objects that contains objects that identify the Amazon Cognito user pool that makes up the work team. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools.html">Amazon Cognito User Pools</a>.</p> <p>All of the <code>CognitoMemberDefinition</code> objects that make up the member definition must have the same <code>ClientId</code> and <code>UserPool</code> values.</p>
    #[serde(rename = "MemberDefinitions")]
    pub member_definitions: Vec<MemberDefinition>,
    /// <p>Configures notification of workers regarding available or expiring work items.</p>
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p><p/></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the work team. Use this name to identify the work team.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateWorkteamResponse {
    /// <p>The Amazon Resource Name (ARN) of the work team. You can use this ARN to identify the work team.</p>
    #[serde(rename = "WorkteamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workteam_arn: Option<String>,
}

/// <p>Describes the location of the channel data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    /// <p>The S3 location of the data source that is associated with a channel.</p>
    #[serde(rename = "S3DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_source: Option<S3DataSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAlgorithmInput {
    /// <p>The name of the algorithm to delete.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCodeRepositoryInput {
    /// <p>The name of the Git repository to delete.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
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
pub struct DeleteModelPackageInput {
    /// <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
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
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTagsOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWorkteamRequest {
    /// <p>The name of the work team to delete.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteWorkteamResponse {
    /// <p>Returns <code>true</code> if the work team was successfully deleted; otherwise, returns <code>false</code>.</p>
    #[serde(rename = "Success")]
    pub success: bool,
}

/// <p>Gets the Amazon EC2 Container Registry path of the docker image of the model that is hosted in this <a>ProductionVariant</a>.</p> <p>If you used the <code>registry/repository[:tag]</code> form to specify the image path of the primary container when you created the model hosted in this <code>ProductionVariant</code>, the path resolves to a path of the form <code>registry/repository[@digest]</code>. A digest is a hash value that identifies a specific version of an image. For information about Amazon ECR paths, see <a href="http://docs.aws.amazon.com//AmazonECR/latest/userguide/docker-pull-ecr-image.html">Pulling an Image</a> in the <i>Amazon ECR User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct DescribeAlgorithmInput {
    /// <p>The name of the algorithm to describe.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAlgorithmOutput {
    /// <p>The Amazon Resource Name (ARN) of the algorithm.</p>
    #[serde(rename = "AlgorithmArn")]
    pub algorithm_arn: String,
    /// <p>A brief summary about the algorithm.</p>
    #[serde(rename = "AlgorithmDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_description: Option<String>,
    /// <p>The name of the algorithm being described.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    /// <p>The current status of the algorithm.</p>
    #[serde(rename = "AlgorithmStatus")]
    pub algorithm_status: String,
    /// <p>Details about the current status of the algorithm.</p>
    #[serde(rename = "AlgorithmStatusDetails")]
    pub algorithm_status_details: AlgorithmStatusDetails,
    /// <p>Whether the algorithm is certified to be listed in AWS Marketplace.</p>
    #[serde(rename = "CertifyForMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certify_for_marketplace: Option<bool>,
    /// <p>A timestamp specifying when the algorithm was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Details about inference jobs that the algorithm runs.</p>
    #[serde(rename = "InferenceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_specification: Option<InferenceSpecification>,
    /// <p>The product identifier of the algorithm.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// <p>Details about training jobs run by this algorithm.</p>
    #[serde(rename = "TrainingSpecification")]
    pub training_specification: TrainingSpecification,
    /// <p>Details about configurations for one or more training jobs that Amazon SageMaker runs to test the algorithm.</p>
    #[serde(rename = "ValidationSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_specification: Option<AlgorithmValidationSpecification>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCodeRepositoryInput {
    /// <p>The name of the Git repository to describe.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCodeRepositoryOutput {
    /// <p>The Amazon Resource Name (ARN) of the Git repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
    /// <p>The name of the Git repository.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
    /// <p>The date and time that the repository was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Configuration details about the repository, including the URL where the repository is located, the default branch, and the Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the repository.</p>
    #[serde(rename = "GitConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_config: Option<GitConfig>,
    /// <p>The date and time that the repository was last changed.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCompilationJobRequest {
    /// <p>The name of the model compilation job that you want information about.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCompilationJobResponse {
    /// <p>The time when the model compilation job on a compilation job instance ended. For a successful or stopped job, this is when the job's model artifacts have finished uploading. For a failed job, this is when Amazon SageMaker detected that the job failed. </p>
    #[serde(rename = "CompilationEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker assumes to perform the model compilation job.</p>
    #[serde(rename = "CompilationJobArn")]
    pub compilation_job_arn: String,
    /// <p>The name of the model compilation job.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
    /// <p>The status of the model compilation job.</p>
    #[serde(rename = "CompilationJobStatus")]
    pub compilation_job_status: String,
    /// <p>The time when the model compilation job started the <code>CompilationJob</code> instances. </p> <p>You are billed for the time between this timestamp and the timestamp in the <a>DescribeCompilationJobResponse$CompilationEndTime</a> field. In Amazon CloudWatch Logs, the start time might be later than this time. That's because it takes time to download the compilation job, which depends on the size of the compilation job container. </p>
    #[serde(rename = "CompilationStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation_start_time: Option<f64>,
    /// <p>The time that the model compilation job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If a model compilation job failed, the reason it failed. </p>
    #[serde(rename = "FailureReason")]
    pub failure_reason: String,
    /// <p>Information about the location in Amazon S3 of the input model artifacts, the name and shape of the expected data inputs, and the framework in which the model was trained.</p>
    #[serde(rename = "InputConfig")]
    pub input_config: InputConfig,
    /// <p>The time that the status of the model compilation job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>Information about the location in Amazon S3 that has been configured for storing the model artifacts used in the compilation job.</p>
    #[serde(rename = "ModelArtifacts")]
    pub model_artifacts: ModelArtifacts,
    /// <p>Information about the output location for the compiled model and the target device that the model runs on.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: OutputConfig,
    /// <p>The Amazon Resource Name (ARN) of the model compilation job.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The duration allowed for model compilation.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointConfigInput {
    /// <p>The name of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p><p>The status of the endpoint.</p> <ul> <li> <p> <code>OutOfService</code>: Endpoint is not available to take incoming requests.</p> </li> <li> <p> <code>Creating</code>: <a>CreateEndpoint</a> is executing.</p> </li> <li> <p> <code>Updating</code>: <a>UpdateEndpoint</a> or <a>UpdateEndpointWeightsAndCapacities</a> is executing.</p> </li> <li> <p> <code>SystemUpdating</code>: Endpoint is undergoing maintenance and cannot be updated or deleted or re-scaled until it has completed. This maintenance operation does not change any customer-specified values such as VPC config, KMS encryption, model, instance type, or instance count.</p> </li> <li> <p> <code>RollingBack</code>: Endpoint fails to scale up or down or change its variant weight and is in the process of rolling back to its previous configuration. Once the rollback completes, endpoint returns to an <code>InService</code> status. This transitional status only applies to an endpoint that has autoscaling enabled and is undergoing variant weight or capacity changes as part of an <a>UpdateEndpointWeightsAndCapacities</a> call or when the <a>UpdateEndpointWeightsAndCapacities</a> operation is called explicitly.</p> </li> <li> <p> <code>InService</code>: Endpoint is available to process incoming requests.</p> </li> <li> <p> <code>Deleting</code>: <a>DeleteEndpoint</a> is executing.</p> </li> <li> <p> <code>Failed</code>: Endpoint could not be created, updated, or re-scaled. Use <a>DescribeEndpointOutput$FailureReason</a> for information about the failure. <a>DeleteEndpoint</a> is the only operation that can be performed on a failed endpoint.</p> </li> </ul></p>
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>If the hyperparameter tuning job is an warm start tuning job with a <code>WarmStartType</code> of <code>IDENTICAL_DATA_AND_ALGORITHM</code>, this is the <a>TrainingJobSummary</a> for the training job with the best objective metric value of all training jobs launched by this tuning job and all parent jobs specified for the warm start tuning job.</p>
    #[serde(rename = "OverallBestTrainingJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_best_training_job: Option<HyperParameterTrainingJobSummary>,
    /// <p>The <a>HyperParameterTrainingJobDefinition</a> object that specifies the definition of the training jobs that this tuning job launches.</p>
    #[serde(rename = "TrainingJobDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_definition: Option<HyperParameterTrainingJobDefinition>,
    /// <p>The <a>TrainingJobStatusCounters</a> object that specifies the number of training jobs, categorized by status, that this tuning job launched.</p>
    #[serde(rename = "TrainingJobStatusCounters")]
    pub training_job_status_counters: TrainingJobStatusCounters,
    /// <p>The configuration for starting the hyperparameter parameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p>
    #[serde(rename = "WarmStartConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_start_config: Option<HyperParameterTuningJobWarmStartConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLabelingJobRequest {
    /// <p>The name of the labeling job to return information for.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeLabelingJobResponse {
    /// <p>The date and time that the labeling job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If the job failed, the reason that it failed. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Configuration information required for human workers to complete a labeling task.</p>
    #[serde(rename = "HumanTaskConfig")]
    pub human_task_config: HumanTaskConfig,
    /// <p>Input configuration information for the labeling job, such as the Amazon S3 location of the data objects and the location of the manifest file that describes the data objects.</p>
    #[serde(rename = "InputConfig")]
    pub input_config: LabelingJobInputConfig,
    /// <p>A unique identifier for work done as part of a labeling job.</p>
    #[serde(rename = "JobReferenceCode")]
    pub job_reference_code: String,
    /// <p>The attribute used as the label in the output manifest file.</p>
    #[serde(rename = "LabelAttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_attribute_name: Option<String>,
    /// <p>The S3 location of the JSON file that defines the categories used to label data objects.</p> <p>The file is a JSON structure in the following format:</p> <p> <code>{</code> </p> <p> <code> "document-version": "2018-11-28"</code> </p> <p> <code> "labels": [</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 1</i>"</code> </p> <p> <code> },</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 2</i>"</code> </p> <p> <code> },</code> </p> <p> <code> ...</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label n</i>"</code> </p> <p> <code> }</code> </p> <p> <code> ]</code> </p> <p> <code>}</code> </p>
    #[serde(rename = "LabelCategoryConfigS3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_category_config_s3_uri: Option<String>,
    /// <p>Provides a breakdown of the number of data objects labeled by humans, the number of objects labeled by machine, the number of objects than couldn't be labeled, and the total number of objects labeled. </p>
    #[serde(rename = "LabelCounters")]
    pub label_counters: LabelCounters,
    /// <p>Configuration information for automated data labeling.</p>
    #[serde(rename = "LabelingJobAlgorithmsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_algorithms_config: Option<LabelingJobAlgorithmsConfig>,
    /// <p>The Amazon Resource Name (ARN) of the labeling job.</p>
    #[serde(rename = "LabelingJobArn")]
    pub labeling_job_arn: String,
    /// <p>The name assigned to the labeling job when it was created.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
    /// <p>The location of the output produced by the labeling job.</p>
    #[serde(rename = "LabelingJobOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_output: Option<LabelingJobOutput>,
    /// <p>The processing status of the labeling job. </p>
    #[serde(rename = "LabelingJobStatus")]
    pub labeling_job_status: String,
    /// <p>The date and time that the labeling job was last updated.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>The location of the job's output data and the AWS Key Management Service key ID for the key used to encrypt the output data, if any.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: LabelingJobOutputConfig,
    /// <p>The Amazon Resource Name (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during data labeling.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>A set of conditions for stopping a labeling job. If any of the conditions are met, the job is automatically stopped.</p>
    #[serde(rename = "StoppingConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_conditions: Option<LabelingJobStoppingConditions>,
    /// <p>An array of key/value pairs. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeModelInput {
    /// <p>The name of the model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeModelOutput {
    /// <p>The containers in the inference pipeline.</p>
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerDefinition>>,
    /// <p>A timestamp that shows when the model was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p><p>If <code>True</code>, no inbound or outbound network calls can be made to or from the model container.</p> <note> <p>The Semantic Segmentation built-in algorithm does not support network isolation.</p> </note></p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_container: Option<ContainerDefinition>,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that this model has access to. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeModelPackageInput {
    /// <p>The name of the model package to describe.</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeModelPackageOutput {
    /// <p>Whether the model package is certified for listing on AWS Marketplace.</p>
    #[serde(rename = "CertifyForMarketplace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certify_for_marketplace: Option<bool>,
    /// <p>A timestamp specifying when the model package was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Details about inference jobs that can be run with models based on this model package.</p>
    #[serde(rename = "InferenceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_specification: Option<InferenceSpecification>,
    /// <p>The Amazon Resource Name (ARN) of the model package.</p>
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: String,
    /// <p>A brief summary of the model package.</p>
    #[serde(rename = "ModelPackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_description: Option<String>,
    /// <p>The name of the model package being described.</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
    /// <p>The current status of the model package.</p>
    #[serde(rename = "ModelPackageStatus")]
    pub model_package_status: String,
    /// <p>Details about the current status of the model package.</p>
    #[serde(rename = "ModelPackageStatusDetails")]
    pub model_package_status_details: ModelPackageStatusDetails,
    /// <p>Details about the algorithm that was used to create the model package.</p>
    #[serde(rename = "SourceAlgorithmSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_algorithm_specification: Option<SourceAlgorithmSpecification>,
    /// <p>Configurations for one or more transform jobs that Amazon SageMaker runs to test the model package.</p>
    #[serde(rename = "ValidationSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_specification: Option<ModelPackageValidationSpecification>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeNotebookInstanceOutput {
    /// <p>A list of the Elastic Inference (EI) instance types associated with this notebook instance. Currently only one EI instance type can be associated with a notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A timestamp. Use this parameter to return the time when the notebook instance was created</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository: Option<String>,
    /// <p>Describes whether Amazon SageMaker provides internet access to the notebook instance. If this value is set to <i>Disabled</i>, the notebook instance does not have internet access, and cannot connect to Amazon SageMaker training and endpoint services.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>.</p>
    #[serde(rename = "DirectInternetAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_internet_access: Option<String>,
    /// <p>If status is <code>Failed</code>, the reason it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The type of ML compute instance running on the notebook instance.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The AWS KMS key ID Amazon SageMaker uses to encrypt data when storing it on the ML storage volume attached to the instance. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>A timestamp. Use this parameter to retrieve the time when the notebook instance was last modified. </p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The network interface IDs that Amazon SageMaker created at the time of creating the instance. </p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
    /// <p>Returns the name of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a> </p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name: Option<String>,
    /// <p>The name of the Amazon SageMaker notebook instance. </p>
    #[serde(rename = "NotebookInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_name: Option<String>,
    /// <p>The status of the notebook instance.</p>
    #[serde(rename = "NotebookInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the instance. </p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>Whether root access is enabled or disabled for users of the notebook instance.</p> <note> <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p> </note></p>
    #[serde(rename = "RootAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_access: Option<String>,
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
    /// <p>The size, in GB, of the ML storage volume attached to the notebook instance.</p>
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubscribedWorkteamRequest {
    /// <p>The Amazon Resource Name (ARN) of the subscribed work team to describe.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeSubscribedWorkteamResponse {
    /// <p>A <code>Workteam</code> instance that contains information about the work team.</p>
    #[serde(rename = "SubscribedWorkteam")]
    pub subscribed_workteam: SubscribedWorkteam,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrainingJobRequest {
    /// <p>The name of the training job.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTrainingJobResponse {
    /// <p>Information about the algorithm used for training, and algorithm metadata. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p>A timestamp that indicates when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p><p>If you want to allow inbound or outbound network calls, except for calls between peers within a training cluster for distributed training, choose <code>True</code>. If you enable network isolation for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p> <note> <p>The Semantic Segmentation built-in algorithm does not support network isolation.</p> </note></p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    /// <p>If the training job failed, the reason it failed. </p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A collection of <code>MetricData</code> objects that specify the names, values, and dates and times that the training algorithm emitted to Amazon CloudWatch.</p>
    #[serde(rename = "FinalMetricDataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_metric_data_list: Option<Vec<MetricData>>,
    /// <p>Algorithm-specific parameters. </p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects that describes each data input channel. </p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<Channel>>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SageMaker Ground Truth labeling job that created the transform or training job.</p>
    #[serde(rename = "LabelingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_arn: Option<String>,
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
    /// <p><p> Provides detailed information about the state of the training job. For detailed information on the secondary status of the training job, see <code>StatusMessage</code> under <a>SecondaryStatusTransition</a>.</p> <p>Amazon SageMaker provides primary statuses and secondary statuses that apply to each of them:</p> <dl> <dt>InProgress</dt> <dd> <ul> <li> <p> <code>Starting</code> - Starting the training job.</p> </li> <li> <p> <code>Downloading</code> - An optional stage for algorithms that support <code>File</code> training input mode. It indicates that data is being downloaded to the ML storage volumes.</p> </li> <li> <p> <code>Training</code> - Training is in progress.</p> </li> <li> <p> <code>Uploading</code> - Training is complete and the model artifacts are being uploaded to the S3 location.</p> </li> </ul> </dd> <dt>Completed</dt> <dd> <ul> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> </ul> </dd> <dt>Failed</dt> <dd> <ul> <li> <p> <code>Failed</code> - The training job has failed. The reason for the failure is returned in the <code>FailureReason</code> field of <code>DescribeTrainingJobResponse</code>.</p> </li> </ul> </dd> <dt>Stopped</dt> <dd> <ul> <li> <p> <code>MaxRuntimeExceeded</code> - The job stopped because it exceeded the maximum allowed runtime.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> </dd> <dt>Stopping</dt> <dd> <ul> <li> <p> <code>Stopping</code> - Stopping the training job.</p> </li> </ul> </dd> </dl> <important> <p>Valid values for <code>SecondaryStatus</code> are subject to change. </p> </important> <p>We no longer support the following secondary statuses:</p> <ul> <li> <p> <code>LaunchingMLInstances</code> </p> </li> <li> <p> <code>PreparingTrainingStack</code> </p> </li> <li> <p> <code>DownloadingTrainingImage</code> </p> </li> </ul></p>
    #[serde(rename = "SecondaryStatus")]
    pub secondary_status: String,
    /// <p>A history of all of the secondary statuses that the training job has transitioned through.</p>
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
    /// <p>The status of the training job.</p> <p>Amazon SageMaker provides the following training job statuses:</p> <ul> <li> <p> <code>InProgress</code> - The training is in progress.</p> </li> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> <li> <p> <code>Failed</code> - The training job has failed. To see the reason for the failure, see the <code>FailureReason</code> field in the response to a <code>DescribeTrainingJobResponse</code> call.</p> </li> <li> <p> <code>Stopping</code> - The training job is stopping.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> <p>For more detailed information, see <code>SecondaryStatus</code>. </p>
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
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that this training job has access to. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTransformJobResponse {
    /// <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set <code>SplitType</code> to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>A timestamp that shows when the transform Job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>If the transform job failed, <code>FailureReason</code> describes why it failed. A transform job creates a log file, which includes error messages, and stores it as an Amazon S3 object. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/logging-cloudwatch.html">Log Amazon SageMaker Events with Amazon CloudWatch</a>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SageMaker Ground Truth labeling job that created the transform or training job.</p>
    #[serde(rename = "LabelingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_arn: Option<String>,
    /// <p>The maximum number of parallel requests on each instance node that can be launched in a transform job. The default value is 1.</p>
    #[serde(rename = "MaxConcurrentTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transforms: Option<i64>,
    /// <p>The maximum payload size, in MB, used in the transform job.</p>
    #[serde(rename = "MaxPayloadInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_in_mb: Option<i64>,
    /// <p>The name of the model used in the transform job.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
    /// <p>Indicates when the transform job has been completed, or has stopped or failed. You are billed for the time interval between this time and the value of <code>TransformStartTime</code>.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkteamRequest {
    /// <p>The name of the work team to return a description of.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeWorkteamResponse {
    /// <p>A <code>Workteam</code> instance that contains information about the work team. </p>
    #[serde(rename = "Workteam")]
    pub workteam: Workteam,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The status of the endpoint.</p> <ul> <li> <p> <code>OutOfService</code>: Endpoint is not available to take incoming requests.</p> </li> <li> <p> <code>Creating</code>: <a>CreateEndpoint</a> is executing.</p> </li> <li> <p> <code>Updating</code>: <a>UpdateEndpoint</a> or <a>UpdateEndpointWeightsAndCapacities</a> is executing.</p> </li> <li> <p> <code>SystemUpdating</code>: Endpoint is undergoing maintenance and cannot be updated or deleted or re-scaled until it has completed. This maintenance operation does not change any customer-specified values such as VPC config, KMS encryption, model, instance type, or instance count.</p> </li> <li> <p> <code>RollingBack</code>: Endpoint fails to scale up or down or change its variant weight and is in the process of rolling back to its previous configuration. Once the rollback completes, endpoint returns to an <code>InService</code> status. This transitional status only applies to an endpoint that has autoscaling enabled and is undergoing variant weight or capacity changes as part of an <a>UpdateEndpointWeightsAndCapacities</a> call or when the <a>UpdateEndpointWeightsAndCapacities</a> operation is called explicitly.</p> </li> <li> <p> <code>InService</code>: Endpoint is available to process incoming requests.</p> </li> <li> <p> <code>Deleting</code>: <a>DeleteEndpoint</a> is executing.</p> </li> <li> <p> <code>Failed</code>: Endpoint could not be created, updated, or re-scaled. Use <a>DescribeEndpointOutput$FailureReason</a> for information about the failure. <a>DeleteEndpoint</a> is the only operation that can be performed on a failed endpoint.</p> </li> </ul> <p>To get a list of endpoints with a specified status, use the <a>ListEndpointsInput$StatusEquals</a> filter.</p>
    #[serde(rename = "EndpointStatus")]
    pub endpoint_status: String,
    /// <p>A timestamp that shows when the endpoint was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
}

/// <p><p>A conditional statement for a search expression that includes a Boolean operator, a resource property, and a value.</p> <p>If you don&#39;t specify an <code>Operator</code> and a <code>Value</code>, the filter searches for only the specified property. For example, defining a <code>Filter</code> for the <code>FailureReason</code> for the <code>TrainingJob</code> <code>Resource</code> searches for training job objects that have a value in the <code>FailureReason</code> field.</p> <p>If you specify a <code>Value</code>, but not an <code>Operator</code>, Amazon SageMaker uses the equals operator as the default.</p> <p>In search, there are several property types:</p> <dl> <dt>Metrics</dt> <dd> <p>To define a metric filter, enter a value using the form <code>&quot;Metrics.&lt;name&gt;&quot;</code>, where <code>&lt;name&gt;</code> is a metric name. For example, the following filter searches for training jobs with an <code>&quot;accuracy&quot;</code> metric greater than <code>&quot;0.9&quot;</code>:</p> <p> <code>{</code> </p> <p> <code>&quot;Name&quot;: &quot;Metrics.accuracy&quot;,</code> </p> <p> <code>&quot;Operator&quot;: &quot;GREATER<em>THAN&quot;,</code> </p> <p> <code>&quot;Value&quot;: &quot;0.9&quot;</code> </p> <p> <code>}</code> </p> </dd> <dt>HyperParameters</dt> <dd> <p>To define a hyperparameter filter, enter a value with the form <code>&quot;HyperParameters.&lt;name&gt;&quot;</code>. Decimal hyperparameter values are treated as a decimal in a comparison if the specified <code>Value</code> is also a decimal value. If the specified <code>Value</code> is an integer, the decimal hyperparameter values are treated as integers. For example, the following filter is satisfied by training jobs with a <code>&quot;learning</em>rate&quot;</code> hyperparameter that is less than <code>&quot;0.5&quot;</code>:</p> <p> <code> {</code> </p> <p> <code> &quot;Name&quot;: &quot;HyperParameters.learning<em>rate&quot;,</code> </p> <p> <code> &quot;Operator&quot;: &quot;LESS</em>THAN&quot;,</code> </p> <p> <code> &quot;Value&quot;: &quot;0.5&quot;</code> </p> <p> <code> }</code> </p> </dd> <dt>Tags</dt> <dd> <p>To define a tag filter, enter a value with the form <code>&quot;Tags.&lt;key&gt;&quot;</code>.</p> </dd> </dl></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>A property name. For example, <code>TrainingJobName</code>. For the list of valid property names returned in a search result for each supported resource, see <a>TrainingJob</a> properties. You must specify a valid property name for the resource.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A Boolean binary operator that is used to evaluate the filter. The operator field contains one of the following values:</p> <dl> <dt>Equals</dt> <dd> <p>The specified resource in <code>Name</code> equals the specified <code>Value</code>.</p> </dd> <dt>NotEquals</dt> <dd> <p>The specified resource in <code>Name</code> does not equal the specified <code>Value</code>.</p> </dd> <dt>GreaterThan</dt> <dd> <p>The specified resource in <code>Name</code> is greater than the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>GreaterThanOrEqualTo</dt> <dd> <p>The specified resource in <code>Name</code> is greater than or equal to the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>LessThan</dt> <dd> <p>The specified resource in <code>Name</code> is less than the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>LessThanOrEqualTo</dt> <dd> <p>The specified resource in <code>Name</code> is less than or equal to the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>Contains</dt> <dd> <p>Only supported for text-based properties. The word-list of the property contains the specified <code>Value</code>.</p> </dd> </dl> <p>If you have specified a filter <code>Value</code>, the default is <code>Equals</code>.</p>
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>A value used with <code>Resource</code> and <code>Operator</code> to determine if objects satisfy the filter's condition. For numerical properties, <code>Value</code> must be an integer or floating-point decimal. For timestamp properties, <code>Value</code> must be an ISO 8601 date-time string of the following format: <code>YYYY-mm-dd'T'HH:MM:SS</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Shows the final value for the objective metric for a training job that was launched by a hyperparameter tuning job. You define the objective metric in the <code>HyperParameterTuningJobObjective</code> parameter of <a>HyperParameterTuningJobConfig</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSearchSuggestionsRequest {
    /// <p>The name of the Amazon SageMaker resource to Search for. The only valid <code>Resource</code> value is <code>TrainingJob</code>.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>Limits the property names that are included in the response.</p>
    #[serde(rename = "SuggestionQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_query: Option<SuggestionQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSearchSuggestionsResponse {
    /// <p>A list of property names for a <code>Resource</code> that match a <code>SuggestionQuery</code>.</p>
    #[serde(rename = "PropertyNameSuggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name_suggestions: Option<Vec<PropertyNameSuggestion>>,
}

/// <p>Specifies configuration details for a Git repository in your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitConfig {
    /// <p>The default branch for the Git repository.</p>
    #[serde(rename = "Branch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// <p>The URL where the Git repository is located.</p>
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the git repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p> <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

/// <p>Specifies configuration details for a Git repository when the repository is updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GitConfigForUpdate {
    /// <p>The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the git repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p> <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

/// <p>Information required for human workers to complete a labeling task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanTaskConfig {
    /// <p>Configures how labels are consolidated across human workers.</p>
    #[serde(rename = "AnnotationConsolidationConfig")]
    pub annotation_consolidation_config: AnnotationConsolidationConfig,
    /// <p>Defines the maximum number of data objects that can be labeled by human workers at the same time. Each object may have more than one worker at one time.</p>
    #[serde(rename = "MaxConcurrentTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_task_count: Option<i64>,
    /// <p>The number of human workers that will label an object. </p>
    #[serde(rename = "NumberOfHumanWorkersPerDataObject")]
    pub number_of_human_workers_per_data_object: i64,
    /// <p><p>The Amazon Resource Name (ARN) of a Lambda function that is run before a data object is sent to a human worker. Use this function to provide input to a custom labeling job.</p> <p>For the built-in bounding box, image classification, semantic segmentation, and text classification task types, Amazon SageMaker Ground Truth provides the following Lambda functions:</p> <p> <b>US East (Northern Virginia) (us-east-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-TextMultiClass</code> </p> </li> </ul> <p> <b>US East (Ohio) (us-east-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-TextMultiClass</code> </p> </li> </ul> <p> <b>US West (Oregon) (us-west-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-TextMultiClass</code> </p> </li> </ul> <p> <b>EU (Ireland) (eu-west-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-TextMultiClass</code> </p> </li> </ul> <p> <b>Asia Pacific (Tokyo (ap-northeast-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-TextMultiClass</code> </p> </li> </ul> <p> <b>Asia Pacific (Sydney (ap-southeast-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-TextMultiClass</code> </p> </li> </ul></p>
    #[serde(rename = "PreHumanTaskLambdaArn")]
    pub pre_human_task_lambda_arn: String,
    /// <p>The price that you pay for each task performed by a public worker.</p>
    #[serde(rename = "PublicWorkforceTaskPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_workforce_task_price: Option<PublicWorkforceTaskPrice>,
    /// <p>The length of time that a task remains available for labelling by human workers.</p>
    #[serde(rename = "TaskAvailabilityLifetimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_availability_lifetime_in_seconds: Option<i64>,
    /// <p>A description of the task for your human workers.</p>
    #[serde(rename = "TaskDescription")]
    pub task_description: String,
    /// <p>Keywords used to describe the task so that workers on Amazon Mechanical Turk can discover the task.</p>
    #[serde(rename = "TaskKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_keywords: Option<Vec<String>>,
    /// <p>The amount of time that a worker has to complete a task.</p>
    #[serde(rename = "TaskTimeLimitInSeconds")]
    pub task_time_limit_in_seconds: i64,
    /// <p>A title for the task for your human workers.</p>
    #[serde(rename = "TaskTitle")]
    pub task_title: String,
    /// <p>Information about the user interface that workers use to complete the labeling task.</p>
    #[serde(rename = "UiConfig")]
    pub ui_config: UiConfig,
    /// <p>The Amazon Resource Name (ARN) of the work team assigned to complete the tasks.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

/// <p>Specifies which training algorithm to use for training jobs that a hyperparameter tuning job launches and the metrics to monitor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterAlgorithmSpecification {
    /// <p>The name of the resource algorithm to use for the hyperparameter tuning job. If you specify a value for this parameter, do not specify a value for <code>TrainingImage</code>.</p>
    #[serde(rename = "AlgorithmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_name: Option<String>,
    /// <p>An array of <a>MetricDefinition</a> objects that specify the metrics that the algorithm emits.</p>
    #[serde(rename = "MetricDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,
    /// <p> The registry path of the Docker image that contains the training algorithm. For information about Docker registry paths for built-in algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-algo-docker-registry-paths.html">Algorithms Provided by Amazon SageMaker: Common Parameters</a>. Amazon SageMaker supports both <code>registry/repository[:tag]</code> and <code>registry/repository[@digest]</code> image path formats. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>.</p>
    #[serde(rename = "TrainingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_image: Option<String>,
    /// <p>The input mode that the algorithm supports: File or Pipe. In File input mode, Amazon SageMaker downloads the training data from Amazon S3 to the storage volume that is attached to the training instance and mounts the directory to the Docker volume for the training container. In Pipe input mode, Amazon SageMaker streams data directly from Amazon S3 to the container. </p> <p>If you specify File mode, make sure that you provision the storage volume that is attached to the training instance with enough capacity to accommodate the training data downloaded from Amazon S3, the model artifacts, and intermediate information.</p> <p/> <p>For more information about input modes, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>Defines a hyperparameter to be used by an algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterSpecification {
    /// <p>The default value for this hyperparameter. If a default value is specified, a hyperparameter cannot be required.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A brief description of the hyperparameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether this hyperparameter is required.</p>
    #[serde(rename = "IsRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// <p>Indicates whether this hyperparameter is tunable in a hyperparameter tuning job.</p>
    #[serde(rename = "IsTunable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    /// <p>The name of this hyperparameter. The name must be unique.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The allowed range for this hyperparameter.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ParameterRange>,
    /// <p>The type of this hyperparameter. The valid types are <code>Integer</code>, <code>Continuous</code>, <code>Categorical</code>, and <code>FreeText</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Defines the training jobs launched by a hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTrainingJobDefinition {
    /// <p>The <a>HyperParameterAlgorithmSpecification</a> object that specifies the resource algorithm to use for the training jobs that the tuning job launches.</p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: HyperParameterAlgorithmSpecification,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p><p>Isolates the training container. No inbound or outbound network calls can be made, except for calls between peers within a training cluster for distributed training. If network isolation is used for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p> <note> <p>The Semantic Segmentation built-in algorithm does not support network isolation.</p> </note></p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    /// <p>An array of <a>Channel</a> objects that specify the input for the training jobs that the tuning job launches.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<Channel>>,
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
    /// <p>The <a>VpcConfig</a> object that specifies the VPC that you want the training jobs that this hyperparameter tuning job launches to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Specifies summary information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Specifies the time when the training job ends on training instances. You are billed for the time interval between the value of <code>TrainingStartTime</code> and this time. For successful jobs and stopped jobs, this is the time after model artifacts are uploaded. For failed jobs, this is the time when Amazon SageMaker detects a job failure.</p>
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
    /// <p>The HyperParameter tuning job that launched the training job.</p>
    #[serde(rename = "TuningJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_job_name: Option<String>,
}

/// <p>Configures a hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTuningJobConfig {
    /// <p>The <a>HyperParameterTuningJobObjective</a> object that specifies the objective metric for this tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_tuning_job_objective: Option<HyperParameterTuningJobObjective>,
    /// <p>The <a>ParameterRanges</a> object that specifies the ranges of hyperparameters that this tuning job searches.</p>
    #[serde(rename = "ParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_ranges: Option<ParameterRanges>,
    /// <p>The <a>ResourceLimits</a> object that specifies the maximum number of training jobs and parallel training jobs for this tuning job.</p>
    #[serde(rename = "ResourceLimits")]
    pub resource_limits: ResourceLimits,
    /// <p>Specifies how hyperparameter tuning chooses the combinations of hyperparameter values to use for the training job it launches. To use the Bayesian search stategy, set this to <code>Bayesian</code>. To randomly search, set it to <code>Random</code>. For information about search strategies, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-how-it-works.html">How Hyperparameter Tuning Works</a>.</p>
    #[serde(rename = "Strategy")]
    pub strategy: String,
    /// <p><p>Specifies whether to use early stopping for training jobs launched by the hyperparameter tuning job. This can be one of the following values (the default value is <code>OFF</code>):</p> <dl> <dt>OFF</dt> <dd> <p>Training jobs launched by the hyperparameter tuning job do not use early stopping.</p> </dd> <dt>AUTO</dt> <dd> <p>Amazon SageMaker stops training jobs launched by the hyperparameter tuning job when they are unlikely to perform better than previously completed training jobs. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-early-stopping.html">Stop Training Jobs Early</a>.</p> </dd> </dl></p>
    #[serde(rename = "TrainingJobEarlyStoppingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_early_stopping_type: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p><p>Specifies the configuration for a hyperparameter tuning job that uses one or more previous hyperparameter tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p> <p>All training jobs launched by the new hyperparameter tuning job are evaluated by using the objective metric, and the training job that performs the best is compared to the best training jobs from the parent tuning jobs. From these, the training job that performs the best as measured by the objective metric is returned as the overall best training job.</p> <note> <p>All training jobs launched by parent hyperparameter tuning jobs and the new hyperparameter tuning jobs count against the limit of training jobs for the tuning job.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTuningJobWarmStartConfig {
    /// <p>An array of hyperparameter tuning jobs that are used as the starting point for the new hyperparameter tuning job. For more information about warm starting a hyperparameter tuning job, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-warm-start.html">Using a Previous Hyperparameter Tuning Job as a Starting Point</a>.</p> <p>Hyperparameter tuning jobs created before October 1, 2018 cannot be used as parent jobs for warm start tuning jobs.</p>
    #[serde(rename = "ParentHyperParameterTuningJobs")]
    pub parent_hyper_parameter_tuning_jobs: Vec<ParentHyperParameterTuningJob>,
    /// <p><p>Specifies one of the following:</p> <dl> <dt>IDENTICAL<em>DATA</em>AND<em>ALGORITHM</dt> <dd> <p>The new hyperparameter tuning job uses the same input data and training image as the parent tuning jobs. You can change the hyperparameter ranges to search and the maximum number of training jobs that the hyperparameter tuning job launches. You cannot use a new version of the training algorithm, unless the changes in the new version do not affect the algorithm itself. For example, changes that improve logging or adding support for a different data format are allowed. You can also change hyperparameters from tunable to static, and from static to tunable, but the total number of static plus tunable hyperparameters must remain the same as it is in all parent jobs. The objective metric for the new tuning job must be the same as for all parent jobs.</p> </dd> <dt>TRANSFER</em>LEARNING</dt> <dd> <p>The new hyperparameter tuning job can include input data, hyperparameter ranges, maximum number of concurrent training jobs, and maximum number of training jobs that are different than those of its parent hyperparameter tuning jobs. The training image can also be a different version from the version used in the parent hyperparameter tuning job. You can also change hyperparameters from tunable to static, and from static to tunable, but the total number of static plus tunable hyperparameters must remain the same as it is in all parent jobs. The objective metric for the new tuning job must be the same as for all parent jobs.</p> </dd> </dl></p>
    #[serde(rename = "WarmStartType")]
    pub warm_start_type: String,
}

/// <p>Defines how to perform inference generation after a training job is run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceSpecification {
    /// <p>The Amazon ECR registry path of the Docker image that contains the inference code.</p>
    #[serde(rename = "Containers")]
    pub containers: Vec<ModelPackageContainerDefinition>,
    /// <p>The supported MIME types for the input data.</p>
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Vec<String>,
    /// <p>A list of the instance types that are used to generate inferences in real-time.</p>
    #[serde(rename = "SupportedRealtimeInferenceInstanceTypes")]
    pub supported_realtime_inference_instance_types: Vec<String>,
    /// <p>The supported MIME types for the output data.</p>
    #[serde(rename = "SupportedResponseMIMETypes")]
    pub supported_response_mime_types: Vec<String>,
    /// <p>A list of the instance types on which a transformation job can be run or on which an endpoint can be deployed.</p>
    #[serde(rename = "SupportedTransformInstanceTypes")]
    pub supported_transform_instance_types: Vec<String>,
}

/// <p>Contains information about the location of input model artifacts, the name and shape of the expected data inputs, and the framework in which the model was trained.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputConfig {
    /// <p><p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. The data inputs are <a>InputConfig$Framework</a> specific. </p> <ul> <li> <p> <code>TensorFlow</code>: You must specify the name and shape (NHWC format) of the expected data inputs using a dictionary format for your trained model. The dictionary formats required for the console and CLI are different.</p> <ul> <li> <p>Examples for one input:</p> <ul> <li> <p>If using the console, <code>{&quot;input&quot;:[1,1024,1024,3]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input&quot;:[1,1024,1024,3]}</code> </p> </li> </ul> </li> <li> <p>Examples for two inputs:</p> <ul> <li> <p>If using the console, <code>{&quot;data1&quot;: [1,28,28,1], &quot;data2&quot;:[1,28,28,1]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;data1&quot;: [1,28,28,1], &quot;data2&quot;:[1,28,28,1]}</code> </p> </li> </ul> </li> </ul> </li> <li> <p> <code>MXNET/ONNX</code>: You must specify the name and shape (NCHW format) of the expected data inputs in order using a dictionary format for your trained model. The dictionary formats required for the console and CLI are different.</p> <ul> <li> <p>Examples for one input:</p> <ul> <li> <p>If using the console, <code>{&quot;data&quot;:[1,3,1024,1024]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;data&quot;:[1,3,1024,1024]}</code> </p> </li> </ul> </li> <li> <p>Examples for two inputs:</p> <ul> <li> <p>If using the console, <code>{&quot;var1&quot;: [1,1,28,28], &quot;var2&quot;:[1,1,28,28]} </code> </p> </li> <li> <p>If using the CLI, <code>{&quot;var1&quot;: [1,1,28,28], &quot;var2&quot;:[1,1,28,28]}</code> </p> </li> </ul> </li> </ul> </li> <li> <p> <code>PyTorch</code>: You can either specify the name and shape (NCHW format) of expected data inputs in order using a dictionary format for your trained model or you can specify the shape only using a list format. The dictionary formats required for the console and CLI are different. The list formats for the console and CLI are the same.</p> <ul> <li> <p>Examples for one input in dictionary format:</p> <ul> <li> <p>If using the console, <code>{&quot;input0&quot;:[1,3,224,224]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input0&quot;:[1,3,224,224]}</code> </p> </li> </ul> </li> <li> <p>Example for one input in list format: <code>[[1,3,224,224]]</code> </p> </li> <li> <p>Examples for two inputs in dictionary format:</p> <ul> <li> <p>If using the console, <code>{&quot;input0&quot;:[1,3,224,224], &quot;input1&quot;:[1,3,224,224]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input0&quot;:[1,3,224,224], &quot;input1&quot;:[1,3,224,224]} </code> </p> </li> </ul> </li> <li> <p>Example for two inputs in list format: <code>[[1,3,224,224], [1,3,224,224]]</code> </p> </li> </ul> </li> <li> <p> <code>XGBOOST</code>: input data name and shape are not needed.</p> </li> </ul></p>
    #[serde(rename = "DataInputConfig")]
    pub data_input_config: String,
    /// <p>Identifies the framework in which the model was trained. For example: TENSORFLOW.</p>
    #[serde(rename = "Framework")]
    pub framework: String,
    /// <p>The S3 path where the model artifacts, which result from model training, are stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix).</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
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
    /// <p><p>The scale that hyperparameter tuning uses to search the hyperparameter range. For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com//sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon SageMaker hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparemeter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have only values greater than 0.</p> </dd> </dl></p>
    #[serde(rename = "ScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

/// <p>Defines the possible values for an integer hyperparameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerParameterRangeSpecification {
    /// <p>The maximum integer value allowed.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: String,
    /// <p>The minimum integer value allowed.</p>
    #[serde(rename = "MinValue")]
    pub min_value: String,
}

/// <p>Provides a breakdown of the number of objects labeled.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelCounters {
    /// <p>The total number of objects that could not be labeled due to an error.</p>
    #[serde(rename = "FailedNonRetryableError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_non_retryable_error: Option<i64>,
    /// <p>The total number of objects labeled by a human worker.</p>
    #[serde(rename = "HumanLabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_labeled: Option<i64>,
    /// <p>The total number of objects labeled by automated data labeling.</p>
    #[serde(rename = "MachineLabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_labeled: Option<i64>,
    /// <p>The total number of objects labeled.</p>
    #[serde(rename = "TotalLabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_labeled: Option<i64>,
    /// <p>The total number of objects not yet labeled.</p>
    #[serde(rename = "Unlabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlabeled: Option<i64>,
}

/// <p>Provides counts for human-labeled tasks in the labeling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelCountersForWorkteam {
    /// <p>The total number of data objects labeled by a human worker.</p>
    #[serde(rename = "HumanLabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_labeled: Option<i64>,
    /// <p>The total number of data objects that need to be labeled by a human worker.</p>
    #[serde(rename = "PendingHuman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_human: Option<i64>,
    /// <p>The total number of tasks in the labeling job.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// <p>Provides configuration information for auto-labeling of your data objects. A <code>LabelingJobAlgorithmsConfig</code> object must be supplied in order to use auto-labeling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobAlgorithmsConfig {
    /// <p>At the end of an auto-label job Amazon SageMaker Ground Truth sends the Amazon Resource Nam (ARN) of the final model used for auto-labeling. You can use this model as the starting point for subsequent similar jobs by providing the ARN of the model here. </p>
    #[serde(rename = "InitialActiveLearningModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_active_learning_model_arn: Option<String>,
    /// <p><p>Specifies the Amazon Resource Name (ARN) of the algorithm used for auto-labeling. You must select one of the following ARNs:</p> <ul> <li> <p> <i>Image classification</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/image-classification</code> </p> </li> <li> <p> <i>Text classification</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/text-classification</code> </p> </li> <li> <p> <i>Object detection</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/object-detection</code> </p> </li> </ul></p>
    #[serde(rename = "LabelingJobAlgorithmSpecificationArn")]
    pub labeling_job_algorithm_specification_arn: String,
    /// <p>Provides configuration information for a labeling job.</p>
    #[serde(rename = "LabelingJobResourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_resource_config: Option<LabelingJobResourceConfig>,
}

/// <p>Attributes of the data specified by the customer. Use these to describe the data to be labeled.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content. Amazon SageMaker may restrict the Amazon Mechanical Turk workers that can view your task based on this information.</p>
    #[serde(rename = "ContentClassifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classifiers: Option<Vec<String>>,
}

/// <p>Provides information about the location of input data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobDataSource {
    /// <p>The Amazon S3 location of the input data objects.</p>
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: LabelingJobS3DataSource,
}

/// <p>Provides summary information for a work team.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelingJobForWorkteamSummary {
    /// <p>The date and time that the labeling job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>A unique identifier for a labeling job. You can use this to refer to a specific labeling job.</p>
    #[serde(rename = "JobReferenceCode")]
    pub job_reference_code: String,
    /// <p>Provides information about the progress of a labeling job.</p>
    #[serde(rename = "LabelCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_counters: Option<LabelCountersForWorkteam>,
    /// <p>The name of the labeling job that the work team is assigned to.</p>
    #[serde(rename = "LabelingJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_name: Option<String>,
    /// <p>The configured number of workers per data object.</p>
    #[serde(rename = "NumberOfHumanWorkersPerDataObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_human_workers_per_data_object: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "WorkRequesterAccountId")]
    pub work_requester_account_id: String,
}

/// <p>Input configuration information for a labeling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobInputConfig {
    /// <p>Attributes of the data specified by the customer.</p>
    #[serde(rename = "DataAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<LabelingJobDataAttributes>,
    /// <p>The location of the input data.</p>
    #[serde(rename = "DataSource")]
    pub data_source: LabelingJobDataSource,
}

/// <p>Specifies the location of the output produced by the labeling job. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelingJobOutput {
    /// <p>The Amazon Resource Name (ARN) for the most recent Amazon SageMaker model trained as part of automated data labeling. </p>
    #[serde(rename = "FinalActiveLearningModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_active_learning_model_arn: Option<String>,
    /// <p>The Amazon S3 bucket location of the manifest file for labeled data. </p>
    #[serde(rename = "OutputDatasetS3Uri")]
    pub output_dataset_s3_uri: String,
}

/// <p>Output configuration information for a labeling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobOutputConfig {
    /// <p>The AWS Key Management Service ID of the key used to encrypt the output data, if any.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The Amazon S3 location to write output data.</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Provides configuration information for labeling jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobResourceConfig {
    /// <p>The AWS Key Management Service key ID for the key used to encrypt the output data, if any.</p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
}

/// <p>The Amazon S3 location of the input data objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobS3DataSource {
    /// <p>The Amazon S3 location of the manifest file that describes the input data objects.</p>
    #[serde(rename = "ManifestS3Uri")]
    pub manifest_s3_uri: String,
}

/// <p>A set of conditions for stopping a labeling job. If any of the conditions are met, the job is automatically stopped. You can use these conditions to control the cost of data labeling.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelingJobStoppingConditions {
    /// <p>The maximum number of objects that can be labeled by human workers.</p>
    #[serde(rename = "MaxHumanLabeledObjectCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_human_labeled_object_count: Option<i64>,
    /// <p>The maximum number of input data objects that should be labeled.</p>
    #[serde(rename = "MaxPercentageOfInputDatasetLabeled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_percentage_of_input_dataset_labeled: Option<i64>,
}

/// <p>Provides summary information about a labeling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelingJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used to consolidate the annotations from individual workers into a label for a data object. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-annotation-consolidation.html">Annotation Consolidation</a>.</p>
    #[serde(rename = "AnnotationConsolidationLambdaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_consolidation_lambda_arn: Option<String>,
    /// <p>The date and time that the job was created (timestamp).</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If the <code>LabelingJobStatus</code> field is <code>Failed</code>, this field contains a description of the error.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Input configuration for the labeling job.</p>
    #[serde(rename = "InputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_config: Option<LabelingJobInputConfig>,
    /// <p>Counts showing the progress of the labeling job.</p>
    #[serde(rename = "LabelCounters")]
    pub label_counters: LabelCounters,
    /// <p>The Amazon Resource Name (ARN) assigned to the labeling job when it was created.</p>
    #[serde(rename = "LabelingJobArn")]
    pub labeling_job_arn: String,
    /// <p>The name of the labeling job.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
    /// <p>The location of the output produced by the labeling job.</p>
    #[serde(rename = "LabelingJobOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_output: Option<LabelingJobOutput>,
    /// <p>The current status of the labeling job. </p>
    #[serde(rename = "LabelingJobStatus")]
    pub labeling_job_status: String,
    /// <p>The date and time that the job was last modified (timestamp).</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>The Amazon Resource Name (ARN) of a Lambda function. The function is run before each data object is sent to a worker.</p>
    #[serde(rename = "PreHumanTaskLambdaArn")]
    pub pre_human_task_lambda_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the work team assigned to the job.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAlgorithmsInput {
    /// <p>A filter that returns only algorithms created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only algorithms created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The maximum number of algorithms to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the algorithm name. This filter returns only algorithms whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the response to a previous <code>ListAlgorithms</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of algorithms, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAlgorithmsOutput {
    /// <p>&gt;An array of <code>AlgorithmSummary</code> objects, each of which lists an algorithm.</p>
    #[serde(rename = "AlgorithmSummaryList")]
    pub algorithm_summary_list: Vec<AlgorithmSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of algorithms, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCodeRepositoriesInput {
    /// <p>A filter that returns only Git repositories that were created after the specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only Git repositories that were created before the specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only Git repositories that were last modified after the specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only Git repositories that were last modified before the specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of Git repositories to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the Git repositories name. This filter returns only repositories whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of a <code>ListCodeRepositoriesOutput</code> request was truncated, the response includes a <code>NextToken</code>. To get the next set of Git repositories, use the token in the next request.</p>
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListCodeRepositoriesOutput {
    /// <p><p>Gets a list of summaries of the Git repositories. Each summary specifies the following values for the repository: </p> <ul> <li> <p>Name</p> </li> <li> <p>Amazon Resource Name (ARN)</p> </li> <li> <p>Creation time</p> </li> <li> <p>Last modified time</p> </li> <li> <p>Configuration information, including the URL location of the repository and the ARN of the AWS Secrets Manager secret that contains the credentials used to access the repository.</p> </li> </ul></p>
    #[serde(rename = "CodeRepositorySummaryList")]
    pub code_repository_summary_list: Vec<CodeRepositorySummary>,
    /// <p>If the result of a <code>ListCodeRepositoriesOutput</code> request was truncated, the response includes a <code>NextToken</code>. To get the next set of Git repositories, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCompilationJobsRequest {
    /// <p>A filter that returns the model compilation jobs that were created after a specified time. </p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns the model compilation jobs that were created before a specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns the model compilation jobs that were modified after a specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns the model compilation jobs that were modified before a specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of model compilation jobs to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A filter that returns the model compilation jobs whose name contains a specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListCompilationJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model compilation jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The field by which to sort results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that retrieves model compilation jobs with a specific <a>DescribeCompilationJobResponse$CompilationJobStatus</a> status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListCompilationJobsResponse {
    /// <p>An array of <a>CompilationJobSummary</a> objects, each describing a model compilation job. </p>
    #[serde(rename = "CompilationJobSummaries")]
    pub compilation_job_summaries: Vec<CompilationJobSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this <code>NextToken</code>. To retrieve the next set of model compilation jobs, use this token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEndpointConfigsInput {
    /// <p>A filter that returns only endpoint configurations with a creation time greater than or equal to the specified time (timestamp).</p>
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
    /// <p>The sort order for results. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>A filter that returns only endpoints with a creation time greater than or equal to the specified time (timestamp).</p>
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
    /// <p>The sort order for results. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p> A filter that returns only endpoints with the specified status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListLabelingJobsForWorkteamRequest {
    /// <p>A filter that returns only labeling jobs created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only labeling jobs created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter the limits jobs to only the ones whose job reference code contains the specified string.</p>
    #[serde(rename = "JobReferenceCodeContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference_code_contains: Option<String>,
    /// <p>The maximum number of labeling jobs to return in each page of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous <code>ListLabelingJobsForWorkteam</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of labeling jobs, use the token in the next request.</p>
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
    /// <p>The Amazon Resource Name (ARN) of the work team for which you want to see labeling jobs for.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLabelingJobsForWorkteamResponse {
    /// <p>An array of <code>LabelingJobSummary</code> objects, each describing a labeling job.</p>
    #[serde(rename = "LabelingJobSummaryList")]
    pub labeling_job_summary_list: Vec<LabelingJobForWorkteamSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of labeling jobs, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLabelingJobsRequest {
    /// <p>A filter that returns only labeling jobs created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only labeling jobs created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only labeling jobs modified after the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only labeling jobs modified before the specified time (timestamp).</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of labeling jobs to return in each page of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the labeling job name. This filter returns only labeling jobs whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListLabelingJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of labeling jobs, use the token in the next request.</p>
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
    /// <p>A filter that retrieves only labeling jobs with a specific status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLabelingJobsResponse {
    /// <p>An array of <code>LabelingJobSummary</code> objects, each describing a labeling job.</p>
    #[serde(rename = "LabelingJobSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_summary_list: Option<Vec<LabelingJobSummary>>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of labeling jobs, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListModelPackagesInput {
    /// <p>A filter that returns only model packages created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only model packages created before the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The maximum number of model packages to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the model package name. This filter returns only model packages whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the response to a previous <code>ListModelPackages</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model packages, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is <code>Ascending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListModelPackagesOutput {
    /// <p>An array of <code>ModelPackageSummary</code> objects, each of which lists a model package.</p>
    #[serde(rename = "ModelPackageSummaryList")]
    pub model_package_summary_list: Vec<ModelPackageSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of model packages, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListModelsInput {
    /// <p>A filter that returns only models with a creation time greater than or equal to the specified time (timestamp).</p>
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
    /// <p>The sort order for results. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>A filter that returns only notebook instances with associated with the specified git repository.</p>
    #[serde(rename = "AdditionalCodeRepositoryEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repository_equals: Option<String>,
    /// <p>A filter that returns only notebook instances that were created after the specified time (timestamp).</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only notebook instances that were created before the specified time (timestamp). </p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A string in the name or URL of a Git repository associated with this notebook instance. This filter returns only notebook instances associated with a git repository with a name that contains the specified string.</p>
    #[serde(rename = "DefaultCodeRepositoryContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository_contains: Option<String>,
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
    /// <p><p> If the previous call to the <code>ListNotebookInstances</code> is truncated, the response includes a <code>NextToken</code>. You can use this token in your subsequent <code>ListNotebookInstances</code> request to fetch the next set of notebook instances. </p> <note> <p>You might specify a filter or a sort order in your request. When response is truncated, you must use the same values for the filer and sort order in the next request. </p> </note></p>
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListSubscribedWorkteamsRequest {
    /// <p>The maximum number of work teams to return in each page of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the work team name. This filter returns only work teams whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListSubscribedWorkteams</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of labeling jobs, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSubscribedWorkteamsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of work teams, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>Workteam</code> objects, each describing a work team.</p>
    #[serde(rename = "SubscribedWorkteams")]
    pub subscribed_workteams: Vec<SubscribedWorkteam>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListTransformJobsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of transform jobs, use it in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>TransformJobSummary</code> objects.</p>
    #[serde(rename = "TransformJobSummaries")]
    pub transform_job_summaries: Vec<TransformJobSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWorkteamsRequest {
    /// <p>The maximum number of work teams to return in each page of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the work team's name. This filter returns only work teams whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListWorkteams</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of labeling jobs, use the token in the next request.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListWorkteamsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of work teams, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>Workteam</code> objects, each describing a work team.</p>
    #[serde(rename = "Workteams")]
    pub workteams: Vec<Workteam>,
}

/// <p>Defines the Amazon Cognito user group that is part of a work team.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberDefinition {
    /// <p>The Amazon Cognito user group that is part of the work team.</p>
    #[serde(rename = "CognitoMemberDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_member_definition: Option<CognitoMemberDefinition>,
}

/// <p>The name, value, and date and time of a metric that was emitted to Amazon CloudWatch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MetricData {
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The date and time that the algorithm emitted the metric.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
}

/// <p>Specifies a metric that the training algorithm writes to <code>stderr</code> or <code>stdout</code>. Amazon SageMakerhyperparameter tuning captures all defined metrics. You specify one metric that a hyperparameter tuning job uses as its objective metric to choose the best training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinition {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A regular expression that searches the output of a training job and gets the value of the metric. For more information about using regular expressions to define metrics, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-metrics.html">Defining Objective Metrics</a>.</p>
    #[serde(rename = "Regex")]
    pub regex: String,
}

/// <p>Provides information about the location that is configured for storing model artifacts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModelArtifacts {
    /// <p>The path of the S3 object that contains the model artifacts. For example, <code>s3://bucket-name/keynameprefix/model.tar.gz</code>.</p>
    #[serde(rename = "S3ModelArtifacts")]
    pub s3_model_artifacts: String,
}

/// <p>Describes the Docker container for the model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPackageContainerDefinition {
    /// <p>The DNS host name for the Docker container.</p>
    #[serde(rename = "ContainerHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_hostname: Option<String>,
    /// <p>The Amazon EC2 Container Registry (Amazon ECR) path where inference code is stored.</p> <p>If you are using your own custom algorithm instead of an algorithm provided by Amazon SageMaker, the inference code must meet Amazon SageMaker requirements. Amazon SageMaker supports both <code>registry/repository[:tag]</code> and <code>registry/repository[@digest]</code> image path formats. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>.</p>
    #[serde(rename = "Image")]
    pub image: String,
    /// <p>An MD5 hash of the training algorithm that identifies the Docker image used for training.</p>
    #[serde(rename = "ImageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p>
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<String>,
    /// <p>The AWS Marketplace product ID of the model package.</p>
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// <p>Specifies the validation and image scan statuses of the model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModelPackageStatusDetails {
    /// <p>The status of the scan of the Docker image container for the model package.</p>
    #[serde(rename = "ImageScanStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_statuses: Option<Vec<ModelPackageStatusItem>>,
    /// <p>The validation status of the model package.</p>
    #[serde(rename = "ValidationStatuses")]
    pub validation_statuses: Vec<ModelPackageStatusItem>,
}

/// <p>Represents the overall status of a model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModelPackageStatusItem {
    /// <p>if the overall status is <code>Failed</code>, the reason for the failure.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the model package for which the overall status is being reported.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The current status.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Provides summary information about a model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModelPackageSummary {
    /// <p>A timestamp that shows when the model package was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the model package.</p>
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: String,
    /// <p>A brief description of the model package.</p>
    #[serde(rename = "ModelPackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_description: Option<String>,
    /// <p>The name of the model package.</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
    /// <p>The overall status of the model package.</p>
    #[serde(rename = "ModelPackageStatus")]
    pub model_package_status: String,
}

/// <p>Contains data, such as the inputs and targeted instance types that are used in the process of validating the model package.</p> <p>The data provided in the validation profile is made available to your buyers on AWS Marketplace.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPackageValidationProfile {
    /// <p>The name of the profile for the model package.</p>
    #[serde(rename = "ProfileName")]
    pub profile_name: String,
    /// <p>The <code>TransformJobDefinition</code> object that describes the transform job used for the validation of the model package.</p>
    #[serde(rename = "TransformJobDefinition")]
    pub transform_job_definition: TransformJobDefinition,
}

/// <p>Specifies batch transform jobs that Amazon SageMaker runs to validate your model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPackageValidationSpecification {
    /// <p>An array of <code>ModelPackageValidationProfile</code> objects, each of which specifies a batch transform job that Amazon SageMaker runs to validate your model package.</p>
    #[serde(rename = "ValidationProfiles")]
    pub validation_profiles: Vec<ModelPackageValidationProfile>,
    /// <p>The IAM roles to be used for the validation of the model package.</p>
    #[serde(rename = "ValidationRole")]
    pub validation_role: String,
}

/// <p>Provides summary information about a model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p><p>Defines a list of <code>NestedFilters</code> objects. To satisfy the conditions specified in the <code>NestedFilters</code> call, a resource must satisfy the conditions of all of the filters.</p> <p>For example, you could define a <code>NestedFilters</code> using the training job&#39;s <code>InputDataConfig</code> property to filter on <code>Channel</code> objects. </p> <p>A <code>NestedFilters</code> object contains multiple filters. For example, to find all training jobs whose name contains <code>train</code> and that have <code>cat/data</code> in their <code>S3Uri</code> (specified in <code>InputDataConfig</code>), you need to create a <code>NestedFilters</code> object that specifies the <code>InputDataConfig</code> property with the following <code>Filter</code> objects:</p> <ul> <li> <p> <code>&#39;{Name:&quot;InputDataConfig.ChannelName&quot;, &quot;Operator&quot;:&quot;EQUALS&quot;, &quot;Value&quot;:&quot;train&quot;}&#39;,</code> </p> </li> <li> <p> <code>&#39;{Name:&quot;InputDataConfig.DataSource.S3DataSource.S3Uri&quot;, &quot;Operator&quot;:&quot;CONTAINS&quot;, &quot;Value&quot;:&quot;cat/data&quot;}&#39;</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NestedFilters {
    /// <p>A list of filters. Each filter acts on a property. Filters must contain at least one <code>Filters</code> value. For example, a <code>NestedFilters</code> call might include a filter on the <code>PropertyName</code> parameter of the <code>InputDataConfig</code> property: <code>InputDataConfig.DataSource.S3DataSource.S3Uri</code>.</p>
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
    /// <p>The name of the property to use in the nested filters. The value must match a listed property name, such as <code>InputDataConfig</code>.</p>
    #[serde(rename = "NestedPropertyName")]
    pub nested_property_name: String,
}

/// <p>Provides a summary of a notebook instance lifecycle configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Contains the notebook instance lifecycle configuration script.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookInstanceLifecycleHook {
    /// <p>A base64-encoded string that contains a shell script for a notebook instance lifecycle configuration.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// <p>Provides summary information for an Amazon SageMaker notebook instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NotebookInstanceSummary {
    /// <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A timestamp that shows when the notebook instance was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository: Option<String>,
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
    /// <p>The name of a notebook instance lifecycle configuration associated with this notebook instance.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
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

/// <p>Configures SNS notifications of available or expiring work items for work teams.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfiguration {
    /// <p>The ARN for the SNS topic to which notifications should be published.</p>
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
}

/// <p>Specifies the number of training jobs that this hyperparameter tuning job launched, categorized by the status of their objective metric. The objective metric status shows whether the final objective metric for the training job has been evaluated by the tuning job and used in the hyperparameter tuning process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Contains information about the output location for the compiled model and the device (target) that the model runs on.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputConfig {
    /// <p>Identifies the S3 path where you want Amazon SageMaker to store the model artifacts. For example, s3://bucket-name/key-name-prefix.</p>
    #[serde(rename = "S3OutputLocation")]
    pub s3_output_location: String,
    /// <p>Identifies the device that you want to run your model on after it has been compiled. For example: ml_c5.</p>
    #[serde(rename = "TargetDevice")]
    pub target_device: String,
}

/// <p>Provides information about how to store model training results (model artifacts).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDataConfig {
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption. The <code>KmsKeyId</code> can be any of the following formats: </p> <ul> <li> <p>// KMS Key ID</p> <p> <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// KMS Key Alias</p> <p> <code>"alias/ExampleAlias"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key Alias</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias"</code> </p> </li> </ul> <p>If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateTramsformJob</code> request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Identifies the S3 path where you want Amazon SageMaker to store the model artifacts. For example, <code>s3://bucket-name/key-name-prefix</code>. </p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Defines the possible values for categorical, continuous, and integer hyperparameters to be used by an algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterRange {
    /// <p>A <code>CategoricalParameterRangeSpecification</code> object that defines the possible values for a categorical hyperparameter.</p>
    #[serde(rename = "CategoricalParameterRangeSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_parameter_range_specification: Option<CategoricalParameterRangeSpecification>,
    /// <p>A <code>ContinuousParameterRangeSpecification</code> object that defines the possible values for a continuous hyperparameter.</p>
    #[serde(rename = "ContinuousParameterRangeSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_parameter_range_specification: Option<ContinuousParameterRangeSpecification>,
    /// <p>A <code>IntegerParameterRangeSpecification</code> object that defines the possible values for an integer hyperparameter.</p>
    #[serde(rename = "IntegerParameterRangeSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_range_specification: Option<IntegerParameterRangeSpecification>,
}

/// <p><p>Specifies ranges of integer, continuous, and categorical hyperparameters that a hyperparameter tuning job searches. The hyperparameter tuning job launches training jobs with hyperparameter values within these ranges to find the combination of values that result in the training job with the best performance as measured by the objective metric of the hyperparameter tuning job.</p> <note> <p>You can specify a maximum of 20 hyperparameters that a hyperparameter tuning job can search over. Every possible value of a categorical parameter range counts against this limit.</p> </note></p>
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

/// <p>A previously completed or stopped hyperparameter tuning job to be used as a starting point for a new hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentHyperParameterTuningJob {
    /// <p>The name of the hyperparameter tuning job to be used as a starting point for a new hyperparameter tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_tuning_job_name: Option<String>,
}

/// <p>Identifies a model that you want to host and the resources to deploy for hosting it. If you are deploying multiple models, tell Amazon SageMaker how to distribute traffic among the models by specifying variant weights. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionVariant {
    /// <p>The size of the Elastic Inference (EI) instance to use for the production variant. EI instances provide on-demand GPU computing for inference. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>A type of <code>SuggestionQuery</code>. A suggestion query for retrieving property names that match the specified hint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PropertyNameQuery {
    /// <p>Text that is part of a property's name. The property names of hyperparameter, metric, and tag key names that begin with the specified text in the <code>PropertyNameHint</code>.</p>
    #[serde(rename = "PropertyNameHint")]
    pub property_name_hint: String,
}

/// <p>A property name returned from a <code>GetSearchSuggestions</code> call that specifies a value in the <code>PropertyNameQuery</code> field.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PropertyNameSuggestion {
    /// <p>A suggested property name based on what you entered in the search textbox in the Amazon SageMaker console.</p>
    #[serde(rename = "PropertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}

/// <p><p>Defines the amount of money paid to an Amazon Mechanical Turk worker for each task performed. </p> <p>Use one of the following prices for bounding box tasks. Prices are in US dollars.</p> <ul> <li> <p>0.036</p> </li> <li> <p>0.048</p> </li> <li> <p>0.060</p> </li> <li> <p>0.072</p> </li> <li> <p>0.120</p> </li> <li> <p>0.240</p> </li> <li> <p>0.360</p> </li> <li> <p>0.480</p> </li> <li> <p>0.600</p> </li> <li> <p>0.720</p> </li> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul> <p>Use one of the following prices for image classification, text classification, and custom tasks. Prices are in US dollars.</p> <ul> <li> <p>0.012</p> </li> <li> <p>0.024</p> </li> <li> <p>0.036</p> </li> <li> <p>0.048</p> </li> <li> <p>0.060</p> </li> <li> <p>0.072</p> </li> <li> <p>0.120</p> </li> <li> <p>0.240</p> </li> <li> <p>0.360</p> </li> <li> <p>0.480</p> </li> <li> <p>0.600</p> </li> <li> <p>0.720</p> </li> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul> <p>Use one of the following prices for semantic segmentation tasks. Prices are in US dollars.</p> <ul> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicWorkforceTaskPrice {
    /// <p>Defines the amount of money paid to a worker in United States dollars.</p>
    #[serde(rename = "AmountInUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in_usd: Option<USD>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RenderUiTemplateRequest {
    /// <p>The Amazon Resource Name (ARN) that has access to the S3 objects that are used by the template.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>A <code>RenderableTask</code> object containing a representative task to render.</p>
    #[serde(rename = "Task")]
    pub task: RenderableTask,
    /// <p>A <code>Template</code> object containing the worker UI template to render.</p>
    #[serde(rename = "UiTemplate")]
    pub ui_template: UiTemplate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RenderUiTemplateResponse {
    /// <p>A list of one or more <code>RenderingError</code> objects if any were encountered while rendering the template. If there were no errors, the list is empty.</p>
    #[serde(rename = "Errors")]
    pub errors: Vec<RenderingError>,
    /// <p>A Liquid template that renders the HTML for the worker UI.</p>
    #[serde(rename = "RenderedContent")]
    pub rendered_content: String,
}

/// <p>Contains input values for a task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RenderableTask {
    /// <p>A JSON object that contains values for the variables defined in the template. It is made available to the template under the substitution variable <code>task.input</code>. For example, if you define a variable <code>task.input.text</code> in your template, you can supply the variable in the JSON object as <code>"text": "sample text"</code>.</p>
    #[serde(rename = "Input")]
    pub input: String,
}

/// <p>A description of an error that occurred while rendering the template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RenderingError {
    /// <p>A unique identifier for a specific class of errors.</p>
    #[serde(rename = "Code")]
    pub code: String,
    /// <p>A human-readable message describing the error.</p>
    #[serde(rename = "Message")]
    pub message: String,
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
    /// <p><p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the training job. The <code>VolumeKmsKeyId</code> can be any of the following formats:</p> <ul> <li> <p>// KMS Key ID</p> <p> <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
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
    /// <p>A list of one or more attribute names to use that are found in a specified augmented manifest file.</p>
    #[serde(rename = "AttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    /// <p>If you want Amazon SageMaker to replicate the entire dataset on each ML compute instance that is launched for model training, specify <code>FullyReplicated</code>. </p> <p>If you want Amazon SageMaker to replicate a subset of data on each ML compute instance that is launched for model training, specify <code>ShardedByS3Key</code>. If there are <i>n</i> ML compute instances launched for a training job, each instance gets approximately 1/<i>n</i> of the number of S3 objects. In this case, model training on each machine uses only the subset of training data. </p> <p>Don't choose more ML compute instances for training than available S3 objects. If you do, some nodes won't get any data and you will pay for nodes that aren't getting any training data. This applies in both File and Pipe modes. Keep this in mind when developing algorithms. </p> <p>In distributed training, where you use multiple ML compute EC2 instances, you might choose <code>ShardedByS3Key</code>. If the algorithm requires copying training data to the ML storage volume (when <code>TrainingInputMode</code> is set to <code>File</code>), this copies 1/<i>n</i> of the number of objects. </p>
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<String>,
    /// <p>If you choose <code>S3Prefix</code>, <code>S3Uri</code> identifies a key name prefix. Amazon SageMaker uses all objects that match the specified key name prefix for model training. </p> <p>If you choose <code>ManifestFile</code>, <code>S3Uri</code> identifies an object that is a manifest file containing a list of object keys that you want Amazon SageMaker to use for model training. </p> <p>If you choose <code>AugmentedManifestFile</code>, S3Uri identifies an object that is an augmented manifest file in JSON lines format. This file contains the data you want to use for model training. <code>AugmentedManifestFile</code> can only be used if the Channel's input mode is <code>Pipe</code>.</p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example: </p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> <code>[</code> </p> <p> <code> {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code> &quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code> &quot;relative/path/custdata-2&quot;,</code> </p> <p> <code> ...</code> </p> <p> <code> ]</code> </p> <p> The preceding JSON matches the following <code>s3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer_bucket/some/prefix/relative/path/custdata-2</code> </p> <p> <code>...</code> </p> <p>The complete set of <code>s3uris</code> in this manifest is the input data for the channel for this datasource. The object that each <code>s3uris</code> points to must be readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf. </p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p><p>A multi-expression that searches for the specified resource or resources in a search. All resource objects that satisfy the expression&#39;s condition are included in the search results. You must specify at least one subexpression, filter, or nested filter. A <code>SearchExpression</code> can contain up to twenty elements.</p> <p>A <code>SearchExpression</code> contains the following components:</p> <ul> <li> <p>A list of <code>Filter</code> objects. Each filter defines a simple Boolean expression comprised of a resource property name, Boolean operator, and value.</p> </li> <li> <p>A list of <code>NestedFilter</code> objects. Each nested filter defines a list of Boolean expressions using a list of resource properties. A nested filter is satisfied if a single object in the list satisfies all Boolean expressions.</p> </li> <li> <p>A list of <code>SearchExpression</code> objects. A search expression object can be nested in a list of search expression objects.</p> </li> <li> <p>A Boolean operator: <code>And</code> or <code>Or</code>.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchExpression {
    /// <p>A list of filter objects.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>A list of nested filter objects.</p>
    #[serde(rename = "NestedFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_filters: Option<Vec<NestedFilters>>,
    /// <p>A Boolean operator used to evaluate the search expression. If you want every conditional statement in all lists to be satisfied for the entire search expression to be true, specify <code>And</code>. If only a single conditional statement needs to be true for the entire search expression to be true, specify <code>Or</code>. The default value is <code>And</code>.</p>
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>A list of search expression objects.</p>
    #[serde(rename = "SubExpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_expressions: Option<Vec<SearchExpression>>,
}

/// <p>An individual search result record that contains a single resource object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchRecord {
    /// <p>A <code>TrainingJob</code> object that is returned as part of a <code>Search</code> request.</p>
    #[serde(rename = "TrainingJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job: Option<TrainingJob>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchRequest {
    /// <p>The maximum number of results to return in a <code>SearchResponse</code>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If more than <code>MaxResults</code> resource objects match the specified <code>SearchExpression</code>, the <code>SearchResponse</code> includes a <code>NextToken</code>. The <code>NextToken</code> can be passed to the next <code>SearchRequest</code> to continue retrieving results for the specified <code>SearchExpression</code> and <code>Sort</code> parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the Amazon SageMaker resource to search for. Currently, the only valid <code>Resource</code> value is <code>TrainingJob</code>.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A Boolean conditional statement. Resource objects must satisfy this condition to be included in search results. You must provide at least one subexpression, filter, or nested filter. The maximum number of recursive <code>SubExpressions</code>, <code>NestedFilters</code>, and <code>Filters</code> that can be included in a <code>SearchExpression</code> object is 50.</p>
    #[serde(rename = "SearchExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_expression: Option<SearchExpression>,
    /// <p>The name of the resource property used to sort the <code>SearchResults</code>. The default is <code>LastModifiedTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>How <code>SearchResults</code> are ordered. Valid values are <code>Ascending</code> or <code>Descending</code>. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchResponse {
    /// <p>If the result of the previous <code>Search</code> request was truncated, the response includes a NextToken. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>SearchResult</code> objects.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SearchRecord>>,
}

/// <p><p>An array element of <a>DescribeTrainingJobResponse$SecondaryStatusTransitions</a>. It provides additional details about a status that the training job has transitioned through. A training job can be in one of several states, for example, starting, downloading, training, or uploading. Within each state, there are a number of intermediate states. For example, within the starting state, Amazon SageMaker could be starting the training job or launching the ML instances. These transitional states are referred to as the job&#39;s secondary status. </p> <p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecondaryStatusTransition {
    /// <p>A timestamp that shows when the training job transitioned out of this secondary status state into another secondary status state or when the training job has ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>A timestamp that shows when the training job transitioned to the current secondary status state.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    /// <p><p>Contains a secondary status information from a training job.</p> <p>Status might be one of the following secondary statuses:</p> <dl> <dt>InProgress</dt> <dd> <ul> <li> <p> <code>Starting</code> - Starting the training job.</p> </li> <li> <p> <code>Downloading</code> - An optional stage for algorithms that support <code>File</code> training input mode. It indicates that data is being downloaded to the ML storage volumes.</p> </li> <li> <p> <code>Training</code> - Training is in progress.</p> </li> <li> <p> <code>Uploading</code> - Training is complete and the model artifacts are being uploaded to the S3 location.</p> </li> </ul> </dd> <dt>Completed</dt> <dd> <ul> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> </ul> </dd> <dt>Failed</dt> <dd> <ul> <li> <p> <code>Failed</code> - The training job has failed. The reason for the failure is returned in the <code>FailureReason</code> field of <code>DescribeTrainingJobResponse</code>.</p> </li> </ul> </dd> <dt>Stopped</dt> <dd> <ul> <li> <p> <code>MaxRuntimeExceeded</code> - The job stopped because it exceeded the maximum allowed runtime.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> </dd> <dt>Stopping</dt> <dd> <ul> <li> <p> <code>Stopping</code> - Stopping the training job.</p> </li> </ul> </dd> </dl> <p>We no longer support the following secondary statuses:</p> <ul> <li> <p> <code>LaunchingMLInstances</code> </p> </li> <li> <p> <code>PreparingTrainingStack</code> </p> </li> <li> <p> <code>DownloadingTrainingImage</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p><p>A detailed description of the progress within a secondary status. </p> <p>Amazon SageMaker provides secondary statuses and status messages that apply to each of them:</p> <dl> <dt>Starting</dt> <dd> <ul> <li> <p>Starting the training job.</p> </li> <li> <p>Launching requested ML instances.</p> </li> <li> <p>Insufficient capacity error from EC2 while launching instances, retrying!</p> </li> <li> <p>Launched instance was unhealthy, replacing it!</p> </li> <li> <p>Preparing the instances for training.</p> </li> </ul> </dd> <dt>Training</dt> <dd> <ul> <li> <p>Downloading the training image.</p> </li> <li> <p>Training image download completed. Training in progress.</p> </li> </ul> </dd> </dl> <important> <p>Status messages are subject to change. Therefore, we recommend not including them in code that programmatically initiates actions. For examples, don&#39;t use status messages in if statements.</p> </important> <p>To have an overview of your training job&#39;s progress, view <code>TrainingJobStatus</code> and <code>SecondaryStatus</code> in <a>DescribeTrainingJobResponse</a>, and <code>StatusMessage</code> together. For example, at the start of a training job, you might see the following:</p> <ul> <li> <p> <code>TrainingJobStatus</code> - InProgress</p> </li> <li> <p> <code>SecondaryStatus</code> - Training</p> </li> <li> <p> <code>StatusMessage</code> - Downloading the training image</p> </li> </ul></p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>A configuration for a shuffle option for input data in a channel. If you use <code>S3Prefix</code> for <code>S3DataType</code>, the results of the S3 key prefix matches are shuffled. If you use <code>ManifestFile</code>, the order of the S3 object references in the <code>ManifestFile</code> is shuffled. If you use <code>AugmentedManifestFile</code>, the order of the JSON lines in the <code>AugmentedManifestFile</code> is shuffled. The shuffling order is determined using the <code>Seed</code> value.</p> <p>For Pipe input mode, shuffling is done at the start of every epoch. With large datasets, this ensures that the order of the training data is different for each epoch, and it helps reduce bias and possible overfitting. In a multi-node training job when <code>ShuffleConfig</code> is combined with <code>S3DataDistributionType</code> of <code>ShardedByS3Key</code>, the data is shuffled across nodes so that the content sent to a particular node on the first epoch might be sent to a different node on the second epoch.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShuffleConfig {
    /// <p>Determines the shuffling order in <code>ShuffleConfig</code> value.</p>
    #[serde(rename = "Seed")]
    pub seed: i64,
}

/// <p>Specifies an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your Amazon SageMaker account or an algorithm in AWS Marketplace that you are subscribed to.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceAlgorithm {
    /// <p>The name of an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your Amazon SageMaker account or an algorithm in AWS Marketplace that you are subscribed to.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p>
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<String>,
}

/// <p>A list of algorithms that were used to create a model package.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceAlgorithmSpecification {
    /// <p>A list of the algorithms that were used to create a model package.</p>
    #[serde(rename = "SourceAlgorithms")]
    pub source_algorithms: Vec<SourceAlgorithm>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartNotebookInstanceInput {
    /// <p>The name of the notebook instance to start.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopCompilationJobRequest {
    /// <p>The name of the model compilation job to stop.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job to stop.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopLabelingJobRequest {
    /// <p>The name of the labeling job to stop.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
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
    /// <p>The maximum length of time, in seconds, that the training job can run. If model training does not complete during this time, Amazon SageMaker ends the job. If value is not specified, default value is 1 day. Maximum value is 28 days.</p>
    #[serde(rename = "MaxRuntimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_in_seconds: Option<i64>,
}

/// <p>Describes a work team of a vendor that does the a labelling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubscribedWorkteam {
    /// <p><p/></p>
    #[serde(rename = "ListingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,
    /// <p>The description of the vendor from the Amazon Marketplace.</p>
    #[serde(rename = "MarketplaceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_description: Option<String>,
    /// <p>The title of the service provided by the vendor in the Amazon Marketplace.</p>
    #[serde(rename = "MarketplaceTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_title: Option<String>,
    /// <p>The name of the vendor in the Amazon Marketplace.</p>
    #[serde(rename = "SellerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the vendor that you have subscribed.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

/// <p>Limits the property names that are included in the response.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SuggestionQuery {
    /// <p>A type of <code>SuggestionQuery</code>. Defines a property name hint. Only property names that match the specified hint are included in the response.</p>
    #[serde(rename = "PropertyNameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name_query: Option<PropertyNameQuery>,
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

/// <p>Contains information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TrainingJob {
    /// <p>Information about the algorithm used for training, and algorithm metadata.</p>
    #[serde(rename = "AlgorithmSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_specification: Option<AlgorithmSpecification>,
    /// <p>A timestamp that indicates when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>If the <code>TrainingJob</code> was created with network isolation, the value is set to <code>true</code>. If network isolation is enabled, nodes can't communicate beyond the VPC they run in.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    /// <p>If the training job failed, the reason it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A list of final metric values that are set when the training job completes. Used only if the training job was configured to use metrics.</p>
    #[serde(rename = "FinalMetricDataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_metric_data_list: Option<Vec<MetricData>>,
    /// <p>Algorithm-specific parameters.</p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects that describes each data input channel.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<Channel>>,
    /// <p>The Amazon Resource Name (ARN) of the labeling job.</p>
    #[serde(rename = "LabelingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeling_job_arn: Option<String>,
    /// <p>A timestamp that indicates when the status of the training job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>Information about the Amazon S3 location that is configured for storing model artifacts.</p>
    #[serde(rename = "ModelArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_artifacts: Option<ModelArtifacts>,
    /// <p>The S3 path where model artifacts that you configured when creating the job are stored. Amazon SageMaker creates subfolders for model artifacts.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>Resources, including ML compute instances and ML storage volumes, that are configured for model training.</p>
    #[serde(rename = "ResourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<ResourceConfig>,
    /// <p>The AWS Identity and Access Management (IAM) role configured for the training job.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p> Provides detailed information about the state of the training job. For detailed information about the secondary status of the training job, see <code>StatusMessage</code> under <a>SecondaryStatusTransition</a>.</p> <p>Amazon SageMaker provides primary statuses and secondary statuses that apply to each of them:</p> <dl> <dt>InProgress</dt> <dd> <ul> <li> <p> <code>Starting</code> - Starting the training job.</p> </li> <li> <p> <code>Downloading</code> - An optional stage for algorithms that support <code>File</code> training input mode. It indicates that data is being downloaded to the ML storage volumes.</p> </li> <li> <p> <code>Training</code> - Training is in progress.</p> </li> <li> <p> <code>Uploading</code> - Training is complete and the model artifacts are being uploaded to the S3 location.</p> </li> </ul> </dd> <dt>Completed</dt> <dd> <ul> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> </ul> </dd> <dt>Failed</dt> <dd> <ul> <li> <p> <code>Failed</code> - The training job has failed. The reason for the failure is returned in the <code>FailureReason</code> field of <code>DescribeTrainingJobResponse</code>.</p> </li> </ul> </dd> <dt>Stopped</dt> <dd> <ul> <li> <p> <code>MaxRuntimeExceeded</code> - The job stopped because it exceeded the maximum allowed runtime.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> </dd> <dt>Stopping</dt> <dd> <ul> <li> <p> <code>Stopping</code> - Stopping the training job.</p> </li> </ul> </dd> </dl> <important> <p>Valid values for <code>SecondaryStatus</code> are subject to change. </p> </important> <p>We no longer support the following secondary statuses:</p> <ul> <li> <p> <code>LaunchingMLInstances</code> </p> </li> <li> <p> <code>PreparingTrainingStack</code> </p> </li> <li> <p> <code>DownloadingTrainingImage</code> </p> </li> </ul></p>
    #[serde(rename = "SecondaryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_status: Option<String>,
    /// <p>A history of all of the secondary statuses that the training job has transitioned through.</p>
    #[serde(rename = "SecondaryStatusTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_status_transitions: Option<Vec<SecondaryStatusTransition>>,
    /// <p>The condition under which to stop the training job.</p>
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<StoppingCondition>,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Indicates the time when the training job ends on training instances. You are billed for the time interval between the value of <code>TrainingStartTime</code> and this time. For successful jobs and stopped jobs, this is the time after model artifacts are uploaded. For failed jobs, this is the time when Amazon SageMaker detects a job failure.</p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_arn: Option<String>,
    /// <p>The name of the training job.</p>
    #[serde(rename = "TrainingJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_name: Option<String>,
    /// <p>The status of the training job.</p> <p>Training job statuses are:</p> <ul> <li> <p> <code>InProgress</code> - The training is in progress.</p> </li> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> <li> <p> <code>Failed</code> - The training job has failed. To see the reason for the failure, see the <code>FailureReason</code> field in the response to a <code>DescribeTrainingJobResponse</code> call.</p> </li> <li> <p> <code>Stopping</code> - The training job is stopping.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> <p>For more detailed information, see <code>SecondaryStatus</code>. </p>
    #[serde(rename = "TrainingJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_status: Option<String>,
    /// <p>Indicates the time when the training job starts on training instances. You are billed for the time interval between this time and the value of <code>TrainingEndTime</code>. The start time in CloudWatch Logs might be later than this time. The difference is due to the time it takes to download the training data and to the size of the training container.</p>
    #[serde(rename = "TrainingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the associated hyperparameter tuning job if the training job was launched by a hyperparameter tuning job.</p>
    #[serde(rename = "TuningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_job_arn: Option<String>,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that this training job has access to. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Defines the input needed to run a training job using the algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingJobDefinition {
    /// <p>The hyperparameters used for the training job.</p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects, each of which specifies an input source.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<Channel>,
    /// <p>the path to the S3 bucket where you want to store model artifacts. Amazon SageMaker creates subfolders for the artifacts.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The resources, including the ML compute instances and ML storage volumes, to use for model training.</p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p>Sets a duration for training. Use this parameter to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>The input mode used by the algorithm for the training job. For the input modes that Amazon SageMaker algorithms support, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>.</p> <p>If an algorithm supports the <code>File</code> input mode, Amazon SageMaker downloads the training data from S3 to the provisioned ML storage Volume, and mounts the directory to docker volume for training container. If an algorithm supports the <code>Pipe</code> input mode, Amazon SageMaker streams data directly from S3 to the container.</p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>The numbers of training jobs launched by a hyperparameter tuning job, categorized by status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TrainingJobStatusCounters {
    /// <p>The number of completed training jobs launched by the hyperparameter tuning job.</p>
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Defines how the algorithm is used for a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingSpecification {
    /// <p>A list of <code>MetricDefinition</code> objects, which are used for parsing metrics generated by the algorithm.</p>
    #[serde(rename = "MetricDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_definitions: Option<Vec<MetricDefinition>>,
    /// <p>A list of the <code>HyperParameterSpecification</code> objects, that define the supported hyperparameters. This is required if the algorithm supports automatic model tuning.&gt;</p>
    #[serde(rename = "SupportedHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_hyper_parameters: Option<Vec<HyperParameterSpecification>>,
    /// <p>A list of the instance types that this algorithm can use for training.</p>
    #[serde(rename = "SupportedTrainingInstanceTypes")]
    pub supported_training_instance_types: Vec<String>,
    /// <p>A list of the metrics that the algorithm emits that can be used as the objective metric in a hyperparameter tuning job.</p>
    #[serde(rename = "SupportedTuningJobObjectiveMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tuning_job_objective_metrics: Option<Vec<HyperParameterTuningJobObjective>>,
    /// <p>Indicates whether the algorithm supports distributed training. If set to false, buyers can’t request more than one instance during training.</p>
    #[serde(rename = "SupportsDistributedTraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_distributed_training: Option<bool>,
    /// <p>A list of <code>ChannelSpecification</code> objects, which specify the input sources to be used by the algorithm.</p>
    #[serde(rename = "TrainingChannels")]
    pub training_channels: Vec<ChannelSpecification>,
    /// <p>The Amazon ECR registry path of the Docker image that contains the training algorithm.</p>
    #[serde(rename = "TrainingImage")]
    pub training_image: String,
    /// <p>An MD5 hash of the training algorithm that identifies the Docker image used for training.</p>
    #[serde(rename = "TrainingImageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_image_digest: Option<String>,
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
    /// <p>If your transform data is compressed, specify the compression type. Amazon SageMaker automatically decompresses the data for the transform job accordingly. The default value is <code>None</code>.</p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p>The multipurpose internet mail extension (MIME) type of the data. Amazon SageMaker uses the MIME type with each http call to transfer data to the transform job.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Describes the location of the channel data, which is, the S3 location of the input data that the model can consume.</p>
    #[serde(rename = "DataSource")]
    pub data_source: TransformDataSource,
    /// <p><p>The method to use to split the transform job&#39;s data files into smaller batches. Splitting is necessary when the total size of each object is too large to fit in a single request. You can also use data splitting to improve performance by processing multiple concurrent mini-batches. The default value for <code>SplitType</code> is <code>None</code>, which indicates that input data files are not split, and request payloads contain the entire contents of an input object. Set the value of this parameter to <code>Line</code> to split records on a newline character boundary. <code>SplitType</code> also supports a number of record-oriented binary data formats.</p> <p>When splitting is enabled, the size of a mini-batch depends on the values of the <code>BatchStrategy</code> and <code>MaxPayloadInMB</code> parameters. When the value of <code>BatchStrategy</code> is <code>MultiRecord</code>, Amazon SageMaker sends the maximum number of records in each request, up to the <code>MaxPayloadInMB</code> limit. If the value of <code>BatchStrategy</code> is <code>SingleRecord</code>, Amazon SageMaker sends individual records in each request.</p> <note> <p>Some data formats represent a record as a binary payload wrapped with extra padding bytes. When splitting is applied to a binary data format, padding is removed if the value of <code>BatchStrategy</code> is set to <code>SingleRecord</code>. Padding is not removed if the value of <code>BatchStrategy</code> is set to <code>MultiRecord</code>.</p> <p>For more information about the RecordIO, see <a href="http://mxnet.io/architecture/note_data_loading.html#data-format">Data Format</a> in the MXNet documentation. For more information about the TFRecord, see <a href="https://www.tensorflow.org/guide/datasets#consuming_tfrecord_data">Consuming TFRecord data</a> in the TensorFlow documentation.</p> </note></p>
    #[serde(rename = "SplitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_type: Option<String>,
}

/// <p>Defines the input needed to run a transform job using the inference specification specified in the algorithm.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformJobDefinition {
    /// <p>A string that determines the number of records included in a single mini-batch.</p> <p> <code>SingleRecord</code> means only one record is used per mini-batch. <code>MultiRecord</code> means a mini-batch is set to contain as many records that can fit within the <code>MaxPayloadInMB</code> limit.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The maximum number of parallel requests that can be sent to each instance in a transform job. The default value is 1.</p>
    #[serde(rename = "MaxConcurrentTransforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transforms: Option<i64>,
    /// <p>The maximum payload size allowed, in MB. A payload is the data portion of a record (without metadata).</p>
    #[serde(rename = "MaxPayloadInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_in_mb: Option<i64>,
    /// <p>A description of the input source and the way the transform job consumes it.</p>
    #[serde(rename = "TransformInput")]
    pub transform_input: TransformInput,
    /// <p>Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p>
    #[serde(rename = "TransformOutput")]
    pub transform_output: TransformOutput,
    /// <p>Identifies the ML compute instances for the transform job.</p>
    #[serde(rename = "TransformResources")]
    pub transform_resources: TransformResources,
}

/// <p>Provides a summary of a transform job. Multiple <code>TransformJobSummary</code> objects are returned as a list after in response to a <a>ListTransformJobs</a> call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Describes the results of a transform job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformOutput {
    /// <p>The MIME type used to specify the output data. Amazon SageMaker uses the MIME type with each http call to transfer data from the transform job.</p>
    #[serde(rename = "Accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>Defines how to assemble the results of the transform job as a single S3 object. Choose a format that is most convenient to you. To concatenate the results in binary format, specify <code>None</code>. To add a newline character at the end of every transformed record, specify <code>Line</code>.</p>
    #[serde(rename = "AssembleWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assemble_with: Option<String>,
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption. The <code>KmsKeyId</code> can be any of the following formats: </p> <ul> <li> <p>// KMS Key ID</p> <p> <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// KMS Key Alias</p> <p> <code>"alias/ExampleAlias"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key Alias</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias"</code> </p> </li> </ul> <p>If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateTramsformJob</code> request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The Amazon S3 path where you want Amazon SageMaker to store the results of the transform job. For example, <code>s3://bucket-name/key-name-prefix</code>.</p> <p>For every S3 object used as input for the transform job, batch transform stores the transformed data with an .<code>out</code> suffix in a corresponding subfolder in the location in the output prefix. For example, for the input data stored at <code>s3://bucket-name/input-name-prefix/dataset01/data.csv</code>, batch transform stores the transformed data at <code>s3://bucket-name/output-name-prefix/input-name-prefix/data.csv.out</code>. Batch transform doesn't upload partially processed objects. For an input S3 object that contains multiple records, it creates an .<code>out</code> file only if the transform job succeeds on the entire file. When the input contains multiple S3 objects, the batch transform job processes the listed S3 objects and uploads only the output for successfully processed objects. If any object fails in the transform job batch transform marks the job as failed to prompt investigation.</p>
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
    /// <p><p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the batch transform job. The <code>VolumeKmsKeyId</code> can be any of the following formats:</p> <ul> <li> <p>// KMS Key ID</p> <p> <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
}

/// <p>Describes the S3 data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformS3DataSource {
    /// <p>If you choose <code>S3Prefix</code>, <code>S3Uri</code> identifies a key name prefix. Amazon SageMaker uses all objects with the specified key name prefix for batch transform. </p> <p>If you choose <code>ManifestFile</code>, <code>S3Uri</code> identifies an object that is a manifest file containing a list of object keys that you want Amazon SageMaker to use for batch transform. </p> <p>The following values are compatible: <code>ManifestFile</code>, <code>S3Prefix</code> </p> <p>The following value is not compatible: <code>AugmentedManifestFile</code> </p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example:</p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> <code>[</code> </p> <p> <code> {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code> &quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code> &quot;relative/path/custdata-2&quot;,</code> </p> <p> <code> ...</code> </p> <p> <code> ]</code> </p> <p> The preceding JSON matches the following <code>S3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer_bucket/some/prefix/relative/path/custdata-1</code> </p> <p> <code>...</code> </p> <p> The complete set of <code>S3Uris</code> in this manifest constitutes the input data for the channel for this datasource. The object that each <code>S3Uris</code> points to must be readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf.</p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Represents an amount of money in United States dollars/</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct USD {
    /// <p>The fractional portion, in cents, of the amount. </p>
    #[serde(rename = "Cents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cents: Option<i64>,
    /// <p>The whole number of dollars in the amount.</p>
    #[serde(rename = "Dollars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dollars: Option<i64>,
    /// <p>Fractions of a cent, in tenths.</p>
    #[serde(rename = "TenthFractionsOfACent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenth_fractions_of_a_cent: Option<i64>,
}

/// <p>Provided configuration information for the worker UI for a labeling job. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiConfig {
    /// <p>The Amazon S3 bucket location of the UI template. For more information about the contents of a UI template, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-custom-templates-step2.html"> Creating Your Custom Labeling Task Template</a>.</p>
    #[serde(rename = "UiTemplateS3Uri")]
    pub ui_template_s3_uri: String,
}

/// <p>The Liquid template for the worker user interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UiTemplate {
    /// <p>The content of the Liquid template for the worker user interface.</p>
    #[serde(rename = "Content")]
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCodeRepositoryInput {
    /// <p>The name of the Git repository to update.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p> <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    #[serde(rename = "GitConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_config: Option<GitConfigForUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCodeRepositoryOutput {
    /// <p>The ARN of the Git repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEndpointWeightsAndCapacitiesOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNotebookInstanceInput {
    /// <p>A list of the Elastic Inference (EI) instance types to associate with this notebook instance. Currently only one EI instance type can be associated with a notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>The Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository: Option<String>,
    /// <p>A list of the Elastic Inference (EI) instance types to remove from this notebook instance. This operation is idempotent. If you specify an accelerator type that is not associated with the notebook instance when you call this method, it does not throw an error.</p>
    #[serde(rename = "DisassociateAcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_accelerator_types: Option<bool>,
    /// <p>A list of names or URLs of the default Git repositories to remove from this notebook instance. This operation is idempotent. If you specify a Git repository that is not associated with the notebook instance when you call this method, it does not throw an error.</p>
    #[serde(rename = "DisassociateAdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_additional_code_repositories: Option<bool>,
    /// <p>The name or URL of the default Git repository to remove from this notebook instance. This operation is idempotent. If you specify a Git repository that is not associated with the notebook instance when you call this method, it does not throw an error.</p>
    #[serde(rename = "DisassociateDefaultCodeRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_default_code_repository: Option<bool>,
    /// <p>Set to <code>true</code> to remove the notebook instance lifecycle configuration currently associated with the notebook instance. This operation is idempotent. If you specify a lifecycle configuration that is not associated with the notebook instance when you call this method, it does not throw an error.</p>
    #[serde(rename = "DisassociateLifecycleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_lifecycle_config: Option<bool>,
    /// <p>The Amazon ML compute instance type.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    #[serde(rename = "LifecycleConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_config_name: Option<String>,
    /// <p>The name of the notebook instance to update.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker can assume to access the notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p> <note> <p>If you set this to <code>Disabled</code>, users don&#39;t have root access on the notebook instance, but lifecycle configuration scripts still run with root permissions.</p> </note></p>
    #[serde(rename = "RootAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_access: Option<String>,
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p>
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_gb: Option<i64>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateNotebookInstanceLifecycleConfigOutput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateNotebookInstanceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWorkteamRequest {
    /// <p>An updated description for the work team.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of <code>MemberDefinition</code> objects that contain the updated work team members.</p>
    #[serde(rename = "MemberDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_definitions: Option<Vec<MemberDefinition>>,
    /// <p>Configures SNS topic notifications for available or expiring work items</p>
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>The name of the work team to update.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateWorkteamResponse {
    /// <p>A <code>Workteam</code> object that describes the updated work team.</p>
    #[serde(rename = "Workteam")]
    pub workteam: Workteam,
}

/// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfig {
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. Specify the security groups for the VPC that is specified in the <code>Subnets</code> field.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p><p>The ID of the subnets in the VPC to which you want to connect your training job or model. </p> <note> <p>Amazon EC2 P3 accelerated computing instances are not available in the c/d/e availability zones of region us-east-1. If you want to create endpoints with P3 instances in VPC mode in region us-east-1, create subnets in a/b/f availability zones instead.</p> </note></p>
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

/// <p>Provides details about a labeling work team.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Workteam {
    /// <p>The date and time that the work team was created (timestamp).</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A description of the work team.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The date and time that the work team was last updated (timestamp).</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The Amazon Cognito user groups that make up the work team.</p>
    #[serde(rename = "MemberDefinitions")]
    pub member_definitions: Vec<MemberDefinition>,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>The Amazon Marketplace identifier for a vendor's work team.</p>
    #[serde(rename = "ProductListingIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_listing_ids: Option<Vec<String>>,
    /// <p>The URI of the labeling job's user interface. Workers open this URI to start labeling your data objects.</p>
    #[serde(rename = "SubDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domain: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the work team.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
    /// <p>The name of the work team.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CreateAlgorithm
#[derive(Debug, PartialEq)]
pub enum CreateAlgorithmError {}

impl CreateAlgorithmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAlgorithmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAlgorithmError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CreateCodeRepository
#[derive(Debug, PartialEq)]
pub enum CreateCodeRepositoryError {}

impl CreateCodeRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCodeRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCodeRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCodeRepositoryError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CreateCompilationJob
#[derive(Debug, PartialEq)]
pub enum CreateCompilationJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateCompilationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCompilationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateCompilationJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateCompilationJobError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCompilationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCompilationJobError {
    fn description(&self) -> &str {
        match *self {
            CreateCompilationJobError::ResourceInUse(ref cause) => cause,
            CreateCompilationJobError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateEndpointError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateEndpointConfig
#[derive(Debug, PartialEq)]
pub enum CreateEndpointConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateEndpointConfigError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateHyperParameterTuningJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateHyperParameterTuningJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateHyperParameterTuningJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateHyperParameterTuningJobError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateLabelingJob
#[derive(Debug, PartialEq)]
pub enum CreateLabelingJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateLabelingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLabelingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateLabelingJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateLabelingJobError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLabelingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLabelingJobError {
    fn description(&self) -> &str {
        match *self {
            CreateLabelingJobError::ResourceInUse(ref cause) => cause,
            CreateLabelingJobError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateModelError::ResourceLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateModelPackage
#[derive(Debug, PartialEq)]
pub enum CreateModelPackageError {}

impl CreateModelPackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelPackageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateModelPackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateModelPackageError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CreateNotebookInstance
#[derive(Debug, PartialEq)]
pub enum CreateNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateNotebookInstanceError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum CreateNotebookInstanceLifecycleConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateNotebookInstanceLifecycleConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateNotebookInstanceLifecycleConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreatePresignedNotebookInstanceUrl
#[derive(Debug, PartialEq)]
pub enum CreatePresignedNotebookInstanceUrlError {}

impl CreatePresignedNotebookInstanceUrlError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePresignedNotebookInstanceUrlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePresignedNotebookInstanceUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePresignedNotebookInstanceUrlError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CreateTrainingJob
#[derive(Debug, PartialEq)]
pub enum CreateTrainingJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrainingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateTrainingJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateTrainingJobError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl CreateTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTransformJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateTransformJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateTransformJobError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateWorkteam
#[derive(Debug, PartialEq)]
pub enum CreateWorkteamError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateWorkteamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateWorkteamError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateWorkteamError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkteamError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkteamError::ResourceInUse(ref cause) => cause,
            CreateWorkteamError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAlgorithm
#[derive(Debug, PartialEq)]
pub enum DeleteAlgorithmError {}

impl DeleteAlgorithmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAlgorithmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAlgorithmError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteCodeRepository
#[derive(Debug, PartialEq)]
pub enum DeleteCodeRepositoryError {}

impl DeleteCodeRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCodeRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCodeRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCodeRepositoryError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteEndpointConfig
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointConfigError {}

impl DeleteEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEndpointConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointConfigError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {}

impl DeleteModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteModelError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteModelPackage
#[derive(Debug, PartialEq)]
pub enum DeleteModelPackageError {}

impl DeleteModelPackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelPackageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteModelPackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteModelPackageError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteNotebookInstance
#[derive(Debug, PartialEq)]
pub enum DeleteNotebookInstanceError {}

impl DeleteNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum DeleteNotebookInstanceLifecycleConfigError {}

impl DeleteNotebookInstanceLifecycleConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteNotebookInstanceLifecycleConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteWorkteam
#[derive(Debug, PartialEq)]
pub enum DeleteWorkteamError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl DeleteWorkteamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(DeleteWorkteamError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWorkteamError {
    fn description(&self) -> &str {
        match *self {
            DeleteWorkteamError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlgorithm
#[derive(Debug, PartialEq)]
pub enum DescribeAlgorithmError {}

impl DescribeAlgorithmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlgorithmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlgorithmError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeCodeRepository
#[derive(Debug, PartialEq)]
pub enum DescribeCodeRepositoryError {}

impl DescribeCodeRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCodeRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCodeRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCodeRepositoryError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeCompilationJob
#[derive(Debug, PartialEq)]
pub enum DescribeCompilationJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeCompilationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCompilationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeCompilationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCompilationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCompilationJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeCompilationJobError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointError {}

impl DescribeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeEndpointConfig
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointConfigError {}

impl DescribeEndpointConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEndpointConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointConfigError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum DescribeHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeHyperParameterTuningJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeHyperParameterTuningJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(
                        DescribeHyperParameterTuningJobError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeLabelingJob
#[derive(Debug, PartialEq)]
pub enum DescribeLabelingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeLabelingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLabelingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeLabelingJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeLabelingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLabelingJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeLabelingJobError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeModel
#[derive(Debug, PartialEq)]
pub enum DescribeModelError {}

impl DescribeModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeModelError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeModelPackage
#[derive(Debug, PartialEq)]
pub enum DescribeModelPackageError {}

impl DescribeModelPackageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeModelPackageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeModelPackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeModelPackageError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeNotebookInstance
#[derive(Debug, PartialEq)]
pub enum DescribeNotebookInstanceError {}

impl DescribeNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum DescribeNotebookInstanceLifecycleConfigError {}

impl DescribeNotebookInstanceLifecycleConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeNotebookInstanceLifecycleConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeNotebookInstanceLifecycleConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotebookInstanceLifecycleConfigError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeSubscribedWorkteam
#[derive(Debug, PartialEq)]
pub enum DescribeSubscribedWorkteamError {}

impl DescribeSubscribedWorkteamError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSubscribedWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSubscribedWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubscribedWorkteamError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeTrainingJob
#[derive(Debug, PartialEq)]
pub enum DescribeTrainingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrainingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeTrainingJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeTransformJob
#[derive(Debug, PartialEq)]
pub enum DescribeTransformJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTransformJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeTransformJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeWorkteam
#[derive(Debug, PartialEq)]
pub enum DescribeWorkteamError {}

impl DescribeWorkteamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkteamError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetSearchSuggestions
#[derive(Debug, PartialEq)]
pub enum GetSearchSuggestionsError {}

impl GetSearchSuggestionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSearchSuggestionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSearchSuggestionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSearchSuggestionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListAlgorithms
#[derive(Debug, PartialEq)]
pub enum ListAlgorithmsError {}

impl ListAlgorithmsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAlgorithmsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAlgorithmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAlgorithmsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListCodeRepositories
#[derive(Debug, PartialEq)]
pub enum ListCodeRepositoriesError {}

impl ListCodeRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCodeRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCodeRepositoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCodeRepositoriesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListCompilationJobs
#[derive(Debug, PartialEq)]
pub enum ListCompilationJobsError {}

impl ListCompilationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCompilationJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCompilationJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCompilationJobsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListEndpointConfigs
#[derive(Debug, PartialEq)]
pub enum ListEndpointConfigsError {}

impl ListEndpointConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEndpointConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListEndpointConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEndpointConfigsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListEndpoints
#[derive(Debug, PartialEq)]
pub enum ListEndpointsError {}

impl ListEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEndpointsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListHyperParameterTuningJobs
#[derive(Debug, PartialEq)]
pub enum ListHyperParameterTuningJobsError {}

impl ListHyperParameterTuningJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListHyperParameterTuningJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHyperParameterTuningJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHyperParameterTuningJobsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListLabelingJobs
#[derive(Debug, PartialEq)]
pub enum ListLabelingJobsError {}

impl ListLabelingJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLabelingJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLabelingJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLabelingJobsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListLabelingJobsForWorkteam
#[derive(Debug, PartialEq)]
pub enum ListLabelingJobsForWorkteamError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl ListLabelingJobsForWorkteamError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListLabelingJobsForWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(
                        ListLabelingJobsForWorkteamError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLabelingJobsForWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLabelingJobsForWorkteamError {
    fn description(&self) -> &str {
        match *self {
            ListLabelingJobsForWorkteamError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListModelPackages
#[derive(Debug, PartialEq)]
pub enum ListModelPackagesError {}

impl ListModelPackagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListModelPackagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListModelPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListModelPackagesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListModels
#[derive(Debug, PartialEq)]
pub enum ListModelsError {}

impl ListModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListModelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListModelsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListNotebookInstanceLifecycleConfigs
#[derive(Debug, PartialEq)]
pub enum ListNotebookInstanceLifecycleConfigsError {}

impl ListNotebookInstanceLifecycleConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListNotebookInstanceLifecycleConfigsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListNotebookInstanceLifecycleConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNotebookInstanceLifecycleConfigsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListNotebookInstances
#[derive(Debug, PartialEq)]
pub enum ListNotebookInstancesError {}

impl ListNotebookInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNotebookInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListNotebookInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNotebookInstancesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListSubscribedWorkteams
#[derive(Debug, PartialEq)]
pub enum ListSubscribedWorkteamsError {}

impl ListSubscribedWorkteamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSubscribedWorkteamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSubscribedWorkteamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSubscribedWorkteamsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListTrainingJobs
#[derive(Debug, PartialEq)]
pub enum ListTrainingJobsError {}

impl ListTrainingJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrainingJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTrainingJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTrainingJobsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListTrainingJobsForHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum ListTrainingJobsForHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl ListTrainingJobsForHyperParameterTuningJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTrainingJobsForHyperParameterTuningJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(
                        ListTrainingJobsForHyperParameterTuningJobError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListTransformJobs
#[derive(Debug, PartialEq)]
pub enum ListTransformJobsError {}

impl ListTransformJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTransformJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTransformJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTransformJobsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListWorkteams
#[derive(Debug, PartialEq)]
pub enum ListWorkteamsError {}

impl ListWorkteamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorkteamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWorkteamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkteamsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by RenderUiTemplate
#[derive(Debug, PartialEq)]
pub enum RenderUiTemplateError {}

impl RenderUiTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RenderUiTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RenderUiTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RenderUiTemplateError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by Search
#[derive(Debug, PartialEq)]
pub enum SearchError {}

impl SearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by StartNotebookInstance
#[derive(Debug, PartialEq)]
pub enum StartNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl StartNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(StartNotebookInstanceError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StopCompilationJob
#[derive(Debug, PartialEq)]
pub enum StopCompilationJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopCompilationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopCompilationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopCompilationJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopCompilationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCompilationJobError {
    fn description(&self) -> &str {
        match *self {
            StopCompilationJobError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by StopHyperParameterTuningJob
#[derive(Debug, PartialEq)]
pub enum StopHyperParameterTuningJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopHyperParameterTuningJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopHyperParameterTuningJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(
                        StopHyperParameterTuningJobError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StopLabelingJob
#[derive(Debug, PartialEq)]
pub enum StopLabelingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopLabelingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopLabelingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopLabelingJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopLabelingJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopLabelingJobError {
    fn description(&self) -> &str {
        match *self {
            StopLabelingJobError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by StopNotebookInstance
#[derive(Debug, PartialEq)]
pub enum StopNotebookInstanceError {}

impl StopNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopNotebookInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopNotebookInstanceError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by StopTrainingJob
#[derive(Debug, PartialEq)]
pub enum StopTrainingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopTrainingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTrainingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopTrainingJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StopTransformJob
#[derive(Debug, PartialEq)]
pub enum StopTransformJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopTransformJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTransformJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopTransformJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateCodeRepository
#[derive(Debug, PartialEq)]
pub enum UpdateCodeRepositoryError {}

impl UpdateCodeRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCodeRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCodeRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCodeRepositoryError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl UpdateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(UpdateEndpointError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateEndpointWeightsAndCapacities
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointWeightsAndCapacitiesError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl UpdateEndpointWeightsAndCapacitiesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateEndpointWeightsAndCapacitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateEndpointWeightsAndCapacitiesError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateNotebookInstance
#[derive(Debug, PartialEq)]
pub enum UpdateNotebookInstanceError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl UpdateNotebookInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNotebookInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateNotebookInstanceError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateNotebookInstanceLifecycleConfig
#[derive(Debug, PartialEq)]
pub enum UpdateNotebookInstanceLifecycleConfigError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl UpdateNotebookInstanceLifecycleConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateNotebookInstanceLifecycleConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateWorkteam
#[derive(Debug, PartialEq)]
pub enum UpdateWorkteamError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl UpdateWorkteamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWorkteamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(UpdateWorkteamError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateWorkteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateWorkteamError {
    fn description(&self) -> &str {
        match *self {
            UpdateWorkteamError::ResourceLimitExceeded(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the SageMaker API. SageMaker clients implement this trait.
pub trait SageMaker {
    /// <p><p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints.</p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <note> <p>Tags that you add to a hyperparameter tuning job by calling this API are also added to any training jobs that the hyperparameter tuning job launches after you call this API, but not to training jobs that the hyperparameter tuning job launched before you called this API. To make sure that the tags associated with a hyperparameter tuning job are also added to all training jobs that the hyperparameter tuning job launches, add the tags when you first create the tuning job by specifying them in the <code>Tags</code> parameter of <a>CreateHyperParameterTuningJob</a> </p> </note></p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Create a machine learning algorithm that you can use in Amazon SageMaker and list in the AWS Marketplace.</p>
    fn create_algorithm(
        &self,
        input: CreateAlgorithmInput,
    ) -> RusotoFuture<CreateAlgorithmOutput, CreateAlgorithmError>;

    /// <p>Creates a Git repository as a resource in your Amazon SageMaker account. You can associate the repository with notebook instances so that you can use Git source control for the notebooks you create. The Git repository is a resource in your Amazon SageMaker account, so it can be associated with more than one notebook instance, and it persists independently from the lifecycle of any notebook instances it is associated with.</p> <p>The repository can be hosted either in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository.</p>
    fn create_code_repository(
        &self,
        input: CreateCodeRepositoryInput,
    ) -> RusotoFuture<CreateCodeRepositoryOutput, CreateCodeRepositoryError>;

    /// <p>Starts a model compilation job. After the model has been compiled, Amazon SageMaker saves the resulting model artifacts to an Amazon Simple Storage Service (Amazon S3) bucket that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts with AWS IoT Greengrass. In that case, deploy them as an ML resource.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p>A name for the compilation job</p> </li> <li> <p> Information about the input model artifacts </p> </li> <li> <p>The output location for the compiled model and the device (target) that the model runs on </p> </li> <li> <p> <code>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker assumes to perform the model compilation job</code> </p> </li> </ul> <p>You can also provide a <code>Tag</code> to track the model compilation job's resource use and costs. The response body contains the <code>CompilationJobArn</code> for the compiled job.</p> <p>To stop a model compilation job, use <a>StopCompilationJob</a>. To get information about a particular model compilation job, use <a>DescribeCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    fn create_compilation_job(
        &self,
        input: CreateCompilationJobRequest,
    ) -> RusotoFuture<CreateCompilationJobResponse, CreateCompilationJobError>;

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> <p> You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS i an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> RusotoFuture<CreateEndpointOutput, CreateEndpointError>;

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> RusotoFuture<CreateEndpointConfigOutput, CreateEndpointConfigError>;

    /// <p>Starts a hyperparameter tuning job. A hyperparameter tuning job finds the best version of a model by running many training jobs on your dataset using the algorithm you choose and values for hyperparameters within ranges that you specify. It then chooses the hyperparameter values that result in a model that performs the best, as measured by an objective metric that you choose.</p>
    fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> RusotoFuture<CreateHyperParameterTuningJobResponse, CreateHyperParameterTuningJobError>;

    /// <p>Creates a job that uses workers to label the data objects in your input dataset. You can use the labeled data to train machine learning models.</p> <p>You can select your workforce from one of three providers:</p> <ul> <li> <p>A private workforce that you create. It can include employees, contractors, and outside experts. Use a private workforce when want the data to stay within your organization or when a specific set of skills is required.</p> </li> <li> <p>One or more vendors that you select from the AWS Marketplace. Vendors provide expertise in specific areas. </p> </li> <li> <p>The Amazon Mechanical Turk workforce. This is the largest workforce, but it should only be used for public data or data that has been stripped of any personally identifiable information.</p> </li> </ul> <p>You can also use <i>automated data labeling</i> to reduce the number of data objects that need to be labeled by a human. Automated data labeling uses <i>active learning</i> to determine if a data object can be labeled by machine or if it needs to be sent to a human worker. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-automated-labeling.html">Using Automated Data Labeling</a>.</p> <p>The data objects to be labeled are contained in an Amazon S3 bucket. You create a <i>manifest file</i> that describes the location of each object. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-data.html">Using Input and Output Data</a>.</p> <p>The output can be used as the manifest file for another labeling job or as training data for your machine learning models.</p>
    fn create_labeling_job(
        &self,
        input: CreateLabelingJobRequest,
    ) -> RusotoFuture<CreateLabelingJobResponse, CreateLabelingJobError>;

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe a primary container. For the primary container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model for predictions.</p> <p>Use this API to create a model if you want to use Amazon SageMaker hosting services or run a batch transform job.</p> <p>To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>To run a batch transform using your model, you start a job with the <code>CreateTransformJob</code> API. Amazon SageMaker uses your model and your dataset to get inferences which are then saved to a specified S3 location.</p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter.</p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances or for batch transform jobs. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    fn create_model(
        &self,
        input: CreateModelInput,
    ) -> RusotoFuture<CreateModelOutput, CreateModelError>;

    /// <p>Creates a model package that you can use to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p> <p>To create a model package by specifying a Docker container that contains your inference code and the Amazon S3 location of your model artifacts, provide values for <code>InferenceSpecification</code>. To create a model from an algorithm resource that you created or subscribed to in AWS Marketplace, provide a value for <code>SourceAlgorithmSpecification</code>.</p>
    fn create_model_package(
        &self,
        input: CreateModelPackageInput,
    ) -> RusotoFuture<CreateModelPackageOutput, CreateModelPackageError>;

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN).</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> RusotoFuture<CreateNotebookInstanceOutput, CreateNotebookInstanceError>;

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        CreateNotebookInstanceLifecycleConfigOutput,
        CreateNotebookInstanceLifecycleConfigError,
    >;

    /// <p><p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page.</p> <p>You can restrict access to this API and to the URL that it returns to a list of IP addresses that you specify. To restrict access, attach an IAM policy that denies access to this API unless the call comes from an IP address in the specified list to every AWS Identity and Access Management user, group, or role used to access the notebook instance. Use the <code>NotIpAddress</code> condition operator and the <code>aws:SourceIP</code> condition context key to specify the list of IP addresses that you want to have access to the notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-ip-filter.html">Limit Access to a Notebook Instance by IP Address</a>.</p> <note> <p>The URL that you get from a call to is valid only for 5 minutes. If you try to use the URL after the 5-minute limit expires, you are directed to the AWS console sign-in page.</p> </note></p>
    fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> RusotoFuture<
        CreatePresignedNotebookInstanceUrlOutput,
        CreatePresignedNotebookInstanceUrlError,
    >;

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a machine learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to influence the quality of the final model. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - Sets a duration for training. Use this parameter to cap model training costs. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> RusotoFuture<CreateTrainingJobResponse, CreateTrainingJobError>;

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model in the same AWS Region and AWS account. For information on creating a model, see <a>CreateModel</a>.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p> For more information about how batch transformation works Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">How It Works</a>. </p>
    fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> RusotoFuture<CreateTransformJobResponse, CreateTransformJobError>;

    /// <p>Creates a new work team for labeling your data. A work team is defined by one or more Amazon Cognito user pools. You must first create the user pools before you can create a work team.</p> <p>You cannot create more than 25 work teams in an account and region.</p>
    fn create_workteam(
        &self,
        input: CreateWorkteamRequest,
    ) -> RusotoFuture<CreateWorkteamResponse, CreateWorkteamError>;

    /// <p>Removes the specified algorithm from your account.</p>
    fn delete_algorithm(
        &self,
        input: DeleteAlgorithmInput,
    ) -> RusotoFuture<(), DeleteAlgorithmError>;

    /// <p>Deletes the specified Git repository from your account.</p>
    fn delete_code_repository(
        &self,
        input: DeleteCodeRepositoryInput,
    ) -> RusotoFuture<(), DeleteCodeRepositoryError>;

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError>;

    /// <p>Deletes an endpoint configuration. The <code>DeleteEndpointConfig</code> API deletes only the specified configuration. It does not delete endpoints created using the configuration. </p>
    fn delete_endpoint_config(
        &self,
        input: DeleteEndpointConfigInput,
    ) -> RusotoFuture<(), DeleteEndpointConfigError>;

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    fn delete_model(&self, input: DeleteModelInput) -> RusotoFuture<(), DeleteModelError>;

    /// <p>Deletes a model package.</p> <p>A model package is used to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p>
    fn delete_model_package(
        &self,
        input: DeleteModelPackageInput,
    ) -> RusotoFuture<(), DeleteModelPackageError>;

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

    /// <p><p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource&#39;s tags, use the <code>ListTags</code> API. </p> <note> <p>When you call this API to delete tags from a hyperparameter tuning job, the deleted tags are not removed from training jobs that the hyperparameter tuning job launched before you called this API.</p> </note></p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError>;

    /// <p>Deletes an existing work team. This operation can't be undone.</p>
    fn delete_workteam(
        &self,
        input: DeleteWorkteamRequest,
    ) -> RusotoFuture<DeleteWorkteamResponse, DeleteWorkteamError>;

    /// <p>Returns a description of the specified algorithm that is in your account.</p>
    fn describe_algorithm(
        &self,
        input: DescribeAlgorithmInput,
    ) -> RusotoFuture<DescribeAlgorithmOutput, DescribeAlgorithmError>;

    /// <p>Gets details about the specified Git repository.</p>
    fn describe_code_repository(
        &self,
        input: DescribeCodeRepositoryInput,
    ) -> RusotoFuture<DescribeCodeRepositoryOutput, DescribeCodeRepositoryError>;

    /// <p>Returns information about a model compilation job.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    fn describe_compilation_job(
        &self,
        input: DescribeCompilationJobRequest,
    ) -> RusotoFuture<DescribeCompilationJobResponse, DescribeCompilationJobError>;

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

    /// <p>Gets information about a labeling job.</p>
    fn describe_labeling_job(
        &self,
        input: DescribeLabelingJobRequest,
    ) -> RusotoFuture<DescribeLabelingJobResponse, DescribeLabelingJobError>;

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> RusotoFuture<DescribeModelOutput, DescribeModelError>;

    /// <p>Returns a description of the specified model package, which is used to create Amazon SageMaker models or list them on AWS Marketplace.</p> <p>To create models in Amazon SageMaker, buyers can subscribe to model packages listed on AWS Marketplace.</p>
    fn describe_model_package(
        &self,
        input: DescribeModelPackageInput,
    ) -> RusotoFuture<DescribeModelPackageOutput, DescribeModelPackageError>;

    /// <p>Returns information about a notebook instance.</p>
    fn describe_notebook_instance(
        &self,
        input: DescribeNotebookInstanceInput,
    ) -> RusotoFuture<DescribeNotebookInstanceOutput, DescribeNotebookInstanceError>;

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        DescribeNotebookInstanceLifecycleConfigOutput,
        DescribeNotebookInstanceLifecycleConfigError,
    >;

    /// <p>Gets information about a work team provided by a vendor. It returns details about the subscription with a vendor in the AWS Marketplace.</p>
    fn describe_subscribed_workteam(
        &self,
        input: DescribeSubscribedWorkteamRequest,
    ) -> RusotoFuture<DescribeSubscribedWorkteamResponse, DescribeSubscribedWorkteamError>;

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

    /// <p>Gets information about a specific work team. You can see information such as the create date, the last updated date, membership information, and the work team's Amazon Resource Name (ARN).</p>
    fn describe_workteam(
        &self,
        input: DescribeWorkteamRequest,
    ) -> RusotoFuture<DescribeWorkteamResponse, DescribeWorkteamError>;

    /// <p>An auto-complete API for the search functionality in the Amazon SageMaker console. It returns suggestions of possible matches for the property name to use in <code>Search</code> queries. Provides suggestions for <code>HyperParameters</code>, <code>Tags</code>, and <code>Metrics</code>.</p>
    fn get_search_suggestions(
        &self,
        input: GetSearchSuggestionsRequest,
    ) -> RusotoFuture<GetSearchSuggestionsResponse, GetSearchSuggestionsError>;

    /// <p>Lists the machine learning algorithms that have been created.</p>
    fn list_algorithms(
        &self,
        input: ListAlgorithmsInput,
    ) -> RusotoFuture<ListAlgorithmsOutput, ListAlgorithmsError>;

    /// <p>Gets a list of the Git repositories in your account.</p>
    fn list_code_repositories(
        &self,
        input: ListCodeRepositoriesInput,
    ) -> RusotoFuture<ListCodeRepositoriesOutput, ListCodeRepositoriesError>;

    /// <p>Lists model compilation jobs that satisfy various filters.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about a particular model compilation job you have created, use <a>DescribeCompilationJob</a>.</p>
    fn list_compilation_jobs(
        &self,
        input: ListCompilationJobsRequest,
    ) -> RusotoFuture<ListCompilationJobsResponse, ListCompilationJobsError>;

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

    /// <p>Gets a list of labeling jobs.</p>
    fn list_labeling_jobs(
        &self,
        input: ListLabelingJobsRequest,
    ) -> RusotoFuture<ListLabelingJobsResponse, ListLabelingJobsError>;

    /// <p>Gets a list of labeling jobs assigned to a specified work team.</p>
    fn list_labeling_jobs_for_workteam(
        &self,
        input: ListLabelingJobsForWorkteamRequest,
    ) -> RusotoFuture<ListLabelingJobsForWorkteamResponse, ListLabelingJobsForWorkteamError>;

    /// <p>Lists the model packages that have been created.</p>
    fn list_model_packages(
        &self,
        input: ListModelPackagesInput,
    ) -> RusotoFuture<ListModelPackagesOutput, ListModelPackagesError>;

    /// <p>Lists models created with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
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

    /// <p>Gets a list of the work teams that you are subscribed to in the AWS Marketplace. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    fn list_subscribed_workteams(
        &self,
        input: ListSubscribedWorkteamsRequest,
    ) -> RusotoFuture<ListSubscribedWorkteamsResponse, ListSubscribedWorkteamsError>;

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

    /// <p>Gets a list of work teams that you have defined in a region. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    fn list_workteams(
        &self,
        input: ListWorkteamsRequest,
    ) -> RusotoFuture<ListWorkteamsResponse, ListWorkteamsError>;

    /// <p>Renders the UI template so that you can preview the worker's experience. </p>
    fn render_ui_template(
        &self,
        input: RenderUiTemplateRequest,
    ) -> RusotoFuture<RenderUiTemplateResponse, RenderUiTemplateError>;

    /// <p>Finds Amazon SageMaker resources that match a search query. Matching resource objects are returned as a list of <code>SearchResult</code> objects in the response. You can sort the search results by any resource property in a ascending or descending order.</p> <p>You can query against the following value types: numerical, text, Booleans, and timestamps.</p>
    fn search(&self, input: SearchRequest) -> RusotoFuture<SearchResponse, SearchError>;

    /// <p>Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, Amazon SageMaker sets the notebook instance status to <code>InService</code>. A notebook instance's status must be <code>InService</code> before you can connect to your Jupyter notebook. </p>
    fn start_notebook_instance(
        &self,
        input: StartNotebookInstanceInput,
    ) -> RusotoFuture<(), StartNotebookInstanceError>;

    /// <p>Stops a model compilation job.</p> <p> To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal. This gracefully shuts the job down. If the job hasn't stopped, it sends the SIGKILL signal.</p> <p>When it receives a <code>StopCompilationJob</code> request, Amazon SageMaker changes the <a>CompilationJobSummary$CompilationJobStatus</a> of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the <a>CompilationJobSummary$CompilationJobStatus</a> to <code>Stopped</code>. </p>
    fn stop_compilation_job(
        &self,
        input: StopCompilationJobRequest,
    ) -> RusotoFuture<(), StopCompilationJobError>;

    /// <p>Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched.</p> <p>All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). All data that the training jobs write to Amazon CloudWatch Logs are still available in CloudWatch. After the tuning job moves to the <code>Stopped</code> state, it releases all reserved resources for the tuning job.</p>
    fn stop_hyper_parameter_tuning_job(
        &self,
        input: StopHyperParameterTuningJobRequest,
    ) -> RusotoFuture<(), StopHyperParameterTuningJobError>;

    /// <p>Stops a running labeling job. A job that is stopped cannot be restarted. Any results obtained before the job is stopped are placed in the Amazon S3 output bucket.</p>
    fn stop_labeling_job(
        &self,
        input: StopLabelingJobRequest,
    ) -> RusotoFuture<(), StopLabelingJobError>;

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. Amazon SageMaker stops charging you for the ML compute instance when you call <code>StopNotebookInstance</code>.</p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> RusotoFuture<(), StopNotebookInstanceError>;

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> RusotoFuture<(), StopTrainingJobError>;

    /// <p>Stops a transform job.</p> <p>When Amazon SageMaker receives a <code>StopTransformJob</code> request, the status of the job changes to <code>Stopping</code>. After Amazon SageMaker stops the job, the status is set to <code>Stopped</code>. When you stop a transform job before it is completed, Amazon SageMaker doesn't store the job's output in Amazon S3.</p>
    fn stop_transform_job(
        &self,
        input: StopTransformJobRequest,
    ) -> RusotoFuture<(), StopTransformJobError>;

    /// <p>Updates the specified Git repository with the specified values.</p>
    fn update_code_repository(
        &self,
        input: UpdateCodeRepositoryInput,
    ) -> RusotoFuture<UpdateCodeRepositoryOutput, UpdateCodeRepositoryError>;

    /// <p><p>Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> RusotoFuture<UpdateEndpointOutput, UpdateEndpointError>;

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> RusotoFuture<
        UpdateEndpointWeightsAndCapacitiesOutput,
        UpdateEndpointWeightsAndCapacitiesError,
    >;

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements.</p>
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

    /// <p>Updates an existing work team with new member definitions or description.</p>
    fn update_workteam(
        &self,
        input: UpdateWorkteamRequest,
    ) -> RusotoFuture<UpdateWorkteamResponse, UpdateWorkteamError>;
}
/// A client for the SageMaker API.
#[derive(Clone)]
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
    /// <p><p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints.</p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <note> <p>Tags that you add to a hyperparameter tuning job by calling this API are also added to any training jobs that the hyperparameter tuning job launches after you call this API, but not to training jobs that the hyperparameter tuning job launched before you called this API. To make sure that the tags associated with a hyperparameter tuning job are also added to all training jobs that the hyperparameter tuning job launches, add the tags when you first create the tuning job by specifying them in the <code>Tags</code> parameter of <a>CreateHyperParameterTuningJob</a> </p> </note></p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.AddTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<AddTagsOutput, _>()
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

    /// <p>Create a machine learning algorithm that you can use in Amazon SageMaker and list in the AWS Marketplace.</p>
    fn create_algorithm(
        &self,
        input: CreateAlgorithmInput,
    ) -> RusotoFuture<CreateAlgorithmOutput, CreateAlgorithmError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAlgorithmOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAlgorithmError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Git repository as a resource in your Amazon SageMaker account. You can associate the repository with notebook instances so that you can use Git source control for the notebooks you create. The Git repository is a resource in your Amazon SageMaker account, so it can be associated with more than one notebook instance, and it persists independently from the lifecycle of any notebook instances it is associated with.</p> <p>The repository can be hosted either in <a href="http://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository.</p>
    fn create_code_repository(
        &self,
        input: CreateCodeRepositoryInput,
    ) -> RusotoFuture<CreateCodeRepositoryOutput, CreateCodeRepositoryError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateCodeRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCodeRepositoryOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCodeRepositoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts a model compilation job. After the model has been compiled, Amazon SageMaker saves the resulting model artifacts to an Amazon Simple Storage Service (Amazon S3) bucket that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts with AWS IoT Greengrass. In that case, deploy them as an ML resource.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p>A name for the compilation job</p> </li> <li> <p> Information about the input model artifacts </p> </li> <li> <p>The output location for the compiled model and the device (target) that the model runs on </p> </li> <li> <p> <code>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker assumes to perform the model compilation job</code> </p> </li> </ul> <p>You can also provide a <code>Tag</code> to track the model compilation job's resource use and costs. The response body contains the <code>CompilationJobArn</code> for the compiled job.</p> <p>To stop a model compilation job, use <a>StopCompilationJob</a>. To get information about a particular model compilation job, use <a>DescribeCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    fn create_compilation_job(
        &self,
        input: CreateCompilationJobRequest,
    ) -> RusotoFuture<CreateCompilationJobResponse, CreateCompilationJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateCompilationJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCompilationJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCompilationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> <p> You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS i an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> RusotoFuture<CreateEndpointOutput, CreateEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateEndpointOutput, _>()
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

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> RusotoFuture<CreateEndpointConfigOutput, CreateEndpointConfigError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateEndpointConfigOutput, _>()
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

    /// <p>Starts a hyperparameter tuning job. A hyperparameter tuning job finds the best version of a model by running many training jobs on your dataset using the algorithm you choose and values for hyperparameters within ranges that you specify. It then chooses the hyperparameter values that result in a model that performs the best, as measured by an objective metric that you choose.</p>
    fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> RusotoFuture<CreateHyperParameterTuningJobResponse, CreateHyperParameterTuningJobError>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateHyperParameterTuningJobResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateHyperParameterTuningJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a job that uses workers to label the data objects in your input dataset. You can use the labeled data to train machine learning models.</p> <p>You can select your workforce from one of three providers:</p> <ul> <li> <p>A private workforce that you create. It can include employees, contractors, and outside experts. Use a private workforce when want the data to stay within your organization or when a specific set of skills is required.</p> </li> <li> <p>One or more vendors that you select from the AWS Marketplace. Vendors provide expertise in specific areas. </p> </li> <li> <p>The Amazon Mechanical Turk workforce. This is the largest workforce, but it should only be used for public data or data that has been stripped of any personally identifiable information.</p> </li> </ul> <p>You can also use <i>automated data labeling</i> to reduce the number of data objects that need to be labeled by a human. Automated data labeling uses <i>active learning</i> to determine if a data object can be labeled by machine or if it needs to be sent to a human worker. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-automated-labeling.html">Using Automated Data Labeling</a>.</p> <p>The data objects to be labeled are contained in an Amazon S3 bucket. You create a <i>manifest file</i> that describes the location of each object. For more information, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/sms-data.html">Using Input and Output Data</a>.</p> <p>The output can be used as the manifest file for another labeling job or as training data for your machine learning models.</p>
    fn create_labeling_job(
        &self,
        input: CreateLabelingJobRequest,
    ) -> RusotoFuture<CreateLabelingJobResponse, CreateLabelingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateLabelingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLabelingJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateLabelingJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe a primary container. For the primary container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model for predictions.</p> <p>Use this API to create a model if you want to use Amazon SageMaker hosting services or run a batch transform job.</p> <p>To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>To run a batch transform using your model, you start a job with the <code>CreateTransformJob</code> API. Amazon SageMaker uses your model and your dataset to get inferences which are then saved to a specified S3 location.</p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter.</p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances or for batch transform jobs. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    fn create_model(
        &self,
        input: CreateModelInput,
    ) -> RusotoFuture<CreateModelOutput, CreateModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateModelOutput, _>()
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

    /// <p>Creates a model package that you can use to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p> <p>To create a model package by specifying a Docker container that contains your inference code and the Amazon S3 location of your model artifacts, provide values for <code>InferenceSpecification</code>. To create a model from an algorithm resource that you created or subscribed to in AWS Marketplace, provide a value for <code>SourceAlgorithmSpecification</code>.</p>
    fn create_model_package(
        &self,
        input: CreateModelPackageInput,
    ) -> RusotoFuture<CreateModelPackageOutput, CreateModelPackageError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateModelPackage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateModelPackageOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateModelPackageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN).</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> RusotoFuture<CreateNotebookInstanceOutput, CreateNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateNotebookInstanceOutput, _>()
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

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        CreateNotebookInstanceLifecycleConfigOutput,
        CreateNotebookInstanceLifecycleConfigError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.CreateNotebookInstanceLifecycleConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateNotebookInstanceLifecycleConfigOutput, _>()
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

    /// <p><p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page.</p> <p>You can restrict access to this API and to the URL that it returns to a list of IP addresses that you specify. To restrict access, attach an IAM policy that denies access to this API unless the call comes from an IP address in the specified list to every AWS Identity and Access Management user, group, or role used to access the notebook instance. Use the <code>NotIpAddress</code> condition operator and the <code>aws:SourceIP</code> condition context key to specify the list of IP addresses that you want to have access to the notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-ip-filter.html">Limit Access to a Notebook Instance by IP Address</a>.</p> <note> <p>The URL that you get from a call to is valid only for 5 minutes. If you try to use the URL after the 5-minute limit expires, you are directed to the AWS console sign-in page.</p> </note></p>
    fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> RusotoFuture<
        CreatePresignedNotebookInstanceUrlOutput,
        CreatePresignedNotebookInstanceUrlError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.CreatePresignedNotebookInstanceUrl",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePresignedNotebookInstanceUrlOutput, _>()
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

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a machine learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to influence the quality of the final model. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - Sets a duration for training. Use this parameter to cap model training costs. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> RusotoFuture<CreateTrainingJobResponse, CreateTrainingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTrainingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTrainingJobResponse, _>()
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

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model in the same AWS Region and AWS account. For information on creating a model, see <a>CreateModel</a>.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p> For more information about how batch transformation works Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">How It Works</a>. </p>
    fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> RusotoFuture<CreateTransformJobResponse, CreateTransformJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTransformJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTransformJobResponse, _>()
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

    /// <p>Creates a new work team for labeling your data. A work team is defined by one or more Amazon Cognito user pools. You must first create the user pools before you can create a work team.</p> <p>You cannot create more than 25 work teams in an account and region.</p>
    fn create_workteam(
        &self,
        input: CreateWorkteamRequest,
    ) -> RusotoFuture<CreateWorkteamResponse, CreateWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateWorkteamResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWorkteamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified algorithm from your account.</p>
    fn delete_algorithm(
        &self,
        input: DeleteAlgorithmInput,
    ) -> RusotoFuture<(), DeleteAlgorithmError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAlgorithmError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified Git repository from your account.</p>
    fn delete_code_repository(
        &self,
        input: DeleteCodeRepositoryInput,
    ) -> RusotoFuture<(), DeleteCodeRepositoryError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteCodeRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCodeRepositoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    fn delete_endpoint(&self, input: DeleteEndpointInput) -> RusotoFuture<(), DeleteEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    fn delete_model(&self, input: DeleteModelInput) -> RusotoFuture<(), DeleteModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Deletes a model package.</p> <p>A model package is used to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p>
    fn delete_model_package(
        &self,
        input: DeleteModelPackageInput,
    ) -> RusotoFuture<(), DeleteModelPackageError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteModelPackage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteModelPackageError::from_response(response))),
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.DeleteNotebookInstanceLifecycleConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p><p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource&#39;s tags, use the <code>ListTags</code> API. </p> <note> <p>When you call this API to delete tags from a hyperparameter tuning job, the deleted tags are not removed from training jobs that the hyperparameter tuning job launched before you called this API.</p> </note></p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTagsOutput, _>()
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

    /// <p>Deletes an existing work team. This operation can't be undone.</p>
    fn delete_workteam(
        &self,
        input: DeleteWorkteamRequest,
    ) -> RusotoFuture<DeleteWorkteamResponse, DeleteWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWorkteamResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWorkteamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a description of the specified algorithm that is in your account.</p>
    fn describe_algorithm(
        &self,
        input: DescribeAlgorithmInput,
    ) -> RusotoFuture<DescribeAlgorithmOutput, DescribeAlgorithmError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAlgorithmOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAlgorithmError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details about the specified Git repository.</p>
    fn describe_code_repository(
        &self,
        input: DescribeCodeRepositoryInput,
    ) -> RusotoFuture<DescribeCodeRepositoryOutput, DescribeCodeRepositoryError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeCodeRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCodeRepositoryOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeCodeRepositoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a model compilation job.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    fn describe_compilation_job(
        &self,
        input: DescribeCompilationJobRequest,
    ) -> RusotoFuture<DescribeCompilationJobResponse, DescribeCompilationJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeCompilationJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCompilationJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeCompilationJobError::from_response(response))
                    }),
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEndpointOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpointConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEndpointConfigOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeHyperParameterTuningJobResponse, _>()
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

    /// <p>Gets information about a labeling job.</p>
    fn describe_labeling_job(
        &self,
        input: DescribeLabelingJobRequest,
    ) -> RusotoFuture<DescribeLabelingJobResponse, DescribeLabelingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeLabelingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeLabelingJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeLabelingJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> RusotoFuture<DescribeModelOutput, DescribeModelError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeModelOutput, _>()
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

    /// <p>Returns a description of the specified model package, which is used to create Amazon SageMaker models or list them on AWS Marketplace.</p> <p>To create models in Amazon SageMaker, buyers can subscribe to model packages listed on AWS Marketplace.</p>
    fn describe_model_package(
        &self,
        input: DescribeModelPackageInput,
    ) -> RusotoFuture<DescribeModelPackageOutput, DescribeModelPackageError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeModelPackage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeModelPackageOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeModelPackageError::from_response(response))
                    }),
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeNotebookInstanceOutput, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotebookInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> RusotoFuture<
        DescribeNotebookInstanceLifecycleConfigOutput,
        DescribeNotebookInstanceLifecycleConfigError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.DescribeNotebookInstanceLifecycleConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeNotebookInstanceLifecycleConfigOutput, _>()
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

    /// <p>Gets information about a work team provided by a vendor. It returns details about the subscription with a vendor in the AWS Marketplace.</p>
    fn describe_subscribed_workteam(
        &self,
        input: DescribeSubscribedWorkteamRequest,
    ) -> RusotoFuture<DescribeSubscribedWorkteamResponse, DescribeSubscribedWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeSubscribedWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSubscribedWorkteamResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSubscribedWorkteamError::from_response(response))
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTrainingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTrainingJobResponse, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTransformJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTransformJobResponse, _>()
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

    /// <p>Gets information about a specific work team. You can see information such as the create date, the last updated date, membership information, and the work team's Amazon Resource Name (ARN).</p>
    fn describe_workteam(
        &self,
        input: DescribeWorkteamRequest,
    ) -> RusotoFuture<DescribeWorkteamResponse, DescribeWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeWorkteamResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeWorkteamError::from_response(response))),
                )
            }
        })
    }

    /// <p>An auto-complete API for the search functionality in the Amazon SageMaker console. It returns suggestions of possible matches for the property name to use in <code>Search</code> queries. Provides suggestions for <code>HyperParameters</code>, <code>Tags</code>, and <code>Metrics</code>.</p>
    fn get_search_suggestions(
        &self,
        input: GetSearchSuggestionsRequest,
    ) -> RusotoFuture<GetSearchSuggestionsResponse, GetSearchSuggestionsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.GetSearchSuggestions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSearchSuggestionsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSearchSuggestionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the machine learning algorithms that have been created.</p>
    fn list_algorithms(
        &self,
        input: ListAlgorithmsInput,
    ) -> RusotoFuture<ListAlgorithmsOutput, ListAlgorithmsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListAlgorithms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAlgorithmsOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAlgorithmsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of the Git repositories in your account.</p>
    fn list_code_repositories(
        &self,
        input: ListCodeRepositoriesInput,
    ) -> RusotoFuture<ListCodeRepositoriesOutput, ListCodeRepositoriesError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListCodeRepositories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCodeRepositoriesOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCodeRepositoriesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists model compilation jobs that satisfy various filters.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about a particular model compilation job you have created, use <a>DescribeCompilationJob</a>.</p>
    fn list_compilation_jobs(
        &self,
        input: ListCompilationJobsRequest,
    ) -> RusotoFuture<ListCompilationJobsResponse, ListCompilationJobsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListCompilationJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCompilationJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListCompilationJobsError::from_response(response))
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpointConfigs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListEndpointConfigsOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListEndpointsOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListHyperParameterTuningJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListHyperParameterTuningJobsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListHyperParameterTuningJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of labeling jobs.</p>
    fn list_labeling_jobs(
        &self,
        input: ListLabelingJobsRequest,
    ) -> RusotoFuture<ListLabelingJobsResponse, ListLabelingJobsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListLabelingJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLabelingJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLabelingJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of labeling jobs assigned to a specified work team.</p>
    fn list_labeling_jobs_for_workteam(
        &self,
        input: ListLabelingJobsForWorkteamRequest,
    ) -> RusotoFuture<ListLabelingJobsForWorkteamResponse, ListLabelingJobsForWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListLabelingJobsForWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLabelingJobsForWorkteamResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLabelingJobsForWorkteamError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the model packages that have been created.</p>
    fn list_model_packages(
        &self,
        input: ListModelPackagesInput,
    ) -> RusotoFuture<ListModelPackagesOutput, ListModelPackagesError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListModelPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListModelPackagesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListModelPackagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists models created with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
    fn list_models(
        &self,
        input: ListModelsInput,
    ) -> RusotoFuture<ListModelsOutput, ListModelsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListModelsOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.ListNotebookInstanceLifecycleConfigs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListNotebookInstanceLifecycleConfigsOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListNotebookInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListNotebookInstancesOutput, _>()
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

    /// <p>Gets a list of the work teams that you are subscribed to in the AWS Marketplace. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    fn list_subscribed_workteams(
        &self,
        input: ListSubscribedWorkteamsRequest,
    ) -> RusotoFuture<ListSubscribedWorkteamsResponse, ListSubscribedWorkteamsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListSubscribedWorkteams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSubscribedWorkteamsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSubscribedWorkteamsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the tags for the specified Amazon SageMaker resource.</p>
    fn list_tags(&self, input: ListTagsInput) -> RusotoFuture<ListTagsOutput, ListTagsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<ListTagsOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTrainingJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTrainingJobsResponse, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.ListTrainingJobsForHyperParameterTuningJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTrainingJobsForHyperParameterTuningJobResponse, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTransformJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTransformJobsResponse, _>()
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

    /// <p>Gets a list of work teams that you have defined in a region. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    fn list_workteams(
        &self,
        input: ListWorkteamsRequest,
    ) -> RusotoFuture<ListWorkteamsResponse, ListWorkteamsError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListWorkteams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWorkteamsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWorkteamsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Renders the UI template so that you can preview the worker's experience. </p>
    fn render_ui_template(
        &self,
        input: RenderUiTemplateRequest,
    ) -> RusotoFuture<RenderUiTemplateResponse, RenderUiTemplateError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.RenderUiTemplate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RenderUiTemplateResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RenderUiTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Finds Amazon SageMaker resources that match a search query. Matching resource objects are returned as a list of <code>SearchResult</code> objects in the response. You can sort the search results by any resource property in a ascending or descending order.</p> <p>You can query against the following value types: numerical, text, Booleans, and timestamps.</p>
    fn search(&self, input: SearchRequest) -> RusotoFuture<SearchResponse, SearchError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.Search");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<SearchResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchError::from_response(response))),
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StartNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Stops a model compilation job.</p> <p> To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal. This gracefully shuts the job down. If the job hasn't stopped, it sends the SIGKILL signal.</p> <p>When it receives a <code>StopCompilationJob</code> request, Amazon SageMaker changes the <a>CompilationJobSummary$CompilationJobStatus</a> of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the <a>CompilationJobSummary$CompilationJobStatus</a> to <code>Stopped</code>. </p>
    fn stop_compilation_job(
        &self,
        input: StopCompilationJobRequest,
    ) -> RusotoFuture<(), StopCompilationJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopCompilationJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopCompilationJobError::from_response(response))),
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopHyperParameterTuningJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Stops a running labeling job. A job that is stopped cannot be restarted. Any results obtained before the job is stopped are placed in the Amazon S3 output bucket.</p>
    fn stop_labeling_job(
        &self,
        input: StopLabelingJobRequest,
    ) -> RusotoFuture<(), StopLabelingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopLabelingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopLabelingJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. Amazon SageMaker stops charging you for the ML compute instance when you call <code>StopNotebookInstance</code>.</p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> RusotoFuture<(), StopNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> RusotoFuture<(), StopTrainingJobError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTrainingJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTransformJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

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

    /// <p>Updates the specified Git repository with the specified values.</p>
    fn update_code_repository(
        &self,
        input: UpdateCodeRepositoryInput,
    ) -> RusotoFuture<UpdateCodeRepositoryOutput, UpdateCodeRepositoryError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateCodeRepository");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCodeRepositoryOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateCodeRepositoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> RusotoFuture<UpdateEndpointOutput, UpdateEndpointError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateEndpointOutput, _>()
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

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> RusotoFuture<
        UpdateEndpointWeightsAndCapacitiesOutput,
        UpdateEndpointWeightsAndCapacitiesError,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.UpdateEndpointWeightsAndCapacities",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateEndpointWeightsAndCapacitiesOutput, _>()
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

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements.</p>
    fn update_notebook_instance(
        &self,
        input: UpdateNotebookInstanceInput,
    ) -> RusotoFuture<UpdateNotebookInstanceOutput, UpdateNotebookInstanceError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateNotebookInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateNotebookInstanceOutput, _>()
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
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.UpdateNotebookInstanceLifecycleConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateNotebookInstanceLifecycleConfigOutput, _>()
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

    /// <p>Updates an existing work team with new member definitions or description.</p>
    fn update_workteam(
        &self,
        input: UpdateWorkteamRequest,
    ) -> RusotoFuture<UpdateWorkteamResponse, UpdateWorkteamError> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateWorkteamResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateWorkteamError::from_response(response))),
                )
            }
        })
    }
}
