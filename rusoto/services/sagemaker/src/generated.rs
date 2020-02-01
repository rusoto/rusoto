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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of <code>Tag</code> objects. Each tag is a key-value pair. Only the <code>key</code> parameter is required. If you don't specify a value, Amazon SageMaker sets the value to an empty string. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>To generate and save time-series metrics during training, set to <code>true</code>. The default is <code>false</code> and time-series metrics aren&#39;t generated except in the following cases:</p> <ul> <li> <p>You use one of the Amazon SageMaker built-in algorithms</p> </li> <li> <p>You use one of the following <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/pre-built-containers-frameworks-deep-learning.html">Prebuilt Amazon SageMaker Docker Images</a>:</p> <ul> <li> <p>Tensorflow (version &gt;= 1.15)</p> </li> <li> <p>MXNet (version &gt;= 1.6)</p> </li> <li> <p>PyTorch (version &gt;= 1.3)</p> </li> </ul> </li> <li> <p>You specify at least one <a>MetricDefinition</a> </p> </li> </ul></p>
    #[serde(rename = "EnableSageMakerMetricsTimeSeries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sage_maker_metrics_time_series: Option<bool>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The Amazon Resource Name (ARN) of a Lambda function implements the logic for annotation consolidation.</p> <p>For the built-in bounding box, image classification, semantic segmentation, and text classification task types, Amazon SageMaker Ground Truth provides the following Lambda functions:</p> <ul> <li> <p> <i>Bounding box</i> - Finds the most similar boxes from different workers based on the Jaccard index of the boxes.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-BoundingBox</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-BoundingBox</code> </p> </li> <li> <p> <i>Image classification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of an image based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-ImageMultiClass</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-ImageMultiClass</code> </p> </li> <li> <p> <i>Semantic segmentation</i> - Treats each pixel in an image as a multi-class classification and treats pixel annotations from workers as "votes" for the correct label.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-SemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-SemanticSegmentation</code> </p> </li> <li> <p> <i>Text classification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of text based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-TextMultiClass</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-TextMultiClass</code> </p> </li> <li> <p> <i>Named entity recognition</i> - Groups similar selections and calculates aggregate boundaries, resolving to most-assigned label.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-NamedEntityRecognition</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-NamedEntityRecognition</code> </p> </li> <li> <p> <i>Bounding box verification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of verification judgement for bounding box labels based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-VerificationBoundingBox</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-VerificationBoundingBox</code> </p> </li> <li> <p> <i>Semantic segmentation verification</i> - Uses a variant of the Expectation Maximization approach to estimate the true class of verification judgement for semantic segmentation labels based on annotations from individual workers.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-VerificationSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <i>Bounding box adjustment</i> - Finds the most similar boxes from different workers based on the Jaccard index of the adjusted annotations.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-AdjustmentBoundingBox</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-AdjustmentBoundingBox</code> </p> </li> <li> <p> <i>Semantic segmentation adjustment</i> - Treats each pixel in an image as a multi-class classification and treats pixel adjusted annotations from workers as "votes" for the correct label.</p> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:ACS-AdjustmentSemanticSegmentation</code> </p> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:ACS-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-annotation-consolidation.html">Annotation Consolidation</a>.</p>
    #[serde(rename = "AnnotationConsolidationLambdaArn")]
    pub annotation_consolidation_lambda_arn: String,
}

/// <p>The app's details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AppDetails {
    /// <p>The name of the app.</p>
    #[serde(rename = "AppName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// <p>The type of app.</p>
    #[serde(rename = "AppType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<String>,
}

/// <p>Configuration to run a processing job in a specified container image.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSpecification {
    /// <p>The arguments for a container used to run a processing job.</p>
    #[serde(rename = "ContainerArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arguments: Option<Vec<String>>,
    /// <p>The entrypoint for a container used to run a processing job.</p>
    #[serde(rename = "ContainerEntrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_entrypoint: Option<Vec<String>>,
    /// <p>The container image to be run by the processing job.</p>
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateTrialComponentRequest {
    /// <p>The name of the component to associated with the trial.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
    /// <p>The name of the trial to associate with.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateTrialComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
    /// <p>The ARN of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
}

/// <p>An AutoPilot job will return recommendations, or candidates. Each candidate has futher details about the steps involed, and the status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLCandidate {
    /// <p>The candidate name.</p>
    #[serde(rename = "CandidateName")]
    pub candidate_name: String,
    /// <p>The candidate's status.</p>
    #[serde(rename = "CandidateStatus")]
    pub candidate_status: String,
    /// <p>The candidate's steps.</p>
    #[serde(rename = "CandidateSteps")]
    pub candidate_steps: Vec<AutoMLCandidateStep>,
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The end time.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The failure reason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "FinalAutoMLJobObjectiveMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_auto_ml_job_objective_metric: Option<FinalAutoMLJobObjectiveMetric>,
    /// <p>The inference containers.</p>
    #[serde(rename = "InferenceContainers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_containers: Option<Vec<AutoMLContainerDefinition>>,
    /// <p>The last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>The objective status.</p>
    #[serde(rename = "ObjectiveStatus")]
    pub objective_status: String,
}

/// <p>Information about the steps for a Candidate, and what step it is working on.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLCandidateStep {
    /// <p>The ARN for the Candidate's step.</p>
    #[serde(rename = "CandidateStepArn")]
    pub candidate_step_arn: String,
    /// <p>The name for the Candidate's step.</p>
    #[serde(rename = "CandidateStepName")]
    pub candidate_step_name: String,
    /// <p>Whether the Candidate is at the transform, training, or processing step.</p>
    #[serde(rename = "CandidateStepType")]
    pub candidate_step_type: String,
}

/// <p>Similar to Channel. A channel is a named input source that training algorithms can consume. Refer to Channel for detailed descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLChannel {
    /// <p>You can use Gzip or None. The default value is None.</p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p>The data source.</p>
    #[serde(rename = "DataSource")]
    pub data_source: AutoMLDataSource,
    /// <p>The name of the target variable in supervised learning, a.k.a. ‘y’.</p>
    #[serde(rename = "TargetAttributeName")]
    pub target_attribute_name: String,
}

/// <p>A list of container definitions that describe the different containers that make up one AutoML candidate. Refer to ContainerDefinition for more details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLContainerDefinition {
    /// <p>Environment variables to set in the container. Refer to ContainerDefinition for more details.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ECR path of the container. Refer to ContainerDefinition for more details.</p>
    #[serde(rename = "Image")]
    pub image: String,
    /// <p>The location of the model artifacts. Refer to ContainerDefinition for more details.</p>
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: String,
}

/// <p>The data source for the AutoPilot job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLDataSource {
    /// <p>The Amazon S3 location of the data.</p>
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: AutoMLS3DataSource,
}

/// <p>Artifacts that are generation during a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLJobArtifacts {
    /// <p>The URL to the notebook location.</p>
    #[serde(rename = "CandidateDefinitionNotebookLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_definition_notebook_location: Option<String>,
    /// <p>The URL to the notebook location.</p>
    #[serde(rename = "DataExplorationNotebookLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_exploration_notebook_location: Option<String>,
}

/// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLJobCompletionCriteria {
    /// <p>The maximum time, in seconds, an AutoML job is allowed to wait for a trial to complete. It must be equal to or greater than MaxRuntimePerTrainingJobInSeconds.</p>
    #[serde(rename = "MaxAutoMLJobRuntimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_auto_ml_job_runtime_in_seconds: Option<i64>,
    /// <p>The maximum number of times a training job is allowed to run.</p>
    #[serde(rename = "MaxCandidates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_candidates: Option<i64>,
    /// <p>The maximum time, in seconds, a job is allowed to run.</p>
    #[serde(rename = "MaxRuntimePerTrainingJobInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_per_training_job_in_seconds: Option<i64>,
}

/// <p>A collection of settings used for a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLJobConfig {
    /// <p>How long a job is allowed to run, or how many candidates a job is allowed to generate.</p>
    #[serde(rename = "CompletionCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<AutoMLJobCompletionCriteria>,
    /// <p>Security configuration for traffic encryption or Amazon VPC settings.</p>
    #[serde(rename = "SecurityConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config: Option<AutoMLSecurityConfig>,
}

/// <p>Applies a metric to minimize or maximize for the job's objective.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLJobObjective {
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
}

/// <p>Provides a summary about a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLJobSummary {
    /// <p>The ARN of the job.</p>
    #[serde(rename = "AutoMLJobArn")]
    pub auto_ml_job_arn: String,
    /// <p>The name of the object you are requesting.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
    /// <p>The job's secondary status.</p>
    #[serde(rename = "AutoMLJobSecondaryStatus")]
    pub auto_ml_job_secondary_status: String,
    /// <p>The job's status.</p>
    #[serde(rename = "AutoMLJobStatus")]
    pub auto_ml_job_status: String,
    /// <p>When the job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The end time.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The failure reason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>When the job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
}

/// <p>The output data configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLOutputDataConfig {
    /// <p>The AWS KMS encryption key ID.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The Amazon S3 output path. Must be 128 characters or less.</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>The Amazon S3 data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLS3DataSource {
    /// <p>The data type.</p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p>The URL to the Amazon S3 data source.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Security options.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMLSecurityConfig {
    /// <p>Whether to use traffic encryption between the container layers.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>The key used to encrypt stored data.</p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>VPC configuration.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureContentTypeHeader {
    /// <p><p/></p>
    #[serde(rename = "CsvContentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_content_types: Option<Vec<String>>,
    /// <p><p/></p>
    #[serde(rename = "JsonContentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_content_types: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureOption {
    /// <p><p/></p>
    #[serde(rename = "CaptureMode")]
    pub capture_mode: String,
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
    /// <p><p/> <p>Specify RecordIO as the value when input data is in raw format but the training algorithm requires the RecordIO format. In this case, Amazon SageMaker wraps each individual S3 object in a RecordIO record. If the input data is already in RecordIO format, you don&#39;t need to set this attribute. For more information, see <a href="https://mxnet.apache.org/api/architecture/note_data_loading#data-format">Create a Dataset Using RecordIO</a>. </p> <p>In File mode, leave this field unset or set it to None.</p></p>
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

/// <p>Contains information about the output location for managed spot training checkpoint data. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckpointConfig {
    /// <p>(Optional) The local directory where checkpoints are written. The default directory is <code>/opt/ml/checkpoints/</code>. </p>
    #[serde(rename = "LocalPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    /// <p>Identifies the S3 path where you want Amazon SageMaker to store checkpoints. For example, <code>s3://bucket-name/key-name-prefix</code>.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Specifies summary information about a Git repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Configuration information for tensor collections.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionConfiguration {
    /// <p>The name of the tensor collection. The name must be unique relative to other rule configuration names.</p>
    #[serde(rename = "CollectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    /// <p>Parameter values for the tensor collection. The allowed parameters are <code>"name"</code>, <code>"include_regex"</code>, <code>"reduction_config"</code>, <code>"save_config"</code>, <code>"tensor_names"</code>, and <code>"save_histogram"</code>.</p>
    #[serde(rename = "CollectionParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A summary of a model compilation job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>This parameter is ignored for models that contain only a <code>PrimaryContainer</code>.</p> <p>When a <code>ContainerDefinition</code> is part of an inference pipeline, the value of the parameter uniquely identifies the container for the purposes of logging and metrics. For information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/inference-pipeline-logs-metrics.html">Use Logs and Metrics to Monitor an Inference Pipeline</a>. If you don't specify a value for this parameter for a <code>ContainerDefinition</code> that is part of an inference pipeline, a unique name is automatically assigned based on the position of the <code>ContainerDefinition</code> in the pipeline. If you specify a value for the <code>ContainerHostName</code> for any <code>ContainerDefinition</code> that is part of an inference pipeline, you must specify a value for the <code>ContainerHostName</code> parameter of every <code>ContainerDefinition</code> in that pipeline.</p>
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
    /// <p>Whether the container hosts a single model or multiple models.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p><p>The S3 path where the model artifacts, which result from model training, are stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). The S3 path is required for Amazon SageMaker built-in algorithms, but not if you use your own algorithms. For more information on built-in algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-algo-docker-registry-paths.html">Common Parameters</a>. </p> <p>If you provide a value for this parameter, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provide. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <important> <p>If you use a built-in algorithm to create a model, Amazon SageMaker requires that you provide a S3 path to the model artifacts in <code>ModelDataUrl</code>.</p> </important></p>
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<String>,
    /// <p>The name or Amazon Resource Name (ARN) of the model package to use to create the model.</p>
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
    /// <p><p>The scale that hyperparameter tuning uses to search the hyperparameter range. For information about choosing a hyperparameter scale, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon SageMaker hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have only values greater than 0.</p> </dd> <dt>ReverseLogarithmic</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a reverse logarithmic scale.</p> <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0&lt;=x&lt;1.0.</p> </dd> </dl></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAlgorithmOutput {
    /// <p>The Amazon Resource Name (ARN) of the new algorithm.</p>
    #[serde(rename = "AlgorithmArn")]
    pub algorithm_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAppRequest {
    /// <p>The name of the app.</p>
    #[serde(rename = "AppName")]
    pub app_name: String,
    /// <p>The type of app.</p>
    #[serde(rename = "AppType")]
    pub app_type: String,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The instance type and quantity.</p>
    #[serde(rename = "ResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_spec: Option<ResourceSpec>,
    /// <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAppResponse {
    /// <p>The app's Amazon Resource Name (ARN).</p>
    #[serde(rename = "AppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAutoMLJobRequest {
    /// <p>Contains CompletionCriteria and SecurityConfig.</p>
    #[serde(rename = "AutoMLJobConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_config: Option<AutoMLJobConfig>,
    /// <p>Identifies an AutoPilot job. Must be unique to your account and is case-insensitive.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
    /// <p>Defines the job's objective. You provide a MetricName and AutoML will infer minimize or maximize. If this is not provided, the most commonly used ObjectiveMetric for problem type will be selected.</p>
    #[serde(rename = "AutoMLJobObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_objective: Option<AutoMLJobObjective>,
    /// <p>This will generate possible candidates without training a model. A candidate is a combination of data preprocessors, algorithms, and algorithm parameter settings.</p>
    #[serde(rename = "GenerateCandidateDefinitionsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_candidate_definitions_only: Option<bool>,
    /// <p>Similar to InputDataConfig supported by Tuning. Format(s) supported: CSV.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<AutoMLChannel>,
    /// <p>Similar to OutputDataConfig supported by Tuning. Format(s) supported: CSV.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: AutoMLOutputDataConfig,
    /// <p>Defines the kind of preprocessing and algorithms intended for the candidates. Options include: BinaryClassification, MulticlassClassification, and Regression.</p>
    #[serde(rename = "ProblemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_type: Option<String>,
    /// <p>The ARN of the role that will be used to access the data.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAutoMLJobResponse {
    /// <p>When a job is created, it is assigned a unique ARN.</p>
    #[serde(rename = "AutoMLJobArn")]
    pub auto_ml_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCodeRepositoryInput {
    /// <p>The name of the Git repository. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
    /// <p>Specifies details about the repository, including the URL where the repository is located, the default branch, and credentials to use to access the repository.</p>
    #[serde(rename = "GitConfig")]
    pub git_config: GitConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCodeRepositoryOutput {
    /// <p>The Amazon Resource Name (ARN) of the new repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Specifies a limit to how long a model compilation job can run. When the job reaches the time limit, Amazon SageMaker ends the compilation job. Use this API to cap model training costs.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCompilationJobResponse {
    /// <p><p>If the action is successful, the service sends back an HTTP 200 response. Amazon SageMaker returns the following data in JSON format:</p> <ul> <li> <p> <code>CompilationJobArn</code>: The Amazon Resource Name (ARN) of the compiled job.</p> </li> </ul></p>
    #[serde(rename = "CompilationJobArn")]
    pub compilation_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDomainRequest {
    /// <p>The mode of authentication that member use to access the domain.</p>
    #[serde(rename = "AuthMode")]
    pub auth_mode: String,
    /// <p>The default user settings.</p>
    #[serde(rename = "DefaultUserSettings")]
    pub default_user_settings: UserSettings,
    /// <p>A name for the domain.</p>
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// <p>The AWS Key Management Service encryption key ID.</p>
    #[serde(rename = "HomeEfsFileSystemKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_efs_file_system_kms_key_id: Option<String>,
    /// <p>Security setting to limit to a set of subnets.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Security setting to limit the domain's communication to a Amazon Virtual Private Cloud.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDomainResponse {
    /// <p>The Amazon Resource Name (ARN) of the created domain.</p>
    #[serde(rename = "DomainArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The URL to the created domain.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEndpointConfigInput {
    #[serde(rename = "DataCaptureConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_capture_config: Option<DataCaptureConfig>,
    /// <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p><p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p> <p>The KmsKeyId can be any of the following formats: </p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the AWS Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in AWS KMS </a> </p> <note> <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can&#39;t request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p> <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p> <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEndpointConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint configuration. </p>
    #[serde(rename = "EndpointConfigArn")]
    pub endpoint_config_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEndpointOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateExperimentRequest {
    /// <p>The description of the experiment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the experiment as displayed. The name doesn't need to be unique. If you don't specify <code>DisplayName</code>, the value in <code>ExperimentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the experiment. The name must be unique in your AWS account and is not case-sensitive.</p>
    #[serde(rename = "ExperimentName")]
    pub experiment_name: String,
    /// <p>A list of tags to associate with the experiment. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateExperimentResponse {
    /// <p>The Amazon Resource Name (ARN) of the experiment.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFlowDefinitionRequest {
    /// <p>The name of your flow definition.</p>
    #[serde(rename = "FlowDefinitionName")]
    pub flow_definition_name: String,
    /// <p>An object containing information about the events that trigger a human workflow.</p>
    #[serde(rename = "HumanLoopActivationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_config: Option<HumanLoopActivationConfig>,
    /// <p>An object containing information about the tasks the human reviewers will perform.</p>
    #[serde(rename = "HumanLoopConfig")]
    pub human_loop_config: HumanLoopConfig,
    /// <p>An object containing information about where the human review results will be uploaded.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: FlowDefinitionOutputConfig,
    /// <p>The Amazon Resource Name (ARN) of the role needed to call other services on your behalf. For example, <code>arn:aws:iam::1234567890:role/service-role/AmazonSageMaker-ExecutionRole-20180111T151298</code>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>An array of key-value pairs that contain metadata to help you categorize and organize a flow definition. Each tag consists of a key and a value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFlowDefinitionResponse {
    /// <p>The Amazon Resource Name (ARN) of the flow definition you create.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHumanTaskUiRequest {
    /// <p>The name of the user interface you are creating.</p>
    #[serde(rename = "HumanTaskUiName")]
    pub human_task_ui_name: String,
    /// <p>An array of key-value pairs that contain metadata to help you categorize and organize a human review workflow user interface. Each tag consists of a key and a value, both of which you define.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UiTemplate")]
    pub ui_template: UiTemplate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHumanTaskUiResponse {
    /// <p>The Amazon Resource Name (ARN) of the human review workflow user interface you create.</p>
    #[serde(rename = "HumanTaskUiArn")]
    pub human_task_ui_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p/></p>
    #[serde(rename = "TrainingJobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_definitions: Option<Vec<HyperParameterTrainingJobDefinition>>,
    /// <p><p>Specifies the configuration for starting the hyperparameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p> <p>All training jobs launched by the new hyperparameter tuning job are evaluated by using the objective metric. If you specify <code>IDENTICAL<em>DATA</em>AND_ALGORITHM</code> as the <code>WarmStartType</code> value for the warm start configuration, the training job that performs the best in the new tuning job is compared to the best training jobs from the parent tuning jobs. From these, the training job that performs the best as measured by the objective metric is returned as the overall best training job.</p> <note> <p>All training jobs launched by parent hyperparameter tuning jobs and the new hyperparameter tuning jobs count against the limit of training jobs for the tuning job.</p> </note></p>
    #[serde(rename = "WarmStartConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_start_config: Option<HyperParameterTuningJobWarmStartConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHyperParameterTuningJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the tuning job. Amazon SageMaker assigns an ARN to a hyperparameter tuning job when you create it.</p>
    #[serde(rename = "HyperParameterTuningJobArn")]
    pub hyper_parameter_tuning_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLabelingJobRequest {
    /// <p>Configures the labeling task and how it is presented to workers; including, but not limited to price, keywords, and batch size (task count).</p>
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
    /// <p>An array of key/value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLabelingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the labeling job. You use this ARN to identify the labeling job.</p>
    #[serde(rename = "LabelingJobArn")]
    pub labeling_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateModelInput {
    /// <p>Specifies the containers in the inference pipeline.</p>
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerDefinition>>,
    /// <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateModelOutput {
    /// <p>The ARN of the model created in Amazon SageMaker.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateModelPackageOutput {
    /// <p>The Amazon Resource Name (ARN) of the new model package.</p>
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMonitoringScheduleRequest {
    /// <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    #[serde(rename = "MonitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleConfig,
    /// <p>The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
    /// <p>(Optional) An array of key-value pairs. For more information, see <a href=" https://docs-aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMonitoringScheduleResponse {
    /// <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleArn")]
    pub monitoring_schedule_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNotebookInstanceInput {
    /// <p>A list of Elastic Inference (EI) instance types to associate with this notebook instance. Currently, only one instance type can be associated with a notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
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
    /// <p>The Amazon Resource Name (ARN) of a AWS Key Management Service key that Amazon SageMaker uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotebookInstanceLifecycleConfigOutput {
    /// <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotebookInstanceOutput {
    /// <p>The Amazon Resource Name (ARN) of the notebook instance. </p>
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePresignedDomainUrlRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The session expiration duration in seconds.</p>
    #[serde(rename = "SessionExpirationDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expiration_duration_in_seconds: Option<i64>,
    /// <p>The name of the UserProfile to sign-in as.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePresignedDomainUrlResponse {
    /// <p>The presigned URL.</p>
    #[serde(rename = "AuthorizedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePresignedNotebookInstanceUrlOutput {
    /// <p>A JSON object that contains the URL string. </p>
    #[serde(rename = "AuthorizedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProcessingJobRequest {
    /// <p>Configures the processing job to run a specified Docker container image.</p>
    #[serde(rename = "AppSpecification")]
    pub app_specification: AppSpecification,
    /// <p>Sets the environment variables in the Docker container.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
    /// <p>Networking options for a processing job.</p>
    #[serde(rename = "NetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,
    /// <p>For each input, data is downloaded from S3 into the processing container before the processing job begins running if "S3InputMode" is set to <code>File</code>.</p>
    #[serde(rename = "ProcessingInputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_inputs: Option<Vec<ProcessingInput>>,
    /// <p> The name of the processing job. The name must be unique within an AWS Region in the AWS account.</p>
    #[serde(rename = "ProcessingJobName")]
    pub processing_job_name: String,
    /// <p>Output configuration for the processing job.</p>
    #[serde(rename = "ProcessingOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_output_config: Option<ProcessingOutputConfig>,
    /// <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p>
    #[serde(rename = "ProcessingResources")]
    pub processing_resources: ProcessingResources,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The time limit for how long the processing job is allowed to run.</p>
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<ProcessingStoppingCondition>,
    /// <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs-aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProcessingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the processing job.</p>
    #[serde(rename = "ProcessingJobArn")]
    pub processing_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrainingJobRequest {
    /// <p>The registry path of the Docker image that contains the training algorithm and algorithm-specific metadata, including the input mode. For more information about algorithms provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about providing your own algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p>Contains information about the output location for managed spot training checkpoint data.</p>
    #[serde(rename = "CheckpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<CheckpointConfig>,
    #[serde(rename = "DebugHookConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_hook_config: Option<DebugHookConfig>,
    /// <p>Configuration information for debugging rules.</p>
    #[serde(rename = "DebugRuleConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_rule_configurations: Option<Vec<DebugRuleConfiguration>>,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-encrypt.html">Protect Communications Between ML Compute Instances in a Distributed Training Job</a>.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>To train models using managed spot training, choose <code>True</code>. Managed spot training provides a fully managed and scalable infrastructure for training machine learning models. this option is useful when training jobs can be interrupted and when there is flexibility when the training job is run. </p> <p>The complete and intermediate results of jobs are stored in an Amazon S3 bucket, and can be used as a starting point to train models incrementally. Amazon SageMaker provides metrics and logs in CloudWatch. They can be used to see when managed spot training jobs are running, interrupted, resumed, or completed. </p>
    #[serde(rename = "EnableManagedSpotTraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_spot_training: Option<bool>,
    /// <p>Isolates the training container. No inbound or outbound network calls can be made, except for calls between peers within a training cluster for distributed training. If you enable network isolation for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
    /// <p>Algorithm-specific parameters that influence the quality of the model. You set hyperparameters before you start the learning process. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> <p>You can specify a maximum of 100 hyperparameters. Each hyperparameter is a key-value pair. Each key and value is limited to 256 characters, as specified by the <code>Length Constraint</code>. </p>
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of <code>Channel</code> objects. Each channel is a named input source. <code>InputDataConfig</code> describes the input data and its location. </p> <p>Algorithms can accept input data from one or more channels. For example, an algorithm might have two channels of input data, <code>training_data</code> and <code>validation_data</code>. The configuration for each channel provides the S3, EFS, or FSx location where the input data is stored. It also provides information about the stored data: the MIME type, compression method, and whether the data is wrapped in RecordIO format. </p> <p>Depending on the input mode that the algorithm supports, Amazon SageMaker either copies input data files from an S3 bucket to a local directory in the Docker container, or makes it available as input streams. For example, if you specify an EFS location, input data files will be made available as input streams. They do not need to be downloaded.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<Vec<Channel>>,
    /// <p>Specifies the path to the S3 location where you want to store model artifacts. Amazon SageMaker creates subfolders for the artifacts. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The resources, including the ML compute instances and ML storage volumes, to use for model training. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use ML storage volumes for scratch space. If you want Amazon SageMaker to use the ML storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p>
    #[serde(rename = "ResourceConfig")]
    pub resource_config: ResourceConfig,
    /// <p><p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf. </p> <p>During model training, Amazon SageMaker needs your permission to read input data from an S3 bucket, download a Docker image that contains training code, write model artifacts to an S3 bucket, write logs to Amazon CloudWatch Logs, and publish metrics to Amazon CloudWatch. You grant permissions for all of these tasks to an IAM role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Specifies a limit to how long a model training job can run. When the job reaches the time limit, Amazon SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TensorBoardOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tensor_board_output_config: Option<TensorBoardOutputConfig>,
    /// <p>The name of the training job. The name must be unique within an AWS Region in an AWS account. </p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
    /// <p>A <a>VpcConfig</a> object that specifies the VPC that you want your training job to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTrainingJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the training job.</p>
    #[serde(rename = "TrainingJobArn")]
    pub training_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTransformJobRequest {
    /// <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set the <code>SplitType</code> property of the <a>DataProcessing</a> object to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p> <p>To use only one record when making an HTTP invocation request to a container, set <code>BatchStrategy</code> to <code>SingleRecord</code> and <code>SplitType</code> to <code>Line</code>.</p> <p>To fit as many records in a mini-batch as can fit within the <code>MaxPayloadInMB</code> limit, set <code>BatchStrategy</code> to <code>MultiRecord</code> and <code>SplitType</code> to <code>Line</code>.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>The data structure used to specify the data to be used for inference in a batch transform job and to associate the data that is relevant to the prediction results in the output. The input filter provided allows you to exclude input data that is not needed for inference in a batch transform job. The output filter provided allows you to include input data relevant to interpreting the predictions in the output from the job. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html">Associate Prediction Results with their Corresponding Input Records</a>.</p>
    #[serde(rename = "DataProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_processing: Option<DataProcessing>,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
    /// <p>The maximum number of parallel requests that can be sent to each instance in a transform job. If <code>MaxConcurrentTransforms</code> is set to <code>0</code> or left unset, Amazon SageMaker checks the optional execution-parameters to determine the settings for your chosen algorithm. If the execution-parameters endpoint is not enabled, the default value is <code>1</code>. For more information on execution-parameters, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-batch-code.html#your-algorithms-batch-code-how-containe-serves-requests">How Containers Serve Requests</a>. For built-in algorithms, you don't need to set a value for <code>MaxConcurrentTransforms</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTransformJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the transform job.</p>
    #[serde(rename = "TransformJobArn")]
    pub transform_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrialComponentRequest {
    /// <p>The name of the component as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the component ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input artifacts for the component. Examples of input artifacts are datasets, algorithms, hyperparameters, source code, and instance types.</p>
    #[serde(rename = "InputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The output artifacts for the component. Examples of output artifacts are metrics, snapshots, logs, and images.</p>
    #[serde(rename = "OutputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The hyperparameters for the component.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, TrialComponentParameterValue>>,
    /// <p>When the component started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The status of the component. States include:</p> <ul> <li> <p>InProgress</p> </li> <li> <p>Completed</p> </li> <li> <p>Failed</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialComponentStatus>,
    /// <p>A list of tags to associate with the component. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the component. The name must be unique in your AWS account and is not case-sensitive.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTrialComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrialRequest {
    /// <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the experiment to associate the trial with.</p>
    #[serde(rename = "ExperimentName")]
    pub experiment_name: String,
    /// <p>A list of tags to associate with the trial. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the trial. The name must be unique in your AWS account and is not case-sensitive.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTrialResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserProfileRequest {
    /// <p>The ID of the associated Domain.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>A specifier for the type of value specified in SingleSignOnUserValue. Currently, the only supported value is "UserName". If the Domain's AuthMode is SSO, this field is required. If the Domain's AuthMode is not SSO, this field cannot be specified. </p>
    #[serde(rename = "SingleSignOnUserIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on_user_identifier: Option<String>,
    /// <p>The username of the associated AWS Single Sign-On User for this UserProfile. If the Domain's AuthMode is SSO, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not SSO, this field cannot be specified. </p>
    #[serde(rename = "SingleSignOnUserValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on_user_value: Option<String>,
    /// <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A name for the UserProfile.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
    /// <p>A collection of settings.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserProfileResponse {
    /// <p>The user profile Amazon Resource Name (ARN).</p>
    #[serde(rename = "UserProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWorkteamRequest {
    /// <p>A description of the work team.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A list of <code>MemberDefinition</code> objects that contains objects that identify the Amazon Cognito user pool that makes up the work team. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools.html">Amazon Cognito User Pools</a>.</p> <p>All of the <code>CognitoMemberDefinition</code> objects that make up the member definition must have the same <code>ClientId</code> and <code>UserPool</code> values.</p>
    #[serde(rename = "MemberDefinitions")]
    pub member_definitions: Vec<MemberDefinition>,
    /// <p>Configures notification of workers regarding available or expiring work items.</p>
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>An array of key-value pairs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html">Resource Tag</a> and <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i> AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the work team. Use this name to identify the work team.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorkteamResponse {
    /// <p>The Amazon Resource Name (ARN) of the work team. You can use this ARN to identify the work team.</p>
    #[serde(rename = "WorkteamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workteam_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataCaptureConfig {
    /// <p><p/></p>
    #[serde(rename = "CaptureContentTypeHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_content_type_header: Option<CaptureContentTypeHeader>,
    /// <p><p/></p>
    #[serde(rename = "CaptureOptions")]
    pub capture_options: Vec<CaptureOption>,
    /// <p><p/></p>
    #[serde(rename = "DestinationS3Uri")]
    pub destination_s3_uri: String,
    /// <p><p/></p>
    #[serde(rename = "EnableCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_capture: Option<bool>,
    /// <p><p/></p>
    #[serde(rename = "InitialSamplingPercentage")]
    pub initial_sampling_percentage: i64,
    /// <p><p/></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataCaptureConfigSummary {
    /// <p><p/></p>
    #[serde(rename = "CaptureStatus")]
    pub capture_status: String,
    /// <p><p/></p>
    #[serde(rename = "CurrentSamplingPercentage")]
    pub current_sampling_percentage: i64,
    /// <p><p/></p>
    #[serde(rename = "DestinationS3Uri")]
    pub destination_s3_uri: String,
    /// <p><p/></p>
    #[serde(rename = "EnableCapture")]
    pub enable_capture: bool,
    /// <p><p/></p>
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: String,
}

/// <p>The data structure used to specify the data to be used for inference in a batch transform job and to associate the data that is relevant to the prediction results in the output. The input filter provided allows you to exclude input data that is not needed for inference in a batch transform job. The output filter provided allows you to include input data relevant to interpreting the predictions in the output from the job. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html">Associate Prediction Results with their Corresponding Input Records</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataProcessing {
    /// <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html#data-processing-operators">JSONPath</a> expression used to select a portion of the input data to pass to the algorithm. Use the <code>InputFilter</code> parameter to exclude fields, such as an ID column, from the input. If you want Amazon SageMaker to pass the entire input dataset to the algorithm, accept the default value <code>$</code>.</p> <p>Examples: <code>"$"</code>, <code>"$[1:]"</code>, <code>"$.features"</code> </p>
    #[serde(rename = "InputFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_filter: Option<String>,
    /// <p>Specifies the source of the data to join with the transformed data. The valid values are <code>None</code> and <code>Input</code>. The default value is <code>None</code>, which specifies not to join the input with the transformed data. If you want the batch transform job to join the original input data with the transformed data, set <code>JoinSource</code> to <code>Input</code>. </p> <p>For JSON or JSONLines objects, such as a JSON array, Amazon SageMaker adds the transformed data to the input JSON object in an attribute called <code>SageMakerOutput</code>. The joined result for JSON must be a key-value pair object. If the input is not a key-value pair object, Amazon SageMaker creates a new JSON file. In the new JSON file, and the input data is stored under the <code>SageMakerInput</code> key and the results are stored in <code>SageMakerOutput</code>.</p> <p>For CSV files, Amazon SageMaker combines the transformed data with the input data at the end of the input data and stores it in the output file. The joined data has the joined input data followed by the transformed data and the output is a CSV file. </p>
    #[serde(rename = "JoinSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_source: Option<String>,
    /// <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html#data-processing-operators">JSONPath</a> expression used to select a portion of the joined dataset to save in the output file for a batch transform job. If you want Amazon SageMaker to store the entire input dataset in the output file, leave the default value, <code>$</code>. If you specify indexes that aren't within the dimension size of the joined dataset, you get an error.</p> <p>Examples: <code>"$"</code>, <code>"$[0,5:]"</code>, <code>"$['id','SageMakerOutput']"</code> </p>
    #[serde(rename = "OutputFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_filter: Option<String>,
}

/// <p>Describes the location of the channel data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    /// <p>The file system that is associated with a channel.</p>
    #[serde(rename = "FileSystemDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_data_source: Option<FileSystemDataSource>,
    /// <p>The S3 location of the data source that is associated with a channel.</p>
    #[serde(rename = "S3DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_source: Option<S3DataSource>,
}

/// <p>Configuration information for the debug hook parameters, collection configuration, and storage paths.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugHookConfig {
    /// <p>Configuration information for tensor collections.</p>
    #[serde(rename = "CollectionConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_configurations: Option<Vec<CollectionConfiguration>>,
    /// <p>Configuration information for the debug hook parameters.</p>
    #[serde(rename = "HookParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Path to local storage location for tensors. Defaults to <code>/opt/ml/output/tensors/</code>.</p>
    #[serde(rename = "LocalPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    /// <p>Path to Amazon S3 storage location for tensors.</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Configuration information for debugging rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugRuleConfiguration {
    /// <p>The instance type to deploy for a training job.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>Path to local storage location for output of rules. Defaults to <code>/opt/ml/processing/output/rule/</code>.</p>
    #[serde(rename = "LocalPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    /// <p>The name of the rule configuration. It must be unique relative to other rule configuration names.</p>
    #[serde(rename = "RuleConfigurationName")]
    pub rule_configuration_name: String,
    /// <p>The Amazon Elastic Container (ECR) Image for the managed rule evaluation.</p>
    #[serde(rename = "RuleEvaluatorImage")]
    pub rule_evaluator_image: String,
    /// <p> Runtime configuration for rule container.</p>
    #[serde(rename = "RuleParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Path to Amazon S3 storage location for rules.</p>
    #[serde(rename = "S3OutputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_path: Option<String>,
    /// <p>The size, in GB, of the ML storage volume attached to the processing instance.</p>
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_gb: Option<i64>,
}

/// <p>Information about the status of the rule evaluation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DebugRuleEvaluationStatus {
    /// <p>Timestamp when the rule evaluation status was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the rule configuration</p>
    #[serde(rename = "RuleConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_configuration_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the rule evaluation job.</p>
    #[serde(rename = "RuleEvaluationJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluation_job_arn: Option<String>,
    /// <p>Status of the rule evaluation.</p>
    #[serde(rename = "RuleEvaluationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluation_status: Option<String>,
    /// <p>Details from the rule evaluation.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAlgorithmInput {
    /// <p>The name of the algorithm to delete.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppRequest {
    /// <p>The name of the app.</p>
    #[serde(rename = "AppName")]
    pub app_name: String,
    /// <p>The type of app.</p>
    #[serde(rename = "AppType")]
    pub app_type: String,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCodeRepositoryInput {
    /// <p>The name of the Git repository to delete.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDomainRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The retention policy for this domain, which specifies which resources will be retained after the Domain is deleted. By default, all resources are retained (not automatically deleted). </p>
    #[serde(rename = "RetentionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointConfigInput {
    /// <p>The name of the endpoint configuration that you want to delete.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointInput {
    /// <p>The name of the endpoint that you want to delete.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteExperimentRequest {
    /// <p>The name of the experiment to delete.</p>
    #[serde(rename = "ExperimentName")]
    pub experiment_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteExperimentResponse {
    /// <p>The Amazon Resource Name (ARN) of the experiment that is being deleted.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFlowDefinitionRequest {
    /// <p>The name of the flow definition you are deleting.</p>
    #[serde(rename = "FlowDefinitionName")]
    pub flow_definition_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFlowDefinitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteModelInput {
    /// <p>The name of the model to delete.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteModelPackageInput {
    /// <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMonitoringScheduleRequest {
    /// <p>The name of the monitoring schedule to delete.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotebookInstanceInput {
    /// <p>The name of the Amazon SageMaker notebook instance to delete.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration to delete.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsInput {
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags you want to delete.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array or one or more tag keys to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrialComponentRequest {
    /// <p>The name of the component to delete.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTrialComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the component is being deleted.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrialRequest {
    /// <p>The name of the trial to delete.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTrialResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial that is being deleted.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserProfileRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWorkteamRequest {
    /// <p>The name of the work team to delete.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWorkteamResponse {
    /// <p>Returns <code>true</code> if the work team was successfully deleted; otherwise, returns <code>false</code>.</p>
    #[serde(rename = "Success")]
    pub success: bool,
}

/// <p>Gets the Amazon EC2 Container Registry path of the docker image of the model that is hosted in this <a>ProductionVariant</a>.</p> <p>If you used the <code>registry/repository[:tag]</code> form to specify the image path of the primary container when you created the model hosted in this <code>ProductionVariant</code>, the path resolves to a path of the form <code>registry/repository[@digest]</code>. A digest is a hash value that identifies a specific version of an image. For information about Amazon ECR paths, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-pull-ecr-image.html">Pulling an Image</a> in the <i>Amazon ECR User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAlgorithmInput {
    /// <p>The name of the algorithm to describe.</p>
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAppRequest {
    /// <p>The name of the app.</p>
    #[serde(rename = "AppName")]
    pub app_name: String,
    /// <p>The type of app.</p>
    #[serde(rename = "AppType")]
    pub app_type: String,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAppResponse {
    /// <p>The app's Amazon Resource Name (ARN).</p>
    #[serde(rename = "AppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    /// <p>The name of the app.</p>
    #[serde(rename = "AppName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// <p>The type of app.</p>
    #[serde(rename = "AppType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The failure reason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The timestamp of the last health check.</p>
    #[serde(rename = "LastHealthCheckTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_check_timestamp: Option<f64>,
    /// <p>The timestamp of the last user's activity.</p>
    #[serde(rename = "LastUserActivityTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_user_activity_timestamp: Option<f64>,
    /// <p>The instance type and quantity.</p>
    #[serde(rename = "ResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_spec: Option<ResourceSpec>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAutoMLJobRequest {
    /// <p>Request information about a job using that job's unique name.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAutoMLJobResponse {
    /// <p>Returns the job's ARN.</p>
    #[serde(rename = "AutoMLJobArn")]
    pub auto_ml_job_arn: String,
    /// <p>Returns information on the job's artifacts found in AutoMLJobArtifacts.</p>
    #[serde(rename = "AutoMLJobArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_artifacts: Option<AutoMLJobArtifacts>,
    /// <p>Returns the job's config.</p>
    #[serde(rename = "AutoMLJobConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_config: Option<AutoMLJobConfig>,
    /// <p>Returns the name of a job.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
    /// <p>Returns the job's objective.</p>
    #[serde(rename = "AutoMLJobObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_objective: Option<AutoMLJobObjective>,
    /// <p>Returns the job's AutoMLJobSecondaryStatus.</p>
    #[serde(rename = "AutoMLJobSecondaryStatus")]
    pub auto_ml_job_secondary_status: String,
    /// <p>Returns the job's AutoMLJobStatus.</p>
    #[serde(rename = "AutoMLJobStatus")]
    pub auto_ml_job_status: String,
    /// <p>Returns the job's BestCandidate.</p>
    #[serde(rename = "BestCandidate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_candidate: Option<AutoMLCandidate>,
    /// <p>Returns the job's creation time.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Returns the job's end time.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Returns the job's FailureReason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>Returns the job's output from GenerateCandidateDefinitionsOnly.</p>
    #[serde(rename = "GenerateCandidateDefinitionsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_candidate_definitions_only: Option<bool>,
    /// <p>Returns the job's input data config.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: Vec<AutoMLChannel>,
    /// <p>Returns the job's last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>Returns the job's output data config.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: AutoMLOutputDataConfig,
    /// <p>Returns the job's problem type.</p>
    #[serde(rename = "ProblemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_type: Option<String>,
    /// <p>This contains ProblemType, AutoMLJobObjective and CompletionCriteria. They’re auto-inferred values, if not provided by you. If you do provide them, then they’ll be the same as provided.</p>
    #[serde(rename = "ResolvedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_attributes: Option<ResolvedAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCodeRepositoryInput {
    /// <p>The name of the Git repository to describe.</p>
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCompilationJobRequest {
    /// <p>The name of the model compilation job that you want information about.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Specifies a limit to how long a model compilation job can run. When the job reaches the time limit, Amazon SageMaker ends the compilation job. Use this API to cap model training costs.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDomainRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDomainResponse {
    /// <p>The domain's authentication mode.</p>
    #[serde(rename = "AuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<String>,
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Settings which are applied to all UserProfile in this domain, if settings are not explicitly specified in a given UserProfile. </p>
    #[serde(rename = "DefaultUserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user_settings: Option<UserSettings>,
    /// <p>The domain's Amazon Resource Name (ARN).</p>
    #[serde(rename = "DomainArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The failure reason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The ID of the Amazon Elastic File System (EFS) managed by this Domain.</p>
    #[serde(rename = "HomeEfsFileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_efs_file_system_id: Option<String>,
    /// <p>The AWS Key Management Service encryption key ID.</p>
    #[serde(rename = "HomeEfsFileSystemKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_efs_file_system_kms_key_id: Option<String>,
    /// <p>The last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The SSO managed application instance ID.</p>
    #[serde(rename = "SingleSignOnManagedApplicationInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on_managed_application_instance_id: Option<String>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Security setting to limit to a set of subnets.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The domain's URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The ID of the Amazon Virtual Private Cloud.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointConfigInput {
    /// <p>The name of the endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointConfigOutput {
    /// <p>A timestamp that shows when the endpoint configuration was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    #[serde(rename = "DataCaptureConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_capture_config: Option<DataCaptureConfig>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointInput {
    /// <p>The name of the endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointOutput {
    /// <p>A timestamp that shows when the endpoint was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    #[serde(rename = "DataCaptureConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_capture_config: Option<DataCaptureConfigSummary>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeExperimentRequest {
    /// <p>The name of the experiment to describe.</p>
    #[serde(rename = "ExperimentName")]
    pub experiment_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeExperimentResponse {
    /// <p>Who created the experiment.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the experiment was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description of the experiment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the experiment as displayed. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the experiment.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
    /// <p>The name of the experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>Who last modified the experiment.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the experiment was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The ARN of the source and, optionally, the type.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ExperimentSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFlowDefinitionRequest {
    /// <p>The name of the flow definition.</p>
    #[serde(rename = "FlowDefinitionName")]
    pub flow_definition_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFlowDefinitionResponse {
    /// <p>The timestamp when the flow definition was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p><p/></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow defintion.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionName")]
    pub flow_definition_name: String,
    /// <p>The status of the flow definition. Valid values are listed below.</p>
    #[serde(rename = "FlowDefinitionStatus")]
    pub flow_definition_status: String,
    /// <p>An object containing information about what triggers a human review workflow.</p>
    #[serde(rename = "HumanLoopActivationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_config: Option<HumanLoopActivationConfig>,
    /// <p>An object containing information about who works on the task, the workforce task price, and other task details.</p>
    #[serde(rename = "HumanLoopConfig")]
    pub human_loop_config: HumanLoopConfig,
    /// <p>An object containing information about the output file.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: FlowDefinitionOutputConfig,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) execution role for the flow definition.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHumanTaskUiRequest {
    /// <p>The name of the human task user interface you want information about.</p>
    #[serde(rename = "HumanTaskUiName")]
    pub human_task_ui_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHumanTaskUiResponse {
    /// <p>The timestamp when the human task user interface was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the human task user interface.</p>
    #[serde(rename = "HumanTaskUiArn")]
    pub human_task_ui_arn: String,
    /// <p>The name of the human task user interface.</p>
    #[serde(rename = "HumanTaskUiName")]
    pub human_task_ui_name: String,
    #[serde(rename = "UiTemplate")]
    pub ui_template: UiTemplateInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job to describe.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p/></p>
    #[serde(rename = "TrainingJobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_definitions: Option<Vec<HyperParameterTrainingJobDefinition>>,
    /// <p>The <a>TrainingJobStatusCounters</a> object that specifies the number of training jobs, categorized by status, that this tuning job launched.</p>
    #[serde(rename = "TrainingJobStatusCounters")]
    pub training_job_status_counters: TrainingJobStatusCounters,
    /// <p>The configuration for starting the hyperparameter parameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p>
    #[serde(rename = "WarmStartConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_start_config: Option<HyperParameterTuningJobWarmStartConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLabelingJobRequest {
    /// <p>The name of the labeling job to return information for.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The S3 location of the JSON file that defines the categories used to label data objects. Please note the following label-category limits:</p> <ul> <li> <p>Semantic segmentation labeling jobs using automated labeling: 20 labels</p> </li> <li> <p>Box bounding labeling jobs (all): 10 lables</p> </li> </ul> <p>The file is a JSON structure in the following format:</p> <p> <code>{</code> </p> <p> <code> "document-version": "2018-11-28"</code> </p> <p> <code> "labels": [</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 1</i>"</code> </p> <p> <code> },</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 2</i>"</code> </p> <p> <code> },</code> </p> <p> <code> ...</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label n</i>"</code> </p> <p> <code> }</code> </p> <p> <code> ]</code> </p> <p> <code>}</code> </p>
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
    /// <p>An array of key/value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeModelInput {
    /// <p>The name of the model.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeModelOutput {
    /// <p>The containers in the inference pipeline.</p>
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerDefinition>>,
    /// <p>A timestamp that shows when the model was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>If <code>True</code>, no inbound or outbound network calls can be made to or from the model container.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeModelPackageInput {
    /// <p>The name of the model package to describe.</p>
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMonitoringScheduleRequest {
    /// <p>Name of a previously created monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMonitoringScheduleResponse {
    /// <p>The time at which the monitoring job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p> The name of the endpoint for the monitoring job.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>A string, up to one KB in size, that contains the reason a monitoring job failed, if it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The time at which the monitoring job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>Describes metadata on the last execution to run, if there was one.</p>
    #[serde(rename = "LastMonitoringExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_monitoring_execution_summary: Option<MonitoringExecutionSummary>,
    /// <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleArn")]
    pub monitoring_schedule_arn: String,
    /// <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    #[serde(rename = "MonitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleConfig,
    /// <p>Name of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
    /// <p>The status of an monitoring job.</p>
    #[serde(rename = "MonitoringScheduleStatus")]
    pub monitoring_schedule_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNotebookInstanceInput {
    /// <p>The name of the notebook instance that you want information about.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration to describe.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNotebookInstanceOutput {
    /// <p>A list of the Elastic Inference (EI) instance types associated with this notebook instance. Currently only one EI instance type can be associated with a notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A timestamp. Use this parameter to return the time when the notebook instance was created</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProcessingJobRequest {
    /// <p>The name of the processing job. The name must be unique within an AWS Region in the AWS account.</p>
    #[serde(rename = "ProcessingJobName")]
    pub processing_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProcessingJobResponse {
    /// <p>Configures the processing job to run a specified container image.</p>
    #[serde(rename = "AppSpecification")]
    pub app_specification: AppSpecification,
    /// <p>The ARN of an AutoML job associated with this processing job.</p>
    #[serde(rename = "AutoMLJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_arn: Option<String>,
    /// <p>The time at which the processing job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The environment variables set in the Docker container.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>An optional string, up to one KB in size, that contains metadata from the processing container when the processing job exits.</p>
    #[serde(rename = "ExitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_message: Option<String>,
    /// <p>The configuration information used to create an experiment.</p>
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
    /// <p>A string, up to one KB in size, that contains the reason a processing job failed, if it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The time at which the processing job was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The ARN of a monitoring schedule for an endpoint associated with this processing job.</p>
    #[serde(rename = "MonitoringScheduleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_schedule_arn: Option<String>,
    /// <p>Networking options for a processing job.</p>
    #[serde(rename = "NetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,
    /// <p>The time at which the processing job completed.</p>
    #[serde(rename = "ProcessingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<f64>,
    /// <p>The inputs for a processing job.</p>
    #[serde(rename = "ProcessingInputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_inputs: Option<Vec<ProcessingInput>>,
    /// <p>The Amazon Resource Name (ARN) of the processing job.</p>
    #[serde(rename = "ProcessingJobArn")]
    pub processing_job_arn: String,
    /// <p>The name of the processing job. The name must be unique within an AWS Region in the AWS account.</p>
    #[serde(rename = "ProcessingJobName")]
    pub processing_job_name: String,
    /// <p>Provides the status of a processing job.</p>
    #[serde(rename = "ProcessingJobStatus")]
    pub processing_job_status: String,
    /// <p>Output configuration for the processing job.</p>
    #[serde(rename = "ProcessingOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_output_config: Option<ProcessingOutputConfig>,
    /// <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p>
    #[serde(rename = "ProcessingResources")]
    pub processing_resources: ProcessingResources,
    /// <p>The time at which the processing job started.</p>
    #[serde(rename = "ProcessingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_start_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The time limit for how long the processing job is allowed to run.</p>
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<ProcessingStoppingCondition>,
    /// <p>The ARN of a training job associated with this processing job.</p>
    #[serde(rename = "TrainingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSubscribedWorkteamRequest {
    /// <p>The Amazon Resource Name (ARN) of the subscribed work team to describe.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSubscribedWorkteamResponse {
    /// <p>A <code>Workteam</code> instance that contains information about the work team.</p>
    #[serde(rename = "SubscribedWorkteam")]
    pub subscribed_workteam: SubscribedWorkteam,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrainingJobRequest {
    /// <p>The name of the training job.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrainingJobResponse {
    /// <p>Information about the algorithm used for training, and algorithm metadata. </p>
    #[serde(rename = "AlgorithmSpecification")]
    pub algorithm_specification: AlgorithmSpecification,
    /// <p><p/></p>
    #[serde(rename = "AutoMLJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_arn: Option<String>,
    /// <p>The billable time in seconds.</p> <p>You can calculate the savings from using managed spot training using the formula <code>(1 - BillableTimeInSeconds / TrainingTimeInSeconds) * 100</code>. For example, if <code>BillableTimeInSeconds</code> is 100 and <code>TrainingTimeInSeconds</code> is 500, the savings is 80%.</p>
    #[serde(rename = "BillableTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_time_in_seconds: Option<i64>,
    #[serde(rename = "CheckpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<CheckpointConfig>,
    /// <p>A timestamp that indicates when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    #[serde(rename = "DebugHookConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_hook_config: Option<DebugHookConfig>,
    /// <p>Configuration information for debugging rules.</p>
    #[serde(rename = "DebugRuleConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_rule_configurations: Option<Vec<DebugRuleConfiguration>>,
    /// <p>Status about the debug rule evaluation.</p>
    #[serde(rename = "DebugRuleEvaluationStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_rule_evaluation_statuses: Option<Vec<DebugRuleEvaluationStatus>>,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithms in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>A Boolean indicating whether managed spot training is enabled (<code>True</code>) or not (<code>False</code>).</p>
    #[serde(rename = "EnableManagedSpotTraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_spot_training: Option<bool>,
    /// <p>If you want to allow inbound or outbound network calls, except for calls between peers within a training cluster for distributed training, choose <code>True</code>. If you enable network isolation for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
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
    /// <p><p> Provides detailed information about the state of the training job. For detailed information on the secondary status of the training job, see <code>StatusMessage</code> under <a>SecondaryStatusTransition</a>.</p> <p>Amazon SageMaker provides primary statuses and secondary statuses that apply to each of them:</p> <dl> <dt>InProgress</dt> <dd> <ul> <li> <p> <code>Starting</code> - Starting the training job.</p> </li> <li> <p> <code>Downloading</code> - An optional stage for algorithms that support <code>File</code> training input mode. It indicates that data is being downloaded to the ML storage volumes.</p> </li> <li> <p> <code>Training</code> - Training is in progress.</p> </li> <li> <p> <code>Interrupted</code> - The job stopped because the managed spot training instances were interrupted. </p> </li> <li> <p> <code>Uploading</code> - Training is complete and the model artifacts are being uploaded to the S3 location.</p> </li> </ul> </dd> <dt>Completed</dt> <dd> <ul> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> </ul> </dd> <dt>Failed</dt> <dd> <ul> <li> <p> <code>Failed</code> - The training job has failed. The reason for the failure is returned in the <code>FailureReason</code> field of <code>DescribeTrainingJobResponse</code>.</p> </li> </ul> </dd> <dt>Stopped</dt> <dd> <ul> <li> <p> <code>MaxRuntimeExceeded</code> - The job stopped because it exceeded the maximum allowed runtime.</p> </li> <li> <p> <code>MaxWaitTmeExceeded</code> - The job stopped because it exceeded the maximum allowed wait time.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> </dd> <dt>Stopping</dt> <dd> <ul> <li> <p> <code>Stopping</code> - Stopping the training job.</p> </li> </ul> </dd> </dl> <important> <p>Valid values for <code>SecondaryStatus</code> are subject to change. </p> </important> <p>We no longer support the following secondary statuses:</p> <ul> <li> <p> <code>LaunchingMLInstances</code> </p> </li> <li> <p> <code>PreparingTrainingStack</code> </p> </li> <li> <p> <code>DownloadingTrainingImage</code> </p> </li> </ul></p>
    #[serde(rename = "SecondaryStatus")]
    pub secondary_status: String,
    /// <p>A history of all of the secondary statuses that the training job has transitioned through.</p>
    #[serde(rename = "SecondaryStatusTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_status_transitions: Option<Vec<SecondaryStatusTransition>>,
    /// <p>Specifies a limit to how long a model training job can run. It also specifies the maximum time to wait for a spot instance. When the job reaches the time limit, Amazon SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    #[serde(rename = "TensorBoardOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tensor_board_output_config: Option<TensorBoardOutputConfig>,
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
    /// <p>The training time in seconds.</p>
    #[serde(rename = "TrainingTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_time_in_seconds: Option<i64>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTransformJobRequest {
    /// <p>The name of the transform job that you want to view details of.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTransformJobResponse {
    /// <p><p/></p>
    #[serde(rename = "AutoMLJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_arn: Option<String>,
    /// <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set <code>SplitType</code> to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p>
    #[serde(rename = "BatchStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
    /// <p>A timestamp that shows when the transform Job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    #[serde(rename = "DataProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_processing: Option<DataProcessing>,
    /// <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
    /// <p>If the transform job failed, <code>FailureReason</code> describes why it failed. A transform job creates a log file, which includes error messages, and stores it as an Amazon S3 object. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/logging-cloudwatch.html">Log Amazon SageMaker Events with Amazon CloudWatch</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrialComponentRequest {
    /// <p>The name of the trial component to describe.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrialComponentResponse {
    /// <p>Who created the component.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the component was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the component as displayed. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the component ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input artifacts of the component.</p>
    #[serde(rename = "InputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>Who last modified the component.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the component was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The metrics for the component.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TrialComponentMetricSummary>>,
    /// <p>The output artifacts of the component.</p>
    #[serde(rename = "OutputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The hyperparameters of the component.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, TrialComponentParameterValue>>,
    /// <p>The Amazon Resource Name (ARN) of the source and, optionally, the job type.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TrialComponentSource>,
    /// <p>When the component started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The status of the component. States include:</p> <ul> <li> <p>InProgress</p> </li> <li> <p>Completed</p> </li> <li> <p>Failed</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialComponentStatus>,
    /// <p>The Amazon Resource Name (ARN) of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
    /// <p>The name of the trial component.</p>
    #[serde(rename = "TrialComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrialRequest {
    /// <p>The name of the trial to describe.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrialResponse {
    /// <p>Who created the trial.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the trial was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the trial as displayed. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the experiment the trial is part of.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>Who last modified the trial.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the trial was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source and, optionally, the job type.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TrialSource>,
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
    /// <p>The name of the trial.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserProfileRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserProfileResponse {
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The failure reason.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The homa Amazon Elastic File System (EFS) Uid.</p>
    #[serde(rename = "HomeEfsFileSystemUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_efs_file_system_uid: Option<String>,
    /// <p>The last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The SSO user identifier.</p>
    #[serde(rename = "SingleSignOnUserIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on_user_identifier: Option<String>,
    /// <p>The SSO user value.</p>
    #[serde(rename = "SingleSignOnUserValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on_user_value: Option<String>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user profile Amazon Resource Name (ARN).</p>
    #[serde(rename = "UserProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_arn: Option<String>,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<String>,
    /// <p>A collection of settings.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkforceRequest {
    /// <p>The name of the private workforce whose access you want to restrict. <code>WorkforceName</code> is automatically set to <code>"default"</code> when a workforce is created and cannot be modified. </p>
    #[serde(rename = "WorkforceName")]
    pub workforce_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkforceResponse {
    /// <p>A single private workforce, which is automatically created when you create your first private work team. You can create one private work force in each AWS Region. By default, any workforce related API operation used in a specific region will apply to the workforce created in that region. To learn how to create a private workforce, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-create-private.html">Create a Private Workforce</a>.</p>
    #[serde(rename = "Workforce")]
    pub workforce: Workforce,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkteamRequest {
    /// <p>The name of the work team to return a description of.</p>
    #[serde(rename = "WorkteamName")]
    pub workteam_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkteamResponse {
    /// <p>A <code>Workteam</code> instance that contains information about the work team. </p>
    #[serde(rename = "Workteam")]
    pub workteam: Workteam,
}

/// <p>Specifies weight and capacity values for a production variant.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateTrialComponentRequest {
    /// <p>The name of the component to disassociate from the trial.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
    /// <p>The name of the trial to disassociate from.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateTrialComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
    /// <p>The ARN of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
}

/// <p>The domain's details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainDetails {
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The domain's Amazon Resource Name (ARN).</p>
    #[serde(rename = "DomainArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The domain name.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The domain's URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Provides summary information for an endpoint configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Input object for the endpoint</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointInput {
    /// <p>An endpoint in customer's account which has enabled <code>DataCaptureConfig</code> enabled.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>Path to the filesystem where the endpoint data is available to the container.</p>
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    /// <p>Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key. Defauts to <code>FullyReplicated</code> </p>
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<String>,
    /// <p>Whether the <code>Pipe</code> or <code>File</code> is used as the input mode for transfering data for the monitoring job. <code>Pipe</code> mode is recommended for large datasets. <code>File</code> mode is useful for small files that fit in memory. Defaults to <code>File</code>.</p>
    #[serde(rename = "S3InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_mode: Option<String>,
}

/// <p>Provides summary information for an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>A summary of the properties of an experiment as returned by the <a>Search</a> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Experiment {
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the experiment was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description of the experiment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the experiment as displayed. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the experiment.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
    /// <p>The name of the experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the experiment was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ExperimentSource>,
    /// <p>The list of tags that are associated with the experiment. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Configuration for the experiment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// <p>The name of the experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>Display name for the trial component.</p>
    #[serde(rename = "TrialComponentDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_display_name: Option<String>,
    /// <p>The name of the trial.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
}

/// <p>The source of the experiment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExperimentSource {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The source type.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>A summary of the properties of an experiment. To get the complete set of properties, call the <a>DescribeExperiment</a> API and provide the <code>ExperimentName</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExperimentSummary {
    /// <p>When the experiment was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the experiment as displayed. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the experiment.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
    /// <p>The name of the experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[serde(rename = "ExperimentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_source: Option<ExperimentSource>,
    /// <p>When the experiment was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

/// <p>Specifies a file system data source for a channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSystemDataSource {
    /// <p>The full path to the directory to associate with the channel.</p>
    #[serde(rename = "DirectoryPath")]
    pub directory_path: String,
    /// <p>The access mode of the mount of the directory associated with the channel. A directory can be mounted either in <code>ro</code> (read-only) or <code>rw</code> (read-write) mode.</p>
    #[serde(rename = "FileSystemAccessMode")]
    pub file_system_access_mode: String,
    /// <p>The file system id.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The file system type. </p>
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
}

/// <p><p>A conditional statement for a search expression that includes a resource property, a Boolean operator, and a value.</p> <p>If you don&#39;t specify an <code>Operator</code> and a <code>Value</code>, the filter searches for only the specified property. For example, defining a <code>Filter</code> for the <code>FailureReason</code> for the <code>TrainingJob</code> <code>Resource</code> searches for training job objects that have a value in the <code>FailureReason</code> field.</p> <p>If you specify a <code>Value</code>, but not an <code>Operator</code>, Amazon SageMaker uses the equals operator as the default.</p> <p>In search, there are several property types:</p> <dl> <dt>Metrics</dt> <dd> <p>To define a metric filter, enter a value using the form <code>&quot;Metrics.&lt;name&gt;&quot;</code>, where <code>&lt;name&gt;</code> is a metric name. For example, the following filter searches for training jobs with an <code>&quot;accuracy&quot;</code> metric greater than <code>&quot;0.9&quot;</code>:</p> <p> <code>{</code> </p> <p> <code>&quot;Name&quot;: &quot;Metrics.accuracy&quot;,</code> </p> <p> <code>&quot;Operator&quot;: &quot;GREATER<em>THAN&quot;,</code> </p> <p> <code>&quot;Value&quot;: &quot;0.9&quot;</code> </p> <p> <code>}</code> </p> </dd> <dt>HyperParameters</dt> <dd> <p>To define a hyperparameter filter, enter a value with the form <code>&quot;HyperParameters.&lt;name&gt;&quot;</code>. Decimal hyperparameter values are treated as a decimal in a comparison if the specified <code>Value</code> is also a decimal value. If the specified <code>Value</code> is an integer, the decimal hyperparameter values are treated as integers. For example, the following filter is satisfied by training jobs with a <code>&quot;learning</em>rate&quot;</code> hyperparameter that is less than <code>&quot;0.5&quot;</code>:</p> <p> <code> {</code> </p> <p> <code> &quot;Name&quot;: &quot;HyperParameters.learning<em>rate&quot;,</code> </p> <p> <code> &quot;Operator&quot;: &quot;LESS</em>THAN&quot;,</code> </p> <p> <code> &quot;Value&quot;: &quot;0.5&quot;</code> </p> <p> <code> }</code> </p> </dd> <dt>Tags</dt> <dd> <p>To define a tag filter, enter a value with the form <code>&quot;Tags.&lt;key&gt;&quot;</code>.</p> </dd> </dl></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>A property name. For example, <code>TrainingJobName</code>. For the list of valid property names returned in a search result for each supported resource, see <a>TrainingJob</a> properties. You must specify a valid property name for the resource.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A Boolean binary operator that is used to evaluate the filter. The operator field contains one of the following values:</p> <dl> <dt>Equals</dt> <dd> <p>The specified resource in <code>Name</code> equals the specified <code>Value</code>.</p> </dd> <dt>NotEquals</dt> <dd> <p>The specified resource in <code>Name</code> does not equal the specified <code>Value</code>.</p> </dd> <dt>GreaterThan</dt> <dd> <p>The specified resource in <code>Name</code> is greater than the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>GreaterThanOrEqualTo</dt> <dd> <p>The specified resource in <code>Name</code> is greater than or equal to the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>LessThan</dt> <dd> <p>The specified resource in <code>Name</code> is less than the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>LessThanOrEqualTo</dt> <dd> <p>The specified resource in <code>Name</code> is less than or equal to the specified <code>Value</code>. Not supported for text-based properties.</p> </dd> <dt>Contains</dt> <dd> <p>Only supported for text-based properties. The word-list of the property contains the specified <code>Value</code>. A <code>SearchExpression</code> can include only one <code>Contains</code> operator.</p> </dd> </dl> <p>If you have specified a filter <code>Value</code>, the default is <code>Equals</code>.</p>
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>A value used with <code>Resource</code> and <code>Operator</code> to determine if objects satisfy the filter's condition. For numerical properties, <code>Value</code> must be an integer or floating-point decimal. For timestamp properties, <code>Value</code> must be an ISO 8601 date-time string of the following format: <code>YYYY-mm-dd'T'HH:MM:SS</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The candidate result from a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FinalAutoMLJobObjectiveMetric {
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>The metric type used.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    pub value: f32,
}

/// <p>Shows the final value for the objective metric for a training job that was launched by a hyperparameter tuning job. You define the objective metric in the <code>HyperParameterTuningJobObjective</code> parameter of <a>HyperParameterTuningJobConfig</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Contains information about where human output will be stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowDefinitionOutputConfig {
    /// <p>The Amazon Key Management Service (KMS) key ID for server-side encryption.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The Amazon S3 path where the object containing human output will be made available.</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Contains summary information about the flow definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlowDefinitionSummary {
    /// <p>The timestamp when SageMaker created the flow definition.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The reason why the flow definition creation failed. A failure reason is returned only when the flow definition status is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The name of the flow definition.</p>
    #[serde(rename = "FlowDefinitionName")]
    pub flow_definition_name: String,
    /// <p>The status of the flow definition. Valid values:</p>
    #[serde(rename = "FlowDefinitionStatus")]
    pub flow_definition_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSearchSuggestionsRequest {
    /// <p>The name of the Amazon SageMaker resource to Search for.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>Limits the property names that are included in the response.</p>
    #[serde(rename = "SuggestionQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_query: Option<SuggestionQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GitConfigForUpdate {
    /// <p>The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the credentials used to access the git repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p> <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

/// <p>Defines under what conditions SageMaker creates a human loop. Used within .</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanLoopActivationConditionsConfig {
    /// <p>JSON expressing use-case specific conditions declaratively. If any condition is matched, atomic tasks are created against the configured work team. The set of conditions is different for Rekognition and Textract.</p>
    #[serde(rename = "HumanLoopActivationConditions")]
    pub human_loop_activation_conditions: String,
}

/// <p>Provides information about how and under what conditions SageMaker creates a human loop. If <code>HumanLoopActivationConfig</code> is not given, then all requests go to humans.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanLoopActivationConfig {
    /// <p>Container structure for defining under what conditions SageMaker creates a human loop.</p>
    #[serde(rename = "HumanLoopActivationConditionsConfig")]
    pub human_loop_activation_conditions_config: HumanLoopActivationConditionsConfig,
    /// <p>Container for configuring the source of human task requests.</p>
    #[serde(rename = "HumanLoopRequestSource")]
    pub human_loop_request_source: HumanLoopRequestSource,
}

/// <p>Describes the work to be performed by human workers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanLoopConfig {
    /// <p>The Amazon Resource Name (ARN) of the human task user interface.</p>
    #[serde(rename = "HumanTaskUiArn")]
    pub human_task_ui_arn: String,
    #[serde(rename = "PublicWorkforceTaskPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_workforce_task_price: Option<PublicWorkforceTaskPrice>,
    /// <p>The length of time that a task remains available for labeling by human workers.</p>
    #[serde(rename = "TaskAvailabilityLifetimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_availability_lifetime_in_seconds: Option<i64>,
    /// <p>The number of human tasks.</p>
    #[serde(rename = "TaskCount")]
    pub task_count: i64,
    /// <p>A description for the human worker task.</p>
    #[serde(rename = "TaskDescription")]
    pub task_description: String,
    /// <p>Keywords used to describe the task so that workers can discover the task.</p>
    #[serde(rename = "TaskKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_keywords: Option<Vec<String>>,
    /// <p>The amount of time that a worker has to complete a task.</p>
    #[serde(rename = "TaskTimeLimitInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_time_limit_in_seconds: Option<i64>,
    /// <p>A title for the human worker task.</p>
    #[serde(rename = "TaskTitle")]
    pub task_title: String,
    /// <p>Amazon Resource Name (ARN) of a team of workers.</p>
    #[serde(rename = "WorkteamArn")]
    pub workteam_arn: String,
}

/// <p>Container for configuring the source of human task requests.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanLoopRequestSource {
    /// <p>Specifies whether Amazon Rekognition or Amazon Textract are used as the integration source. The default field settings and JSON parsing rules are different based on the integration source. Valid values:</p>
    #[serde(rename = "AwsManagedHumanLoopRequestSource")]
    pub aws_managed_human_loop_request_source: String,
}

/// <p>Information required for human workers to complete a labeling task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanTaskConfig {
    /// <p>Configures how labels are consolidated across human workers.</p>
    #[serde(rename = "AnnotationConsolidationConfig")]
    pub annotation_consolidation_config: AnnotationConsolidationConfig,
    /// <p>Defines the maximum number of data objects that can be labeled by human workers at the same time. Also referred to as batch size. Each object may have more than one worker at one time. The default value is 1000 objects.</p>
    #[serde(rename = "MaxConcurrentTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_task_count: Option<i64>,
    /// <p>The number of human workers that will label an object. </p>
    #[serde(rename = "NumberOfHumanWorkersPerDataObject")]
    pub number_of_human_workers_per_data_object: i64,
    /// <p><p>The Amazon Resource Name (ARN) of a Lambda function that is run before a data object is sent to a human worker. Use this function to provide input to a custom labeling job.</p> <p>For the built-in bounding box, image classification, semantic segmentation, and text classification task types, Amazon SageMaker Ground Truth provides the following Lambda functions:</p> <p> <b>US East (Northern Virginia) (us-east-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-1:432418664414:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>US East (Ohio) (us-east-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-east-2:266458841044:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>US West (Oregon) (us-west-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:us-west-2:081040173940:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Canada (Central) (ca-central-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ca-central-1:918755190332:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>EU (Ireland) (eu-west-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-1:568282634449:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>EU (London) (eu-west-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-west-2:487402164563:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>EU Frankfurt (eu-central-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:eu-central-1:203001061592:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Asia Pacific (Tokyo) (ap-northeast-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-1:477331159723:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Asia Pacific (Seoul) (ap-northeast-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-northeast-2:845288260483:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Asia Pacific (Mumbai) (ap-south-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-south-1:565803892007:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Asia Pacific (Singapore) (ap-southeast-1):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-1:377565633583:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul> <p> <b>Asia Pacific (Sydney) (ap-southeast-2):</b> </p> <ul> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-BoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-ImageMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-SemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-TextMultiClass</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-NamedEntityRecognition</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-VerificationBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-VerificationSemanticSegmentation</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-AdjustmentBoundingBox</code> </p> </li> <li> <p> <code>arn:aws:lambda:ap-southeast-2:454466003867:function:PRE-AdjustmentSemanticSegmentation</code> </p> </li> </ul></p>
    #[serde(rename = "PreHumanTaskLambdaArn")]
    pub pre_human_task_lambda_arn: String,
    /// <p>The price that you pay for each task performed by an Amazon Mechanical Turk worker.</p>
    #[serde(rename = "PublicWorkforceTaskPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_workforce_task_price: Option<PublicWorkforceTaskPrice>,
    /// <p>The length of time that a task remains available for labeling by human workers. <b>If you choose the Amazon Mechanical Turk workforce, the maximum is 12 hours (43200)</b>. The default value is 864000 seconds (1 day). For private and vendor workforces, the maximum is as listed.</p>
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

/// <p>Container for human task user interface information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanTaskUiSummary {
    /// <p>A timestamp when SageMaker created the human task user interface.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the human task user interface.</p>
    #[serde(rename = "HumanTaskUiArn")]
    pub human_task_ui_arn: String,
    /// <p>The name of the human task user interface.</p>
    #[serde(rename = "HumanTaskUiName")]
    pub human_task_ui_name: String,
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
    #[serde(rename = "CheckpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<CheckpointConfig>,
    /// <p>The job definition name.</p>
    #[serde(rename = "DefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_name: Option<String>,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>A Boolean indicating whether managed spot training is enabled (<code>True</code>) or not (<code>False</code>).</p>
    #[serde(rename = "EnableManagedSpotTraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_spot_training: Option<bool>,
    /// <p>Isolates the training container. No inbound or outbound network calls can be made, except for calls between peers within a training cluster for distributed training. If network isolation is used for training jobs that are configured to use a VPC, Amazon SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "HyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_ranges: Option<ParameterRanges>,
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
    /// <p>Specifies a limit to how long a model hyperparameter training job can run. It also specifies how long you are willing to wait for a managed spot training job to complete. When the job reaches the a limit, Amazon SageMaker ends the training job. Use this API to cap model training costs.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    #[serde(rename = "TuningObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_objective: Option<HyperParameterTuningJobObjective>,
    /// <p>The <a>VpcConfig</a> object that specifies the VPC that you want the training jobs that this hyperparameter tuning job launches to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Specifies summary information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The training job definition name.</p>
    #[serde(rename = "TrainingJobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_definition_name: Option<String>,
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
    /// <p>Specifies how hyperparameter tuning chooses the combinations of hyperparameter values to use for the training job it launches. To use the Bayesian search strategy, set this to <code>Bayesian</code>. To randomly search, set it to <code>Random</code>. For information about search strategies, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-how-it-works.html">How Hyperparameter Tuning Works</a>.</p>
    #[serde(rename = "Strategy")]
    pub strategy: String,
    /// <p><p>Specifies whether to use early stopping for training jobs launched by the hyperparameter tuning job. This can be one of the following values (the default value is <code>OFF</code>):</p> <dl> <dt>OFF</dt> <dd> <p>Training jobs launched by the hyperparameter tuning job do not use early stopping.</p> </dd> <dt>AUTO</dt> <dd> <p>Amazon SageMaker stops training jobs launched by the hyperparameter tuning job when they are unlikely to perform better than previously completed training jobs. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-early-stopping.html">Stop Training Jobs Early</a>.</p> </dd> </dl></p>
    #[serde(rename = "TrainingJobEarlyStoppingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_early_stopping_type: Option<String>,
    /// <p>The tuning job's completion criteria.</p>
    #[serde(rename = "TuningJobCompletionCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_job_completion_criteria: Option<TuningJobCompletionCriteria>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>An array of hyperparameter tuning jobs that are used as the starting point for the new hyperparameter tuning job. For more information about warm starting a hyperparameter tuning job, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-warm-start.html">Using a Previous Hyperparameter Tuning Job as a Starting Point</a>.</p> <p>Hyperparameter tuning jobs created before October 1, 2018 cannot be used as parent jobs for warm start tuning jobs.</p>
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
    /// <p><p>Specifies the name and shape of the expected data inputs for your trained model with a JSON dictionary form. The data inputs are <a>InputConfig$Framework</a> specific. </p> <ul> <li> <p> <code>TensorFlow</code>: You must specify the name and shape (NHWC format) of the expected data inputs using a dictionary format for your trained model. The dictionary formats required for the console and CLI are different.</p> <ul> <li> <p>Examples for one input:</p> <ul> <li> <p>If using the console, <code>{&quot;input&quot;:[1,1024,1024,3]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input&quot;:[1,1024,1024,3]}</code> </p> </li> </ul> </li> <li> <p>Examples for two inputs:</p> <ul> <li> <p>If using the console, <code>{&quot;data1&quot;: [1,28,28,1], &quot;data2&quot;:[1,28,28,1]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;data1&quot;: [1,28,28,1], &quot;data2&quot;:[1,28,28,1]}</code> </p> </li> </ul> </li> </ul> </li> <li> <p> <code>KERAS</code>: You must specify the name and shape (NCHW format) of expected data inputs using a dictionary format for your trained model. Note that while Keras model artifacts should be uploaded in NHWC (channel-last) format, <code>DataInputConfig</code> should be specified in NCHW (channel-first) format. The dictionary formats required for the console and CLI are different.</p> <ul> <li> <p>Examples for one input:</p> <ul> <li> <p>If using the console, <code>{&quot;input<em>1&quot;:[1,3,224,224]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input</em>1&quot;:[1,3,224,224]}</code> </p> </li> </ul> </li> <li> <p>Examples for two inputs:</p> <ul> <li> <p>If using the console, <code>{&quot;input<em>1&quot;: [1,3,224,224], &quot;input</em>2&quot;:[1,3,224,224]} </code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input<em>1&quot;: [1,3,224,224], &quot;input</em>2&quot;:[1,3,224,224]}</code> </p> </li> </ul> </li> </ul> </li> <li> <p> <code>MXNET/ONNX</code>: You must specify the name and shape (NCHW format) of the expected data inputs in order using a dictionary format for your trained model. The dictionary formats required for the console and CLI are different.</p> <ul> <li> <p>Examples for one input:</p> <ul> <li> <p>If using the console, <code>{&quot;data&quot;:[1,3,1024,1024]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;data&quot;:[1,3,1024,1024]}</code> </p> </li> </ul> </li> <li> <p>Examples for two inputs:</p> <ul> <li> <p>If using the console, <code>{&quot;var1&quot;: [1,1,28,28], &quot;var2&quot;:[1,1,28,28]} </code> </p> </li> <li> <p>If using the CLI, <code>{&quot;var1&quot;: [1,1,28,28], &quot;var2&quot;:[1,1,28,28]}</code> </p> </li> </ul> </li> </ul> </li> <li> <p> <code>PyTorch</code>: You can either specify the name and shape (NCHW format) of expected data inputs in order using a dictionary format for your trained model or you can specify the shape only using a list format. The dictionary formats required for the console and CLI are different. The list formats for the console and CLI are the same.</p> <ul> <li> <p>Examples for one input in dictionary format:</p> <ul> <li> <p>If using the console, <code>{&quot;input0&quot;:[1,3,224,224]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input0&quot;:[1,3,224,224]}</code> </p> </li> </ul> </li> <li> <p>Example for one input in list format: <code>[[1,3,224,224]]</code> </p> </li> <li> <p>Examples for two inputs in dictionary format:</p> <ul> <li> <p>If using the console, <code>{&quot;input0&quot;:[1,3,224,224], &quot;input1&quot;:[1,3,224,224]}</code> </p> </li> <li> <p>If using the CLI, <code>{&quot;input0&quot;:[1,3,224,224], &quot;input1&quot;:[1,3,224,224]} </code> </p> </li> </ul> </li> <li> <p>Example for two inputs in list format: <code>[[1,3,224,224], [1,3,224,224]]</code> </p> </li> </ul> </li> <li> <p> <code>XGBOOST</code>: input data name and shape are not needed.</p> </li> </ul></p>
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
    /// <p><p>The scale that hyperparameter tuning uses to search the hyperparameter range. For information about choosing a hyperparameter scale, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon SageMaker hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have only values greater than 0.</p> </dd> </dl></p>
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

/// <p>Jupyter server's app settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JupyterServerAppSettings {
    /// <p>The instance type and quantity.</p>
    #[serde(rename = "DefaultResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

/// <p>The kernel gateway app settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KernelGatewayAppSettings {
    /// <p>The instance type and quantity.</p>
    #[serde(rename = "DefaultResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

/// <p>Provides a breakdown of the number of objects labeled.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>Specifies the Amazon Resource Name (ARN) of the algorithm used for auto-labeling. You must select one of the following ARNs:</p> <ul> <li> <p> <i>Image classification</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/image-classification</code> </p> </li> <li> <p> <i>Text classification</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/text-classification</code> </p> </li> <li> <p> <i>Object detection</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/object-detection</code> </p> </li> <li> <p> <i>Semantic Segmentation</i> </p> <p> <code>arn:aws:sagemaker:<i>region</i>:027400017018:labeling-job-algorithm-specification/semantic-segmentation</code> </p> </li> </ul></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The AWS Key Management Service ID of the key used to encrypt the output data, if any.</p> <p>If you use a KMS key ID or an alias of your master key, the Amazon SageMaker execution role must include permissions to call <code>kms:Encrypt</code>. If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. Amazon SageMaker uses server-side encryption with KMS-managed keys for <code>LabelingJobOutputConfig</code>. If you use a bucket policy with an <code>s3:PutObject</code> permission that only allows objects with server-side encryption, set the condition key of <code>s3:x-amz-server-side-encryption</code> to <code>"aws:kms"</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateLabelingJob</code> request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
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
    /// <p><p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the training job. The <code>VolumeKmsKeyId</code> can be any of the following formats:</p> <ul> <li> <p>// KMS Key ID</p> <p> <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
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

/// <p><p>A set of conditions for stopping a labeling job. If any of the conditions are met, the job is automatically stopped. You can use these conditions to control the cost of data labeling.</p> <note> <p>Labeling jobs fail after 30 days with an appropriate client error message.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LabelingJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function used to consolidate the annotations from individual workers into a label for a data object. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-annotation-consolidation.html">Annotation Consolidation</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAppsRequest {
    /// <p>A parameter to search for the domain ID.</p>
    #[serde(rename = "DomainIdEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id_equals: Option<String>,
    /// <p>Returns a list up to a specified limit.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is CreationTime.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is Ascending.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A parameter to search by user profile name.</p>
    #[serde(rename = "UserProfileNameEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAppsResponse {
    /// <p>The list of apps.</p>
    #[serde(rename = "Apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<AppDetails>>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAutoMLJobsRequest {
    /// <p>Request a list of jobs, using a filter for time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>Request a list of jobs, using a filter for time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>Request a list of jobs, using a filter for time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>Request a list of jobs, using a filter for time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>Request a list of jobs up to a specified limit.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Request a list of jobs, using a search filter for name.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is AutoMLJobName.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is Descending.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>Request a list of jobs, using a filter for status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAutoMLJobsResponse {
    /// <p>Returns a summary list of jobs.</p>
    #[serde(rename = "AutoMLJobSummaries")]
    pub auto_ml_job_summaries: Vec<AutoMLJobSummary>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCandidatesForAutoMLJobRequest {
    /// <p>List the Candidates created for the job by providing the job's name.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
    /// <p>List the Candidates for the job and filter by candidate name.</p>
    #[serde(rename = "CandidateNameEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_name_equals: Option<String>,
    /// <p>List the job's Candidates up to a specified limit.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is Descending.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is Ascending.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>List the Candidates for the job and filter by status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCandidatesForAutoMLJobResponse {
    /// <p>Summaries about the Candidates.</p>
    #[serde(rename = "Candidates")]
    pub candidates: Vec<AutoMLCandidate>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainsRequest {
    /// <p>Returns a list up to a specified limit.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainsResponse {
    /// <p>The list of domains.</p>
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainDetails>>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListExperimentsRequest {
    /// <p>A filter that returns only experiments created after the specified time.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>A filter that returns only experiments created before the specified time.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>The maximum number of experiments to return in the response. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous call to <code>ListExperiments</code> didn't return the full set of experiments, the call returns a token for getting the next set of experiments.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. The default value is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListExperimentsResponse {
    /// <p>A list of the summaries of your experiments.</p>
    #[serde(rename = "ExperimentSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_summaries: Option<Vec<ExperimentSummary>>,
    /// <p>A token for getting the next set of experiments, if there are any.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFlowDefinitionsRequest {
    /// <p>A filter that returns only flow definitions with a creation time greater than or equal to the specified timestamp.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only flow definitions that were created before the specified timestamp.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> will be provided in the output that you can use to resume pagination.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional value that specifies whether you want the results sorted in <code>Ascending</code> or <code>Descending</code> order.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFlowDefinitionsResponse {
    /// <p>An array of objects describing the flow definitions.</p>
    #[serde(rename = "FlowDefinitionSummaries")]
    pub flow_definition_summaries: Vec<FlowDefinitionSummary>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHumanTaskUisRequest {
    /// <p>A filter that returns only human task user interfaces with a creation time greater than or equal to the specified timestamp.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only human task user interfaces that were created before the specified timestamp.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> will be provided in the output that you can use to resume pagination.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional value that specifies whether you want the results sorted in <code>Ascending</code> or <code>Descending</code> order.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHumanTaskUisResponse {
    /// <p>An array of objects describing the human task user interfaces.</p>
    #[serde(rename = "HumanTaskUiSummaries")]
    pub human_task_ui_summaries: Vec<HumanTaskUiSummary>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMonitoringExecutionsRequest {
    /// <p>A filter that returns only jobs created after a specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only jobs created before a specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>Name of a specific endpoint to fetch jobs for.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>A filter that returns only jobs modified before a specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only jobs modified after a specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of jobs to return in the response. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Name of a specific schedule to fetch jobs for.</p>
    #[serde(rename = "MonitoringScheduleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_schedule_name: Option<String>,
    /// <p>The token returned if the response is truncated. To retrieve the next set of job executions, use it in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filter for jobs scheduled after a specified time.</p>
    #[serde(rename = "ScheduledTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_after: Option<f64>,
    /// <p>Filter for jobs scheduled before a specified time.</p>
    #[serde(rename = "ScheduledTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_before: Option<f64>,
    /// <p>Whether to sort results by <code>Status</code>, <code>CreationTime</code>, <code>ScheduledTime</code> field. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>Whether to sort the results in <code>Ascending</code> or <code>Descending</code> order. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that retrieves only jobs with a specific status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMonitoringExecutionsResponse {
    /// <p>A JSON array in which each element is a summary for a monitoring execution.</p>
    #[serde(rename = "MonitoringExecutionSummaries")]
    pub monitoring_execution_summaries: Vec<MonitoringExecutionSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of jobs, use it in the subsequent reques</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMonitoringSchedulesRequest {
    /// <p>A filter that returns only monitoring schedules created after a specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only monitoring schedules created before a specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>Name of a specific endpoint to fetch schedules for.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>A filter that returns only monitoring schedules modified after a specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only monitoring schedules modified before a specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of jobs to return in the response. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Filter for monitoring schedules whose name contains a specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>The token returned if the response is truncated. To retrieve the next set of job executions, use it in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Whether to sort results by <code>Status</code>, <code>CreationTime</code>, <code>ScheduledTime</code> field. The default is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>Whether to sort the results in <code>Ascending</code> or <code>Descending</code> order. The default is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that returns only monitoring schedules modified before a specified time.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMonitoringSchedulesResponse {
    /// <p>A JSON array in which each element is a summary for a monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleSummaries")]
    pub monitoring_schedule_summaries: Vec<MonitoringScheduleSummary>,
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of jobs, use it in the subsequent reques</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProcessingJobsRequest {
    /// <p>A filter that returns only processing jobs created after the specified time.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>A filter that returns only processing jobs created after the specified time.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>A filter that returns only processing jobs modified after the specified time.</p>
    #[serde(rename = "LastModifiedTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_after: Option<f64>,
    /// <p>A filter that returns only processing jobs modified before the specified time.</p>
    #[serde(rename = "LastModifiedTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_before: Option<f64>,
    /// <p>The maximum number of processing jobs to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string in the processing job name. This filter returns only processing jobs whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous <code>ListProcessingJobs</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of processing jobs, use the token in the next request.</p>
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
    /// <p>A filter that retrieves only processing jobs with a specific status.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProcessingJobsResponse {
    /// <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of processing jobs, use it in the subsequent request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>ProcessingJobSummary</code> objects, each listing a processing job.</p>
    #[serde(rename = "ProcessingJobSummaries")]
    pub processing_job_summaries: Vec<ProcessingJobSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTrialComponentsRequest {
    /// <p>A filter that returns only components created after the specified time.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>A filter that returns only components created before the specified time.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>A filter that returns only components that are part of the specified experiment. If you specify <code>ExperimentName</code>, you can't filter by <code>SourceArn</code> or <code>TrialName</code>.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>The maximum number of components to return in the response. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous call to <code>ListTrialComponents</code> didn't return the full set of components, the call returns a token for getting the next set of components.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. The default value is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A filter that returns only components that have the specified source Amazon Resource Name (ARN). If you specify <code>SourceArn</code>, you can't filter by <code>ExperimentName</code> or <code>TrialName</code>.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>A filter that returns only components that are part of the specified trial. If you specify <code>TrialName</code>, you can't filter by <code>ExperimentName</code> or <code>SourceArn</code>.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTrialComponentsResponse {
    /// <p>A token for getting the next set of components, if there are any.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the summaries of your trial components.</p>
    #[serde(rename = "TrialComponentSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_summaries: Option<Vec<TrialComponentSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTrialsRequest {
    /// <p>A filter that returns only trials created after the specified time.</p>
    #[serde(rename = "CreatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>A filter that returns only trials created before the specified time.</p>
    #[serde(rename = "CreatedBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>A filter that returns only trials that are part of the specified experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>The maximum number of trials to return in the response. The default value is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous call to <code>ListTrials</code> didn't return the full set of trials, the call returns a token for getting the next set of trials.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order. The default value is <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTrialsResponse {
    /// <p>A token for getting the next set of trials, if there are any.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the summaries of your trials.</p>
    #[serde(rename = "TrialSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_summaries: Option<Vec<TrialSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUserProfilesRequest {
    /// <p>A parameter by which to filter the results.</p>
    #[serde(rename = "DomainIdEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id_equals: Option<String>,
    /// <p>Returns a list up to a specified limit.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The parameter by which to sort the results. The default is CreationTime.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sort order for the results. The default is Ascending.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// <p>A parameter by which to filter the results.</p>
    #[serde(rename = "UserProfileNameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name_contains: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUserProfilesResponse {
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of user profiles.</p>
    #[serde(rename = "UserProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profiles: Option<Vec<UserProfileDetails>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Specifies a metric that the training algorithm writes to <code>stderr</code> or <code>stdout</code> . Amazon SageMakerhyperparameter tuning captures all defined metrics. You specify one metric that a hyperparameter tuning job uses as its objective metric to choose the best training job.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Container image configuration object for the monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringAppSpecification {
    /// <p>An array of arguments for the container used to run the monitoring job.</p>
    #[serde(rename = "ContainerArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arguments: Option<Vec<String>>,
    /// <p>Specifies the entrypoint for a container used to run the monitoring job.</p>
    #[serde(rename = "ContainerEntrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_entrypoint: Option<Vec<String>>,
    /// <p>The container image to be run by the monitoring job.</p>
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
    /// <p>An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.</p>
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_analytics_processor_source_uri: Option<String>,
    /// <p>An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.</p>
    #[serde(rename = "RecordPreprocessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_preprocessor_source_uri: Option<String>,
}

/// <p>Configuration for monitoring constraints and monitoring statistics. These baseline resources are compared against the results of the current job from the series of jobs scheduled to collect data periodically.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringBaselineConfig {
    /// <p>The baseline constraint file in Amazon S3 that the current monitoring job should validated against.</p>
    #[serde(rename = "ConstraintsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints_resource: Option<MonitoringConstraintsResource>,
    /// <p>The baseline statistics file in Amazon S3 that the current monitoring job should be validated against.</p>
    #[serde(rename = "StatisticsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_resource: Option<MonitoringStatisticsResource>,
}

/// <p>Configuration for the cluster used to run model monitoring jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringClusterConfig {
    /// <p>The number of ML compute instances to use in the model monitoring job. For distributed processing jobs, specify a value greater than 1. The default value is 1.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The ML compute instance type for the processing job.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the model monitoring job.</p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>The size of the ML storage volume, in gigabytes, that you want to provision. You must specify sufficient ML storage for your scenario.</p>
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,
}

/// <p>The constraints resource for a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringConstraintsResource {
    /// <p>The Amazon S3 URI for the constraints resource.</p>
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

/// <p>Summary of information about the last monitoring job to run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MonitoringExecutionSummary {
    /// <p>The time at which the monitoring job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The name of teh endpoint used to run the monitoring job.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>Contains the reason a monitoring job failed, if it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A timestamp that indicates the last time the monitoring job was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>The status of the monitoring job.</p>
    #[serde(rename = "MonitoringExecutionStatus")]
    pub monitoring_execution_status: String,
    /// <p>The name of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
    /// <p>The Amazon Resource Name (ARN) of the monitoring job.</p>
    #[serde(rename = "ProcessingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_job_arn: Option<String>,
    /// <p>The time the monitoring job was scheduled.</p>
    #[serde(rename = "ScheduledTime")]
    pub scheduled_time: f64,
}

/// <p>The inputs for a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringInput {
    /// <p>The endpoint for a monitoring job.</p>
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: EndpointInput,
}

/// <p>Defines the monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringJobDefinition {
    /// <p>Baseline configuration used to validate that the data conforms to the specified constraints and statistics</p>
    #[serde(rename = "BaselineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_config: Option<MonitoringBaselineConfig>,
    /// <p>Sets the environment variables in the Docker container.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    /// <p>Configures the monitoring job to run a specified Docker container image.</p>
    #[serde(rename = "MonitoringAppSpecification")]
    pub monitoring_app_specification: MonitoringAppSpecification,
    /// <p>The array of inputs for the monitoring job. Currently we support monitoring an Amazon SageMaker Endpoint.</p>
    #[serde(rename = "MonitoringInputs")]
    pub monitoring_inputs: Vec<MonitoringInput>,
    /// <p>The array of outputs from the monitoring job to be uploaded to Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "MonitoringOutputConfig")]
    pub monitoring_output_config: MonitoringOutputConfig,
    /// <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a monitoring job. In distributed processing, you specify more than one instance.</p>
    #[serde(rename = "MonitoringResources")]
    pub monitoring_resources: MonitoringResources,
    /// <p>Specifies networking options for an monitoring job.</p>
    #[serde(rename = "NetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Specifies a time limit for how long the monitoring job is allowed to run.</p>
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<MonitoringStoppingCondition>,
}

/// <p>The output object for a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringOutput {
    /// <p>The Amazon S3 storage location where the results of a monitoring job are saved.</p>
    #[serde(rename = "S3Output")]
    pub s3_output: MonitoringS3Output,
}

/// <p>The output configuration for monitoring jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringOutputConfig {
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Monitoring outputs for monitoring jobs. This is where the output of the periodic monitoring jobs is uploaded.</p>
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,
}

/// <p>Identifies the resources to deploy for a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringResources {
    /// <p>The configuration for the cluster resources used to run the processing job.</p>
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: MonitoringClusterConfig,
}

/// <p>Information about where and how you want to store the results of a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringS3Output {
    /// <p>The local path to the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job. LocalPath is an absolute path for the output data.</p>
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    /// <p>Whether to upload the results of the monitoring job continuously or after the job completes.</p>
    #[serde(rename = "S3UploadMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_upload_mode: Option<String>,
    /// <p>A URI that identifies the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Configures the monitoring schedule and defines the monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringScheduleConfig {
    /// <p>Defines the monitoring job.</p>
    #[serde(rename = "MonitoringJobDefinition")]
    pub monitoring_job_definition: MonitoringJobDefinition,
    /// <p>Configures the monitoring schedule.</p>
    #[serde(rename = "ScheduleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfig>,
}

/// <p>Summarizes the monitoring schedule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MonitoringScheduleSummary {
    /// <p>The creation time of the monitoring schedule.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The name of the endpoint using the monitoring schedule.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>The last time the monitoring schedule was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleArn")]
    pub monitoring_schedule_arn: String,
    /// <p>The name of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
    /// <p>The status of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleStatus")]
    pub monitoring_schedule_status: String,
}

/// <p>The statistics resource for a monitoring job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringStatisticsResource {
    /// <p>The Amazon S3 URI for the statistics resource.</p>
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

/// <p>A time limit for how long the monitoring job is allowed to run before stopping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringStoppingCondition {
    /// <p>The maximum runtime allowed in seconds.</p>
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: i64,
}

/// <p><p>Defines a list of <code>NestedFilters</code> objects. To satisfy the conditions specified in the <code>NestedFilters</code> call, a resource must satisfy the conditions of all of the filters.</p> <p>For example, you could define a <code>NestedFilters</code> using the training job&#39;s <code>InputDataConfig</code> property to filter on <code>Channel</code> objects. </p> <p>A <code>NestedFilters</code> object contains multiple filters. For example, to find all training jobs whose name contains <code>train</code> and that have <code>cat/data</code> in their <code>S3Uri</code> (specified in <code>InputDataConfig</code>), you need to create a <code>NestedFilters</code> object that specifies the <code>InputDataConfig</code> property with the following <code>Filter</code> objects:</p> <ul> <li> <p> <code>&#39;{Name:&quot;InputDataConfig.ChannelName&quot;, &quot;Operator&quot;:&quot;EQUALS&quot;, &quot;Value&quot;:&quot;train&quot;}&#39;,</code> </p> </li> <li> <p> <code>&#39;{Name:&quot;InputDataConfig.DataSource.S3DataSource.S3Uri&quot;, &quot;Operator&quot;:&quot;CONTAINS&quot;, &quot;Value&quot;:&quot;cat/data&quot;}&#39;</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NestedFilters {
    /// <p>A list of filters. Each filter acts on a property. Filters must contain at least one <code>Filters</code> value. For example, a <code>NestedFilters</code> call might include a filter on the <code>PropertyName</code> parameter of the <code>InputDataConfig</code> property: <code>InputDataConfig.DataSource.S3DataSource.S3Uri</code>.</p>
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
    /// <p>The name of the property to use in the nested filters. The value must match a listed property name, such as <code>InputDataConfig</code>.</p>
    #[serde(rename = "NestedPropertyName")]
    pub nested_property_name: String,
}

/// <p>Networking options for a job, such as network traffic encryption between containers, whether to allow inbound and outbound network calls to and from containers, and the VPC subnets and security groups to use for VPC-enabled jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// <p>Whether to allow inbound and outbound network calls to and from the containers used for the processing job.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Provides a summary of a notebook instance lifecycle configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotebookInstanceSummary {
    /// <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>A timestamp that shows when the notebook instance was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption. The <code>KmsKeyId</code> can be any of the following formats: </p> <ul> <li> <p>// KMS Key ID</p> <p> <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>// KMS Key Alias</p> <p> <code>"alias/ExampleAlias"</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key Alias</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias"</code> </p> </li> </ul> <p>If you use a KMS key ID or an alias of your master key, the Amazon SageMaker execution role must include permissions to call <code>kms:Encrypt</code>. If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. Amazon SageMaker uses server-side encryption with KMS-managed keys for <code>OutputDataConfig</code>. If you use a bucket policy with an <code>s3:PutObject</code> permission that only allows objects with server-side encryption, set the condition key of <code>s3:x-amz-server-side-encryption</code> to <code>"aws:kms"</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateTrainingJob</code>, <code>CreateTransformJob</code>, or <code>CreateHyperParameterTuningJob</code> requests. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
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

/// <p>The trial that a trial component is associated with and the experiment the trial is part of. A component might not be associated with a trial. A component can be associated with multiple trials.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Parent {
    /// <p>The name of the experiment.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    /// <p>The name of the trial.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
}

/// <p>A previously completed or stopped hyperparameter tuning job to be used as a starting point for a new hyperparameter tuning job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentHyperParameterTuningJob {
    /// <p>The name of the hyperparameter tuning job to be used as a starting point for a new hyperparameter tuning job.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameter_tuning_job_name: Option<String>,
}

/// <p>Configuration for the cluster used to run a processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingClusterConfig {
    /// <p>The number of ML compute instances to use in the processing job. For distributed processing jobs, specify a value greater than 1. The default value is 1.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The ML compute instance type for the processing job.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the processing job. </p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>The size of the ML storage volume in gigabytes that you want to provision. You must specify sufficient ML storage for your scenario.</p>
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,
}

/// <p>The inputs for a processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingInput {
    /// <p>The name of the inputs for the processing job.</p>
    #[serde(rename = "InputName")]
    pub input_name: String,
    /// <p>The S3 inputs for the processing job. </p>
    #[serde(rename = "S3Input")]
    pub s3_input: ProcessingS3Input,
}

/// <p>Summary of information about a processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProcessingJobSummary {
    /// <p>The time at which the processing job was created.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>An optional string, up to one KB in size, that contains metadata from the processing container when the processing job exits.</p>
    #[serde(rename = "ExitMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_message: Option<String>,
    /// <p>A string, up to one KB in size, that contains the reason a processing job failed, if it failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A timestamp that indicates the last time the processing job was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The time at which the processing job completed.</p>
    #[serde(rename = "ProcessingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the processing job..</p>
    #[serde(rename = "ProcessingJobArn")]
    pub processing_job_arn: String,
    /// <p>The name of the processing job.</p>
    #[serde(rename = "ProcessingJobName")]
    pub processing_job_name: String,
    /// <p>The status of the processing job.</p>
    #[serde(rename = "ProcessingJobStatus")]
    pub processing_job_status: String,
}

/// <p>Describes the results of a processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingOutput {
    /// <p>The name for the processing job output.</p>
    #[serde(rename = "OutputName")]
    pub output_name: String,
    /// <p>Configuration for processing job outputs in Amazon S3.</p>
    #[serde(rename = "S3Output")]
    pub s3_output: ProcessingS3Output,
}

/// <p>The output configuration for the processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingOutputConfig {
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the processing job output. <code>KmsKeyId</code> can be an ID of a KMS key, ARN of a KMS key, alias of a KMS key, or alias of a KMS key. The <code>KmsKeyId</code> is applied to all outputs.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Output configuration information for a processing job.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<ProcessingOutput>,
}

/// <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingResources {
    /// <p>The configuration for the resources in a cluster used to run the processing job.</p>
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ProcessingClusterConfig,
}

/// <p>Information about where and how you want to obtain the inputs for an processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingS3Input {
    /// <p>The local path to the Amazon S3 bucket where you want Amazon SageMaker to download the inputs to run a processing job. <code>LocalPath</code> is an absolute path to the input data.</p>
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    /// <p>Whether to use <code>Gzip</code> compresion for Amazon S3 storage.</p>
    #[serde(rename = "S3CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compression_type: Option<String>,
    /// <p>Whether the data stored in Amazon S3 is <code>FullyReplicated</code> or <code>ShardedByS3Key</code>.</p>
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<String>,
    /// <p>Whether you use an <code>S3Prefix</code> or a <code>ManifestFile</code> for the data type. If you choose <code>S3Prefix</code>, <code>S3Uri</code> identifies a key name prefix. Amazon SageMaker uses all objects with the specified key name prefix for the processing job. If you choose <code>ManifestFile</code>, <code>S3Uri</code> identifies an object that is a manifest file containing a list of object keys that you want Amazon SageMaker to use for the processing job.</p>
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    /// <p>Wether to use <code>File</code> or <code>Pipe</code> input mode. In <code>File</code> mode, Amazon SageMaker copies the data from the input source onto the local Amazon Elastic Block Store (Amazon EBS) volumes before starting your training algorithm. This is the most commonly used input mode. In <code>Pipe</code> mode, Amazon SageMaker streams input data from the source directly to your algorithm without using the EBS volume.</p>
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: String,
    /// <p>The URI for the Amazon S3 storage where you want Amazon SageMaker to download the artifacts needed to run a processing job.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Information about where and how you want to store the results of an processing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingS3Output {
    /// <p>The local path to the Amazon S3 bucket where you want Amazon SageMaker to save the results of an processing job. <code>LocalPath</code> is an absolute path to the input data.</p>
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    /// <p>Whether to upload the results of the processing job continuously or after the job completes.</p>
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: String,
    /// <p>A URI that identifies the Amazon S3 bucket where you want Amazon SageMaker to save the results of a processing job.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Specifies a time limit for how long the processing job is allowed to run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingStoppingCondition {
    /// <p>Specifies the maximum runtime in seconds.</p>
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: i64,
}

/// <p>Identifies a model that you want to host and the resources to deploy for hosting it. If you are deploying multiple models, tell Amazon SageMaker how to distribute traffic among the models by specifying variant weights. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionVariant {
    /// <p>The size of the Elastic Inference (EI) instance to use for the production variant. EI instances provide on-demand GPU computing for inference. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Part of the <code>SuggestionQuery</code> type. Specifies a hint for retrieving property names that begin with the specified text.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PropertyNameQuery {
    /// <p>Text that begins a property's name.</p>
    #[serde(rename = "PropertyNameHint")]
    pub property_name_hint: String,
}

/// <p>A property name returned from a <code>GetSearchSuggestions</code> call that specifies a value in the <code>PropertyNameQuery</code> field.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PropertyNameSuggestion {
    /// <p>A suggested property name based on what you entered in the search textbox in the Amazon SageMaker console.</p>
    #[serde(rename = "PropertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}

/// <p><p>Defines the amount of money paid to an Amazon Mechanical Turk worker for each task performed. </p> <p>Use one of the following prices for bounding box tasks. Prices are in US dollars and should be based on the complexity of the task; the longer it takes in your initial testing, the more you should offer.</p> <ul> <li> <p>0.036</p> </li> <li> <p>0.048</p> </li> <li> <p>0.060</p> </li> <li> <p>0.072</p> </li> <li> <p>0.120</p> </li> <li> <p>0.240</p> </li> <li> <p>0.360</p> </li> <li> <p>0.480</p> </li> <li> <p>0.600</p> </li> <li> <p>0.720</p> </li> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul> <p>Use one of the following prices for image classification, text classification, and custom tasks. Prices are in US dollars.</p> <ul> <li> <p>0.012</p> </li> <li> <p>0.024</p> </li> <li> <p>0.036</p> </li> <li> <p>0.048</p> </li> <li> <p>0.060</p> </li> <li> <p>0.072</p> </li> <li> <p>0.120</p> </li> <li> <p>0.240</p> </li> <li> <p>0.360</p> </li> <li> <p>0.480</p> </li> <li> <p>0.600</p> </li> <li> <p>0.720</p> </li> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul> <p>Use one of the following prices for semantic segmentation tasks. Prices are in US dollars.</p> <ul> <li> <p>0.840</p> </li> <li> <p>0.960</p> </li> <li> <p>1.080</p> </li> <li> <p>1.200</p> </li> </ul> <p>Use one of the following prices for Textract AnalyzeDocument Important Form Key Amazon Augmented AI review tasks. Prices are in US dollars.</p> <ul> <li> <p>2.400 </p> </li> <li> <p>2.280 </p> </li> <li> <p>2.160 </p> </li> <li> <p>2.040 </p> </li> <li> <p>1.920 </p> </li> <li> <p>1.800 </p> </li> <li> <p>1.680 </p> </li> <li> <p>1.560 </p> </li> <li> <p>1.440 </p> </li> <li> <p>1.320 </p> </li> <li> <p>1.200 </p> </li> <li> <p>1.080 </p> </li> <li> <p>0.960 </p> </li> <li> <p>0.840 </p> </li> <li> <p>0.720 </p> </li> <li> <p>0.600 </p> </li> <li> <p>0.480 </p> </li> <li> <p>0.360 </p> </li> <li> <p>0.240 </p> </li> <li> <p>0.120 </p> </li> <li> <p>0.072 </p> </li> <li> <p>0.060 </p> </li> <li> <p>0.048 </p> </li> <li> <p>0.036 </p> </li> <li> <p>0.024 </p> </li> <li> <p>0.012 </p> </li> </ul> <p>Use one of the following prices for Rekognition DetectModerationLabels Amazon Augmented AI review tasks. Prices are in US dollars.</p> <ul> <li> <p>1.200 </p> </li> <li> <p>1.080 </p> </li> <li> <p>0.960 </p> </li> <li> <p>0.840 </p> </li> <li> <p>0.720 </p> </li> <li> <p>0.600 </p> </li> <li> <p>0.480 </p> </li> <li> <p>0.360 </p> </li> <li> <p>0.240 </p> </li> <li> <p>0.120 </p> </li> <li> <p>0.072 </p> </li> <li> <p>0.060 </p> </li> <li> <p>0.048 </p> </li> <li> <p>0.036 </p> </li> <li> <p>0.024 </p> </li> <li> <p>0.012 </p> </li> </ul> <p>Use one of the following prices for Amazon Augmented AI custom human review tasks. Prices are in US dollars.</p> <ul> <li> <p>1.200 </p> </li> <li> <p>1.080 </p> </li> <li> <p>0.960 </p> </li> <li> <p>0.840 </p> </li> <li> <p>0.720 </p> </li> <li> <p>0.600 </p> </li> <li> <p>0.480 </p> </li> <li> <p>0.360 </p> </li> <li> <p>0.240 </p> </li> <li> <p>0.120 </p> </li> <li> <p>0.072 </p> </li> <li> <p>0.060 </p> </li> <li> <p>0.048 </p> </li> <li> <p>0.036 </p> </li> <li> <p>0.024 </p> </li> <li> <p>0.012 </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicWorkforceTaskPrice {
    /// <p>Defines the amount of money paid to an Amazon Mechanical Turk worker in United States dollars.</p>
    #[serde(rename = "AmountInUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in_usd: Option<USD>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RenderableTask {
    /// <p>A JSON object that contains values for the variables defined in the template. It is made available to the template under the substitution variable <code>task.input</code>. For example, if you define a variable <code>task.input.text</code> in your template, you can supply the variable in the JSON object as <code>"text": "sample text"</code>.</p>
    #[serde(rename = "Input")]
    pub input: String,
}

/// <p>A description of an error that occurred while rendering the template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RenderingError {
    /// <p>A unique identifier for a specific class of errors.</p>
    #[serde(rename = "Code")]
    pub code: String,
    /// <p>A human-readable message describing the error.</p>
    #[serde(rename = "Message")]
    pub message: String,
}

/// <p>The resolved attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolvedAttributes {
    #[serde(rename = "AutoMLJobObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_objective: Option<AutoMLJobObjective>,
    #[serde(rename = "CompletionCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<AutoMLJobCompletionCriteria>,
    /// <p>The problem type.</p>
    #[serde(rename = "ProblemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_type: Option<String>,
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
    /// <p><p>The AWS KMS key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance(s) that run the training job.</p> <note> <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can&#39;t request a <code>VolumeKmsKeyId</code> when using an instance type with local storage.</p> <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p> <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p> </note> <p>The <code>VolumeKmsKeyId</code> can be in any of the following formats:</p> <ul> <li> <p>// KMS Key ID</p> <p> <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>// Amazon Resource Name (ARN) of a KMS Key</p> <p> <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p><p>The size of the ML storage volume that you want to provision. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use the ML storage volume for scratch space. If you want to store the training data in the ML storage volume, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. </p> <p>You must specify sufficient ML storage for your scenario. </p> <note> <p> Amazon SageMaker supports only the General Purpose SSD (gp2) ML storage volume type. </p> </note> <note> <p>Certain Nitro-based instances include local storage with a fixed total size, dependent on the instance type. When using these instances for training, Amazon SageMaker mounts the local instance storage instead of Amazon EBS gp2 storage. You can&#39;t request a <code>VolumeSizeInGB</code> greater than the total size of the local instance storage.</p> <p>For a list of instance types that support local instance storage, including the total size per instance type, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p> </note></p>
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

/// <p>The instance type and quantity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// <p>The Amazon Resource Name (ARN) of the environment.</p>
    #[serde(rename = "EnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_arn: Option<String>,
    /// <p>The instance type.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

/// <p>The retention policy.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RetentionPolicy {
    /// <p>The home Amazon Elastic File System (EFS).</p>
    #[serde(rename = "HomeEfsFileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_efs_file_system: Option<String>,
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
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example: </p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> The preceding JSON matches the following <code>s3Uris</code>: </p> <p> <code>[ {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code>&quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code>&quot;relative/path/custdata-2&quot;,</code> </p> <p> <code>...</code> </p> <p> <code>&quot;relative/path/custdata-N&quot;</code> </p> <p> <code>]</code> </p> <p> The preceding JSON matches the following <code>s3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer<em>bucket/some/prefix/relative/path/custdata-2</code> </p> <p> <code>...</code> </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/custdata-N</code> </p> <p>The complete set of <code>s3uris</code> in this manifest is the input data for the channel for this datasource. The object that each <code>s3uris</code> points to must be readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf. </p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Configuration details about the monitoring schedule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleConfig {
    /// <p><p>A cron expression that describes details about the monitoring schedule.</p> <p>Currently the only supported cron expressions are:</p> <ul> <li> <p>If you want to set the job to start every hour, please use the following:</p> <p> <code>Hourly: cron(0 * ? * * *)</code> </p> </li> <li> <p>If you want to start the job daily:</p> <p> <code>cron(0 [00-23] ? * * *)</code> </p> </li> </ul> <p>For example, the following are valid cron expressions:</p> <ul> <li> <p>Daily at noon UTC: <code>cron(0 12 ? * * *)</code> </p> </li> <li> <p>Daily at midnight UTC: <code>cron(0 0 ? * * *)</code> </p> </li> </ul> <p>To support running every 6, 12 hours, the following are also supported:</p> <p> <code>cron(0 [00-23]/[01-24] ? * * *)</code> </p> <p>For example, the following are valid cron expressions:</p> <ul> <li> <p>Every 12 hours, starting at 5pm UTC: <code>cron(0 17/12 ? * * *)</code> </p> </li> <li> <p>Every two hours starting at midnight: <code>cron(0 0/2 ? * * *)</code> </p> </li> </ul> <note> <ul> <li> <p>Even though the cron expression is set to start at 5PM UTC, note that there could be a delay of 0-20 minutes from the actual requested time to run the execution. </p> </li> <li> <p>We recommend that if you would like a daily schedule, you do not provide this parameter. Amazon SageMaker will pick a time for running every day.</p> </li> </ul> </note></p>
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
}

/// <p><p>A multi-expression that searches for the specified resource or resources in a search. All resource objects that satisfy the expression&#39;s condition are included in the search results. You must specify at least one subexpression, filter, or nested filter. A <code>SearchExpression</code> can contain up to twenty elements.</p> <p>A <code>SearchExpression</code> contains the following components:</p> <ul> <li> <p>A list of <code>Filter</code> objects. Each filter defines a simple Boolean expression comprised of a resource property name, Boolean operator, and value. A <code>SearchExpression</code> can include only one <code>Contains</code> operator.</p> </li> <li> <p>A list of <code>NestedFilter</code> objects. Each nested filter defines a list of Boolean expressions using a list of resource properties. A nested filter is satisfied if a single object in the list satisfies all Boolean expressions.</p> </li> <li> <p>A list of <code>SearchExpression</code> objects. A search expression object can be nested in a list of search expression objects.</p> </li> <li> <p>A Boolean operator: <code>And</code> or <code>Or</code>.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchRecord {
    /// <p>A summary of the properties of an experiment.</p>
    #[serde(rename = "Experiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment: Option<Experiment>,
    /// <p>A <code>TrainingJob</code> object that is returned as part of a <code>Search</code> request.</p>
    #[serde(rename = "TrainingJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job: Option<TrainingJob>,
    /// <p>A summary of the properties of a trial.</p>
    #[serde(rename = "Trial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<Trial>,
    /// <p>A summary of the properties of a trial component.</p>
    #[serde(rename = "TrialComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component: Option<TrialComponent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchRequest {
    /// <p>The maximum number of results to return in a <code>SearchResponse</code>.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If more than <code>MaxResults</code> resource objects match the specified <code>SearchExpression</code>, the <code>SearchResponse</code> includes a <code>NextToken</code>. The <code>NextToken</code> can be passed to the next <code>SearchRequest</code> to continue retrieving results for the specified <code>SearchExpression</code> and <code>Sort</code> parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the Amazon SageMaker resource to search for.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>A detailed description of the progress within a secondary status. </p> <p>Amazon SageMaker provides secondary statuses and status messages that apply to each of them:</p> <dl> <dt>Starting</dt> <dd> <ul> <li> <p>Starting the training job.</p> </li> <li> <p>Launching requested ML instances.</p> </li> <li> <p>Insufficient capacity error from EC2 while launching instances, retrying!</p> </li> <li> <p>Launched instance was unhealthy, replacing it!</p> </li> <li> <p>Preparing the instances for training.</p> </li> </ul> </dd> <dt>Training</dt> <dd> <ul> <li> <p>Downloading the training image.</p> </li> <li> <p>Training image download completed. Training in progress.</p> </li> </ul> </dd> </dl> <important> <p>Status messages are subject to change. Therefore, we recommend not including them in code that programmatically initiates actions. For examples, don&#39;t use status messages in if statements.</p> </important> <p>To have an overview of your training job&#39;s progress, view <code>TrainingJobStatus</code> and <code>SecondaryStatus</code> in <a>DescribeTrainingJob</a>, and <code>StatusMessage</code> together. For example, at the start of a training job, you might see the following:</p> <ul> <li> <p> <code>TrainingJobStatus</code> - InProgress</p> </li> <li> <p> <code>SecondaryStatus</code> - Training</p> </li> <li> <p> <code>StatusMessage</code> - Downloading the training image</p> </li> </ul></p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>The sharing settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingSettings {
    /// <p>The notebook output option.</p>
    #[serde(rename = "NotebookOutputOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_output_option: Option<String>,
    /// <p>The AWS Key Management Service encryption key ID.</p>
    #[serde(rename = "S3KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_kms_key_id: Option<String>,
    /// <p>The Amazon S3 output path.</p>
    #[serde(rename = "S3OutputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_path: Option<String>,
}

/// <p>A configuration for a shuffle option for input data in a channel. If you use <code>S3Prefix</code> for <code>S3DataType</code>, the results of the S3 key prefix matches are shuffled. If you use <code>ManifestFile</code>, the order of the S3 object references in the <code>ManifestFile</code> is shuffled. If you use <code>AugmentedManifestFile</code>, the order of the JSON lines in the <code>AugmentedManifestFile</code> is shuffled. The shuffling order is determined using the <code>Seed</code> value.</p> <p>For Pipe input mode, when <code>ShuffleConfig</code> is specified shuffling is done at the start of every epoch. With large datasets, this ensures that the order of the training data is different for each epoch, and it helps reduce bias and possible overfitting. In a multi-node training job when <code>ShuffleConfig</code> is combined with <code>S3DataDistributionType</code> of <code>ShardedByS3Key</code>, the data is shuffled across nodes so that the content sent to a particular node on the first epoch might be sent to a different node on the second epoch.</p>
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

/// <p>A list of IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>). Used to create an allow list of IP addresses for a private workforce. For more information, see .</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceIpConfig {
    /// <p><p>A list of one to four <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">Classless Inter-Domain Routing</a> (CIDR) values.</p> <p>Maximum: 4 CIDR values</p> <note> <p>The following Length Constraints apply to individual CIDR values in the CIDR value list.</p> </note></p>
    #[serde(rename = "Cidrs")]
    pub cidrs: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMonitoringScheduleRequest {
    /// <p>The name of the schedule to start.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartNotebookInstanceInput {
    /// <p>The name of the notebook instance to start.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopAutoMLJobRequest {
    /// <p>The name of the object you are requesting.</p>
    #[serde(rename = "AutoMLJobName")]
    pub auto_ml_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopCompilationJobRequest {
    /// <p>The name of the model compilation job to stop.</p>
    #[serde(rename = "CompilationJobName")]
    pub compilation_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopHyperParameterTuningJobRequest {
    /// <p>The name of the tuning job to stop.</p>
    #[serde(rename = "HyperParameterTuningJobName")]
    pub hyper_parameter_tuning_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopLabelingJobRequest {
    /// <p>The name of the labeling job to stop.</p>
    #[serde(rename = "LabelingJobName")]
    pub labeling_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopMonitoringScheduleRequest {
    /// <p>The name of the schedule to stop.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopNotebookInstanceInput {
    /// <p>The name of the notebook instance to terminate.</p>
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopProcessingJobRequest {
    /// <p>The name of the processing job to stop.</p>
    #[serde(rename = "ProcessingJobName")]
    pub processing_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTrainingJobRequest {
    /// <p>The name of the training job to stop.</p>
    #[serde(rename = "TrainingJobName")]
    pub training_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTransformJobRequest {
    /// <p>The name of the transform job to stop.</p>
    #[serde(rename = "TransformJobName")]
    pub transform_job_name: String,
}

/// <p><p>Specifies a limit to how long a model training or compilation job can run. It also specifies how long you are willing to wait for a managed spot training job to complete. When the job reaches the time limit, Amazon SageMaker ends the training or compilation job. Use this API to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p> <p>The training algorithms provided by Amazon SageMaker automatically save the intermediate results of a model training job when possible. This attempt to save artifacts is only a best effort case as model might not be in a state from which it can be saved. For example, if training has just started, the model might not be ready to save. When saved, this intermediate data is a valid model artifact. You can use it to create a model with <code>CreateModel</code>.</p> <note> <p>The Neural Topic Model (NTM) currently does not support saving intermediate model artifacts. When training NTMs, make sure that the maximum runtime is sufficient for the training job to complete.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoppingCondition {
    /// <p>The maximum length of time, in seconds, that the training or compilation job can run. If job does not complete during this time, Amazon SageMaker ends the job. If value is not specified, default value is 1 day. The maximum value is 28 days.</p>
    #[serde(rename = "MaxRuntimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runtime_in_seconds: Option<i64>,
    /// <p>The maximum length of time, in seconds, how long you are willing to wait for a managed spot training job to complete. It is the amount of time spent waiting for Spot capacity plus the amount of time the training job runs. It must be equal to or greater than <code>MaxRuntimeInSeconds</code>. </p>
    #[serde(rename = "MaxWaitTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_time_in_seconds: Option<i64>,
}

/// <p>Describes a work team of a vendor that does the a labelling job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Specified in the <a>GetSearchSuggestions</a> request. Limits the property names that are included in the response.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SuggestionQuery {
    /// <p>Defines a property name hint. Only property names that begin with the specified hint are included in the response.</p>
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

/// <p>The TensorBoard app settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TensorBoardAppSettings {
    /// <p>The instance type and quantity.</p>
    #[serde(rename = "DefaultResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

/// <p>Configuration of storage locations for TensorBoard output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TensorBoardOutputConfig {
    /// <p>Path to local storage location for tensorBoard output. Defaults to <code>/opt/ml/output/tensorboard</code>.</p>
    #[serde(rename = "LocalPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    /// <p>Path to Amazon S3 storage location for TensorBoard output.</p>
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
}

/// <p>Contains information about a training job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrainingJob {
    /// <p>Information about the algorithm used for training, and algorithm metadata.</p>
    #[serde(rename = "AlgorithmSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_specification: Option<AlgorithmSpecification>,
    /// <p>The Amazon Resource Name (ARN) of the job.</p>
    #[serde(rename = "AutoMLJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_job_arn: Option<String>,
    /// <p>The billable time in seconds.</p>
    #[serde(rename = "BillableTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_time_in_seconds: Option<i64>,
    #[serde(rename = "CheckpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_config: Option<CheckpointConfig>,
    /// <p>A timestamp that indicates when the training job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DebugHookConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_hook_config: Option<DebugHookConfig>,
    /// <p>Information about the debug rule configuration.</p>
    #[serde(rename = "DebugRuleConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_rule_configurations: Option<Vec<DebugRuleConfiguration>>,
    /// <p>Information about the evaluation status of the rules for the training job.</p>
    #[serde(rename = "DebugRuleEvaluationStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_rule_evaluation_statuses: Option<Vec<DebugRuleEvaluationStatus>>,
    /// <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training.</p>
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    /// <p>When true, enables managed spot training using Amazon EC2 Spot instances to run training jobs instead of on-demand instances. For more information, see <a>model-managed-spot-training</a>.</p>
    #[serde(rename = "EnableManagedSpotTraining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_managed_spot_training: Option<bool>,
    /// <p>If the <code>TrainingJob</code> was created with network isolation, the value is set to <code>true</code>. If network isolation is enabled, nodes can't communicate beyond the VPC they run in.</p>
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "ExperimentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_config: Option<ExperimentConfig>,
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
    /// <p>Specifies a limit to how long a model training job can run. When the job reaches the time limit, Amazon SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p>
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<StoppingCondition>,
    /// <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TensorBoardOutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tensor_board_output_config: Option<TensorBoardOutputConfig>,
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
    /// <p>The training time in seconds.</p>
    #[serde(rename = "TrainingTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_time_in_seconds: Option<i64>,
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
    /// <p>Specifies a limit to how long a model training job can run. When the job reaches the time limit, Amazon SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts.</p>
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: StoppingCondition,
    /// <p>The input mode used by the algorithm for the training job. For the input modes that Amazon SageMaker algorithms support, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>.</p> <p>If an algorithm supports the <code>File</code> input mode, Amazon SageMaker downloads the training data from S3 to the provisioned ML storage Volume, and mounts the directory to docker volume for training container. If an algorithm supports the <code>Pipe</code> input mode, Amazon SageMaker streams data directly from S3 to the container.</p>
    #[serde(rename = "TrainingInputMode")]
    pub training_input_mode: String,
}

/// <p>The numbers of training jobs launched by a hyperparameter tuning job, categorized by status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Indicates whether the algorithm supports distributed training. If set to false, buyers can't request more than one instance during training.</p>
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
    /// <p><p>The method to use to split the transform job&#39;s data files into smaller batches. Splitting is necessary when the total size of each object is too large to fit in a single request. You can also use data splitting to improve performance by processing multiple concurrent mini-batches. The default value for <code>SplitType</code> is <code>None</code>, which indicates that input data files are not split, and request payloads contain the entire contents of an input object. Set the value of this parameter to <code>Line</code> to split records on a newline character boundary. <code>SplitType</code> also supports a number of record-oriented binary data formats.</p> <p>When splitting is enabled, the size of a mini-batch depends on the values of the <code>BatchStrategy</code> and <code>MaxPayloadInMB</code> parameters. When the value of <code>BatchStrategy</code> is <code>MultiRecord</code>, Amazon SageMaker sends the maximum number of records in each request, up to the <code>MaxPayloadInMB</code> limit. If the value of <code>BatchStrategy</code> is <code>SingleRecord</code>, Amazon SageMaker sends individual records in each request.</p> <note> <p>Some data formats represent a record as a binary payload wrapped with extra padding bytes. When splitting is applied to a binary data format, padding is removed if the value of <code>BatchStrategy</code> is set to <code>SingleRecord</code>. Padding is not removed if the value of <code>BatchStrategy</code> is set to <code>MultiRecord</code>.</p> <p>For more information about <code>RecordIO</code>, see <a href="https://mxnet.apache.org/api/faq/recordio">Create a Dataset Using RecordIO</a> in the MXNet documentation. For more information about <code>TFRecord</code>, see <a href="https://www.tensorflow.org/guide/datasets#consuming_tfrecord_data">Consuming TFRecord data</a> in the TensorFlow documentation.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption. The <code>KmsKeyId</code> can be any of the following formats: </p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS-Managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p>The KMS key policy must grant permission to the IAM role that you specify in your <a>CreateModel</a> request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in AWS KMS</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
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
    /// <p>The number of ML compute instances to use in the transform job. For distributed transform jobs, specify a value greater than 1. The default value is <code>1</code>.</p>
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,
    /// <p>The ML compute instance type for the transform job. If you are using built-in algorithms to transform moderately sized datasets, we recommend using ml.m4.xlarge or <code>ml.m5.large</code> instance types.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p><p>The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt model data on the storage volume attached to the ML compute instance(s) that run the batch transform job. The <code>VolumeKmsKeyId</code> can be any of the following formats:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul></p>
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
    /// <p><p>Depending on the value specified for the <code>S3DataType</code>, identifies either a key name prefix or a manifest. For example:</p> <ul> <li> <p> A key name prefix might look like this: <code>s3://bucketname/exampleprefix</code>. </p> </li> <li> <p> A manifest might look like this: <code>s3://bucketname/example.manifest</code> </p> <p> The manifest is an S3 object which is a JSON file with the following format: </p> <p> <code>[ {&quot;prefix&quot;: &quot;s3://customer<em>bucket/some/prefix/&quot;},</code> </p> <p> <code>&quot;relative/path/to/custdata-1&quot;,</code> </p> <p> <code>&quot;relative/path/custdata-2&quot;,</code> </p> <p> <code>...</code> </p> <p> <code>&quot;relative/path/custdata-N&quot;</code> </p> <p> <code>]</code> </p> <p> The preceding JSON matches the following <code>s3Uris</code>: </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/to/custdata-1</code> </p> <p> <code>s3://customer<em>bucket/some/prefix/relative/path/custdata-2</code> </p> <p> <code>...</code> </p> <p> <code>s3://customer</em>bucket/some/prefix/relative/path/custdata-N</code> </p> <p> The complete set of <code>S3Uris</code> in this manifest constitutes the input data for the channel for this datasource. The object that each <code>S3Uris</code> points to must be readable by the IAM role that Amazon SageMaker uses to perform tasks on your behalf.</p> </li> </ul></p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>A summary of the properties of a trial as returned by the <a>Search</a> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trial {
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the trial was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the trial as displayed. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the experiment the trial is part of.</p>
    #[serde(rename = "ExperimentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_name: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>Who last modified the trial.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TrialSource>,
    /// <p>The list of tags that are associated with the trial. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
    /// <p>A list of the components associated with the trial. For each component, a summary of the component's properties is included.</p>
    #[serde(rename = "TrialComponentSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_summaries: Option<Vec<TrialComponentSimpleSummary>>,
    /// <p>The name of the trial.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
}

/// <p>A summary of the properties of a trial component as returned by the <a>Search</a> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponent {
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the component was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the component as displayed. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the component ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input artifacts of the component.</p>
    #[serde(rename = "InputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the component was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The metrics for the component.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TrialComponentMetricSummary>>,
    /// <p>The output artifacts of the component.</p>
    #[serde(rename = "OutputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The hyperparameters of the component.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, TrialComponentParameterValue>>,
    /// <p>An array of the parents of the component. A parent is a trial the component is associated with and the experiment the trial is part of. A component might not have any parents.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TrialComponentSource>,
    /// <p>The source of the trial component.&gt;</p>
    #[serde(rename = "SourceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_detail: Option<TrialComponentSourceDetail>,
    /// <p>When the component started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialComponentStatus>,
    /// <p>The list of tags that are associated with the component. You can use <a>Search</a> API to search on the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
    /// <p>The name of the trial component.</p>
    #[serde(rename = "TrialComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_name: Option<String>,
}

/// <p>Represents an input or output artifact of a trial component. You specify <code>TrialComponentArtifact</code> as part of the <code>InputArtifacts</code> and <code>OutputArtifacts</code> parameters in the <a>CreateTrialComponent</a> request.</p> <p>Examples of input artifacts are datasets, algorithms, hyperparameters, source code, and instance types. Examples of output artifacts are metrics, snapshots, logs, and images.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrialComponentArtifact {
    /// <p>The media type of the artifact, which indicates the type of data in the artifact file. The media type consists of a <i>type</i> and a <i>subtype</i> concatenated with a slash (/) character, for example, text/csv, image/jpeg, and s3/uri. The type specifies the category of the media. The subtype specifies the kind of data.</p>
    #[serde(rename = "MediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// <p>The location of the artifact.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A summary of the metrics of a trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponentMetricSummary {
    /// <p>The average value of the metric.</p>
    #[serde(rename = "Avg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    /// <p>The number of samples used to generate the metric.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The most recent value of the metric.</p>
    #[serde(rename = "Last")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// <p>The maximum value of the metric.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The minimum value of the metric.</p>
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The standard deviation of the metric.</p>
    #[serde(rename = "StdDev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_dev: Option<f64>,
    /// <p>When the metric was last updated.</p>
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<f64>,
}

/// <p>The value of a hyperparameter. Only one of <code>NumberValue</code> or <code>StringValue</code> can be specified.</p> <p>This object is specified in the <a>CreateTrialComponent</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrialComponentParameterValue {
    /// <p>The numeric value of a numeric hyperparameter. If you specify a value for this parameter, you can't specify the <code>StringValue</code> parameter.</p>
    #[serde(rename = "NumberValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    /// <p>The string value of a categorical hyperparameter. If you specify a value for this parameter, you can't specify the <code>NumberValue</code> parameter.</p>
    #[serde(rename = "StringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>A short summary of a trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponentSimpleSummary {
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the component was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
    /// <p>The name of the trial component.</p>
    #[serde(rename = "TrialComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_name: Option<String>,
    #[serde(rename = "TrialComponentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_source: Option<TrialComponentSource>,
}

/// <p>The source of the trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponentSource {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The source job type.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>Detailed information about the source of a trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponentSourceDetail {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "TrainingJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job: Option<TrainingJob>,
}

/// <p>The status of the trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrialComponentStatus {
    /// <p>If the component failed, a message describing why.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status of the trial component.</p>
    #[serde(rename = "PrimaryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_status: Option<String>,
}

/// <p>A summary of the properties of a trial component. To get all the properties, call the <a>DescribeTrialComponent</a> API and provide the <code>TrialComponentName</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialComponentSummary {
    /// <p>Who created the component.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,
    /// <p>When the component was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the component as displayed. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the component ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Who last modified the component.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,
    /// <p>When the component was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>When the component started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>The status of the component. States include:</p> <ul> <li> <p>InProgress</p> </li> <li> <p>Completed</p> </li> <li> <p>Failed</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialComponentStatus>,
    /// <p>The ARN of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
    /// <p>The name of the trial component.</p>
    #[serde(rename = "TrialComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_name: Option<String>,
    #[serde(rename = "TrialComponentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_source: Option<TrialComponentSource>,
}

/// <p>The source of the trial.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialSource {
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The source job type.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>A summary of the properties of a trial. To get the complete set of properties, call the <a>DescribeTrial</a> API and provide the <code>TrialName</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrialSummary {
    /// <p>When the trial was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the trial as displayed. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the trial was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
    /// <p>The name of the trial.</p>
    #[serde(rename = "TrialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_name: Option<String>,
    #[serde(rename = "TrialSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_source: Option<TrialSource>,
}

/// <p>The job completion criteria.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TuningJobCompletionCriteria {
    /// <p>The objective metric's value.</p>
    #[serde(rename = "TargetObjectiveMetricValue")]
    pub target_objective_metric_value: f32,
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
    /// <p>The Amazon S3 bucket location of the UI template. For more information about the contents of a UI template, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-custom-templates-step2.html"> Creating Your Custom Labeling Task Template</a>.</p>
    #[serde(rename = "UiTemplateS3Uri")]
    pub ui_template_s3_uri: String,
}

/// <p>The Liquid template for the worker user interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UiTemplate {
    /// <p>The content of the Liquid template for the worker user interface.</p>
    #[serde(rename = "Content")]
    pub content: String,
}

/// <p>Container for user interface template information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UiTemplateInfo {
    /// <p>The SHA 256 hash that you used to create the request signature.</p>
    #[serde(rename = "ContentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_sha_256: Option<String>,
    /// <p>The URL for the user interface template.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCodeRepositoryOutput {
    /// <p>The ARN of the Git repository.</p>
    #[serde(rename = "CodeRepositoryArn")]
    pub code_repository_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainRequest {
    /// <p>A collection of settings.</p>
    #[serde(rename = "DefaultUserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user_settings: Option<UserSettings>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainResponse {
    /// <p>The domain Amazon Resource Name (ARN).</p>
    #[serde(rename = "DomainArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointInput {
    /// <p>The name of the new endpoint configuration.</p>
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// <p>The name of the endpoint whose configuration you want to update.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointOutput {
    /// <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointWeightsAndCapacitiesInput {
    /// <p>An object that provides new capacity and weight values for a variant.</p>
    #[serde(rename = "DesiredWeightsAndCapacities")]
    pub desired_weights_and_capacities: Vec<DesiredWeightAndCapacity>,
    /// <p>The name of an existing Amazon SageMaker endpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointWeightsAndCapacitiesOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateExperimentRequest {
    /// <p>The description of the experiment.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the experiment as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the experiment to update.</p>
    #[serde(rename = "ExperimentName")]
    pub experiment_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateExperimentResponse {
    /// <p>The Amazon Resource Name (ARN) of the experiment.</p>
    #[serde(rename = "ExperimentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMonitoringScheduleRequest {
    /// <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    #[serde(rename = "MonitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleConfig,
    /// <p>The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account.</p>
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMonitoringScheduleResponse {
    /// <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p>
    #[serde(rename = "MonitoringScheduleArn")]
    pub monitoring_schedule_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNotebookInstanceInput {
    /// <p>A list of the Elastic Inference (EI) instance types to associate with this notebook instance. Currently only one EI instance type can be associated with a notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// <p>The Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with Amazon SageMaker Notebook Instances</a>.</p>
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
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB. ML storage volumes are encrypted, so Amazon SageMaker can't determine the amount of available free space on the volume. Because of this, you can increase the volume size when you update a notebook instance, but you can't decrease the volume size. If you want to decrease the size of the ML storage volume in use, create a new notebook instance with the desired size.</p>
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNotebookInstanceLifecycleConfigInput {
    /// <p>The name of the lifecycle configuration.</p>
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: String,
    /// <p>The shell script that runs only once, when you create a notebook instance. The shell script must be a base64-encoded string.</p>
    #[serde(rename = "OnCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// <p>The shell script that runs every time you start a notebook instance, including when you create the notebook instance. The shell script must be a base64-encoded string.</p>
    #[serde(rename = "OnStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNotebookInstanceLifecycleConfigOutput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNotebookInstanceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTrialComponentRequest {
    /// <p>The name of the component as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>When the component ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Replaces all of the component's input artifacts with the specified artifacts.</p>
    #[serde(rename = "InputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The input artifacts to remove from the component.</p>
    #[serde(rename = "InputArtifactsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts_to_remove: Option<Vec<String>>,
    /// <p>Replaces all of the component's output artifacts with the specified artifacts.</p>
    #[serde(rename = "OutputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<::std::collections::HashMap<String, TrialComponentArtifact>>,
    /// <p>The output artifacts to remove from the component.</p>
    #[serde(rename = "OutputArtifactsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts_to_remove: Option<Vec<String>>,
    /// <p>Replaces all of the component's hyperparameters with the specified hyperparameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, TrialComponentParameterValue>>,
    /// <p>The hyperparameters to remove from the component.</p>
    #[serde(rename = "ParametersToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_to_remove: Option<Vec<String>>,
    /// <p>When the component started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The new status of the component.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialComponentStatus>,
    /// <p>The name of the component to update.</p>
    #[serde(rename = "TrialComponentName")]
    pub trial_component_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTrialComponentResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial component.</p>
    #[serde(rename = "TrialComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_component_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTrialRequest {
    /// <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the trial to update.</p>
    #[serde(rename = "TrialName")]
    pub trial_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTrialResponse {
    /// <p>The Amazon Resource Name (ARN) of the trial.</p>
    #[serde(rename = "TrialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserProfileRequest {
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
    /// <p>A collection of settings.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserProfileResponse {
    /// <p>The user profile Amazon Resource Name (ARN).</p>
    #[serde(rename = "UserProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWorkforceRequest {
    /// <p>A list of one to four worker IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>) that can be used to access tasks assigned to this workforce.</p> <p>Maximum: 4 CIDR values</p>
    #[serde(rename = "SourceIpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_config: Option<SourceIpConfig>,
    /// <p>The name of the private workforce whose access you want to restrict. <code>WorkforceName</code> is automatically set to <code>"default"</code> when a workforce is created and cannot be modified. </p>
    #[serde(rename = "WorkforceName")]
    pub workforce_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWorkforceResponse {
    /// <p>A single private workforce, which is automatically created when you create your first private work team. You can create one private work force in each AWS Region. By default, any workforce related API operation used in a specific region will apply to the workforce created in that region. To learn how to create a private workforce, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-create-private.html">Create a Private Workforce</a>.</p>
    #[serde(rename = "Workforce")]
    pub workforce: Workforce,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWorkteamResponse {
    /// <p>A <code>Workteam</code> object that describes the updated work team.</p>
    #[serde(rename = "Workteam")]
    pub workteam: Workteam,
}

/// <p>Information about the user who created or modified an experiment, trial, or trial component.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserContext {
    /// <p>The domain associated with the user.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user's profile.</p>
    #[serde(rename = "UserProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_arn: Option<String>,
    /// <p>The name of the user's profile.</p>
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<String>,
}

/// <p>The user profile details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserProfileDetails {
    /// <p>The creation time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The domain ID.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>The last modified time.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user profile name.</p>
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<String>,
}

/// <p>A collection of settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    /// <p>The execution role for the user.</p>
    #[serde(rename = "ExecutionRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    /// <p>The Jupyter server's app settings.</p>
    #[serde(rename = "JupyterServerAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,
    /// <p>The kernel gateway app settings.</p>
    #[serde(rename = "KernelGatewayAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,
    /// <p>The security groups.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The sharing settings.</p>
    #[serde(rename = "SharingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_settings: Option<SharingSettings>,
    /// <p>The TensorBoard app settings.</p>
    #[serde(rename = "TensorBoardAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tensor_board_app_settings: Option<TensorBoardAppSettings>,
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

/// <p>A single private workforce, which is automatically created when you create your first private work team. You can create one private work force in each AWS Region. By default, any workforce related API operation used in a specific region will apply to the workforce created in that region. To learn how to create a private workforce, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-create-private.html">Create a Private Workforce</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Workforce {
    /// <p>The most recent date that was used to successfully add one or more IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>) to a private workforce's allow list.</p>
    #[serde(rename = "LastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>A list of one to four IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>) to be added to the workforce allow list.</p>
    #[serde(rename = "SourceIpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_config: Option<SourceIpConfig>,
    /// <p>The Amazon Resource Name (ARN) of the private workforce.</p>
    #[serde(rename = "WorkforceArn")]
    pub workforce_arn: String,
    /// <p>The name of the private workforce whose access you want to restrict. <code>WorkforceName</code> is automatically set to <code>"default"</code> when a workforce is created and cannot be modified. </p>
    #[serde(rename = "WorkforceName")]
    pub workforce_name: String,
}

/// <p>Provides details about a labeling work team.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Configures SNS notifications of available or expiring work items for work teams.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for AddTagsError {}
/// Errors returned by AssociateTrialComponent
#[derive(Debug, PartialEq)]
pub enum AssociateTrialComponentError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl AssociateTrialComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        AssociateTrialComponentError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(AssociateTrialComponentError::ResourceNotFound(
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
impl fmt::Display for AssociateTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateTrialComponentError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateTrialComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateTrialComponentError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAlgorithmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateAlgorithmError {}
/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateAppError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateAppError::ResourceLimitExceeded(err.msg))
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
            CreateAppError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateAppError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAppError {}
/// Errors returned by CreateAutoMLJob
#[derive(Debug, PartialEq)]
pub enum CreateAutoMLJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateAutoMLJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAutoMLJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateAutoMLJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateAutoMLJobError::ResourceLimitExceeded(
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
impl fmt::Display for CreateAutoMLJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAutoMLJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateAutoMLJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAutoMLJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCodeRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateCodeRepositoryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCompilationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCompilationJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateCompilationJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCompilationJobError {}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateDomainError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateDomainError::ResourceLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDomainError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEndpointError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEndpointError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEndpointConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEndpointConfigError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEndpointConfigError {}
/// Errors returned by CreateExperiment
#[derive(Debug, PartialEq)]
pub enum CreateExperimentError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateExperimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateExperimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateExperimentError::ResourceLimitExceeded(
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
impl fmt::Display for CreateExperimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateExperimentError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateExperimentError {}
/// Errors returned by CreateFlowDefinition
#[derive(Debug, PartialEq)]
pub enum CreateFlowDefinitionError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateFlowDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFlowDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateFlowDefinitionError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateFlowDefinitionError::ResourceLimitExceeded(
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
impl fmt::Display for CreateFlowDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFlowDefinitionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateFlowDefinitionError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFlowDefinitionError {}
/// Errors returned by CreateHumanTaskUi
#[derive(Debug, PartialEq)]
pub enum CreateHumanTaskUiError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateHumanTaskUiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHumanTaskUiError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateHumanTaskUiError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateHumanTaskUiError::ResourceLimitExceeded(
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
impl fmt::Display for CreateHumanTaskUiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHumanTaskUiError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateHumanTaskUiError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHumanTaskUiError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateHyperParameterTuningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHyperParameterTuningJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateHyperParameterTuningJobError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateHyperParameterTuningJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLabelingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLabelingJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateLabelingJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLabelingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateModelError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateModelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateModelPackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateModelPackageError {}
/// Errors returned by CreateMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum CreateMonitoringScheduleError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateMonitoringScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateMonitoringScheduleError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateMonitoringScheduleError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMonitoringScheduleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateMonitoringScheduleError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNotebookInstanceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNotebookInstanceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNotebookInstanceLifecycleConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateNotebookInstanceLifecycleConfigError {}
/// Errors returned by CreatePresignedDomainUrl
#[derive(Debug, PartialEq)]
pub enum CreatePresignedDomainUrlError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl CreatePresignedDomainUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePresignedDomainUrlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(CreatePresignedDomainUrlError::ResourceNotFound(
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
impl fmt::Display for CreatePresignedDomainUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePresignedDomainUrlError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePresignedDomainUrlError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePresignedNotebookInstanceUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreatePresignedNotebookInstanceUrlError {}
/// Errors returned by CreateProcessingJob
#[derive(Debug, PartialEq)]
pub enum CreateProcessingJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl CreateProcessingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProcessingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateProcessingJobError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateProcessingJobError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(CreateProcessingJobError::ResourceNotFound(
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
impl fmt::Display for CreateProcessingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProcessingJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateProcessingJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProcessingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProcessingJobError {}
/// Errors returned by CreateTrainingJob
#[derive(Debug, PartialEq)]
pub enum CreateTrainingJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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
                "ResourceNotFound" => {
                    return RusotoError::Service(CreateTrainingJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTrainingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrainingJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateTrainingJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTrainingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrainingJobError {}
/// Errors returned by CreateTransformJob
#[derive(Debug, PartialEq)]
pub enum CreateTransformJobError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
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
                "ResourceNotFound" => {
                    return RusotoError::Service(CreateTransformJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTransformJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTransformJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateTransformJobError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTransformJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTransformJobError {}
/// Errors returned by CreateTrial
#[derive(Debug, PartialEq)]
pub enum CreateTrialError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl CreateTrialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateTrialError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(CreateTrialError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTrialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrialError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateTrialError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrialError {}
/// Errors returned by CreateTrialComponent
#[derive(Debug, PartialEq)]
pub enum CreateTrialComponentError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateTrialComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateTrialComponentError::ResourceLimitExceeded(
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
impl fmt::Display for CreateTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrialComponentError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrialComponentError {}
/// Errors returned by CreateUserProfile
#[derive(Debug, PartialEq)]
pub enum CreateUserProfileError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
}

impl CreateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(CreateUserProfileError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateUserProfileError::ResourceLimitExceeded(
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
impl fmt::Display for CreateUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserProfileError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateUserProfileError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserProfileError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorkteamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateWorkteamError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorkteamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAlgorithmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteAlgorithmError {}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(DeleteAppError::ResourceInUse(err.msg))
                }
                "ResourceNotFound" => {
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
            DeleteAppError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteAppError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCodeRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteCodeRepositoryError {}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(DeleteDomainError::ResourceInUse(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteDomainError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteEndpointError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEndpointConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteEndpointConfigError {}
/// Errors returned by DeleteExperiment
#[derive(Debug, PartialEq)]
pub enum DeleteExperimentError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteExperimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteExperimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteExperimentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteExperimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteExperimentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteExperimentError {}
/// Errors returned by DeleteFlowDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteFlowDefinitionError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteFlowDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFlowDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteFlowDefinitionError::ResourceNotFound(
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
impl fmt::Display for DeleteFlowDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFlowDefinitionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFlowDefinitionError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteModelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteModelPackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteModelPackageError {}
/// Errors returned by DeleteMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteMonitoringScheduleError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteMonitoringScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteMonitoringScheduleError::ResourceNotFound(
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
impl fmt::Display for DeleteMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMonitoringScheduleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteNotebookInstanceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNotebookInstanceLifecycleConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteNotebookInstanceLifecycleConfigError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DeleteTrial
#[derive(Debug, PartialEq)]
pub enum DeleteTrialError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteTrialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteTrialError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTrialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrialError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTrialError {}
/// Errors returned by DeleteTrialComponent
#[derive(Debug, PartialEq)]
pub enum DeleteTrialComponentError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteTrialComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DeleteTrialComponentError::ResourceNotFound(
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
impl fmt::Display for DeleteTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrialComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTrialComponentError {}
/// Errors returned by DeleteUserProfile
#[derive(Debug, PartialEq)]
pub enum DeleteUserProfileError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DeleteUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(DeleteUserProfileError::ResourceInUse(err.msg))
                }
                "ResourceNotFound" => {
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
            DeleteUserProfileError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteUserProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserProfileError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWorkteamError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWorkteamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAlgorithmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeAlgorithmError {}
/// Errors returned by DescribeApp
#[derive(Debug, PartialEq)]
pub enum DescribeAppError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeAppError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAppError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAppError {}
/// Errors returned by DescribeAutoMLJob
#[derive(Debug, PartialEq)]
pub enum DescribeAutoMLJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeAutoMLJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAutoMLJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeAutoMLJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAutoMLJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAutoMLJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAutoMLJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCodeRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeCodeRepositoryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCompilationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCompilationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCompilationJobError {}
/// Errors returned by DescribeDomain
#[derive(Debug, PartialEq)]
pub enum DescribeDomainError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeDomainError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEndpointError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEndpointConfigError {}
/// Errors returned by DescribeExperiment
#[derive(Debug, PartialEq)]
pub enum DescribeExperimentError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeExperimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExperimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeExperimentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeExperimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExperimentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeExperimentError {}
/// Errors returned by DescribeFlowDefinition
#[derive(Debug, PartialEq)]
pub enum DescribeFlowDefinitionError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeFlowDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFlowDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeFlowDefinitionError::ResourceNotFound(
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
impl fmt::Display for DescribeFlowDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFlowDefinitionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFlowDefinitionError {}
/// Errors returned by DescribeHumanTaskUi
#[derive(Debug, PartialEq)]
pub enum DescribeHumanTaskUiError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeHumanTaskUiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHumanTaskUiError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeHumanTaskUiError::ResourceNotFound(
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
impl fmt::Display for DescribeHumanTaskUiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHumanTaskUiError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHumanTaskUiError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHyperParameterTuningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHyperParameterTuningJobError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeHyperParameterTuningJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLabelingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLabelingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLabelingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeModelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeModelPackageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeModelPackageError {}
/// Errors returned by DescribeMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeMonitoringScheduleError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeMonitoringScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeMonitoringScheduleError::ResourceNotFound(
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
impl fmt::Display for DescribeMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMonitoringScheduleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeNotebookInstanceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeNotebookInstanceLifecycleConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeNotebookInstanceLifecycleConfigError {}
/// Errors returned by DescribeProcessingJob
#[derive(Debug, PartialEq)]
pub enum DescribeProcessingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeProcessingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProcessingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeProcessingJobError::ResourceNotFound(
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
impl fmt::Display for DescribeProcessingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProcessingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProcessingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSubscribedWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeSubscribedWorkteamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTrainingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrainingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTrainingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTransformJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTransformJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTransformJobError {}
/// Errors returned by DescribeTrial
#[derive(Debug, PartialEq)]
pub enum DescribeTrialError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeTrialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeTrialError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTrialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrialError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTrialError {}
/// Errors returned by DescribeTrialComponent
#[derive(Debug, PartialEq)]
pub enum DescribeTrialComponentError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeTrialComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeTrialComponentError::ResourceNotFound(
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
impl fmt::Display for DescribeTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrialComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTrialComponentError {}
/// Errors returned by DescribeUserProfile
#[derive(Debug, PartialEq)]
pub enum DescribeUserProfileError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DescribeUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DescribeUserProfileError::ResourceNotFound(
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
impl fmt::Display for DescribeUserProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserProfileError {}
/// Errors returned by DescribeWorkforce
#[derive(Debug, PartialEq)]
pub enum DescribeWorkforceError {}

impl DescribeWorkforceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkforceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkforceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeWorkforceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeWorkteamError {}
/// Errors returned by DisassociateTrialComponent
#[derive(Debug, PartialEq)]
pub enum DisassociateTrialComponentError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl DisassociateTrialComponentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(DisassociateTrialComponentError::ResourceNotFound(
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
impl fmt::Display for DisassociateTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateTrialComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateTrialComponentError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSearchSuggestionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetSearchSuggestionsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAlgorithmsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListAlgorithmsError {}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {}

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAppsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListAppsError {}
/// Errors returned by ListAutoMLJobs
#[derive(Debug, PartialEq)]
pub enum ListAutoMLJobsError {}

impl ListAutoMLJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAutoMLJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAutoMLJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListAutoMLJobsError {}
/// Errors returned by ListCandidatesForAutoMLJob
#[derive(Debug, PartialEq)]
pub enum ListCandidatesForAutoMLJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl ListCandidatesForAutoMLJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListCandidatesForAutoMLJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(ListCandidatesForAutoMLJobError::ResourceNotFound(
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
impl fmt::Display for ListCandidatesForAutoMLJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCandidatesForAutoMLJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCandidatesForAutoMLJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCodeRepositoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListCodeRepositoriesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCompilationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListCompilationJobsError {}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListDomainsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEndpointConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListEndpointConfigsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListEndpointsError {}
/// Errors returned by ListExperiments
#[derive(Debug, PartialEq)]
pub enum ListExperimentsError {}

impl ListExperimentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListExperimentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListExperimentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListExperimentsError {}
/// Errors returned by ListFlowDefinitions
#[derive(Debug, PartialEq)]
pub enum ListFlowDefinitionsError {}

impl ListFlowDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFlowDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFlowDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListFlowDefinitionsError {}
/// Errors returned by ListHumanTaskUis
#[derive(Debug, PartialEq)]
pub enum ListHumanTaskUisError {}

impl ListHumanTaskUisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHumanTaskUisError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHumanTaskUisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListHumanTaskUisError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHyperParameterTuningJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListHyperParameterTuningJobsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLabelingJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListLabelingJobsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLabelingJobsForWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLabelingJobsForWorkteamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLabelingJobsForWorkteamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListModelPackagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListModelPackagesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListModelsError {}
/// Errors returned by ListMonitoringExecutions
#[derive(Debug, PartialEq)]
pub enum ListMonitoringExecutionsError {}

impl ListMonitoringExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMonitoringExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMonitoringExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListMonitoringExecutionsError {}
/// Errors returned by ListMonitoringSchedules
#[derive(Debug, PartialEq)]
pub enum ListMonitoringSchedulesError {}

impl ListMonitoringSchedulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMonitoringSchedulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMonitoringSchedulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListMonitoringSchedulesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNotebookInstanceLifecycleConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListNotebookInstanceLifecycleConfigsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNotebookInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListNotebookInstancesError {}
/// Errors returned by ListProcessingJobs
#[derive(Debug, PartialEq)]
pub enum ListProcessingJobsError {}

impl ListProcessingJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProcessingJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProcessingJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListProcessingJobsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSubscribedWorkteamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSubscribedWorkteamsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTagsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTrainingJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTrainingJobsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTrainingJobsForHyperParameterTuningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrainingJobsForHyperParameterTuningJobError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTrainingJobsForHyperParameterTuningJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTransformJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTransformJobsError {}
/// Errors returned by ListTrialComponents
#[derive(Debug, PartialEq)]
pub enum ListTrialComponentsError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl ListTrialComponentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrialComponentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(ListTrialComponentsError::ResourceNotFound(
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
impl fmt::Display for ListTrialComponentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrialComponentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTrialComponentsError {}
/// Errors returned by ListTrials
#[derive(Debug, PartialEq)]
pub enum ListTrialsError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl ListTrialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(ListTrialsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTrialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrialsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTrialsError {}
/// Errors returned by ListUserProfiles
#[derive(Debug, PartialEq)]
pub enum ListUserProfilesError {}

impl ListUserProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserProfilesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListUserProfilesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorkteamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListWorkteamsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RenderUiTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for RenderUiTemplateError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for SearchError {}
/// Errors returned by StartMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum StartMonitoringScheduleError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StartMonitoringScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StartMonitoringScheduleError::ResourceNotFound(
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
impl fmt::Display for StartMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMonitoringScheduleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartNotebookInstanceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartNotebookInstanceError {}
/// Errors returned by StopAutoMLJob
#[derive(Debug, PartialEq)]
pub enum StopAutoMLJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopAutoMLJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAutoMLJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopAutoMLJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopAutoMLJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopAutoMLJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopAutoMLJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopCompilationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopCompilationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopCompilationJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopHyperParameterTuningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopHyperParameterTuningJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopHyperParameterTuningJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopLabelingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopLabelingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopLabelingJobError {}
/// Errors returned by StopMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum StopMonitoringScheduleError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopMonitoringScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopMonitoringScheduleError::ResourceNotFound(
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
impl fmt::Display for StopMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopMonitoringScheduleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for StopNotebookInstanceError {}
/// Errors returned by StopProcessingJob
#[derive(Debug, PartialEq)]
pub enum StopProcessingJobError {
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl StopProcessingJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopProcessingJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFound" => {
                    return RusotoError::Service(StopProcessingJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopProcessingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopProcessingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopProcessingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopTrainingJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTrainingJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopTrainingJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopTransformJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTransformJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopTransformJobError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCodeRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateCodeRepositoryError {}
/// Errors returned by UpdateDomain
#[derive(Debug, PartialEq)]
pub enum UpdateDomainError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(UpdateDomainError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(UpdateDomainError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UpdateDomainError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDomainError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEndpointError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEndpointWeightsAndCapacitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointWeightsAndCapacitiesError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateEndpointWeightsAndCapacitiesError {}
/// Errors returned by UpdateExperiment
#[derive(Debug, PartialEq)]
pub enum UpdateExperimentError {
    /// <p>There was a conflict when you attempted to modify an experiment, trial, or trial component.</p>
    Conflict(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateExperimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateExperimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateExperimentError::Conflict(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UpdateExperimentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateExperimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateExperimentError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateExperimentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateExperimentError {}
/// Errors returned by UpdateMonitoringSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateMonitoringScheduleError {
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateMonitoringScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMonitoringScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        UpdateMonitoringScheduleError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UpdateMonitoringScheduleError::ResourceNotFound(
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
impl fmt::Display for UpdateMonitoringScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMonitoringScheduleError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateMonitoringScheduleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMonitoringScheduleError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNotebookInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNotebookInstanceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNotebookInstanceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNotebookInstanceLifecycleConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNotebookInstanceLifecycleConfigError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateNotebookInstanceLifecycleConfigError {}
/// Errors returned by UpdateTrial
#[derive(Debug, PartialEq)]
pub enum UpdateTrialError {
    /// <p>There was a conflict when you attempted to modify an experiment, trial, or trial component.</p>
    Conflict(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateTrialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTrialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateTrialError::Conflict(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UpdateTrialError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTrialError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrialError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateTrialError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTrialError {}
/// Errors returned by UpdateTrialComponent
#[derive(Debug, PartialEq)]
pub enum UpdateTrialComponentError {
    /// <p>There was a conflict when you attempted to modify an experiment, trial, or trial component.</p>
    Conflict(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateTrialComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTrialComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateTrialComponentError::Conflict(err.msg))
                }
                "ResourceNotFound" => {
                    return RusotoError::Service(UpdateTrialComponentError::ResourceNotFound(
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
impl fmt::Display for UpdateTrialComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrialComponentError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateTrialComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTrialComponentError {}
/// Errors returned by UpdateUserProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserProfileError {
    /// <p>Resource being accessed is in use.</p>
    ResourceInUse(String),
    /// <p> You have exceeded an Amazon SageMaker resource limit. For example, you might have too many training jobs created. </p>
    ResourceLimitExceeded(String),
    /// <p>Resource being access is not found.</p>
    ResourceNotFound(String),
}

impl UpdateUserProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserProfileError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUse" => {
                    return RusotoError::Service(UpdateUserProfileError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(UpdateUserProfileError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFound" => {
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
            UpdateUserProfileError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateUserProfileError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateUserProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserProfileError {}
/// Errors returned by UpdateWorkforce
#[derive(Debug, PartialEq)]
pub enum UpdateWorkforceError {}

impl UpdateWorkforceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWorkforceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWorkforceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateWorkforceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWorkteamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWorkteamError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWorkteamError {}
/// Trait representing the capabilities of the SageMaker API. SageMaker clients implement this trait.
#[async_trait]
pub trait SageMaker {
    /// <p><p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints.</p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <note> <p>Tags that you add to a hyperparameter tuning job by calling this API are also added to any training jobs that the hyperparameter tuning job launches after you call this API, but not to training jobs that the hyperparameter tuning job launched before you called this API. To make sure that the tags associated with a hyperparameter tuning job are also added to all training jobs that the hyperparameter tuning job launches, add the tags when you first create the tuning job by specifying them in the <code>Tags</code> parameter of <a>CreateHyperParameterTuningJob</a> </p> </note></p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Associates a trial component with a trial. A trial component can be associated with multiple trials. To disassociate a trial component from a trial, call the <a>DisassociateTrialComponent</a> API.</p>
    async fn associate_trial_component(
        &self,
        input: AssociateTrialComponentRequest,
    ) -> Result<AssociateTrialComponentResponse, RusotoError<AssociateTrialComponentError>>;

    /// <p>Create a machine learning algorithm that you can use in Amazon SageMaker and list in the AWS Marketplace.</p>
    async fn create_algorithm(
        &self,
        input: CreateAlgorithmInput,
    ) -> Result<CreateAlgorithmOutput, RusotoError<CreateAlgorithmError>>;

    /// <p>Creates a running App for the specified UserProfile. Supported Apps are JupyterServer and KernelGateway. This operation is automatically invoked by Amazon SageMaker Amazon SageMaker Studio (Studio) upon access to the associated Studio Domain, and when new kernel configurations are selected by the user. A user may have multiple Apps active simultaneously. Apps will automatically terminate and be deleted when stopped from within Studio, or when the DeleteApp API is manually called. UserProfiles are limited to 5 concurrently running Apps at a time.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>>;

    /// <p>Creates an AutoPilot job.</p>
    async fn create_auto_ml_job(
        &self,
        input: CreateAutoMLJobRequest,
    ) -> Result<CreateAutoMLJobResponse, RusotoError<CreateAutoMLJobError>>;

    /// <p>Creates a Git repository as a resource in your Amazon SageMaker account. You can associate the repository with notebook instances so that you can use Git source control for the notebooks you create. The Git repository is a resource in your Amazon SageMaker account, so it can be associated with more than one notebook instance, and it persists independently from the lifecycle of any notebook instances it is associated with.</p> <p>The repository can be hosted either in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository.</p>
    async fn create_code_repository(
        &self,
        input: CreateCodeRepositoryInput,
    ) -> Result<CreateCodeRepositoryOutput, RusotoError<CreateCodeRepositoryError>>;

    /// <p>Starts a model compilation job. After the model has been compiled, Amazon SageMaker saves the resulting model artifacts to an Amazon Simple Storage Service (Amazon S3) bucket that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts with AWS IoT Greengrass. In that case, deploy them as an ML resource.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p>A name for the compilation job</p> </li> <li> <p> Information about the input model artifacts </p> </li> <li> <p>The output location for the compiled model and the device (target) that the model runs on </p> </li> <li> <p> <code>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker assumes to perform the model compilation job</code> </p> </li> </ul> <p>You can also provide a <code>Tag</code> to track the model compilation job's resource use and costs. The response body contains the <code>CompilationJobArn</code> for the compiled job.</p> <p>To stop a model compilation job, use <a>StopCompilationJob</a>. To get information about a particular model compilation job, use <a>DescribeCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    async fn create_compilation_job(
        &self,
        input: CreateCompilationJobRequest,
    ) -> Result<CreateCompilationJobResponse, RusotoError<CreateCompilationJobError>>;

    /// <p>Creates a Domain for Amazon SageMaker Amazon SageMaker Studio (Studio), which can be accessed by end-users in a web browser. A Domain has an associated directory, list of authorized users, and a variety of security, application, policies, and Amazon Virtual Private Cloud configurations. An AWS account is limited to one Domain, per region. Users within a domain can share notebook files and other artifacts with each other. When a Domain is created, an Amazon Elastic File System (EFS) is also created for use by all of the users within the Domain. Each user receives a private home directory within the EFS for notebooks, Git repositories, and data files. </p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResponse, RusotoError<CreateDomainError>>;

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> <p> You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> Result<CreateEndpointOutput, RusotoError<CreateEndpointError>>;

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    async fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> Result<CreateEndpointConfigOutput, RusotoError<CreateEndpointConfigError>>;

    /// <p>Creates an Amazon SageMaker <i>experiment</i>. An experiment is a collection of <i>trials</i> that are observed, compared and evaluated as a group. A trial is a set of steps, called <i>trial components</i>, that produce a machine learning model.</p> <p>The goal of an experiment is to determine the components that produce the best model. Multiple trials are performed, each one isolating and measuring the impact of a change to one or more inputs, while keeping the remaining inputs constant.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to experiments, trials, trial components and then use the <a>Search</a> API to search for the tags.</p> <p>To add a description to an experiment, specify the optional <code>Description</code> parameter. To add a description later, or to change the description, call the <a>UpdateExperiment</a> API.</p> <p>To get a list of all your experiments, call the <a>ListExperiments</a> API. To view an experiment's properties, call the <a>DescribeExperiment</a> API. To get a list of all the trials associated with an experiment, call the <a>ListTrials</a> API. To create a trial call the <a>CreateTrial</a> API.</p>
    async fn create_experiment(
        &self,
        input: CreateExperimentRequest,
    ) -> Result<CreateExperimentResponse, RusotoError<CreateExperimentError>>;

    /// <p>Creates a flow definition.</p>
    async fn create_flow_definition(
        &self,
        input: CreateFlowDefinitionRequest,
    ) -> Result<CreateFlowDefinitionResponse, RusotoError<CreateFlowDefinitionError>>;

    /// <p>Defines the settings you will use for the human review workflow user interface. Reviewers will see a three-panel interface with an instruction area, the item to review, and an input area.</p>
    async fn create_human_task_ui(
        &self,
        input: CreateHumanTaskUiRequest,
    ) -> Result<CreateHumanTaskUiResponse, RusotoError<CreateHumanTaskUiError>>;

    /// <p>Starts a hyperparameter tuning job. A hyperparameter tuning job finds the best version of a model by running many training jobs on your dataset using the algorithm you choose and values for hyperparameters within ranges that you specify. It then chooses the hyperparameter values that result in a model that performs the best, as measured by an objective metric that you choose.</p>
    async fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> Result<
        CreateHyperParameterTuningJobResponse,
        RusotoError<CreateHyperParameterTuningJobError>,
    >;

    /// <p>Creates a job that uses workers to label the data objects in your input dataset. You can use the labeled data to train machine learning models.</p> <p>You can select your workforce from one of three providers:</p> <ul> <li> <p>A private workforce that you create. It can include employees, contractors, and outside experts. Use a private workforce when want the data to stay within your organization or when a specific set of skills is required.</p> </li> <li> <p>One or more vendors that you select from the AWS Marketplace. Vendors provide expertise in specific areas. </p> </li> <li> <p>The Amazon Mechanical Turk workforce. This is the largest workforce, but it should only be used for public data or data that has been stripped of any personally identifiable information.</p> </li> </ul> <p>You can also use <i>automated data labeling</i> to reduce the number of data objects that need to be labeled by a human. Automated data labeling uses <i>active learning</i> to determine if a data object can be labeled by machine or if it needs to be sent to a human worker. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-automated-labeling.html">Using Automated Data Labeling</a>.</p> <p>The data objects to be labeled are contained in an Amazon S3 bucket. You create a <i>manifest file</i> that describes the location of each object. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-data.html">Using Input and Output Data</a>.</p> <p>The output can be used as the manifest file for another labeling job or as training data for your machine learning models.</p>
    async fn create_labeling_job(
        &self,
        input: CreateLabelingJobRequest,
    ) -> Result<CreateLabelingJobResponse, RusotoError<CreateLabelingJobError>>;

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe a primary container. For the primary container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model for predictions.</p> <p>Use this API to create a model if you want to use Amazon SageMaker hosting services or run a batch transform job.</p> <p>To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>To run a batch transform using your model, you start a job with the <code>CreateTransformJob</code> API. Amazon SageMaker uses your model and your dataset to get inferences which are then saved to a specified S3 location.</p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter.</p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances or for batch transform jobs. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    async fn create_model(
        &self,
        input: CreateModelInput,
    ) -> Result<CreateModelOutput, RusotoError<CreateModelError>>;

    /// <p>Creates a model package that you can use to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p> <p>To create a model package by specifying a Docker container that contains your inference code and the Amazon S3 location of your model artifacts, provide values for <code>InferenceSpecification</code>. To create a model from an algorithm resource that you created or subscribed to in AWS Marketplace, provide a value for <code>SourceAlgorithmSpecification</code>.</p>
    async fn create_model_package(
        &self,
        input: CreateModelPackageInput,
    ) -> Result<CreateModelPackageOutput, RusotoError<CreateModelPackageError>>;

    /// <p>Creates a schedule that regularly starts Amazon SageMaker Processing Jobs to monitor the data captured for an Amazon SageMaker Endoint.</p>
    async fn create_monitoring_schedule(
        &self,
        input: CreateMonitoringScheduleRequest,
    ) -> Result<CreateMonitoringScheduleResponse, RusotoError<CreateMonitoringScheduleError>>;

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN). You can't change the name of a notebook instance after you create it.</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    async fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> Result<CreateNotebookInstanceOutput, RusotoError<CreateNotebookInstanceError>>;

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    async fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        CreateNotebookInstanceLifecycleConfigOutput,
        RusotoError<CreateNotebookInstanceLifecycleConfigError>,
    >;

    /// <p>Creates a URL for a specified UserProfile in a Domain. When accessed in a web browser, the user will be automatically signed in to Amazon SageMaker Amazon SageMaker Studio (Studio), and granted access to all of the Apps and files associated with that Amazon Elastic File System (EFS). This operation can only be called when AuthMode equals IAM. </p>
    async fn create_presigned_domain_url(
        &self,
        input: CreatePresignedDomainUrlRequest,
    ) -> Result<CreatePresignedDomainUrlResponse, RusotoError<CreatePresignedDomainUrlError>>;

    /// <p><p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page.</p> <p>IAM authorization policies for this API are also enforced for every HTTP request and WebSocket frame that attempts to connect to the notebook instance.For example, you can restrict access to this API and to the URL that it returns to a list of IP addresses that you specify. Use the <code>NotIpAddress</code> condition operator and the <code>aws:SourceIP</code> condition context key to specify the list of IP addresses that you want to have access to the notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/security_iam_id-based-policy-examples.html#nbi-ip-filter">Limit Access to a Notebook Instance by IP Address</a>.</p> <note> <p>The URL that you get from a call to is valid only for 5 minutes. If you try to use the URL after the 5-minute limit expires, you are directed to the AWS console sign-in page.</p> </note></p>
    async fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> Result<
        CreatePresignedNotebookInstanceUrlOutput,
        RusotoError<CreatePresignedNotebookInstanceUrlError>,
    >;

    /// <p>Creates a processing job.</p>
    async fn create_processing_job(
        &self,
        input: CreateProcessingJobRequest,
    ) -> Result<CreateProcessingJobResponse, RusotoError<CreateProcessingJobError>>;

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a machine learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to enable the estimation of model parameters during training. Hyperparameters can be tuned to optimize this learning process. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3, EFS, or FSx location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 bucket where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>EnableManagedSpotTraining</code> - Optimize the cost of training machine learning models by up to 80% by using Amazon EC2 Spot instances. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-managed-spot-training.html">Managed Spot Training</a>. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - To help cap training costs, use <code>MaxRuntimeInSeconds</code> to set a time limit for training. Use <code>MaxWaitTimeInSeconds</code> to specify how long you are willing to wait for a managed spot training job to complete. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    async fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> Result<CreateTrainingJobResponse, RusotoError<CreateTrainingJobError>>;

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model in the same AWS Region and AWS account. For information on creating a model, see <a>CreateModel</a>.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p>For more information about how batch transformation works, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">Batch Transform</a>.</p>
    async fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> Result<CreateTransformJobResponse, RusotoError<CreateTransformJobError>>;

    /// <p>Creates an Amazon SageMaker <i>trial</i>. A trial is a set of steps called <i>trial components</i> that produce a machine learning model. A trial is part of a single Amazon SageMaker <i>experiment</i>.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to a trial and then use the <a>Search</a> API to search for the tags.</p> <p>To get a list of all your trials, call the <a>ListTrials</a> API. To view a trial's properties, call the <a>DescribeTrial</a> API. To create a trial component, call the <a>CreateTrialComponent</a> API.</p>
    async fn create_trial(
        &self,
        input: CreateTrialRequest,
    ) -> Result<CreateTrialResponse, RusotoError<CreateTrialError>>;

    /// <p><p>Creates a <i>trial component</i>, which is a stage of a machine learning <i>trial</i>. A trial is composed of one or more trial components. A trial component can be used in multiple trials.</p> <p>Trial components include pre-processing jobs, training jobs, and batch transform jobs.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to a trial component and then use the <a>Search</a> API to search for the tags.</p> <note> <p> <code>CreateTrialComponent</code> can only be invoked from within an Amazon SageMaker managed environment. This includes Amazon SageMaker training jobs, processing jobs, transform jobs, and Amazon SageMaker notebooks. A call to <code>CreateTrialComponent</code> from outside one of these environments results in an error.</p> </note></p>
    async fn create_trial_component(
        &self,
        input: CreateTrialComponentRequest,
    ) -> Result<CreateTrialComponentResponse, RusotoError<CreateTrialComponentError>>;

    /// <p>Creates a new user profile. A user profile represents a single user within a Domain, and is the main way to reference a "person" for the purposes of sharing, reporting and other user-oriented features. This entity is created during on-boarding. If an administrator invites a person by email or imports them from SSO, a new UserProfile is automatically created. This entity is the primary holder of settings for an individual user and has a reference to the user's private Amazon Elastic File System (EFS) home directory. </p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResponse, RusotoError<CreateUserProfileError>>;

    /// <p>Creates a new work team for labeling your data. A work team is defined by one or more Amazon Cognito user pools. You must first create the user pools before you can create a work team.</p> <p>You cannot create more than 25 work teams in an account and region.</p>
    async fn create_workteam(
        &self,
        input: CreateWorkteamRequest,
    ) -> Result<CreateWorkteamResponse, RusotoError<CreateWorkteamError>>;

    /// <p>Removes the specified algorithm from your account.</p>
    async fn delete_algorithm(
        &self,
        input: DeleteAlgorithmInput,
    ) -> Result<(), RusotoError<DeleteAlgorithmError>>;

    /// <p>Used to stop and delete an app.</p>
    async fn delete_app(&self, input: DeleteAppRequest) -> Result<(), RusotoError<DeleteAppError>>;

    /// <p>Deletes the specified Git repository from your account.</p>
    async fn delete_code_repository(
        &self,
        input: DeleteCodeRepositoryInput,
    ) -> Result<(), RusotoError<DeleteCodeRepositoryError>>;

    /// <p>Used to delete a domain. If you on-boarded with IAM mode, you will need to delete your domain to on-board again using SSO. Use with caution. All of the members of the domain will lose access to their EFS volume, including data, notebooks, and other artifacts. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<(), RusotoError<DeleteDomainError>>;

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointInput,
    ) -> Result<(), RusotoError<DeleteEndpointError>>;

    /// <p>Deletes an endpoint configuration. The <code>DeleteEndpointConfig</code> API deletes only the specified configuration. It does not delete endpoints created using the configuration. </p>
    async fn delete_endpoint_config(
        &self,
        input: DeleteEndpointConfigInput,
    ) -> Result<(), RusotoError<DeleteEndpointConfigError>>;

    /// <p>Deletes an Amazon SageMaker experiment. All trials associated with the experiment must be deleted first. Use the <a>ListTrials</a> API to get a list of the trials associated with the experiment.</p>
    async fn delete_experiment(
        &self,
        input: DeleteExperimentRequest,
    ) -> Result<DeleteExperimentResponse, RusotoError<DeleteExperimentError>>;

    /// <p>Deletes the specified flow definition.</p>
    async fn delete_flow_definition(
        &self,
        input: DeleteFlowDefinitionRequest,
    ) -> Result<DeleteFlowDefinitionResponse, RusotoError<DeleteFlowDefinitionError>>;

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    async fn delete_model(
        &self,
        input: DeleteModelInput,
    ) -> Result<(), RusotoError<DeleteModelError>>;

    /// <p>Deletes a model package.</p> <p>A model package is used to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p>
    async fn delete_model_package(
        &self,
        input: DeleteModelPackageInput,
    ) -> Result<(), RusotoError<DeleteModelPackageError>>;

    /// <p>Deletes a monitoring schedule. Also stops the schedule had not already been stopped. This does not delete the job execution history of the monitoring schedule. </p>
    async fn delete_monitoring_schedule(
        &self,
        input: DeleteMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<DeleteMonitoringScheduleError>>;

    /// <p><p> Deletes an Amazon SageMaker notebook instance. Before you can delete a notebook instance, you must call the <code>StopNotebookInstance</code> API. </p> <important> <p>When you delete a notebook instance, you lose all of your data. Amazon SageMaker removes the ML compute instance, and deletes the ML storage volume and the network interface associated with the notebook instance. </p> </important></p>
    async fn delete_notebook_instance(
        &self,
        input: DeleteNotebookInstanceInput,
    ) -> Result<(), RusotoError<DeleteNotebookInstanceError>>;

    /// <p>Deletes a notebook instance lifecycle configuration.</p>
    async fn delete_notebook_instance_lifecycle_config(
        &self,
        input: DeleteNotebookInstanceLifecycleConfigInput,
    ) -> Result<(), RusotoError<DeleteNotebookInstanceLifecycleConfigError>>;

    /// <p><p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource&#39;s tags, use the <code>ListTags</code> API. </p> <note> <p>When you call this API to delete tags from a hyperparameter tuning job, the deleted tags are not removed from training jobs that the hyperparameter tuning job launched before you called this API.</p> </note></p>
    async fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> Result<DeleteTagsOutput, RusotoError<DeleteTagsError>>;

    /// <p>Deletes the specified trial. All trial components that make up the trial must be deleted first. Use the <a>DescribeTrialComponent</a> API to get the list of trial components.</p>
    async fn delete_trial(
        &self,
        input: DeleteTrialRequest,
    ) -> Result<DeleteTrialResponse, RusotoError<DeleteTrialError>>;

    /// <p>Deletes the specified trial component. A trial component must be disassociated from all trials before the trial component can be deleted. To disassociate a trial component from a trial, call the <a>DisassociateTrialComponent</a> API.</p>
    async fn delete_trial_component(
        &self,
        input: DeleteTrialComponentRequest,
    ) -> Result<DeleteTrialComponentResponse, RusotoError<DeleteTrialComponentError>>;

    /// <p>Deletes a user profile.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<(), RusotoError<DeleteUserProfileError>>;

    /// <p>Deletes an existing work team. This operation can't be undone.</p>
    async fn delete_workteam(
        &self,
        input: DeleteWorkteamRequest,
    ) -> Result<DeleteWorkteamResponse, RusotoError<DeleteWorkteamError>>;

    /// <p>Returns a description of the specified algorithm that is in your account.</p>
    async fn describe_algorithm(
        &self,
        input: DescribeAlgorithmInput,
    ) -> Result<DescribeAlgorithmOutput, RusotoError<DescribeAlgorithmError>>;

    /// <p>Describes the app.</p>
    async fn describe_app(
        &self,
        input: DescribeAppRequest,
    ) -> Result<DescribeAppResponse, RusotoError<DescribeAppError>>;

    /// <p>Returns information about an Amazon SageMaker job.</p>
    async fn describe_auto_ml_job(
        &self,
        input: DescribeAutoMLJobRequest,
    ) -> Result<DescribeAutoMLJobResponse, RusotoError<DescribeAutoMLJobError>>;

    /// <p>Gets details about the specified Git repository.</p>
    async fn describe_code_repository(
        &self,
        input: DescribeCodeRepositoryInput,
    ) -> Result<DescribeCodeRepositoryOutput, RusotoError<DescribeCodeRepositoryError>>;

    /// <p>Returns information about a model compilation job.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    async fn describe_compilation_job(
        &self,
        input: DescribeCompilationJobRequest,
    ) -> Result<DescribeCompilationJobResponse, RusotoError<DescribeCompilationJobError>>;

    /// <p>The desciption of the domain.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> Result<DescribeDomainResponse, RusotoError<DescribeDomainError>>;

    /// <p>Returns the description of an endpoint.</p>
    async fn describe_endpoint(
        &self,
        input: DescribeEndpointInput,
    ) -> Result<DescribeEndpointOutput, RusotoError<DescribeEndpointError>>;

    /// <p>Returns the description of an endpoint configuration created using the <code>CreateEndpointConfig</code> API.</p>
    async fn describe_endpoint_config(
        &self,
        input: DescribeEndpointConfigInput,
    ) -> Result<DescribeEndpointConfigOutput, RusotoError<DescribeEndpointConfigError>>;

    /// <p>Provides a list of an experiment's properties.</p>
    async fn describe_experiment(
        &self,
        input: DescribeExperimentRequest,
    ) -> Result<DescribeExperimentResponse, RusotoError<DescribeExperimentError>>;

    /// <p>Returns information about the specified flow definition.</p>
    async fn describe_flow_definition(
        &self,
        input: DescribeFlowDefinitionRequest,
    ) -> Result<DescribeFlowDefinitionResponse, RusotoError<DescribeFlowDefinitionError>>;

    /// <p>Returns information about the requested human task user interface.</p>
    async fn describe_human_task_ui(
        &self,
        input: DescribeHumanTaskUiRequest,
    ) -> Result<DescribeHumanTaskUiResponse, RusotoError<DescribeHumanTaskUiError>>;

    /// <p>Gets a description of a hyperparameter tuning job.</p>
    async fn describe_hyper_parameter_tuning_job(
        &self,
        input: DescribeHyperParameterTuningJobRequest,
    ) -> Result<
        DescribeHyperParameterTuningJobResponse,
        RusotoError<DescribeHyperParameterTuningJobError>,
    >;

    /// <p>Gets information about a labeling job.</p>
    async fn describe_labeling_job(
        &self,
        input: DescribeLabelingJobRequest,
    ) -> Result<DescribeLabelingJobResponse, RusotoError<DescribeLabelingJobError>>;

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    async fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> Result<DescribeModelOutput, RusotoError<DescribeModelError>>;

    /// <p>Returns a description of the specified model package, which is used to create Amazon SageMaker models or list them on AWS Marketplace.</p> <p>To create models in Amazon SageMaker, buyers can subscribe to model packages listed on AWS Marketplace.</p>
    async fn describe_model_package(
        &self,
        input: DescribeModelPackageInput,
    ) -> Result<DescribeModelPackageOutput, RusotoError<DescribeModelPackageError>>;

    /// <p>Describes the schedule for a monitoring job.</p>
    async fn describe_monitoring_schedule(
        &self,
        input: DescribeMonitoringScheduleRequest,
    ) -> Result<DescribeMonitoringScheduleResponse, RusotoError<DescribeMonitoringScheduleError>>;

    /// <p>Returns information about a notebook instance.</p>
    async fn describe_notebook_instance(
        &self,
        input: DescribeNotebookInstanceInput,
    ) -> Result<DescribeNotebookInstanceOutput, RusotoError<DescribeNotebookInstanceError>>;

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    async fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        DescribeNotebookInstanceLifecycleConfigOutput,
        RusotoError<DescribeNotebookInstanceLifecycleConfigError>,
    >;

    /// <p>Returns a description of a processing job.</p>
    async fn describe_processing_job(
        &self,
        input: DescribeProcessingJobRequest,
    ) -> Result<DescribeProcessingJobResponse, RusotoError<DescribeProcessingJobError>>;

    /// <p>Gets information about a work team provided by a vendor. It returns details about the subscription with a vendor in the AWS Marketplace.</p>
    async fn describe_subscribed_workteam(
        &self,
        input: DescribeSubscribedWorkteamRequest,
    ) -> Result<DescribeSubscribedWorkteamResponse, RusotoError<DescribeSubscribedWorkteamError>>;

    /// <p>Returns information about a training job.</p>
    async fn describe_training_job(
        &self,
        input: DescribeTrainingJobRequest,
    ) -> Result<DescribeTrainingJobResponse, RusotoError<DescribeTrainingJobError>>;

    /// <p>Returns information about a transform job.</p>
    async fn describe_transform_job(
        &self,
        input: DescribeTransformJobRequest,
    ) -> Result<DescribeTransformJobResponse, RusotoError<DescribeTransformJobError>>;

    /// <p>Provides a list of a trial's properties.</p>
    async fn describe_trial(
        &self,
        input: DescribeTrialRequest,
    ) -> Result<DescribeTrialResponse, RusotoError<DescribeTrialError>>;

    /// <p>Provides a list of a trials component's properties.</p>
    async fn describe_trial_component(
        &self,
        input: DescribeTrialComponentRequest,
    ) -> Result<DescribeTrialComponentResponse, RusotoError<DescribeTrialComponentError>>;

    /// <p>Describes the user profile.</p>
    async fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> Result<DescribeUserProfileResponse, RusotoError<DescribeUserProfileError>>;

    /// <p><p>Lists private workforce information, including workforce name, Amazon Resource Name (ARN), and, if applicable, allowed IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>). Allowable IP address ranges are the IP addresses that workers can use to access tasks. </p> <important> <p>This operation applies only to private workforces.</p> </important></p>
    async fn describe_workforce(
        &self,
        input: DescribeWorkforceRequest,
    ) -> Result<DescribeWorkforceResponse, RusotoError<DescribeWorkforceError>>;

    /// <p>Gets information about a specific work team. You can see information such as the create date, the last updated date, membership information, and the work team's Amazon Resource Name (ARN).</p>
    async fn describe_workteam(
        &self,
        input: DescribeWorkteamRequest,
    ) -> Result<DescribeWorkteamResponse, RusotoError<DescribeWorkteamError>>;

    /// <p>Disassociates a trial component from a trial. This doesn't effect other trials the component is associated with. Before you can delete a component, you must disassociate the component from all trials it is associated with. To associate a trial component with a trial, call the <a>AssociateTrialComponent</a> API.</p>
    async fn disassociate_trial_component(
        &self,
        input: DisassociateTrialComponentRequest,
    ) -> Result<DisassociateTrialComponentResponse, RusotoError<DisassociateTrialComponentError>>;

    /// <p>An auto-complete API for the search functionality in the Amazon SageMaker console. It returns suggestions of possible matches for the property name to use in <code>Search</code> queries. Provides suggestions for <code>HyperParameters</code>, <code>Tags</code>, and <code>Metrics</code>.</p>
    async fn get_search_suggestions(
        &self,
        input: GetSearchSuggestionsRequest,
    ) -> Result<GetSearchSuggestionsResponse, RusotoError<GetSearchSuggestionsError>>;

    /// <p>Lists the machine learning algorithms that have been created.</p>
    async fn list_algorithms(
        &self,
        input: ListAlgorithmsInput,
    ) -> Result<ListAlgorithmsOutput, RusotoError<ListAlgorithmsError>>;

    /// <p>Lists apps.</p>
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResponse, RusotoError<ListAppsError>>;

    /// <p>Request a list of jobs.</p>
    async fn list_auto_ml_jobs(
        &self,
        input: ListAutoMLJobsRequest,
    ) -> Result<ListAutoMLJobsResponse, RusotoError<ListAutoMLJobsError>>;

    /// <p>List the Candidates created for the job.</p>
    async fn list_candidates_for_auto_ml_job(
        &self,
        input: ListCandidatesForAutoMLJobRequest,
    ) -> Result<ListCandidatesForAutoMLJobResponse, RusotoError<ListCandidatesForAutoMLJobError>>;

    /// <p>Gets a list of the Git repositories in your account.</p>
    async fn list_code_repositories(
        &self,
        input: ListCodeRepositoriesInput,
    ) -> Result<ListCodeRepositoriesOutput, RusotoError<ListCodeRepositoriesError>>;

    /// <p>Lists model compilation jobs that satisfy various filters.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about a particular model compilation job you have created, use <a>DescribeCompilationJob</a>.</p>
    async fn list_compilation_jobs(
        &self,
        input: ListCompilationJobsRequest,
    ) -> Result<ListCompilationJobsResponse, RusotoError<ListCompilationJobsError>>;

    /// <p>Lists the domains.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>>;

    /// <p>Lists endpoint configurations.</p>
    async fn list_endpoint_configs(
        &self,
        input: ListEndpointConfigsInput,
    ) -> Result<ListEndpointConfigsOutput, RusotoError<ListEndpointConfigsError>>;

    /// <p>Lists endpoints.</p>
    async fn list_endpoints(
        &self,
        input: ListEndpointsInput,
    ) -> Result<ListEndpointsOutput, RusotoError<ListEndpointsError>>;

    /// <p>Lists all the experiments in your account. The list can be filtered to show only experiments that were created in a specific time range. The list can be sorted by experiment name or creation time.</p>
    async fn list_experiments(
        &self,
        input: ListExperimentsRequest,
    ) -> Result<ListExperimentsResponse, RusotoError<ListExperimentsError>>;

    /// <p>Returns information about the flow definitions in your account.</p>
    async fn list_flow_definitions(
        &self,
        input: ListFlowDefinitionsRequest,
    ) -> Result<ListFlowDefinitionsResponse, RusotoError<ListFlowDefinitionsError>>;

    /// <p>Returns information about the human task user interfaces in your account.</p>
    async fn list_human_task_uis(
        &self,
        input: ListHumanTaskUisRequest,
    ) -> Result<ListHumanTaskUisResponse, RusotoError<ListHumanTaskUisError>>;

    /// <p>Gets a list of <a>HyperParameterTuningJobSummary</a> objects that describe the hyperparameter tuning jobs launched in your account.</p>
    async fn list_hyper_parameter_tuning_jobs(
        &self,
        input: ListHyperParameterTuningJobsRequest,
    ) -> Result<ListHyperParameterTuningJobsResponse, RusotoError<ListHyperParameterTuningJobsError>>;

    /// <p>Gets a list of labeling jobs.</p>
    async fn list_labeling_jobs(
        &self,
        input: ListLabelingJobsRequest,
    ) -> Result<ListLabelingJobsResponse, RusotoError<ListLabelingJobsError>>;

    /// <p>Gets a list of labeling jobs assigned to a specified work team.</p>
    async fn list_labeling_jobs_for_workteam(
        &self,
        input: ListLabelingJobsForWorkteamRequest,
    ) -> Result<ListLabelingJobsForWorkteamResponse, RusotoError<ListLabelingJobsForWorkteamError>>;

    /// <p>Lists the model packages that have been created.</p>
    async fn list_model_packages(
        &self,
        input: ListModelPackagesInput,
    ) -> Result<ListModelPackagesOutput, RusotoError<ListModelPackagesError>>;

    /// <p>Lists models created with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
    async fn list_models(
        &self,
        input: ListModelsInput,
    ) -> Result<ListModelsOutput, RusotoError<ListModelsError>>;

    /// <p>Returns list of all monitoring job executions.</p>
    async fn list_monitoring_executions(
        &self,
        input: ListMonitoringExecutionsRequest,
    ) -> Result<ListMonitoringExecutionsResponse, RusotoError<ListMonitoringExecutionsError>>;

    /// <p>Returns list of all monitoring schedules.</p>
    async fn list_monitoring_schedules(
        &self,
        input: ListMonitoringSchedulesRequest,
    ) -> Result<ListMonitoringSchedulesResponse, RusotoError<ListMonitoringSchedulesError>>;

    /// <p>Lists notebook instance lifestyle configurations created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    async fn list_notebook_instance_lifecycle_configs(
        &self,
        input: ListNotebookInstanceLifecycleConfigsInput,
    ) -> Result<
        ListNotebookInstanceLifecycleConfigsOutput,
        RusotoError<ListNotebookInstanceLifecycleConfigsError>,
    >;

    /// <p>Returns a list of the Amazon SageMaker notebook instances in the requester's account in an AWS Region. </p>
    async fn list_notebook_instances(
        &self,
        input: ListNotebookInstancesInput,
    ) -> Result<ListNotebookInstancesOutput, RusotoError<ListNotebookInstancesError>>;

    /// <p>Lists processing jobs that satisfy various filters.</p>
    async fn list_processing_jobs(
        &self,
        input: ListProcessingJobsRequest,
    ) -> Result<ListProcessingJobsResponse, RusotoError<ListProcessingJobsError>>;

    /// <p>Gets a list of the work teams that you are subscribed to in the AWS Marketplace. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    async fn list_subscribed_workteams(
        &self,
        input: ListSubscribedWorkteamsRequest,
    ) -> Result<ListSubscribedWorkteamsResponse, RusotoError<ListSubscribedWorkteamsError>>;

    /// <p>Returns the tags for the specified Amazon SageMaker resource.</p>
    async fn list_tags(
        &self,
        input: ListTagsInput,
    ) -> Result<ListTagsOutput, RusotoError<ListTagsError>>;

    /// <p>Lists training jobs.</p>
    async fn list_training_jobs(
        &self,
        input: ListTrainingJobsRequest,
    ) -> Result<ListTrainingJobsResponse, RusotoError<ListTrainingJobsError>>;

    /// <p>Gets a list of <a>TrainingJobSummary</a> objects that describe the training jobs that a hyperparameter tuning job launched.</p>
    async fn list_training_jobs_for_hyper_parameter_tuning_job(
        &self,
        input: ListTrainingJobsForHyperParameterTuningJobRequest,
    ) -> Result<
        ListTrainingJobsForHyperParameterTuningJobResponse,
        RusotoError<ListTrainingJobsForHyperParameterTuningJobError>,
    >;

    /// <p>Lists transform jobs.</p>
    async fn list_transform_jobs(
        &self,
        input: ListTransformJobsRequest,
    ) -> Result<ListTransformJobsResponse, RusotoError<ListTransformJobsError>>;

    /// <p><p>Lists the trial components in your account. You can sort the list by trial component name or creation time. You can filter the list to show only components that were created in a specific time range. You can also filter on one of the following:</p> <ul> <li> <p> <code>ExperimentName</code> </p> </li> <li> <p> <code>SourceArn</code> </p> </li> <li> <p> <code>TrialName</code> </p> </li> </ul></p>
    async fn list_trial_components(
        &self,
        input: ListTrialComponentsRequest,
    ) -> Result<ListTrialComponentsResponse, RusotoError<ListTrialComponentsError>>;

    /// <p>Lists the trials in your account. Specify an experiment name to limit the list to the trials that are part of that experiment. The list can be filtered to show only trials that were created in a specific time range. The list can be sorted by trial name or creation time.</p>
    async fn list_trials(
        &self,
        input: ListTrialsRequest,
    ) -> Result<ListTrialsResponse, RusotoError<ListTrialsError>>;

    /// <p>Lists user profiles.</p>
    async fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> Result<ListUserProfilesResponse, RusotoError<ListUserProfilesError>>;

    /// <p>Gets a list of work teams that you have defined in a region. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    async fn list_workteams(
        &self,
        input: ListWorkteamsRequest,
    ) -> Result<ListWorkteamsResponse, RusotoError<ListWorkteamsError>>;

    /// <p>Renders the UI template so that you can preview the worker's experience. </p>
    async fn render_ui_template(
        &self,
        input: RenderUiTemplateRequest,
    ) -> Result<RenderUiTemplateResponse, RusotoError<RenderUiTemplateError>>;

    /// <p>Finds Amazon SageMaker resources that match a search query. Matching resource objects are returned as a list of <code>SearchResult</code> objects in the response. You can sort the search results by any resource property in a ascending or descending order.</p> <p>You can query against the following value types: numeric, text, Boolean, and timestamp.</p>
    async fn search(
        &self,
        input: SearchRequest,
    ) -> Result<SearchResponse, RusotoError<SearchError>>;

    /// <p><p>Starts a previously stopped monitoring schedule.</p> <note> <p>New monitoring schedules are immediately started after creation.</p> </note></p>
    async fn start_monitoring_schedule(
        &self,
        input: StartMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<StartMonitoringScheduleError>>;

    /// <p>Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, Amazon SageMaker sets the notebook instance status to <code>InService</code>. A notebook instance's status must be <code>InService</code> before you can connect to your Jupyter notebook. </p>
    async fn start_notebook_instance(
        &self,
        input: StartNotebookInstanceInput,
    ) -> Result<(), RusotoError<StartNotebookInstanceError>>;

    /// <p>A method for forcing the termination of a running job.</p>
    async fn stop_auto_ml_job(
        &self,
        input: StopAutoMLJobRequest,
    ) -> Result<(), RusotoError<StopAutoMLJobError>>;

    /// <p>Stops a model compilation job.</p> <p> To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal. This gracefully shuts the job down. If the job hasn't stopped, it sends the SIGKILL signal.</p> <p>When it receives a <code>StopCompilationJob</code> request, Amazon SageMaker changes the <a>CompilationJobSummary$CompilationJobStatus</a> of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the <a>CompilationJobSummary$CompilationJobStatus</a> to <code>Stopped</code>. </p>
    async fn stop_compilation_job(
        &self,
        input: StopCompilationJobRequest,
    ) -> Result<(), RusotoError<StopCompilationJobError>>;

    /// <p>Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched.</p> <p>All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). All data that the training jobs write to Amazon CloudWatch Logs are still available in CloudWatch. After the tuning job moves to the <code>Stopped</code> state, it releases all reserved resources for the tuning job.</p>
    async fn stop_hyper_parameter_tuning_job(
        &self,
        input: StopHyperParameterTuningJobRequest,
    ) -> Result<(), RusotoError<StopHyperParameterTuningJobError>>;

    /// <p>Stops a running labeling job. A job that is stopped cannot be restarted. Any results obtained before the job is stopped are placed in the Amazon S3 output bucket.</p>
    async fn stop_labeling_job(
        &self,
        input: StopLabelingJobRequest,
    ) -> Result<(), RusotoError<StopLabelingJobError>>;

    /// <p>Stops a previously started monitoring schedule.</p>
    async fn stop_monitoring_schedule(
        &self,
        input: StopMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<StopMonitoringScheduleError>>;

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. Amazon SageMaker stops charging you for the ML compute instance when you call <code>StopNotebookInstance</code>.</p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    async fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> Result<(), RusotoError<StopNotebookInstanceError>>;

    /// <p>Stops a processing job.</p>
    async fn stop_processing_job(
        &self,
        input: StopProcessingJobRequest,
    ) -> Result<(), RusotoError<StopProcessingJobError>>;

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    async fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> Result<(), RusotoError<StopTrainingJobError>>;

    /// <p>Stops a transform job.</p> <p>When Amazon SageMaker receives a <code>StopTransformJob</code> request, the status of the job changes to <code>Stopping</code>. After Amazon SageMaker stops the job, the status is set to <code>Stopped</code>. When you stop a transform job before it is completed, Amazon SageMaker doesn't store the job's output in Amazon S3.</p>
    async fn stop_transform_job(
        &self,
        input: StopTransformJobRequest,
    ) -> Result<(), RusotoError<StopTransformJobError>>;

    /// <p>Updates the specified Git repository with the specified values.</p>
    async fn update_code_repository(
        &self,
        input: UpdateCodeRepositoryInput,
    ) -> Result<UpdateCodeRepositoryOutput, RusotoError<UpdateCodeRepositoryError>>;

    /// <p>Updates a domain. Changes will impact all of the people in the domain.</p>
    async fn update_domain(
        &self,
        input: UpdateDomainRequest,
    ) -> Result<UpdateDomainResponse, RusotoError<UpdateDomainError>>;

    /// <p><p>Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> Result<UpdateEndpointOutput, RusotoError<UpdateEndpointError>>;

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    async fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> Result<
        UpdateEndpointWeightsAndCapacitiesOutput,
        RusotoError<UpdateEndpointWeightsAndCapacitiesError>,
    >;

    /// <p>Adds, updates, or removes the description of an experiment. Updates the display name of an experiment.</p>
    async fn update_experiment(
        &self,
        input: UpdateExperimentRequest,
    ) -> Result<UpdateExperimentResponse, RusotoError<UpdateExperimentError>>;

    /// <p>Updates a previously created schedule.</p>
    async fn update_monitoring_schedule(
        &self,
        input: UpdateMonitoringScheduleRequest,
    ) -> Result<UpdateMonitoringScheduleResponse, RusotoError<UpdateMonitoringScheduleError>>;

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements.</p>
    async fn update_notebook_instance(
        &self,
        input: UpdateNotebookInstanceInput,
    ) -> Result<UpdateNotebookInstanceOutput, RusotoError<UpdateNotebookInstanceError>>;

    /// <p>Updates a notebook instance lifecycle configuration created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    async fn update_notebook_instance_lifecycle_config(
        &self,
        input: UpdateNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        UpdateNotebookInstanceLifecycleConfigOutput,
        RusotoError<UpdateNotebookInstanceLifecycleConfigError>,
    >;

    /// <p>Updates the display name of a trial.</p>
    async fn update_trial(
        &self,
        input: UpdateTrialRequest,
    ) -> Result<UpdateTrialResponse, RusotoError<UpdateTrialError>>;

    /// <p>Updates one or more properties of a trial component.</p>
    async fn update_trial_component(
        &self,
        input: UpdateTrialComponentRequest,
    ) -> Result<UpdateTrialComponentResponse, RusotoError<UpdateTrialComponentError>>;

    /// <p>Updates a user profile.</p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<UpdateUserProfileResponse, RusotoError<UpdateUserProfileError>>;

    /// <p><p>Restricts access to tasks assigned to workers in the specified workforce to those within specific ranges of IP addresses. You specify allowed IP addresses by creating a list of up to four <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>.</p> <p>By default, a workforce isn&#39;t restricted to specific IP addresses. If you specify a range of IP addresses, workers who attempt to access tasks using any IP address outside the specified range are denied access and get a <code>Not Found</code> error message on the worker portal. After restricting access with this operation, you can see the allowed IP values for a private workforce with the operation.</p> <important> <p>This operation applies only to private workforces.</p> </important></p>
    async fn update_workforce(
        &self,
        input: UpdateWorkforceRequest,
    ) -> Result<UpdateWorkforceResponse, RusotoError<UpdateWorkforceError>>;

    /// <p>Updates an existing work team with new member definitions or description.</p>
    async fn update_workteam(
        &self,
        input: UpdateWorkteamRequest,
    ) -> Result<UpdateWorkteamResponse, RusotoError<UpdateWorkteamError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SageMakerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SageMakerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SageMakerClient {
        SageMakerClient { client, region }
    }
}

#[async_trait]
impl SageMaker for SageMakerClient {
    /// <p><p>Adds or overwrites one or more tags for the specified Amazon SageMaker resource. You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints.</p> <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p> <note> <p>Tags that you add to a hyperparameter tuning job by calling this API are also added to any training jobs that the hyperparameter tuning job launches after you call this API, but not to training jobs that the hyperparameter tuning job launched before you called this API. To make sure that the tags associated with a hyperparameter tuning job are also added to all training jobs that the hyperparameter tuning job launches, add the tags when you first create the tuning job by specifying them in the <code>Tags</code> parameter of <a>CreateHyperParameterTuningJob</a> </p> </note></p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.AddTags");
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

    /// <p>Associates a trial component with a trial. A trial component can be associated with multiple trials. To disassociate a trial component from a trial, call the <a>DisassociateTrialComponent</a> API.</p>
    async fn associate_trial_component(
        &self,
        input: AssociateTrialComponentRequest,
    ) -> Result<AssociateTrialComponentResponse, RusotoError<AssociateTrialComponentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.AssociateTrialComponent");
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
                .deserialize::<AssociateTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateTrialComponentError::from_response(response))
        }
    }

    /// <p>Create a machine learning algorithm that you can use in Amazon SageMaker and list in the AWS Marketplace.</p>
    async fn create_algorithm(
        &self,
        input: CreateAlgorithmInput,
    ) -> Result<CreateAlgorithmOutput, RusotoError<CreateAlgorithmError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateAlgorithmOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAlgorithmError::from_response(response))
        }
    }

    /// <p>Creates a running App for the specified UserProfile. Supported Apps are JupyterServer and KernelGateway. This operation is automatically invoked by Amazon SageMaker Amazon SageMaker Studio (Studio) upon access to the associated Studio Domain, and when new kernel configurations are selected by the user. A user may have multiple Apps active simultaneously. Apps will automatically terminate and be deleted when stopped from within Studio, or when the DeleteApp API is manually called. UserProfiles are limited to 5 concurrently running Apps at a time.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateApp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateAppResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAppError::from_response(response))
        }
    }

    /// <p>Creates an AutoPilot job.</p>
    async fn create_auto_ml_job(
        &self,
        input: CreateAutoMLJobRequest,
    ) -> Result<CreateAutoMLJobResponse, RusotoError<CreateAutoMLJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateAutoMLJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateAutoMLJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAutoMLJobError::from_response(response))
        }
    }

    /// <p>Creates a Git repository as a resource in your Amazon SageMaker account. You can associate the repository with notebook instances so that you can use Git source control for the notebooks you create. The Git repository is a resource in your Amazon SageMaker account, so it can be associated with more than one notebook instance, and it persists independently from the lifecycle of any notebook instances it is associated with.</p> <p>The repository can be hosted either in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit</a> or in any other Git repository.</p>
    async fn create_code_repository(
        &self,
        input: CreateCodeRepositoryInput,
    ) -> Result<CreateCodeRepositoryOutput, RusotoError<CreateCodeRepositoryError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateCodeRepository");
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
                .deserialize::<CreateCodeRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCodeRepositoryError::from_response(response))
        }
    }

    /// <p>Starts a model compilation job. After the model has been compiled, Amazon SageMaker saves the resulting model artifacts to an Amazon Simple Storage Service (Amazon S3) bucket that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts with AWS IoT Greengrass. In that case, deploy them as an ML resource.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p>A name for the compilation job</p> </li> <li> <p> Information about the input model artifacts </p> </li> <li> <p>The output location for the compiled model and the device (target) that the model runs on </p> </li> <li> <p> <code>The Amazon Resource Name (ARN) of the IAM role that Amazon SageMaker assumes to perform the model compilation job</code> </p> </li> </ul> <p>You can also provide a <code>Tag</code> to track the model compilation job's resource use and costs. The response body contains the <code>CompilationJobArn</code> for the compiled job.</p> <p>To stop a model compilation job, use <a>StopCompilationJob</a>. To get information about a particular model compilation job, use <a>DescribeCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    async fn create_compilation_job(
        &self,
        input: CreateCompilationJobRequest,
    ) -> Result<CreateCompilationJobResponse, RusotoError<CreateCompilationJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateCompilationJob");
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
                .deserialize::<CreateCompilationJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCompilationJobError::from_response(response))
        }
    }

    /// <p>Creates a Domain for Amazon SageMaker Amazon SageMaker Studio (Studio), which can be accessed by end-users in a web browser. A Domain has an associated directory, list of authorized users, and a variety of security, application, policies, and Amazon Virtual Private Cloud configurations. An AWS account is limited to one Domain, per region. Users within a domain can share notebook files and other artifacts with each other. When a Domain is created, an Amazon Elastic File System (EFS) is also created for use by all of the users within the Domain. Each user receives a private home directory within the EFS for notebooks, Git repositories, and data files. </p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResponse, RusotoError<CreateDomainError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDomainError::from_response(response))
        }
    }

    /// <p>Creates an endpoint using the endpoint configuration specified in the request. Amazon SageMaker uses the endpoint to provision resources and deploy models. You create the endpoint configuration with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpointConfig.html">CreateEndpointConfig</a> API. </p> <note> <p> Use this API only for hosting models using Amazon SageMaker hosting services. </p> <p> You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note> <p>The endpoint name must be unique within an AWS Region in your AWS account. </p> <p>When it receives the request, Amazon SageMaker creates the endpoint, launches the resources (ML compute instances), and deploys the model(s) on them. </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Creating</code>. After it creates the endpoint, it sets the status to <code>InService</code>. Amazon SageMaker can then process incoming requests for inferences. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API.</p> <p>For an example, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ex1.html">Exercise 1: Using the K-Means Algorithm Provided by Amazon SageMaker</a>. </p> <p>If any of the models hosted at this endpoint get model data from an Amazon S3 location, Amazon SageMaker uses AWS Security Token Service to download model artifacts from the S3 path you provided. AWS STS is activated in your IAM user account by default. If you previously deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointInput,
    ) -> Result<CreateEndpointOutput, RusotoError<CreateEndpointError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateEndpointOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEndpointError::from_response(response))
        }
    }

    /// <p>Creates an endpoint configuration that Amazon SageMaker hosting services uses to deploy models. In the configuration, you identify one or more models, created using the <code>CreateModel</code> API, to deploy and the resources that you want Amazon SageMaker to provision. Then you call the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API.</p> <note> <p> Use this API only if you want to use Amazon SageMaker hosting services to deploy models into production. </p> </note> <p>In the request, you define one or more <code>ProductionVariant</code>s, each of which identifies a model. Each <code>ProductionVariant</code> parameter also describes the resources that you want Amazon SageMaker to provision. This includes the number and type of ML compute instances to deploy. </p> <p>If you are hosting multiple models, you also assign a <code>VariantWeight</code> to specify how much traffic you want to allocate to each model. For example, suppose that you want to host two models, A and B, and you assign traffic weight 2 for model A and 1 for model B. Amazon SageMaker distributes two-thirds of the traffic to Model A, and one-third to model B. </p>
    async fn create_endpoint_config(
        &self,
        input: CreateEndpointConfigInput,
    ) -> Result<CreateEndpointConfigOutput, RusotoError<CreateEndpointConfigError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateEndpointConfig");
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
                .deserialize::<CreateEndpointConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEndpointConfigError::from_response(response))
        }
    }

    /// <p>Creates an Amazon SageMaker <i>experiment</i>. An experiment is a collection of <i>trials</i> that are observed, compared and evaluated as a group. A trial is a set of steps, called <i>trial components</i>, that produce a machine learning model.</p> <p>The goal of an experiment is to determine the components that produce the best model. Multiple trials are performed, each one isolating and measuring the impact of a change to one or more inputs, while keeping the remaining inputs constant.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to experiments, trials, trial components and then use the <a>Search</a> API to search for the tags.</p> <p>To add a description to an experiment, specify the optional <code>Description</code> parameter. To add a description later, or to change the description, call the <a>UpdateExperiment</a> API.</p> <p>To get a list of all your experiments, call the <a>ListExperiments</a> API. To view an experiment's properties, call the <a>DescribeExperiment</a> API. To get a list of all the trials associated with an experiment, call the <a>ListTrials</a> API. To create a trial call the <a>CreateTrial</a> API.</p>
    async fn create_experiment(
        &self,
        input: CreateExperimentRequest,
    ) -> Result<CreateExperimentResponse, RusotoError<CreateExperimentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateExperiment");
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
                .deserialize::<CreateExperimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateExperimentError::from_response(response))
        }
    }

    /// <p>Creates a flow definition.</p>
    async fn create_flow_definition(
        &self,
        input: CreateFlowDefinitionRequest,
    ) -> Result<CreateFlowDefinitionResponse, RusotoError<CreateFlowDefinitionError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateFlowDefinition");
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
                .deserialize::<CreateFlowDefinitionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFlowDefinitionError::from_response(response))
        }
    }

    /// <p>Defines the settings you will use for the human review workflow user interface. Reviewers will see a three-panel interface with an instruction area, the item to review, and an input area.</p>
    async fn create_human_task_ui(
        &self,
        input: CreateHumanTaskUiRequest,
    ) -> Result<CreateHumanTaskUiResponse, RusotoError<CreateHumanTaskUiError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateHumanTaskUi");
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
                .deserialize::<CreateHumanTaskUiResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHumanTaskUiError::from_response(response))
        }
    }

    /// <p>Starts a hyperparameter tuning job. A hyperparameter tuning job finds the best version of a model by running many training jobs on your dataset using the algorithm you choose and values for hyperparameters within ranges that you specify. It then chooses the hyperparameter values that result in a model that performs the best, as measured by an objective metric that you choose.</p>
    async fn create_hyper_parameter_tuning_job(
        &self,
        input: CreateHyperParameterTuningJobRequest,
    ) -> Result<
        CreateHyperParameterTuningJobResponse,
        RusotoError<CreateHyperParameterTuningJobError>,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateHyperParameterTuningJob");
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
                .deserialize::<CreateHyperParameterTuningJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHyperParameterTuningJobError::from_response(response))
        }
    }

    /// <p>Creates a job that uses workers to label the data objects in your input dataset. You can use the labeled data to train machine learning models.</p> <p>You can select your workforce from one of three providers:</p> <ul> <li> <p>A private workforce that you create. It can include employees, contractors, and outside experts. Use a private workforce when want the data to stay within your organization or when a specific set of skills is required.</p> </li> <li> <p>One or more vendors that you select from the AWS Marketplace. Vendors provide expertise in specific areas. </p> </li> <li> <p>The Amazon Mechanical Turk workforce. This is the largest workforce, but it should only be used for public data or data that has been stripped of any personally identifiable information.</p> </li> </ul> <p>You can also use <i>automated data labeling</i> to reduce the number of data objects that need to be labeled by a human. Automated data labeling uses <i>active learning</i> to determine if a data object can be labeled by machine or if it needs to be sent to a human worker. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-automated-labeling.html">Using Automated Data Labeling</a>.</p> <p>The data objects to be labeled are contained in an Amazon S3 bucket. You create a <i>manifest file</i> that describes the location of each object. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-data.html">Using Input and Output Data</a>.</p> <p>The output can be used as the manifest file for another labeling job or as training data for your machine learning models.</p>
    async fn create_labeling_job(
        &self,
        input: CreateLabelingJobRequest,
    ) -> Result<CreateLabelingJobResponse, RusotoError<CreateLabelingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateLabelingJob");
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
                .deserialize::<CreateLabelingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLabelingJobError::from_response(response))
        }
    }

    /// <p>Creates a model in Amazon SageMaker. In the request, you name the model and describe a primary container. For the primary container, you specify the docker image containing inference code, artifacts (from prior training), and custom environment map that the inference code uses when you deploy the model for predictions.</p> <p>Use this API to create a model if you want to use Amazon SageMaker hosting services or run a batch transform job.</p> <p>To host your model, you create an endpoint configuration with the <code>CreateEndpointConfig</code> API, and then create an endpoint with the <code>CreateEndpoint</code> API. Amazon SageMaker then deploys all of the containers that you defined for the model in the hosting environment. </p> <p>To run a batch transform using your model, you start a job with the <code>CreateTransformJob</code> API. Amazon SageMaker uses your model and your dataset to get inferences which are then saved to a specified S3 location.</p> <p>In the <code>CreateModel</code> request, you must define a container with the <code>PrimaryContainer</code> parameter.</p> <p>In the request, you also provide an IAM role that Amazon SageMaker can assume to access model artifacts and docker image for deployment on ML compute hosting instances or for batch transform jobs. In addition, you also use the IAM role to manage permissions the inference code needs. For example, if the inference code access any other AWS resources, you grant necessary permissions via this role.</p>
    async fn create_model(
        &self,
        input: CreateModelInput,
    ) -> Result<CreateModelOutput, RusotoError<CreateModelError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateModelError::from_response(response))
        }
    }

    /// <p>Creates a model package that you can use to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p> <p>To create a model package by specifying a Docker container that contains your inference code and the Amazon S3 location of your model artifacts, provide values for <code>InferenceSpecification</code>. To create a model from an algorithm resource that you created or subscribed to in AWS Marketplace, provide a value for <code>SourceAlgorithmSpecification</code>.</p>
    async fn create_model_package(
        &self,
        input: CreateModelPackageInput,
    ) -> Result<CreateModelPackageOutput, RusotoError<CreateModelPackageError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateModelPackage");
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
                .deserialize::<CreateModelPackageOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateModelPackageError::from_response(response))
        }
    }

    /// <p>Creates a schedule that regularly starts Amazon SageMaker Processing Jobs to monitor the data captured for an Amazon SageMaker Endoint.</p>
    async fn create_monitoring_schedule(
        &self,
        input: CreateMonitoringScheduleRequest,
    ) -> Result<CreateMonitoringScheduleResponse, RusotoError<CreateMonitoringScheduleError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateMonitoringSchedule");
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
                .deserialize::<CreateMonitoringScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMonitoringScheduleError::from_response(response))
        }
    }

    /// <p>Creates an Amazon SageMaker notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook. </p> <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. Amazon SageMaker launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance. </p> <p>Amazon SageMaker also provides a set of example notebooks. Each notebook demonstrates how to use Amazon SageMaker with a specific algorithm or with a machine learning framework. </p> <p>After receiving the request, Amazon SageMaker does the following:</p> <ol> <li> <p>Creates a network interface in the Amazon SageMaker VPC.</p> </li> <li> <p>(Option) If you specified <code>SubnetId</code>, Amazon SageMaker creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, Amazon SageMaker attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p> </li> <li> <p>Launches an EC2 instance of the type specified in the request in the Amazon SageMaker VPC. If you specified <code>SubnetId</code> of your VPC, Amazon SageMaker specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p> </li> </ol> <p>After creating the notebook instance, Amazon SageMaker returns its Amazon Resource Name (ARN). You can't change the name of a notebook instance after you create it.</p> <p>After Amazon SageMaker creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating Amazon SageMaker endpoints, and validate hosted models. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    async fn create_notebook_instance(
        &self,
        input: CreateNotebookInstanceInput,
    ) -> Result<CreateNotebookInstanceOutput, RusotoError<CreateNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateNotebookInstance");
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
                .deserialize::<CreateNotebookInstanceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotebookInstanceError::from_response(response))
        }
    }

    /// <p>Creates a lifecycle configuration that you can associate with a notebook instance. A <i>lifecycle configuration</i> is a collection of shell scripts that run when you create or start a notebook instance.</p> <p>Each lifecycle configuration script has a limit of 16384 characters.</p> <p>The value of the <code>$PATH</code> environment variable that is available to both scripts is <code>/sbin:bin:/usr/sbin:/usr/bin</code>.</p> <p>View CloudWatch Logs for notebook instance lifecycle configurations in log group <code>/aws/sagemaker/NotebookInstances</code> in log stream <code>[notebook-instance-name]/[LifecycleConfigHook]</code>.</p> <p>Lifecycle configuration scripts cannot run for longer than 5 minutes. If a script runs for longer than 5 minutes, it fails and the notebook instance is not created or started.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    async fn create_notebook_instance_lifecycle_config(
        &self,
        input: CreateNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        CreateNotebookInstanceLifecycleConfigOutput,
        RusotoError<CreateNotebookInstanceLifecycleConfigError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNotebookInstanceLifecycleConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotebookInstanceLifecycleConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a URL for a specified UserProfile in a Domain. When accessed in a web browser, the user will be automatically signed in to Amazon SageMaker Amazon SageMaker Studio (Studio), and granted access to all of the Apps and files associated with that Amazon Elastic File System (EFS). This operation can only be called when AuthMode equals IAM. </p>
    async fn create_presigned_domain_url(
        &self,
        input: CreatePresignedDomainUrlRequest,
    ) -> Result<CreatePresignedDomainUrlResponse, RusotoError<CreatePresignedDomainUrlError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreatePresignedDomainUrl");
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
                .deserialize::<CreatePresignedDomainUrlResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePresignedDomainUrlError::from_response(response))
        }
    }

    /// <p><p>Returns a URL that you can use to connect to the Jupyter server from a notebook instance. In the Amazon SageMaker console, when you choose <code>Open</code> next to a notebook instance, Amazon SageMaker opens a new tab showing the Jupyter server home page from the notebook instance. The console uses this API to get the URL and show the page.</p> <p>IAM authorization policies for this API are also enforced for every HTTP request and WebSocket frame that attempts to connect to the notebook instance.For example, you can restrict access to this API and to the URL that it returns to a list of IP addresses that you specify. Use the <code>NotIpAddress</code> condition operator and the <code>aws:SourceIP</code> condition context key to specify the list of IP addresses that you want to have access to the notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/security_iam_id-based-policy-examples.html#nbi-ip-filter">Limit Access to a Notebook Instance by IP Address</a>.</p> <note> <p>The URL that you get from a call to is valid only for 5 minutes. If you try to use the URL after the 5-minute limit expires, you are directed to the AWS console sign-in page.</p> </note></p>
    async fn create_presigned_notebook_instance_url(
        &self,
        input: CreatePresignedNotebookInstanceUrlInput,
    ) -> Result<
        CreatePresignedNotebookInstanceUrlOutput,
        RusotoError<CreatePresignedNotebookInstanceUrlError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePresignedNotebookInstanceUrlOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePresignedNotebookInstanceUrlError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a processing job.</p>
    async fn create_processing_job(
        &self,
        input: CreateProcessingJobRequest,
    ) -> Result<CreateProcessingJobResponse, RusotoError<CreateProcessingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateProcessingJob");
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
                .deserialize::<CreateProcessingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProcessingJobError::from_response(response))
        }
    }

    /// <p>Starts a model training job. After training completes, Amazon SageMaker saves the resulting model artifacts to an Amazon S3 location that you specify. </p> <p>If you choose to host your model using Amazon SageMaker hosting services, you can use the resulting model artifacts as part of the model. You can also use the artifacts in a machine learning service other than Amazon SageMaker, provided that you know how to use them for inferences. </p> <p>In the request body, you provide the following: </p> <ul> <li> <p> <code>AlgorithmSpecification</code> - Identifies the training algorithm to use. </p> </li> <li> <p> <code>HyperParameters</code> - Specify these algorithm-specific parameters to enable the estimation of model parameters during training. Hyperparameters can be tuned to optimize this learning process. For a list of hyperparameters for each training algorithm provided by Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> </li> <li> <p> <code>InputDataConfig</code> - Describes the training dataset and the Amazon S3, EFS, or FSx location where it is stored.</p> </li> <li> <p> <code>OutputDataConfig</code> - Identifies the Amazon S3 bucket where you want Amazon SageMaker to save the results of model training. </p> <p/> </li> <li> <p> <code>ResourceConfig</code> - Identifies the resources, ML compute instances, and ML storage volumes to deploy for model training. In distributed training, you specify more than one instance. </p> </li> <li> <p> <code>EnableManagedSpotTraining</code> - Optimize the cost of training machine learning models by up to 80% by using Amazon EC2 Spot instances. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-managed-spot-training.html">Managed Spot Training</a>. </p> </li> <li> <p> <code>RoleARN</code> - The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during model training. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete model training. </p> </li> <li> <p> <code>StoppingCondition</code> - To help cap training costs, use <code>MaxRuntimeInSeconds</code> to set a time limit for training. Use <code>MaxWaitTimeInSeconds</code> to specify how long you are willing to wait for a managed spot training job to complete. </p> </li> </ul> <p> For more information about Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p>
    async fn create_training_job(
        &self,
        input: CreateTrainingJobRequest,
    ) -> Result<CreateTrainingJobResponse, RusotoError<CreateTrainingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTrainingJob");
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
                .deserialize::<CreateTrainingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTrainingJobError::from_response(response))
        }
    }

    /// <p>Starts a transform job. A transform job uses a trained model to get inferences on a dataset and saves these results to an Amazon S3 location that you specify.</p> <p>To perform batch transformations, you create a transform job and use the data that you have readily available.</p> <p>In the request body, you provide the following:</p> <ul> <li> <p> <code>TransformJobName</code> - Identifies the transform job. The name must be unique within an AWS Region in an AWS account.</p> </li> <li> <p> <code>ModelName</code> - Identifies the model to use. <code>ModelName</code> must be the name of an existing Amazon SageMaker model in the same AWS Region and AWS account. For information on creating a model, see <a>CreateModel</a>.</p> </li> <li> <p> <code>TransformInput</code> - Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> </li> <li> <p> <code>TransformOutput</code> - Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> </li> <li> <p> <code>TransformResources</code> - Identifies the ML compute instances for the transform job.</p> </li> </ul> <p>For more information about how batch transformation works, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html">Batch Transform</a>.</p>
    async fn create_transform_job(
        &self,
        input: CreateTransformJobRequest,
    ) -> Result<CreateTransformJobResponse, RusotoError<CreateTransformJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTransformJob");
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
                .deserialize::<CreateTransformJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTransformJobError::from_response(response))
        }
    }

    /// <p>Creates an Amazon SageMaker <i>trial</i>. A trial is a set of steps called <i>trial components</i> that produce a machine learning model. A trial is part of a single Amazon SageMaker <i>experiment</i>.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to a trial and then use the <a>Search</a> API to search for the tags.</p> <p>To get a list of all your trials, call the <a>ListTrials</a> API. To view a trial's properties, call the <a>DescribeTrial</a> API. To create a trial component, call the <a>CreateTrialComponent</a> API.</p>
    async fn create_trial(
        &self,
        input: CreateTrialRequest,
    ) -> Result<CreateTrialResponse, RusotoError<CreateTrialError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTrial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateTrialResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTrialError::from_response(response))
        }
    }

    /// <p><p>Creates a <i>trial component</i>, which is a stage of a machine learning <i>trial</i>. A trial is composed of one or more trial components. A trial component can be used in multiple trials.</p> <p>Trial components include pre-processing jobs, training jobs, and batch transform jobs.</p> <p>When you use Amazon SageMaker Studio or the Amazon SageMaker Python SDK, all experiments, trials, and trial components are automatically tracked, logged, and indexed. When you use the AWS SDK for Python (Boto), you must use the logging APIs provided by the SDK.</p> <p>You can add tags to a trial component and then use the <a>Search</a> API to search for the tags.</p> <note> <p> <code>CreateTrialComponent</code> can only be invoked from within an Amazon SageMaker managed environment. This includes Amazon SageMaker training jobs, processing jobs, transform jobs, and Amazon SageMaker notebooks. A call to <code>CreateTrialComponent</code> from outside one of these environments results in an error.</p> </note></p>
    async fn create_trial_component(
        &self,
        input: CreateTrialComponentRequest,
    ) -> Result<CreateTrialComponentResponse, RusotoError<CreateTrialComponentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateTrialComponent");
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
                .deserialize::<CreateTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTrialComponentError::from_response(response))
        }
    }

    /// <p>Creates a new user profile. A user profile represents a single user within a Domain, and is the main way to reference a "person" for the purposes of sharing, reporting and other user-oriented features. This entity is created during on-boarding. If an administrator invites a person by email or imports them from SSO, a new UserProfile is automatically created. This entity is the primary holder of settings for an individual user and has a reference to the user's private Amazon Elastic File System (EFS) home directory. </p>
    async fn create_user_profile(
        &self,
        input: CreateUserProfileRequest,
    ) -> Result<CreateUserProfileResponse, RusotoError<CreateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateUserProfile");
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
                .deserialize::<CreateUserProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserProfileError::from_response(response))
        }
    }

    /// <p>Creates a new work team for labeling your data. A work team is defined by one or more Amazon Cognito user pools. You must first create the user pools before you can create a work team.</p> <p>You cannot create more than 25 work teams in an account and region.</p>
    async fn create_workteam(
        &self,
        input: CreateWorkteamRequest,
    ) -> Result<CreateWorkteamResponse, RusotoError<CreateWorkteamError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.CreateWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorkteamError::from_response(response))
        }
    }

    /// <p>Removes the specified algorithm from your account.</p>
    async fn delete_algorithm(
        &self,
        input: DeleteAlgorithmInput,
    ) -> Result<(), RusotoError<DeleteAlgorithmError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteAlgorithm");
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
            Err(DeleteAlgorithmError::from_response(response))
        }
    }

    /// <p>Used to stop and delete an app.</p>
    async fn delete_app(&self, input: DeleteAppRequest) -> Result<(), RusotoError<DeleteAppError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteApp");
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

    /// <p>Deletes the specified Git repository from your account.</p>
    async fn delete_code_repository(
        &self,
        input: DeleteCodeRepositoryInput,
    ) -> Result<(), RusotoError<DeleteCodeRepositoryError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteCodeRepository");
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
            Err(DeleteCodeRepositoryError::from_response(response))
        }
    }

    /// <p>Used to delete a domain. If you on-boarded with IAM mode, you will need to delete your domain to on-board again using SSO. Use with caution. All of the members of the domain will lose access to their EFS volume, including data, notebooks, and other artifacts. </p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<(), RusotoError<DeleteDomainError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteDomain");
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
            Err(DeleteDomainError::from_response(response))
        }
    }

    /// <p>Deletes an endpoint. Amazon SageMaker frees up all of the resources that were deployed when the endpoint was created. </p> <p>Amazon SageMaker retires any custom KMS key grants associated with the endpoint, meaning you don't need to use the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> API call.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointInput,
    ) -> Result<(), RusotoError<DeleteEndpointError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpoint");
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
            Err(DeleteEndpointError::from_response(response))
        }
    }

    /// <p>Deletes an endpoint configuration. The <code>DeleteEndpointConfig</code> API deletes only the specified configuration. It does not delete endpoints created using the configuration. </p>
    async fn delete_endpoint_config(
        &self,
        input: DeleteEndpointConfigInput,
    ) -> Result<(), RusotoError<DeleteEndpointConfigError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteEndpointConfig");
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
            Err(DeleteEndpointConfigError::from_response(response))
        }
    }

    /// <p>Deletes an Amazon SageMaker experiment. All trials associated with the experiment must be deleted first. Use the <a>ListTrials</a> API to get a list of the trials associated with the experiment.</p>
    async fn delete_experiment(
        &self,
        input: DeleteExperimentRequest,
    ) -> Result<DeleteExperimentResponse, RusotoError<DeleteExperimentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteExperiment");
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
                .deserialize::<DeleteExperimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteExperimentError::from_response(response))
        }
    }

    /// <p>Deletes the specified flow definition.</p>
    async fn delete_flow_definition(
        &self,
        input: DeleteFlowDefinitionRequest,
    ) -> Result<DeleteFlowDefinitionResponse, RusotoError<DeleteFlowDefinitionError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteFlowDefinition");
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
                .deserialize::<DeleteFlowDefinitionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFlowDefinitionError::from_response(response))
        }
    }

    /// <p>Deletes a model. The <code>DeleteModel</code> API deletes only the model entry that was created in Amazon SageMaker when you called the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API. It does not delete model artifacts, inference code, or the IAM role that you specified when creating the model. </p>
    async fn delete_model(
        &self,
        input: DeleteModelInput,
    ) -> Result<(), RusotoError<DeleteModelError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteModel");
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
            Err(DeleteModelError::from_response(response))
        }
    }

    /// <p>Deletes a model package.</p> <p>A model package is used to create Amazon SageMaker models or list on AWS Marketplace. Buyers can subscribe to model packages listed on AWS Marketplace to create models in Amazon SageMaker.</p>
    async fn delete_model_package(
        &self,
        input: DeleteModelPackageInput,
    ) -> Result<(), RusotoError<DeleteModelPackageError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteModelPackage");
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
            Err(DeleteModelPackageError::from_response(response))
        }
    }

    /// <p>Deletes a monitoring schedule. Also stops the schedule had not already been stopped. This does not delete the job execution history of the monitoring schedule. </p>
    async fn delete_monitoring_schedule(
        &self,
        input: DeleteMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<DeleteMonitoringScheduleError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteMonitoringSchedule");
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
            Err(DeleteMonitoringScheduleError::from_response(response))
        }
    }

    /// <p><p> Deletes an Amazon SageMaker notebook instance. Before you can delete a notebook instance, you must call the <code>StopNotebookInstance</code> API. </p> <important> <p>When you delete a notebook instance, you lose all of your data. Amazon SageMaker removes the ML compute instance, and deletes the ML storage volume and the network interface associated with the notebook instance. </p> </important></p>
    async fn delete_notebook_instance(
        &self,
        input: DeleteNotebookInstanceInput,
    ) -> Result<(), RusotoError<DeleteNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteNotebookInstance");
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
            Err(DeleteNotebookInstanceError::from_response(response))
        }
    }

    /// <p>Deletes a notebook instance lifecycle configuration.</p>
    async fn delete_notebook_instance_lifecycle_config(
        &self,
        input: DeleteNotebookInstanceLifecycleConfigInput,
    ) -> Result<(), RusotoError<DeleteNotebookInstanceLifecycleConfigError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "SageMaker.DeleteNotebookInstanceLifecycleConfig",
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
            Err(DeleteNotebookInstanceLifecycleConfigError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Deletes the specified tags from an Amazon SageMaker resource.</p> <p>To list a resource&#39;s tags, use the <code>ListTags</code> API. </p> <note> <p>When you call this API to delete tags from a hyperparameter tuning job, the deleted tags are not removed from training jobs that the hyperparameter tuning job launched before you called this API.</p> </note></p>
    async fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> Result<DeleteTagsOutput, RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Deletes the specified trial. All trial components that make up the trial must be deleted first. Use the <a>DescribeTrialComponent</a> API to get the list of trial components.</p>
    async fn delete_trial(
        &self,
        input: DeleteTrialRequest,
    ) -> Result<DeleteTrialResponse, RusotoError<DeleteTrialError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteTrial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTrialResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTrialError::from_response(response))
        }
    }

    /// <p>Deletes the specified trial component. A trial component must be disassociated from all trials before the trial component can be deleted. To disassociate a trial component from a trial, call the <a>DisassociateTrialComponent</a> API.</p>
    async fn delete_trial_component(
        &self,
        input: DeleteTrialComponentRequest,
    ) -> Result<DeleteTrialComponentResponse, RusotoError<DeleteTrialComponentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteTrialComponent");
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
                .deserialize::<DeleteTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTrialComponentError::from_response(response))
        }
    }

    /// <p>Deletes a user profile.</p>
    async fn delete_user_profile(
        &self,
        input: DeleteUserProfileRequest,
    ) -> Result<(), RusotoError<DeleteUserProfileError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteUserProfile");
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

    /// <p>Deletes an existing work team. This operation can't be undone.</p>
    async fn delete_workteam(
        &self,
        input: DeleteWorkteamRequest,
    ) -> Result<DeleteWorkteamResponse, RusotoError<DeleteWorkteamError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DeleteWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWorkteamError::from_response(response))
        }
    }

    /// <p>Returns a description of the specified algorithm that is in your account.</p>
    async fn describe_algorithm(
        &self,
        input: DescribeAlgorithmInput,
    ) -> Result<DescribeAlgorithmOutput, RusotoError<DescribeAlgorithmError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAlgorithmOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAlgorithmError::from_response(response))
        }
    }

    /// <p>Describes the app.</p>
    async fn describe_app(
        &self,
        input: DescribeAppRequest,
    ) -> Result<DescribeAppResponse, RusotoError<DescribeAppError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeApp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAppResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAppError::from_response(response))
        }
    }

    /// <p>Returns information about an Amazon SageMaker job.</p>
    async fn describe_auto_ml_job(
        &self,
        input: DescribeAutoMLJobRequest,
    ) -> Result<DescribeAutoMLJobResponse, RusotoError<DescribeAutoMLJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeAutoMLJob");
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
                .deserialize::<DescribeAutoMLJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAutoMLJobError::from_response(response))
        }
    }

    /// <p>Gets details about the specified Git repository.</p>
    async fn describe_code_repository(
        &self,
        input: DescribeCodeRepositoryInput,
    ) -> Result<DescribeCodeRepositoryOutput, RusotoError<DescribeCodeRepositoryError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeCodeRepository");
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
                .deserialize::<DescribeCodeRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCodeRepositoryError::from_response(response))
        }
    }

    /// <p>Returns information about a model compilation job.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about multiple model compilation jobs, use <a>ListCompilationJobs</a>.</p>
    async fn describe_compilation_job(
        &self,
        input: DescribeCompilationJobRequest,
    ) -> Result<DescribeCompilationJobResponse, RusotoError<DescribeCompilationJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeCompilationJob");
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
                .deserialize::<DescribeCompilationJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCompilationJobError::from_response(response))
        }
    }

    /// <p>The desciption of the domain.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainRequest,
    ) -> Result<DescribeDomainResponse, RusotoError<DescribeDomainError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDomainError::from_response(response))
        }
    }

    /// <p>Returns the description of an endpoint.</p>
    async fn describe_endpoint(
        &self,
        input: DescribeEndpointInput,
    ) -> Result<DescribeEndpointOutput, RusotoError<DescribeEndpointError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeEndpointOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEndpointError::from_response(response))
        }
    }

    /// <p>Returns the description of an endpoint configuration created using the <code>CreateEndpointConfig</code> API.</p>
    async fn describe_endpoint_config(
        &self,
        input: DescribeEndpointConfigInput,
    ) -> Result<DescribeEndpointConfigOutput, RusotoError<DescribeEndpointConfigError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeEndpointConfig");
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
                .deserialize::<DescribeEndpointConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEndpointConfigError::from_response(response))
        }
    }

    /// <p>Provides a list of an experiment's properties.</p>
    async fn describe_experiment(
        &self,
        input: DescribeExperimentRequest,
    ) -> Result<DescribeExperimentResponse, RusotoError<DescribeExperimentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeExperiment");
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
                .deserialize::<DescribeExperimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeExperimentError::from_response(response))
        }
    }

    /// <p>Returns information about the specified flow definition.</p>
    async fn describe_flow_definition(
        &self,
        input: DescribeFlowDefinitionRequest,
    ) -> Result<DescribeFlowDefinitionResponse, RusotoError<DescribeFlowDefinitionError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeFlowDefinition");
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
                .deserialize::<DescribeFlowDefinitionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFlowDefinitionError::from_response(response))
        }
    }

    /// <p>Returns information about the requested human task user interface.</p>
    async fn describe_human_task_ui(
        &self,
        input: DescribeHumanTaskUiRequest,
    ) -> Result<DescribeHumanTaskUiResponse, RusotoError<DescribeHumanTaskUiError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeHumanTaskUi");
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
                .deserialize::<DescribeHumanTaskUiResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHumanTaskUiError::from_response(response))
        }
    }

    /// <p>Gets a description of a hyperparameter tuning job.</p>
    async fn describe_hyper_parameter_tuning_job(
        &self,
        input: DescribeHyperParameterTuningJobRequest,
    ) -> Result<
        DescribeHyperParameterTuningJobResponse,
        RusotoError<DescribeHyperParameterTuningJobError>,
    > {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeHyperParameterTuningJob");
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
                .deserialize::<DescribeHyperParameterTuningJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHyperParameterTuningJobError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets information about a labeling job.</p>
    async fn describe_labeling_job(
        &self,
        input: DescribeLabelingJobRequest,
    ) -> Result<DescribeLabelingJobResponse, RusotoError<DescribeLabelingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeLabelingJob");
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
                .deserialize::<DescribeLabelingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLabelingJobError::from_response(response))
        }
    }

    /// <p>Describes a model that you created using the <code>CreateModel</code> API.</p>
    async fn describe_model(
        &self,
        input: DescribeModelInput,
    ) -> Result<DescribeModelOutput, RusotoError<DescribeModelError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeModelError::from_response(response))
        }
    }

    /// <p>Returns a description of the specified model package, which is used to create Amazon SageMaker models or list them on AWS Marketplace.</p> <p>To create models in Amazon SageMaker, buyers can subscribe to model packages listed on AWS Marketplace.</p>
    async fn describe_model_package(
        &self,
        input: DescribeModelPackageInput,
    ) -> Result<DescribeModelPackageOutput, RusotoError<DescribeModelPackageError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeModelPackage");
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
                .deserialize::<DescribeModelPackageOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeModelPackageError::from_response(response))
        }
    }

    /// <p>Describes the schedule for a monitoring job.</p>
    async fn describe_monitoring_schedule(
        &self,
        input: DescribeMonitoringScheduleRequest,
    ) -> Result<DescribeMonitoringScheduleResponse, RusotoError<DescribeMonitoringScheduleError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeMonitoringSchedule");
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
                .deserialize::<DescribeMonitoringScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMonitoringScheduleError::from_response(response))
        }
    }

    /// <p>Returns information about a notebook instance.</p>
    async fn describe_notebook_instance(
        &self,
        input: DescribeNotebookInstanceInput,
    ) -> Result<DescribeNotebookInstanceOutput, RusotoError<DescribeNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeNotebookInstance");
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
                .deserialize::<DescribeNotebookInstanceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNotebookInstanceError::from_response(response))
        }
    }

    /// <p>Returns a description of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    async fn describe_notebook_instance_lifecycle_config(
        &self,
        input: DescribeNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        DescribeNotebookInstanceLifecycleConfigOutput,
        RusotoError<DescribeNotebookInstanceLifecycleConfigError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNotebookInstanceLifecycleConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNotebookInstanceLifecycleConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a description of a processing job.</p>
    async fn describe_processing_job(
        &self,
        input: DescribeProcessingJobRequest,
    ) -> Result<DescribeProcessingJobResponse, RusotoError<DescribeProcessingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeProcessingJob");
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
                .deserialize::<DescribeProcessingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProcessingJobError::from_response(response))
        }
    }

    /// <p>Gets information about a work team provided by a vendor. It returns details about the subscription with a vendor in the AWS Marketplace.</p>
    async fn describe_subscribed_workteam(
        &self,
        input: DescribeSubscribedWorkteamRequest,
    ) -> Result<DescribeSubscribedWorkteamResponse, RusotoError<DescribeSubscribedWorkteamError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeSubscribedWorkteam");
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
                .deserialize::<DescribeSubscribedWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSubscribedWorkteamError::from_response(response))
        }
    }

    /// <p>Returns information about a training job.</p>
    async fn describe_training_job(
        &self,
        input: DescribeTrainingJobRequest,
    ) -> Result<DescribeTrainingJobResponse, RusotoError<DescribeTrainingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTrainingJob");
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
                .deserialize::<DescribeTrainingJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrainingJobError::from_response(response))
        }
    }

    /// <p>Returns information about a transform job.</p>
    async fn describe_transform_job(
        &self,
        input: DescribeTransformJobRequest,
    ) -> Result<DescribeTransformJobResponse, RusotoError<DescribeTransformJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTransformJob");
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
                .deserialize::<DescribeTransformJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTransformJobError::from_response(response))
        }
    }

    /// <p>Provides a list of a trial's properties.</p>
    async fn describe_trial(
        &self,
        input: DescribeTrialRequest,
    ) -> Result<DescribeTrialResponse, RusotoError<DescribeTrialError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTrial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTrialResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrialError::from_response(response))
        }
    }

    /// <p>Provides a list of a trials component's properties.</p>
    async fn describe_trial_component(
        &self,
        input: DescribeTrialComponentRequest,
    ) -> Result<DescribeTrialComponentResponse, RusotoError<DescribeTrialComponentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeTrialComponent");
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
                .deserialize::<DescribeTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrialComponentError::from_response(response))
        }
    }

    /// <p>Describes the user profile.</p>
    async fn describe_user_profile(
        &self,
        input: DescribeUserProfileRequest,
    ) -> Result<DescribeUserProfileResponse, RusotoError<DescribeUserProfileError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeUserProfile");
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
                .deserialize::<DescribeUserProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserProfileError::from_response(response))
        }
    }

    /// <p><p>Lists private workforce information, including workforce name, Amazon Resource Name (ARN), and, if applicable, allowed IP address ranges (<a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>). Allowable IP address ranges are the IP addresses that workers can use to access tasks. </p> <important> <p>This operation applies only to private workforces.</p> </important></p>
    async fn describe_workforce(
        &self,
        input: DescribeWorkforceRequest,
    ) -> Result<DescribeWorkforceResponse, RusotoError<DescribeWorkforceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeWorkforce");
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
                .deserialize::<DescribeWorkforceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkforceError::from_response(response))
        }
    }

    /// <p>Gets information about a specific work team. You can see information such as the create date, the last updated date, membership information, and the work team's Amazon Resource Name (ARN).</p>
    async fn describe_workteam(
        &self,
        input: DescribeWorkteamRequest,
    ) -> Result<DescribeWorkteamResponse, RusotoError<DescribeWorkteamError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DescribeWorkteam");
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
                .deserialize::<DescribeWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkteamError::from_response(response))
        }
    }

    /// <p>Disassociates a trial component from a trial. This doesn't effect other trials the component is associated with. Before you can delete a component, you must disassociate the component from all trials it is associated with. To associate a trial component with a trial, call the <a>AssociateTrialComponent</a> API.</p>
    async fn disassociate_trial_component(
        &self,
        input: DisassociateTrialComponentRequest,
    ) -> Result<DisassociateTrialComponentResponse, RusotoError<DisassociateTrialComponentError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.DisassociateTrialComponent");
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
                .deserialize::<DisassociateTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateTrialComponentError::from_response(response))
        }
    }

    /// <p>An auto-complete API for the search functionality in the Amazon SageMaker console. It returns suggestions of possible matches for the property name to use in <code>Search</code> queries. Provides suggestions for <code>HyperParameters</code>, <code>Tags</code>, and <code>Metrics</code>.</p>
    async fn get_search_suggestions(
        &self,
        input: GetSearchSuggestionsRequest,
    ) -> Result<GetSearchSuggestionsResponse, RusotoError<GetSearchSuggestionsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.GetSearchSuggestions");
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
                .deserialize::<GetSearchSuggestionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetSearchSuggestionsError::from_response(response))
        }
    }

    /// <p>Lists the machine learning algorithms that have been created.</p>
    async fn list_algorithms(
        &self,
        input: ListAlgorithmsInput,
    ) -> Result<ListAlgorithmsOutput, RusotoError<ListAlgorithmsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListAlgorithms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListAlgorithmsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAlgorithmsError::from_response(response))
        }
    }

    /// <p>Lists apps.</p>
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResponse, RusotoError<ListAppsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListApps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListAppsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAppsError::from_response(response))
        }
    }

    /// <p>Request a list of jobs.</p>
    async fn list_auto_ml_jobs(
        &self,
        input: ListAutoMLJobsRequest,
    ) -> Result<ListAutoMLJobsResponse, RusotoError<ListAutoMLJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListAutoMLJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListAutoMLJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAutoMLJobsError::from_response(response))
        }
    }

    /// <p>List the Candidates created for the job.</p>
    async fn list_candidates_for_auto_ml_job(
        &self,
        input: ListCandidatesForAutoMLJobRequest,
    ) -> Result<ListCandidatesForAutoMLJobResponse, RusotoError<ListCandidatesForAutoMLJobError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListCandidatesForAutoMLJob");
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
                .deserialize::<ListCandidatesForAutoMLJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListCandidatesForAutoMLJobError::from_response(response))
        }
    }

    /// <p>Gets a list of the Git repositories in your account.</p>
    async fn list_code_repositories(
        &self,
        input: ListCodeRepositoriesInput,
    ) -> Result<ListCodeRepositoriesOutput, RusotoError<ListCodeRepositoriesError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListCodeRepositories");
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
                .deserialize::<ListCodeRepositoriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListCodeRepositoriesError::from_response(response))
        }
    }

    /// <p>Lists model compilation jobs that satisfy various filters.</p> <p>To create a model compilation job, use <a>CreateCompilationJob</a>. To get information about a particular model compilation job you have created, use <a>DescribeCompilationJob</a>.</p>
    async fn list_compilation_jobs(
        &self,
        input: ListCompilationJobsRequest,
    ) -> Result<ListCompilationJobsResponse, RusotoError<ListCompilationJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListCompilationJobs");
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
                .deserialize::<ListCompilationJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListCompilationJobsError::from_response(response))
        }
    }

    /// <p>Lists the domains.</p>
    async fn list_domains(
        &self,
        input: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, RusotoError<ListDomainsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDomainsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainsError::from_response(response))
        }
    }

    /// <p>Lists endpoint configurations.</p>
    async fn list_endpoint_configs(
        &self,
        input: ListEndpointConfigsInput,
    ) -> Result<ListEndpointConfigsOutput, RusotoError<ListEndpointConfigsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpointConfigs");
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
                .deserialize::<ListEndpointConfigsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEndpointConfigsError::from_response(response))
        }
    }

    /// <p>Lists endpoints.</p>
    async fn list_endpoints(
        &self,
        input: ListEndpointsInput,
    ) -> Result<ListEndpointsOutput, RusotoError<ListEndpointsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListEndpointsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEndpointsError::from_response(response))
        }
    }

    /// <p>Lists all the experiments in your account. The list can be filtered to show only experiments that were created in a specific time range. The list can be sorted by experiment name or creation time.</p>
    async fn list_experiments(
        &self,
        input: ListExperimentsRequest,
    ) -> Result<ListExperimentsResponse, RusotoError<ListExperimentsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListExperiments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListExperimentsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListExperimentsError::from_response(response))
        }
    }

    /// <p>Returns information about the flow definitions in your account.</p>
    async fn list_flow_definitions(
        &self,
        input: ListFlowDefinitionsRequest,
    ) -> Result<ListFlowDefinitionsResponse, RusotoError<ListFlowDefinitionsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListFlowDefinitions");
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
                .deserialize::<ListFlowDefinitionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListFlowDefinitionsError::from_response(response))
        }
    }

    /// <p>Returns information about the human task user interfaces in your account.</p>
    async fn list_human_task_uis(
        &self,
        input: ListHumanTaskUisRequest,
    ) -> Result<ListHumanTaskUisResponse, RusotoError<ListHumanTaskUisError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListHumanTaskUis");
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
                .deserialize::<ListHumanTaskUisResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListHumanTaskUisError::from_response(response))
        }
    }

    /// <p>Gets a list of <a>HyperParameterTuningJobSummary</a> objects that describe the hyperparameter tuning jobs launched in your account.</p>
    async fn list_hyper_parameter_tuning_jobs(
        &self,
        input: ListHyperParameterTuningJobsRequest,
    ) -> Result<ListHyperParameterTuningJobsResponse, RusotoError<ListHyperParameterTuningJobsError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListHyperParameterTuningJobs");
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
                .deserialize::<ListHyperParameterTuningJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListHyperParameterTuningJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of labeling jobs.</p>
    async fn list_labeling_jobs(
        &self,
        input: ListLabelingJobsRequest,
    ) -> Result<ListLabelingJobsResponse, RusotoError<ListLabelingJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListLabelingJobs");
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
                .deserialize::<ListLabelingJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListLabelingJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of labeling jobs assigned to a specified work team.</p>
    async fn list_labeling_jobs_for_workteam(
        &self,
        input: ListLabelingJobsForWorkteamRequest,
    ) -> Result<ListLabelingJobsForWorkteamResponse, RusotoError<ListLabelingJobsForWorkteamError>>
    {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListLabelingJobsForWorkteam");
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
                .deserialize::<ListLabelingJobsForWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListLabelingJobsForWorkteamError::from_response(response))
        }
    }

    /// <p>Lists the model packages that have been created.</p>
    async fn list_model_packages(
        &self,
        input: ListModelPackagesInput,
    ) -> Result<ListModelPackagesOutput, RusotoError<ListModelPackagesError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListModelPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListModelPackagesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListModelPackagesError::from_response(response))
        }
    }

    /// <p>Lists models created with the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateModel.html">CreateModel</a> API.</p>
    async fn list_models(
        &self,
        input: ListModelsInput,
    ) -> Result<ListModelsOutput, RusotoError<ListModelsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListModelsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListModelsError::from_response(response))
        }
    }

    /// <p>Returns list of all monitoring job executions.</p>
    async fn list_monitoring_executions(
        &self,
        input: ListMonitoringExecutionsRequest,
    ) -> Result<ListMonitoringExecutionsResponse, RusotoError<ListMonitoringExecutionsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListMonitoringExecutions");
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
                .deserialize::<ListMonitoringExecutionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListMonitoringExecutionsError::from_response(response))
        }
    }

    /// <p>Returns list of all monitoring schedules.</p>
    async fn list_monitoring_schedules(
        &self,
        input: ListMonitoringSchedulesRequest,
    ) -> Result<ListMonitoringSchedulesResponse, RusotoError<ListMonitoringSchedulesError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListMonitoringSchedules");
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
                .deserialize::<ListMonitoringSchedulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListMonitoringSchedulesError::from_response(response))
        }
    }

    /// <p>Lists notebook instance lifestyle configurations created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    async fn list_notebook_instance_lifecycle_configs(
        &self,
        input: ListNotebookInstanceLifecycleConfigsInput,
    ) -> Result<
        ListNotebookInstanceLifecycleConfigsOutput,
        RusotoError<ListNotebookInstanceLifecycleConfigsError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListNotebookInstanceLifecycleConfigsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListNotebookInstanceLifecycleConfigsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of the Amazon SageMaker notebook instances in the requester's account in an AWS Region. </p>
    async fn list_notebook_instances(
        &self,
        input: ListNotebookInstancesInput,
    ) -> Result<ListNotebookInstancesOutput, RusotoError<ListNotebookInstancesError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListNotebookInstances");
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
                .deserialize::<ListNotebookInstancesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListNotebookInstancesError::from_response(response))
        }
    }

    /// <p>Lists processing jobs that satisfy various filters.</p>
    async fn list_processing_jobs(
        &self,
        input: ListProcessingJobsRequest,
    ) -> Result<ListProcessingJobsResponse, RusotoError<ListProcessingJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListProcessingJobs");
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
                .deserialize::<ListProcessingJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListProcessingJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of the work teams that you are subscribed to in the AWS Marketplace. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    async fn list_subscribed_workteams(
        &self,
        input: ListSubscribedWorkteamsRequest,
    ) -> Result<ListSubscribedWorkteamsResponse, RusotoError<ListSubscribedWorkteamsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListSubscribedWorkteams");
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
                .deserialize::<ListSubscribedWorkteamsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSubscribedWorkteamsError::from_response(response))
        }
    }

    /// <p>Returns the tags for the specified Amazon SageMaker resource.</p>
    async fn list_tags(
        &self,
        input: ListTagsInput,
    ) -> Result<ListTagsOutput, RusotoError<ListTagsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Lists training jobs.</p>
    async fn list_training_jobs(
        &self,
        input: ListTrainingJobsRequest,
    ) -> Result<ListTrainingJobsResponse, RusotoError<ListTrainingJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTrainingJobs");
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
                .deserialize::<ListTrainingJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTrainingJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of <a>TrainingJobSummary</a> objects that describe the training jobs that a hyperparameter tuning job launched.</p>
    async fn list_training_jobs_for_hyper_parameter_tuning_job(
        &self,
        input: ListTrainingJobsForHyperParameterTuningJobRequest,
    ) -> Result<
        ListTrainingJobsForHyperParameterTuningJobResponse,
        RusotoError<ListTrainingJobsForHyperParameterTuningJobError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTrainingJobsForHyperParameterTuningJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTrainingJobsForHyperParameterTuningJobError::from_response(response))
        }
    }

    /// <p>Lists transform jobs.</p>
    async fn list_transform_jobs(
        &self,
        input: ListTransformJobsRequest,
    ) -> Result<ListTransformJobsResponse, RusotoError<ListTransformJobsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTransformJobs");
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
                .deserialize::<ListTransformJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTransformJobsError::from_response(response))
        }
    }

    /// <p><p>Lists the trial components in your account. You can sort the list by trial component name or creation time. You can filter the list to show only components that were created in a specific time range. You can also filter on one of the following:</p> <ul> <li> <p> <code>ExperimentName</code> </p> </li> <li> <p> <code>SourceArn</code> </p> </li> <li> <p> <code>TrialName</code> </p> </li> </ul></p>
    async fn list_trial_components(
        &self,
        input: ListTrialComponentsRequest,
    ) -> Result<ListTrialComponentsResponse, RusotoError<ListTrialComponentsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTrialComponents");
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
                .deserialize::<ListTrialComponentsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTrialComponentsError::from_response(response))
        }
    }

    /// <p>Lists the trials in your account. Specify an experiment name to limit the list to the trials that are part of that experiment. The list can be filtered to show only trials that were created in a specific time range. The list can be sorted by trial name or creation time.</p>
    async fn list_trials(
        &self,
        input: ListTrialsRequest,
    ) -> Result<ListTrialsResponse, RusotoError<ListTrialsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListTrials");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTrialsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTrialsError::from_response(response))
        }
    }

    /// <p>Lists user profiles.</p>
    async fn list_user_profiles(
        &self,
        input: ListUserProfilesRequest,
    ) -> Result<ListUserProfilesResponse, RusotoError<ListUserProfilesError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListUserProfiles");
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
                .deserialize::<ListUserProfilesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserProfilesError::from_response(response))
        }
    }

    /// <p>Gets a list of work teams that you have defined in a region. The list may be empty if no work team satisfies the filter specified in the <code>NameContains</code> parameter.</p>
    async fn list_workteams(
        &self,
        input: ListWorkteamsRequest,
    ) -> Result<ListWorkteamsResponse, RusotoError<ListWorkteamsError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.ListWorkteams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListWorkteamsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorkteamsError::from_response(response))
        }
    }

    /// <p>Renders the UI template so that you can preview the worker's experience. </p>
    async fn render_ui_template(
        &self,
        input: RenderUiTemplateRequest,
    ) -> Result<RenderUiTemplateResponse, RusotoError<RenderUiTemplateError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.RenderUiTemplate");
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
                .deserialize::<RenderUiTemplateResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RenderUiTemplateError::from_response(response))
        }
    }

    /// <p>Finds Amazon SageMaker resources that match a search query. Matching resource objects are returned as a list of <code>SearchResult</code> objects in the response. You can sort the search results by any resource property in a ascending or descending order.</p> <p>You can query against the following value types: numeric, text, Boolean, and timestamp.</p>
    async fn search(
        &self,
        input: SearchRequest,
    ) -> Result<SearchResponse, RusotoError<SearchError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.Search");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchError::from_response(response))
        }
    }

    /// <p><p>Starts a previously stopped monitoring schedule.</p> <note> <p>New monitoring schedules are immediately started after creation.</p> </note></p>
    async fn start_monitoring_schedule(
        &self,
        input: StartMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<StartMonitoringScheduleError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StartMonitoringSchedule");
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
            Err(StartMonitoringScheduleError::from_response(response))
        }
    }

    /// <p>Launches an ML compute instance with the latest version of the libraries and attaches your ML storage volume. After configuring the notebook instance, Amazon SageMaker sets the notebook instance status to <code>InService</code>. A notebook instance's status must be <code>InService</code> before you can connect to your Jupyter notebook. </p>
    async fn start_notebook_instance(
        &self,
        input: StartNotebookInstanceInput,
    ) -> Result<(), RusotoError<StartNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StartNotebookInstance");
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
            Err(StartNotebookInstanceError::from_response(response))
        }
    }

    /// <p>A method for forcing the termination of a running job.</p>
    async fn stop_auto_ml_job(
        &self,
        input: StopAutoMLJobRequest,
    ) -> Result<(), RusotoError<StopAutoMLJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopAutoMLJob");
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
            Err(StopAutoMLJobError::from_response(response))
        }
    }

    /// <p>Stops a model compilation job.</p> <p> To stop a job, Amazon SageMaker sends the algorithm the SIGTERM signal. This gracefully shuts the job down. If the job hasn't stopped, it sends the SIGKILL signal.</p> <p>When it receives a <code>StopCompilationJob</code> request, Amazon SageMaker changes the <a>CompilationJobSummary$CompilationJobStatus</a> of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the <a>CompilationJobSummary$CompilationJobStatus</a> to <code>Stopped</code>. </p>
    async fn stop_compilation_job(
        &self,
        input: StopCompilationJobRequest,
    ) -> Result<(), RusotoError<StopCompilationJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopCompilationJob");
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
            Err(StopCompilationJobError::from_response(response))
        }
    }

    /// <p>Stops a running hyperparameter tuning job and all running training jobs that the tuning job launched.</p> <p>All model artifacts output from the training jobs are stored in Amazon Simple Storage Service (Amazon S3). All data that the training jobs write to Amazon CloudWatch Logs are still available in CloudWatch. After the tuning job moves to the <code>Stopped</code> state, it releases all reserved resources for the tuning job.</p>
    async fn stop_hyper_parameter_tuning_job(
        &self,
        input: StopHyperParameterTuningJobRequest,
    ) -> Result<(), RusotoError<StopHyperParameterTuningJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopHyperParameterTuningJob");
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
            Err(StopHyperParameterTuningJobError::from_response(response))
        }
    }

    /// <p>Stops a running labeling job. A job that is stopped cannot be restarted. Any results obtained before the job is stopped are placed in the Amazon S3 output bucket.</p>
    async fn stop_labeling_job(
        &self,
        input: StopLabelingJobRequest,
    ) -> Result<(), RusotoError<StopLabelingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopLabelingJob");
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
            Err(StopLabelingJobError::from_response(response))
        }
    }

    /// <p>Stops a previously started monitoring schedule.</p>
    async fn stop_monitoring_schedule(
        &self,
        input: StopMonitoringScheduleRequest,
    ) -> Result<(), RusotoError<StopMonitoringScheduleError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopMonitoringSchedule");
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
            Err(StopMonitoringScheduleError::from_response(response))
        }
    }

    /// <p>Terminates the ML compute instance. Before terminating the instance, Amazon SageMaker disconnects the ML storage volume from it. Amazon SageMaker preserves the ML storage volume. Amazon SageMaker stops charging you for the ML compute instance when you call <code>StopNotebookInstance</code>.</p> <p>To access data on the ML storage volume for a notebook instance that has been terminated, call the <code>StartNotebookInstance</code> API. <code>StartNotebookInstance</code> launches another ML compute instance, configures it, and attaches the preserved ML storage volume so you can continue your work. </p>
    async fn stop_notebook_instance(
        &self,
        input: StopNotebookInstanceInput,
    ) -> Result<(), RusotoError<StopNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopNotebookInstance");
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
            Err(StopNotebookInstanceError::from_response(response))
        }
    }

    /// <p>Stops a processing job.</p>
    async fn stop_processing_job(
        &self,
        input: StopProcessingJobRequest,
    ) -> Result<(), RusotoError<StopProcessingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopProcessingJob");
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
            Err(StopProcessingJobError::from_response(response))
        }
    }

    /// <p>Stops a training job. To stop a job, Amazon SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms might use this 120-second window to save the model artifacts, so the results of the training is not lost. </p> <p>When it receives a <code>StopTrainingJob</code> request, Amazon SageMaker changes the status of the job to <code>Stopping</code>. After Amazon SageMaker stops the job, it sets the status to <code>Stopped</code>.</p>
    async fn stop_training_job(
        &self,
        input: StopTrainingJobRequest,
    ) -> Result<(), RusotoError<StopTrainingJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTrainingJob");
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
            Err(StopTrainingJobError::from_response(response))
        }
    }

    /// <p>Stops a transform job.</p> <p>When Amazon SageMaker receives a <code>StopTransformJob</code> request, the status of the job changes to <code>Stopping</code>. After Amazon SageMaker stops the job, the status is set to <code>Stopped</code>. When you stop a transform job before it is completed, Amazon SageMaker doesn't store the job's output in Amazon S3.</p>
    async fn stop_transform_job(
        &self,
        input: StopTransformJobRequest,
    ) -> Result<(), RusotoError<StopTransformJobError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.StopTransformJob");
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
            Err(StopTransformJobError::from_response(response))
        }
    }

    /// <p>Updates the specified Git repository with the specified values.</p>
    async fn update_code_repository(
        &self,
        input: UpdateCodeRepositoryInput,
    ) -> Result<UpdateCodeRepositoryOutput, RusotoError<UpdateCodeRepositoryError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateCodeRepository");
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
                .deserialize::<UpdateCodeRepositoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCodeRepositoryError::from_response(response))
        }
    }

    /// <p>Updates a domain. Changes will impact all of the people in the domain.</p>
    async fn update_domain(
        &self,
        input: UpdateDomainRequest,
    ) -> Result<UpdateDomainResponse, RusotoError<UpdateDomainError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainError::from_response(response))
        }
    }

    /// <p><p>Deploys the new <code>EndpointConfig</code> specified in the request, switches to using newly created endpoint, and then deletes resources provisioned for the endpoint using the previous <code>EndpointConfig</code> (there is no availability loss). </p> <p>When Amazon SageMaker receives the request, it sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p> <note> <p>You must not delete an <code>EndpointConfig</code> in use by an endpoint that is live or while the <code>UpdateEndpoint</code> or <code>CreateEndpoint</code> operations are being performed on the endpoint. To update an endpoint, you must create a new <code>EndpointConfig</code>.</p> </note></p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointInput,
    ) -> Result<UpdateEndpointOutput, RusotoError<UpdateEndpointError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateEndpointOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEndpointError::from_response(response))
        }
    }

    /// <p>Updates variant weight of one or more variants associated with an existing endpoint, or capacity of one variant associated with an existing endpoint. When it receives the request, Amazon SageMaker sets the endpoint status to <code>Updating</code>. After updating the endpoint, it sets the status to <code>InService</code>. To check the status of an endpoint, use the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_DescribeEndpoint.html">DescribeEndpoint</a> API. </p>
    async fn update_endpoint_weights_and_capacities(
        &self,
        input: UpdateEndpointWeightsAndCapacitiesInput,
    ) -> Result<
        UpdateEndpointWeightsAndCapacitiesOutput,
        RusotoError<UpdateEndpointWeightsAndCapacitiesError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEndpointWeightsAndCapacitiesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEndpointWeightsAndCapacitiesError::from_response(
                response,
            ))
        }
    }

    /// <p>Adds, updates, or removes the description of an experiment. Updates the display name of an experiment.</p>
    async fn update_experiment(
        &self,
        input: UpdateExperimentRequest,
    ) -> Result<UpdateExperimentResponse, RusotoError<UpdateExperimentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateExperiment");
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
                .deserialize::<UpdateExperimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateExperimentError::from_response(response))
        }
    }

    /// <p>Updates a previously created schedule.</p>
    async fn update_monitoring_schedule(
        &self,
        input: UpdateMonitoringScheduleRequest,
    ) -> Result<UpdateMonitoringScheduleResponse, RusotoError<UpdateMonitoringScheduleError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateMonitoringSchedule");
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
                .deserialize::<UpdateMonitoringScheduleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMonitoringScheduleError::from_response(response))
        }
    }

    /// <p>Updates a notebook instance. NotebookInstance updates include upgrading or downgrading the ML compute instance used for your notebook instance to accommodate changes in your workload requirements.</p>
    async fn update_notebook_instance(
        &self,
        input: UpdateNotebookInstanceInput,
    ) -> Result<UpdateNotebookInstanceOutput, RusotoError<UpdateNotebookInstanceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateNotebookInstance");
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
                .deserialize::<UpdateNotebookInstanceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNotebookInstanceError::from_response(response))
        }
    }

    /// <p>Updates a notebook instance lifecycle configuration created with the <a>CreateNotebookInstanceLifecycleConfig</a> API.</p>
    async fn update_notebook_instance_lifecycle_config(
        &self,
        input: UpdateNotebookInstanceLifecycleConfigInput,
    ) -> Result<
        UpdateNotebookInstanceLifecycleConfigOutput,
        RusotoError<UpdateNotebookInstanceLifecycleConfigError>,
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNotebookInstanceLifecycleConfigOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNotebookInstanceLifecycleConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Updates the display name of a trial.</p>
    async fn update_trial(
        &self,
        input: UpdateTrialRequest,
    ) -> Result<UpdateTrialResponse, RusotoError<UpdateTrialError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateTrial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateTrialResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTrialError::from_response(response))
        }
    }

    /// <p>Updates one or more properties of a trial component.</p>
    async fn update_trial_component(
        &self,
        input: UpdateTrialComponentRequest,
    ) -> Result<UpdateTrialComponentResponse, RusotoError<UpdateTrialComponentError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateTrialComponent");
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
                .deserialize::<UpdateTrialComponentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTrialComponentError::from_response(response))
        }
    }

    /// <p>Updates a user profile.</p>
    async fn update_user_profile(
        &self,
        input: UpdateUserProfileRequest,
    ) -> Result<UpdateUserProfileResponse, RusotoError<UpdateUserProfileError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateUserProfile");
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
                .deserialize::<UpdateUserProfileResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserProfileError::from_response(response))
        }
    }

    /// <p><p>Restricts access to tasks assigned to workers in the specified workforce to those within specific ranges of IP addresses. You specify allowed IP addresses by creating a list of up to four <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">CIDRs</a>.</p> <p>By default, a workforce isn&#39;t restricted to specific IP addresses. If you specify a range of IP addresses, workers who attempt to access tasks using any IP address outside the specified range are denied access and get a <code>Not Found</code> error message on the worker portal. After restricting access with this operation, you can see the allowed IP values for a private workforce with the operation.</p> <important> <p>This operation applies only to private workforces.</p> </important></p>
    async fn update_workforce(
        &self,
        input: UpdateWorkforceRequest,
    ) -> Result<UpdateWorkforceResponse, RusotoError<UpdateWorkforceError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateWorkforce");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateWorkforceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWorkforceError::from_response(response))
        }
    }

    /// <p>Updates an existing work team with new member definitions or description.</p>
    async fn update_workteam(
        &self,
        input: UpdateWorkteamRequest,
    ) -> Result<UpdateWorkteamResponse, RusotoError<UpdateWorkteamError>> {
        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, "/");
        request.set_endpoint_prefix("api.sagemaker".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "SageMaker.UpdateWorkteam");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateWorkteamResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWorkteamError::from_response(response))
        }
    }
}
